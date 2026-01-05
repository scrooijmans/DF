# Metabase Query Execution Flow: Caching, Parameters, and Visualization

## Overview

This document explains in detail how Metabase executes queries when a question or dashboard is viewed, including the caching mechanisms, parameter substitution process, and how execution results flow into visualization components.

## Query Execution Architecture

### High-Level Flow

When a user views a question or dashboard in Metabase, the following process occurs:

1. **Request Received** - API endpoint receives query request
2. **Card/Dashboard Loaded** - Entity retrieved with permissions check
3. **Parameter Resolution** - Dashboard parameters mapped to card parameters
4. **Query Construction** - Query built from card's `dataset_query` with parameters
5. **Cache Check** - Query hash checked against cache
6. **Query Execution** - Query runs against database (if cache miss)
7. **Result Processing** - Results formatted and metadata added
8. **Response** - Results returned to frontend for visualization

## Question (Card) Query Execution

### API Endpoint

When viewing a question, the frontend calls:

```clojure
POST /api/card/:card-id/query
```

This endpoint processes the query:

```clojure
(api.macros/defendpoint :post "/:card-id/query"
  "Run the query associated with a Card."
  [{:keys [card-id]} :- [:map [:card-id ms/PositiveInt]]
   _query-params {:keys [parameters ignore_cache dashboard_id collection_preview]}]
  (qp.card/process-query-for-card
    card-id :api
    :parameters   parameters
    :ignore-cache ignore_cache
    :dashboard-id dashboard_id
    :context      (if collection_preview :collection :question)
    :middleware   {:process-viz-settings? false}))
```

### Process Query for Card

The `process-query-for-card` function orchestrates the execution:

```clojure
(mu/defn process-query-for-card
  [card-id export-format
   & {:keys [parameters constraints context dashboard-id dashcard-id 
             middleware qp make-run ignore-cache]}]
  ;; 1. Load card with permissions check
  (let [card (api/read-check 
               (t2/select-one [:model/Card 
                               :id :name :dataset_query :database_id 
                               :collection_id :type :result_metadata 
                               :visualization_settings :display
                               :cache_invalidated_at :entity_id :created_at]
                              :id card-id))
        
        ;; 2. Load dashboard visualization settings if in dashboard context
        dash-viz (when (and (not= context :question) dashcard-id)
                   (t2/select-one-fn :visualization_settings 
                                    :model/DashboardCard :id dashcard-id))
        card-viz (:visualization_settings card)
        merged-viz (m/deep-merge card-viz dash-viz)
        
        ;; 3. Select query processor (pivot vs normal)
        qp (if (= :pivot (:display card))
             qp.pivot/run-pivot-query
             (or qp process-query-for-card-default-qp))
        
        ;; 4. Create runner function
        runner (make-run qp export-format)
        
        ;; 5. Build query from card with parameters
        query (-> (query-for-card card parameters constraints middleware 
                                   {:dashboard-id dashboard-id})
                  (assoc :viz-settings merged-viz)
                  (update :middleware (fn [middleware]
                                        (merge
                                         {:js-int-to-string? true
                                          :ignore-cached-results? ignore-cache}
                                         middleware))))
        
        ;; 6. Build execution info
        info {:executed-by            api/*current-user-id*
              :context                context
              :card-id                card-id
              :card-name              (:name card)
              :dashboard-id           dashboard-id
              :visualization-settings  merged-viz}]
    
    ;; 7. Validate parameters
    (when (seq parameters)
      (validate-card-parameters card-id 
        (mbql.normalize/normalize-fragment [:parameters] parameters)))
    
    ;; 8. Execute query
    (binding [qp.perms/*card-id* card-id]
      (runner query info))))
```

### Query Construction

The `query-for-card` function builds the executable query:

1. **Extract `dataset_query`** - Gets the query definition from the card
2. **Apply Parameters** - Substitutes parameter values into the query
3. **Normalize** - Converts to normalized MBQL or compiles native SQL
4. **Add Constraints** - Applies query constraints (timeouts, row limits)
5. **Add Metadata** - Includes card metadata and permissions context

## Dashboard Query Execution

### API Endpoint

When viewing a dashboard, each card is queried via:

```clojure
POST /api/dashboard/:dashboard-id/dashcard/:dashcard-id/card/:card-id/query
```

This endpoint processes queries in dashboard context:

```clojure
(api.macros/defendpoint :post "/:dashboard-id/dashcard/:dashcard-id/card/:card-id/query"
  "Run the query associated with a Saved Question (`Card`) in the context 
   of a `Dashboard` that includes it."
  [{:keys [dashboard-id dashcard-id card-id]}]
  _query-params {:keys [dashboard_load_id], :as body}
  (with-dashboard-load-id dashboard_load_id
    (u/prog1 (m/mapply qp.dashboard/process-query-for-dashcard
                       (merge body
                              {:dashboard-id dashboard-id
                               :card-id      card-id
                               :dashcard-id  dashcard-id}))
      (events/publish-event! :event/card-read 
        {:object-id card-id
         :user-id api/*current-user-id*
         :context :dashboard}))))
```

### Process Query for Dashboard Card

The `process-query-for-dashcard` function handles dashboard-specific logic:

1. **Resolve Dashboard Parameters** - Maps dashboard filter values to card parameters
2. **Merge Visualization Settings** - Combines card and dashboard card viz settings
3. **Apply Parameter Mappings** - Uses `parameter_mappings` from dashboard card
4. **Execute Query** - Runs query with dashboard context

## Parameter Substitution

### Parameter Types

Metabase supports several parameter types:

1. **Template Tags** - Native SQL parameters (`{{variable}}`)
2. **MBQL Parameters** - Structured query parameters
3. **Dashboard Parameters** - Filter widgets that map to card parameters

### Native SQL Parameter Substitution

For native SQL queries, Metabase uses template tags:

```sql
SELECT * FROM products
WHERE category = {{category_filter}}
```

The substitution process:

#### 1. Extract Template Tags

```clojure
(mu/defn extract-template-tags
  "Extract template tags from native query text.
   Supports:
   - Variables: {{foo}}
   - Snippets: {{snippet:name}}
   - Card references: {{#123}} or {{#123-card-title-slug}}"
  [query-text existing-tags]
  ;; Parses query text to find {{...}} patterns
  ;; Returns map of tag-name -> tag-definition
)
```

#### 2. Replace Tags with Values

```clojure
(defn replace-tags
  "Processes native query to replace template tags with values."
  [query]
  (if-let [name->tag (seq (get-in query [:native :template-tags]))]
    ;; Apply default values to tags
    (qp.compile/compile 
      (assoc-in query [:native :template-tags] 
                (update-vals name->tag tag-default)))
    (:native query)))
```

#### 3. Type-Specific Substitution

Different tag types are handled differently:

- **Variable Tags** - Direct value substitution
  ```sql
  {{category}} → 'Gizmo'
  ```

- **Dimension Tags** - Field filter substitution
  ```sql
  {{category_filter}} → category = 'Gizmo'
  ```

- **Snippet Tags** - SQL snippet substitution
  ```sql
  {{snippet:my-snippet}} → (SELECT ... FROM ...)
  ```

- **Card Tags** - Saved question substitution
  ```sql
  {{#123-gizmo-orders}} → (SELECT ... FROM ...)
  ```

### Dashboard Parameter Mapping

Dashboard parameters are resolved through a multi-step process:

#### 1. Resolve Dashboard Parameters

```clojure
(mu/defn- resolve-param-for-card
  [card-id dashcard-id param-id->param request-param]
  "Resolves a dashboard parameter for a specific card."
  ;; 1. Find matching dashboard parameter
  (let [matching-param (get param-id->param (:id request-param))]
    
    ;; 2. Find parameter mapping for this card
    (when-let [matching-mapping 
               (some (fn [mapping]
                       (when (and (= (:card_id mapping) card-id)
                                  (= (get-in mapping [:dashcard :id]) dashcard-id))
                         mapping))
                     (:mappings matching-param))]
      
      ;; 3. Validate parameter type
      (when (:type request-param)
        (check-allowed-parameter-value-type ...))
      
      ;; 4. Return merged parameter info
      (merge {:type (:type matching-param)}
             request-param
             {:id     (:id request-param)
              :target (:target matching-mapping)}))))
```

#### 2. Map to Card Parameters

Dashboard parameters map to card parameters via `parameter_mappings`:

```clojure
;; Dashboard card has:
:parameter_mappings [
  {:parameter_id "dashboard-param-1"
   :target [:dimension [:template-tag "category_filter"]]
   :card_id 123}]

;; When dashboard parameter "dashboard-param-1" = "Gizmo"
;; Maps to card's "category_filter" template tag
```

#### 3. Apply to Query

The resolved parameters are applied to the card's query:

```clojure
(defn- dashboard-param-defaults
  "Constructs default parameter values for a card from dashboard."
  [dashboard-param-id->param card-id]
  (into {}
    (comp (filter (fn [[_ {:keys [default]}]]
                    default))
          (map (fn [[param-id {:keys [default mappings]}]]
                 [param-id {:id      param-id
                            :default default
                            :target  (some (fn [{mapping-card-id :card_id
                                                  :keys [target]}]
                                             (when (= mapping-card-id card-id)
                                               target))
                                           mappings)}]))
          (filter (fn [[_ {:keys [target]}]]
                    target)))
    dashboard-param-id->param))
```

### Parameter Resolution Flow

1. **Dashboard Parameters** - User selects filter values
2. **Parameter Mappings** - Dashboard card's `parameter_mappings` link dashboard params to card params
3. **Card Parameters** - Card's template tags or MBQL parameters receive values
4. **Query Substitution** - Values substituted into query
5. **Query Execution** - Modified query executed

## Caching System

### Cache Architecture

Metabase uses a multi-level caching system:

1. **Query Hash** - Unique identifier for query + parameters
2. **Cache Strategy** - Determines if/how to cache
3. **Cache Backend** - Database-backed cache storage
4. **Cache Middleware** - Intercepts queries to check/save cache

### Cache Eligibility

A query is cacheable if:

```clojure
(defn- is-cacheable?
  [{:keys [cache-strategy]}]
  (and (public-settings/enable-query-caching)  ; Caching enabled globally
       (some? cache-strategy)                   ; Has cache strategy
       (not= (:type cache-strategy) :nocache))) ; Not explicitly disabled
```

**Additional Requirements:**
- Query must be permissions-checked (cache bypasses permission middleware)
- Result rows must be < `query-caching-max-kb` when serialized
- Query must have valid `cache-strategy` configuration

### Cache Strategy Hierarchy

Cache strategies are resolved in order of specificity:

```clojure
(defenterprise-schema cache-strategy
  "Returns the granular cache strategy for a card."
  [card dashboard-id]
  (let [qs (for [[i model model-id] 
                 [[1 "question"   (:id card)]
                  [2 "dashboard"  dashboard-id]
                  [3 "database"   (:database_id card)]
                  [4 "root"       0]]
             :when model-id]
            {:from   [:cache_config]
             :select [:id [[:inline i] :ordering]]
             :where  [:and
                      [:= :model [:inline model]]
                      [:= :model_id model-id]]})
        q {:from     [[{:union-all qs} :unused_alias]]
           :select   [:id]
           :order-by :ordering
           :limit    [:inline 1]}
        item (t2/select-one :model/CacheConfig :id q)]
    (cache-config/card-strategy item card)))
```

**Priority Order:**
1. **Question-level** - Specific cache config for this card
2. **Dashboard-level** - Cache config for the dashboard
3. **Database-level** - Cache config for the database
4. **Root-level** - Global default cache config

### Cache Middleware

The cache middleware intercepts query execution:

```clojure
(defn maybe-return-cached-results
  [qp]
  (fn maybe-return-cached-results* [query rff]
    (let [cacheable? (is-cacheable? query)]
      (if cacheable?
        (run-query-with-cache qp query rff)
        (qp query rff)))))
```

### Cache Lookup

When a query is executed, the cache is checked:

```clojure
(mu/defn- run-query-with-cache
  [qp query rff]
  ;; 1. Calculate query hash (includes parameters)
  (let [query-hash (qp.util/query-hash query)
        
        ;; 2. Attempt to retrieve from cache
        [status result] (maybe-reduce-cached-results 
                          (:ignore-cached-results? middleware) 
                          query-hash 
                          cache-strategy 
                          rff)]
    (case status
      ::ok       result  ; Cache hit - return cached results
      ::canceled ::canceled
      ::miss     (let [start-time-ns (System/nanoTime)]
                   ;; Cache miss - execute query and save results
                   (binding [qp.pipeline/*reduce* 
                             (fn reduce' [rff metadata rows]
                               (impl/do-with-serialization
                                 (fn [in-fn result-fn]
                                   (binding [*in-fn*     in-fn
                                             *result-fn* result-fn]
                                     (orig-reduce rff metadata rows)))))]
                     (qp query
                         (fn [metadata]
                           (save-results-xform start-time-ns metadata 
                                                query-hash cache-strategy 
                                                (rff metadata)))))))))
```

### Cache Storage

Cached results are stored in the `query_cache` table:

```clojure
(defn- save-results!
  [^bytes query-hash ^bytes results]
  "Saves query results to cache."
  (let [final-results (encryption/maybe-encrypt-for-stream results)
        timestamp     (t/offset-date-time)]
    (try
      ;; Try to update existing cache entry
      (or (pos? (t2/update! :model/QueryCache 
                            {:query_hash query-hash}
                            {:updated_at timestamp
                             :results    final-results}))
          ;; Or insert new cache entry
          (first (t2/insert-returning-instances! :model/QueryCache
                                                  :updated_at timestamp
                                                  :query_hash query-hash
                                                  :results final-results)))
      (catch Throwable e
        (log/error e "Error saving query results to cache.")))))
```

**Cache Entry Structure:**
- `query_hash` - Unique hash of query + parameters
- `results` - Serialized query results (optionally encrypted)
- `updated_at` - Timestamp of cache creation/update

### Cache Retrieval

Cached results are retrieved using prepared statements:

```clojure
(defn- cached-results
  [query-hash strategy respond]
  "Retrieves cached results for a query hash."
  (t2/with-connection [conn]
    (when-let [stmt (fetch-cache-stmt strategy query-hash conn)]
      (with-open [stmt ^PreparedStatement stmt
                  rs   (.executeQuery stmt)]
        (if-not (.next rs)
          (respond nil)  ; Cache miss
          (with-open [is (encryption/maybe-decrypt-stream 
                          (.getBinaryStream rs 1))]
            (respond is)))))))  ; Cache hit - return stream
```

### Cache TTL (Time To Live)

Cache entries expire based on:

1. **Card-level TTL** - `cache_ttl` field on card (in seconds)
2. **Strategy TTL** - TTL from cache strategy configuration
3. **Default TTL** - System default (if no specific TTL set)

When cache expires:
- Entry remains in database but is ignored
- Query executes fresh
- New results cached with updated timestamp

### Cache Invalidation

Caches are invalidated when:

1. **Card Updated** - `cache_invalidated_at` timestamp updated
2. **Manual Invalidation** - User clicks "Refresh" or sets `ignore_cache=true`
3. **Data Model Changes** - Underlying table/schema changes
4. **Scheduled Refresh** - Enterprise feature for scheduled cache refresh

## Query Processing Pipeline

### Middleware Stack

The query processor uses a middleware pipeline:

```clojure
(defn apply-middleware
  [qp middleware-fns]
  "Applies middleware functions to query processor."
  (reduce
   (fn [qp middleware]
     (if middleware
       (middleware qp)
       qp))
   qp
   middleware-fns))
```

**Key Middleware:**
1. **Cache Middleware** - Checks/saves cache
2. **Permissions Middleware** - Validates data access
3. **Parameter Substitution** - Replaces parameters
4. **Visualization Settings** - Applies formatting
5. **Result Formatting** - Formats output

### Query Execution

The query processor executes the query:

1. **Normalize Query** - Converts to standard form
2. **Compile Query** - Converts MBQL to native SQL (or uses native SQL directly)
3. **Execute Query** - Runs against database
4. **Process Results** - Formats and enriches results
5. **Return Results** - Streams results to response

### Result Processing

Results are processed through multiple stages:

#### 1. Metadata Addition

```clojure
(defn update-viz-settings
  "Processes visualization settings into query metadata."
  [query rff]
  (if process-viz-settings?
    (let [card-viz-settings (viz-settings query)
          normalized-card-viz-settings (mb.viz/db->norm card-viz-settings)
          column-viz-settings (::mb.viz/column-settings card-viz-settings)
          ;; ... process settings ...
          updated-card-viz-settings (-> normalized-card-viz-settings
                                         (assoc ::mb.viz/column-settings ...)
                                         (assoc ::mb.viz/global-column-settings ...))]
      (fn update-viz-settings-rff* [metadata]
        (rff (assoc metadata :viz-settings updated-card-viz-settings))))
    rff))
```

#### 2. Result Formatting

Results are formatted based on:
- **Column Settings** - Per-column formatting rules
- **Global Settings** - System-wide formatting defaults
- **Visualization Type** - Chart-specific formatting
- **Export Format** - CSV, JSON, XLSX formatting

#### 3. Response Construction

```clojure
(defn- success-response
  [{query-hash :hash, :as query-execution} 
   {cache :cache/details :as result}]
  (merge
   (-> query-execution
       add-running-time
       (dissoc :error :hash :executor_id ...))
   (dissoc result :cache/details)
   {:cached                 (when (:cached cache) (:updated_at cache))
    :status                 :completed
    :average_execution_time (when (:cached cache)
                              (query/average-execution-time-ms query-hash))}))
```

## Result Flow to Visualizations

### Response Structure

Query results are returned in a standardized format:

```json
{
  "data": {
    "rows": [[...], [...]],
    "cols": [
      {"name": "column1", "display_name": "Column 1", ...},
      {"name": "column2", "display_name": "Column 2", ...}
    ]
  },
  "json_query": {...},
  "status": "completed",
  "cached": "2024-01-15T10:30:00Z",
  "average_execution_time": 123
}
```

### Frontend Processing

The frontend receives results and processes them:

#### 1. Result Parsing

```javascript
// Frontend receives response
const response = await fetch('/api/card/123/query', {
  method: 'POST',
  body: JSON.stringify({ parameters: [...] })
});

const result = await response.json();
```

#### 2. Data Transformation

Results are transformed for visualization:

```javascript
// Extract rows and columns
const { rows, cols } = result.data;

// Apply column formatting
const formattedRows = rows.map(row => 
  row.map((value, index) => 
    formatValue(value, cols[index])
  )
);

// Group by dimensions/metrics
const dimensions = cols.filter(col => 
  isDimension(col, visualizationSettings)
);
const metrics = cols.filter(col => 
  isMetric(col, visualizationSettings)
);
```

#### 3. Visualization Rendering

Different visualization types process data differently:

**Table Visualization:**
- Direct row/column mapping
- Column settings applied
- Sorting/filtering on frontend

**Chart Visualizations (Bar, Line, Area):**
```javascript
// Transform to chart format
const chartData = {
  dimensions: dimensions.map(d => d.name),
  metrics: metrics.map(m => m.name),
  series: rows.map(row => ({
    x: getDimensionValue(row, dimensions[0]),
    y: getMetricValue(row, metrics[0])
  }))
};

// Render with charting library
renderChart(chartData, visualizationSettings);
```

**Scalar Visualization:**
```javascript
// Single value display
const value = rows[0][0];
renderScalar(formatValue(value, cols[0]));
```

### Visualization Settings Application

Visualization settings from both card and dashboard are merged:

```clojure
;; Backend merges settings
(let [card-viz   (:visualization_settings card)
      dash-viz   (when dashcard-id
                   (t2/select-one-fn :visualization_settings 
                                    :model/DashboardCard :id dashcard-id))
      merged-viz (m/deep-merge card-viz dash-viz)]
  ;; merged-viz used in query processing
)
```

**Settings Include:**
- `graph.dimensions` - Which columns are dimensions
- `graph.metrics` - Which columns are metrics
- `graph.series_labels` - Custom series labels
- `column_settings` - Per-column formatting
- `graph.x_axis.title_text` - X-axis title
- `graph.y_axis.title_text` - Y-axis title

### Real-Time Updates

For dashboards with auto-refresh:

1. **Polling** - Frontend polls query endpoint at interval
2. **Cache Check** - Each poll checks cache first
3. **Incremental Updates** - Only updates if results changed
4. **Visualization Refresh** - Chart re-renders with new data

## Performance Optimizations

### Query Batching

Dashboard queries can be batched:

```clojure
;; Dashboard load ID groups related queries
(with-dashboard-load-id dashboard_load_id
  ;; All queries in this context share metadata provider cache
  ;; Reduces database queries for metadata
)
```

### Metadata Provider Caching

Metadata (tables, fields, etc.) is cached per dashboard load:

```clojure
;; Metadata provider created once per dashboard load
(let [metadata-provider (lib.metadata.jvm/application-database-metadata-provider 
                          database-id)]
  ;; Reused for all cards in dashboard
)
```

### Result Streaming

Large results are streamed to avoid memory issues:

```clojure
(defn process-query-for-card-default-run-fn
  [qp export-format]
  (fn [query info]
    (qp.streaming/streaming-response 
      [rff export-format (u/slugify (:card-name info))]
      (qp (update query :info merge info) rff))))
```

### Parallel Execution

Dashboard cards can execute in parallel:

```javascript
// Frontend can request multiple cards simultaneously
const promises = dashboard.cards.map(card =>
  fetch(`/api/dashboard/${dashboardId}/dashcard/${card.id}/card/${card.card_id}/query`, {
    method: 'POST',
    body: JSON.stringify({ parameters: dashboardParams })
  })
);

const results = await Promise.all(promises);
```

## Summary

### Execution Flow

1. **Request** → API endpoint receives query request
2. **Load Entity** → Card/dashboard loaded with permissions check
3. **Resolve Parameters** → Dashboard parameters mapped to card parameters
4. **Build Query** → Query constructed from `dataset_query` + parameters
5. **Check Cache** → Query hash checked against cache
6. **Execute** → Query runs against database (if cache miss)
7. **Process Results** → Results formatted with metadata
8. **Return** → Results streamed to frontend
9. **Visualize** → Frontend renders visualization

### Key Components

- **Query Processor** - Executes queries with middleware pipeline
- **Cache System** - Multi-level caching with TTL and invalidation
- **Parameter Resolution** - Maps dashboard filters to card parameters
- **Result Processing** - Formats and enriches results
- **Visualization Pipeline** - Transforms results for rendering

### Performance Features

- **Caching** - Reduces database load for repeated queries
- **Streaming** - Handles large result sets efficiently
- **Batching** - Groups related queries for metadata caching
- **Parallel Execution** - Dashboard cards can load simultaneously
- **Incremental Updates** - Only refreshes changed data

This architecture ensures efficient query execution, proper parameter handling, and seamless integration between query results and visualization components.


