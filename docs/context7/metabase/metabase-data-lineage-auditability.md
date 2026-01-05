# Metabase Data Lineage and Auditability: Tracing Chart Data Sources

## Overview

Metabase provides comprehensive mechanisms for tracing where chart data comes from, including source tables, datasets, SQL queries, and nested card references. This document explains how lineage is represented internally and how auditability is maintained through query execution tracking.

## Source Tracking Architecture

### Multi-Level Source Identification

Metabase tracks data sources at multiple levels:

1. **Database Level** - The database connection
2. **Table Level** - Physical tables or views
3. **Card Level** - Saved questions/models used as sources
4. **Query Level** - The actual query definition (MBQL or SQL)
5. **Execution Level** - Individual query executions with metadata

### Source Information Storage

Source information is stored in several places:

#### Card Level (`report_card` table)
```clojure
{:id 123
 :name "Sales by Category"
 :database_id 1                    ; Source database
 :table_id 10                      ; Source table (if MBQL)
 :dataset_query {...}              ; Query definition
 :result_metadata [...]            ; Column metadata
 :source_card_id 456               ; If derived from another card
 :type :question}                  ; :question, :model, or :metric
```

#### Query Execution Level (`query_execution` table)
```clojure
{:id 789
 :card_id 123                      ; Associated card
 :dashboard_id 456                 ; If from dashboard
 :database_id 1                    ; Database executed against
 :started_at "2024-01-15T10:30:00Z"
 :finished_at "2024-01-15T10:30:01Z"
 :running_time 1234                ; Milliseconds
 :result_rows 100
 :native_query_json {...}          ; Actual SQL executed
 :json_query {...}                 ; MBQL query (if applicable)
 :error nil
 :context :question}               ; Execution context
```

## Source Table Extraction

### MBQL Query Source Tables

For MBQL queries, source tables are extracted from the query structure:

```clojure
(mu/defn query->source-table-ids
  "Returns all source table IDs referenced in a query."
  [query]
  (when (seq query)
    (:table-ids (query->source-ids query))))
```

The `query->source-ids` function extracts comprehensive source information:

```clojure
(mu/defn query->source-ids
  "Returns table IDs, card IDs, and native query flag."
  [query]
  (apply merge-with merge-source-ids
         (lib.util.match/match query
           ;; Native query
           (m :guard (every-pred map? :native))
           (if-let [source-card-id (:qp/stage-is-from-source-card m)]
             {:card-ids #{source-card-id}}
             {:native? true})
           
           ;; MBQL query with source table
           (m :guard (every-pred map? #(pos-int? (:source-table %))))
           (merge-with merge-source-ids
                       {:table-ids #{(:source-table m)}}
                       ;; Include source card if present
                       (when-let [source-card-id (:qp/stage-is-from-source-card m)]
                         {:card-ids #{source-card-id}})
                       ;; Recursively process nested queries
                       (query->source-ids (dissoc m :source-table))))))
```

### Source Table Resolution

Source tables are resolved during query processing:

```clojure
(defn resolve-source-tables
  "Middleware to resolve source tables in query."
  [query]
  ;; Extract all source table IDs
  (let [source-table-ids (query->source-table-ids query)]
    ;; Fetch table metadata and store in QP store
    (lib.metadata/bulk-metadata-or-throw 
      query 
      :metadata/table 
      source-table-ids)
    query))
```

### Recursive Source Table Discovery

For nested queries, source tables are discovered recursively:

```clojure
(defn- find-source-table
  "Recursively finds primary source table ID."
  [{:keys [source-table source-query]}]
  (or source-table
      (when source-query
        (recur source-query))))
```

## Card References and Dependencies

### Card as Source

Cards can be used as data sources in queries:

#### 1. Legacy Card Reference (String Format)
```clojure
{:source-table "card__123"}  ; Card ID 123
```

#### 2. MLv2 Source Card Reference
```clojure
{:source-card 123}  ; Direct card ID reference
```

#### 3. Native Query Template Tag Reference
```sql
WITH gizmo_orders AS {{#123-gizmo-orders}}
SELECT * FROM gizmo_orders
```

### Collecting Card Dependencies

Metabase collects all card IDs referenced in a query:

```clojure
(defn collect-card-ids
  "Traverses query to collect all referenced card IDs."
  [query]
  (let [ids (java.util.HashSet.)
        walker (fn [form]
                 (when (map? form)
                   ;; Model references in native queries
                   (when-let [card-id (:card-id form)]
                     (when (int? card-id)
                       (.add ids card-id)))
                   ;; MLv2 source-card
                   (when-let [card-id (:source-card form)]
                     (.add ids card-id))
                   ;; Legacy MBQL card__<id> source-table
                   (when-let [card-id (parse-source-query-id (:source-table form))]
                     (.add ids card-id)))
                 form)]
    (walk/prewalk walker query)
    (seq ids)))
```

### Card Dependency Graph

For nested queries, Metabase builds a dependency graph:

```clojure
(mu/defn- card-subquery-graph
  "Recursively builds dependency graph of sub-queries."
  [metadata-providerable graph card-id]
  (let [card-query (->> (lib.metadata/card metadata-providerable card-id)
                        (qp.fetch-source-query/normalize-card-query metadata-providerable)
                        :dataset-query)]
    (reduce
     (fn [g sub-card-id]
       (card-subquery-graph metadata-providerable
                            (dep/depend g card-id sub-card-id)
                            sub-card-id))
     graph
     (lib/template-tag-card-ids card-query))))
```

**Example Dependency Graph:**
```
Card 100 (Sales Dashboard)
  └─ Card 200 (Sales by Region)
      └─ Card 300 (Customer Model)
          └─ Table: customers
```

### Template Tag Card References

Native queries can reference cards via template tags:

```clojure
(mu/defn template-tag-card-ids
  "Extracts card IDs from template tags."
  [query]
  (not-empty 
    (into #{} 
          (keep (fn [[_k m]] (:card-id m))) 
          (template-tags query))))
```

**Example:**
```sql
-- Template tag references Card 123
WITH orders AS {{#123-recent-orders}}
SELECT * FROM orders
```

The template tag structure:
```clojure
{:template-tags
 {"orders" {:type :card
            :card-id 123
            :display-name "Recent Orders"}}}
```

## Query Metadata and Lineage

### Query Metadata Endpoint

Metabase provides endpoints to fetch comprehensive query metadata:

```clojure
GET /api/card/:id/query_metadata
GET /api/dashboard/:id/query_metadata
```

These endpoints return:
- **Database** - Source database information
- **Tables** - All referenced tables (including virtual tables from cards)
- **Fields** - All referenced fields
- **Cards** - All referenced cards (as virtual tables)

### Batch Metadata Fetching

For dashboards, metadata is fetched in batches:

```clojure
(defn batch-fetch-card-metadata
  "Fetches dependent metadata for cards."
  [cards]
  (let [queries (into (vec (keep :dataset_query cards))
                     ;; Add card-level metadata for models/native queries
                     (comp (filter (fn [card] 
                                     (or (= :model (:type card))
                                         (= :native (-> card :dataset_query :type)))))
                           (map (fn [card] 
                                  {:query {:source-table (str "card__" (u/the-id card))}})))
                     cards)]
    (batch-fetch-query-metadata queries)))
```

### Dependent Metadata

The system identifies all dependent entities:

```clojure
(mu/defn dependent-metadata
  "Returns IDs and types of entities required for frontend."
  [query card-id card-type]
  (into []
        (distinct)
        (concat
         (query-dependents query query)
         (when (and (some? card-id)
                    (#{:model :metric} card-type))
           (cons {:type :table, :id (str "card__" card-id)}
                 (when-let [card (lib.metadata/card query card-id)]
                   (query-dependents query (lib.query/query query card))))))))
```

**Dependent Types:**
- `:database` - Database connections
- `:schema` - Database schemas
- `:table` - Physical tables or virtual tables (cards)
- `:field` - Individual fields
- `:card` - Referenced cards

## Query Execution Tracking

### Query Execution Table

Every query execution is recorded in `query_execution`:

```clojure
{:id 789
 :card_id 123                      ; Card that was queried
 :dashboard_id 456                 ; Dashboard (if applicable)
 :pulse_id nil                     ; Pulse/alert (if applicable)
 :database_id 1                    ; Database executed against
 :started_at "2024-01-15T10:30:00Z"
 :finished_at "2024-01-15T10:30:01Z"
 :running_time 1234                ; Execution time in ms
 :result_rows 100                  ; Number of rows returned
 :result_size 50000                ; Result size in bytes
 :native_query_json {...}          ; Actual SQL executed
 :json_query {...}                 ; MBQL query (if applicable)
 :hash "abc123..."                 ; Query hash for caching
 :cache_hit false                  ; Whether cache was used
 :cache_hash nil                   ; Cache entry hash
 :error nil                         ; Error message (if failed)
 :error_class nil                  ; Error class (if failed)
 :error_stack nil                  ; Stack trace (if failed)
 :context :question                ; :question, :dashboard, :pulse
 :executor_id 42                   ; User who executed
 :executor_name "John Doe"
 :is_sandboxed false               ; Sandboxed execution
 :query_type :native               ; :native or :query
 :status :completed}               ; :completed, :failed, :timeout
```

### Execution Metadata Storage

Execution metadata is saved after query completion:

```clojure
(defn- save-successful-execution-metadata!
  "Saves execution metadata for successful queries."
  [cache-details is-sandboxed? query-execution result-rows pmbql]
  (let [qe-map (assoc query-execution
                       :cache_hit    (boolean (:cached cache-details))
                       :cache_hash   (:hash cache-details)
                       :result_rows  result-rows
                       :is_sandboxed (boolean is-sandboxed?))]
    (save-execution-metadata! qe-map pmbql)))
```

### Native Query Storage

For native SQL queries, the actual SQL is stored:

```clojure
;; Native query JSON structure
{:query "SELECT category, count(*) FROM products WHERE category = 'Gizmo' GROUP BY category"
 :template-tags {...}
 :collection_preview false}
```

This allows:
- **Query Review** - See exact SQL executed
- **Performance Analysis** - Analyze slow queries
- **Debugging** - Troubleshoot query issues
- **Audit Trail** - Complete record of what was executed

## Lineage Representation

### Card Dependencies

Cards track their dependencies through serialization:

```clojure
(defmethod serdes/dependencies "Card"
  [{:keys [collection_id database_id dataset_query parameters 
           parameter_mappings result_metadata table_id 
           source_card_id visualization_settings dashboard_id]}]
  (set
   (concat
    ;; Parameter dependencies
    (mapcat serdes/mbql-deps parameter_mappings)
    (serdes/parameters-deps parameters)
    
    ;; Database and table dependencies
    [[{:model "Database" :id database_id}]]
    (when table_id #{(serdes/table->path table_id)})
    
    ;; Source card dependency
    (when source_card_id 
      #{[{:model "Card" :id source_card_id}]})
    
    ;; Collection and dashboard dependencies
    (when collection_id 
      #{[{:model "Collection" :id collection_id}]})
    (when dashboard_id 
      #{[{:model "Dashboard" :id dashboard_id}]})
    
    ;; Query dependencies (extracted from MBQL)
    (serdes/mbql-deps dataset_query)
    
    ;; Visualization settings dependencies
    (serdes/visualization-settings-deps visualization_settings))))
```

### Descendants Tracking

Cards also track what depends on them:

```clojure
(defmethod serdes/descendants "Card"
  [_model-name id]
  (let [card (t2/select-one :model/Card :id id)
        source-table (some-> card :dataset_query :query :source-table)
        template-tags (some->> card 
                               :dataset_query 
                               :native 
                               :template-tags 
                               vals 
                               (keep :card-id))
        parameters-card-id (some->> card 
                                     :parameters 
                                     (keep (comp :card_id :values_source_config)))]
    (into {} 
          (concat
           ;; Cards using this as source-table
           (when (and (string? source-table)
                      (str/starts-with? source-table "card__"))
             {["Card" (parse-long (subs source-table 6))] {"Card" id}})
           
           ;; Cards referencing this in template tags
           (for [card-id template-tags]
             {["Card" card-id] {"Card" id}})
           
           ;; Parameters using this card for values
           (for [card-id parameters-card-id]
             {["Card" card-id] {"Card" id}})))))
```

### Source Query Resolution

When a card is used as a source, its query is resolved:

```clojure
(mu/defn card-id->source-query-and-metadata
  "Returns source query info for a card."
  ([card-id]
   (card-id->source-query-and-metadata card-id false))
  ([card-id log?]
   (let [card (or (lib.metadata/card (qp.store/metadata-provider) card-id)
                  (throw (ex-info (tru "Card {0} does not exist." card-id)
                                  {:card-id card-id})))
         persisted-info (:lib/persisted-info card)
         {{database-id :database} :dataset_query
          result-metadata         :result-metadata
          card-type               :type} card
         persisted?     (qp.persisted/can-substitute? card persisted-info)
         source-query   (source-query card)]
     {:source-query    (cond-> source-query
                         persisted?
                         (assoc :persisted-info/native
                                (qp.persisted/persisted-info-native-query
                                 (u/the-id (lib.metadata/database (qp.store/metadata-provider)))
                                 persisted-info)))
      :database        database-id
      :source-metadata (sequence (comp (map mbql.normalize/normalize-source-metadata)
                                       (remove :remapped_from))
                                 result-metadata)
      :source-query/model? (= card-type :model)})))
```

## Audit Trail

### Query Execution Audit

Every query execution creates an audit record:

```clojure
;; Query execution includes:
{
  :card_id 123                     ; What was queried
  :dashboard_id 456                ; Where it was queried
  :executor_id 42                  ; Who executed it
  :started_at "2024-01-15T10:30:00Z"
  :finished_at "2024-01-15T10:30:01Z"
  :running_time 1234
  :result_rows 100
  :native_query_json {...}         ; Exact SQL executed
  :json_query {...}                ; MBQL query
  :context :dashboard              ; Execution context
  :status :completed
}
```

### Audit Log (Enterprise)

Enterprise features include comprehensive audit logging:

```clojure
(mu/defn record-event!
  "Records an event in the Audit Log."
  [topic params]
  (when (premium-features/log-enabled?)
    (let [{:keys [user-id model-name model-id details]}
          (construct-event topic params api/*current-user-id*)]
      (t2/insert! :model/AuditLog
                  :topic    unqualified-topic
                  :details  details
                  :model    model-name
                  :model_id model-id
                  :user_id  user-id))))
```

**Audit Log Events:**
- Card creation/update/deletion
- Dashboard creation/update/deletion
- Query execution
- User actions
- Permission changes

### View Log

View events are tracked separately:

```clojure
(mu/defn ^:private record-views!
  "Records view events."
  [view-or-views]
  (when (premium-features/log-enabled?)
    (t2/insert! :model/ViewLog view-or-views)))
```

## Tracing Data Sources

### Source Table Identification

For MBQL queries, source tables are identified:

```clojure
(mu/defn- legacy-query->database-and-table-ids
  "Determines database and table IDs for legacy queries."
  [{database-id :database, query-type :type, 
    {:keys [source-table source-query]} :query}]
  (cond
    (= :native query-type)
    {:database-id database-id, :table-id nil}
    
    (integer? source-table)
    {:database-id database-id, :table-id source-table}
    
    (string? source-table)
    ;; Card reference: "card__123"
    (let [[_ card-id] (re-find #"^card__(\d+)$" source-table)]
      (t2/select-one [:model/Card 
                      [:table_id :table-id] 
                      [:database_id :database-id]]
                     :id (Integer/parseInt card-id)))
    
    (map? source-query)
    ;; Recursive for nested queries
    (legacy-query->database-and-table-ids 
      {:database database-id
       :type     query-type
       :query    source-query})))
```

### Source Card Resolution

When a card is used as a source, it's resolved:

```clojure
(mu/defn- resolve-database-id-for-source-card
  "Resolves database ID for source card."
  [source-card-id]
  (let [card (or (lib.metadata.protocols/card 
                   (bootstrap-metadata-provider) source-card-id)
                 (throw (ex-info (i18n/tru "Card {0} does not exist." source-card-id)
                                 {:card-id source-card-id})))]
    (:database-id card)))
```

### Virtual Tables

Cards appear as virtual tables in the query builder:

```clojure
(mu/defn- cards-virtual-tables
  "Converts cards to virtual table metadata."
  [card-type]
  (for [card (source-query-cards card-type)]
    (api.table/card->virtual-table card :include-fields? include-fields?)))
```

**Virtual Table Structure:**
```clojure
{:id "card__123"                    ; Virtual table ID
 :name "Sales by Region"            ; Card name
 :display_name "Sales by Region"
 :schema "Collection Name"           ; Collection name as schema
 :database_id 1
 :fields [...]                      ; Columns from result_metadata
 :entity_type :entity/GenericTable
 :db_id 1}
```

## User-Facing Lineage Information

### Query Metadata API

Users can fetch comprehensive metadata:

```clojure
GET /api/card/:id/query_metadata
```

**Response includes:**
```json
{
  "databases": [
    {
      "id": 1,
      "name": "Sample Database",
      "engine": "postgres"
    }
  ],
  "tables": [
    {
      "id": 10,
      "name": "products",
      "schema": "public",
      "database_id": 1
    },
    {
      "id": "card__123",
      "name": "Sales by Region",
      "schema": "Analytics",
      "database_id": 1,
      "entity_type": "entity/GenericTable"
    }
  ],
  "fields": [
    {
      "id": 100,
      "name": "category",
      "table_id": 10,
      "base_type": "type/Text"
    }
  ]
}
```

### Source Information in Card Metadata

Cards include source information:

```clojure
;; Card metadata includes:
{
  :id 123
  :name "Sales by Category"
  :database_id 1                    ; Source database
  :table_id 10                      ; Source table (if MBQL)
  :dataset_query {
    :type :query                    ; or :native
    :database 1
    :query {
      :source-table 10              ; or "card__123"
      :aggregation [[:count]]
      :breakout [[:field 100 nil]]
    }
  }
  :result_metadata [
    {
      :name "CATEGORY"
      :display_name "Category"
      :base_type "type/Text"
      :field_ref ["field" 100 nil]
    }
  ]
}
```

### Execution History

Query execution history provides audit trail:

```clojure
;; Query execution records show:
{
  :card_id 123                      ; Which card
  :dashboard_id 456                 ; Which dashboard (if applicable)
  :executor_id 42                   ; Who ran it
  :executor_name "John Doe"
  :started_at "2024-01-15T10:30:00Z"
  :finished_at "2024-01-15T10:30:01Z"
  :running_time 1234
  :result_rows 100
  :native_query_json {              ; Actual SQL
    :query "SELECT category, count(*) FROM products WHERE category = 'Gizmo' GROUP BY category"
  }
  :json_query {                     ; MBQL query
    :type :query
    :query {
      :source-table 10
      :filter [:= [:field 100 nil] "Gizmo"]
      :aggregation [[:count]]
      :breakout [[:field 100 nil]]
    }
  }
  :context :dashboard
  :status :completed
}
```

## Lineage Graph Construction

### Dependency Graph Building

Metabase builds dependency graphs for nested queries:

```clojure
(mu/defn- card-subquery-graph
  "Recursively builds dependency graph."
  [metadata-providerable graph card-id]
  (let [card-query (->> (lib.metadata/card metadata-providerable card-id)
                        (qp.fetch-source-query/normalize-card-query metadata-providerable)
                        :dataset-query)]
    (reduce
     (fn [g sub-card-id]
       (card-subquery-graph metadata-providerable
                            (dep/depend g card-id sub-card-id)
                            sub-card-id))
     graph
     (lib/template-tag-card-ids card-query))))
```

**Graph Structure:**
```clojure
{
  "Card 100" #{"Card 200" "Card 300"},
  "Card 200" #{"Card 300"},
  "Card 300" #{}  ; No dependencies
}
```

### Circular Dependency Detection

The system detects circular dependencies:

```clojure
;; When building dependency graph
(when (:qp/stage-is-from-source-card stage)
  (u/prog1 (vswap! dep-graph
                   dep/depend
                   (tru "Card {0}" (:qp/stage-is-from-source-card stage))
                   (tru "Card {0}" (:source-card stage)))
    ;; This will throw if there's a cycle
    (dep/topo-sort <>)))
```

## Source Information in Queries

### MBQL Query Sources

MBQL queries explicitly reference sources:

```clojure
{:type :query
 :database 1
 :query {
   :source-table 10                ; Direct table reference
   ;; OR
   :source-table "card__123"       ; Card reference
   ;; OR
   :source-query {                 ; Nested query
     :source-table 10
     :aggregation [[:count]]
   }
 }}
```

### Native Query Sources

Native queries reference sources via template tags:

```sql
-- Direct table reference
SELECT * FROM products

-- Card reference
WITH orders AS {{#123-recent-orders}}
SELECT * FROM orders

-- Multiple card references
SELECT * FROM {{#100-customers}} c
JOIN {{#200-orders}} o ON c.id = o.customer_id
```

### Source Card Metadata

When a card is used as a source, its metadata is available:

```clojure
(mu/defn- saved-question-metadata
  "Metadata for saved question used as source."
  [query stage-number card-id options]
  (when card-id
    (when-let [card (lib.metadata/card query card-id)]
      (not-empty (lib.metadata.calculation/visible-columns 
                   query stage-number card options)))))
```

## Auditability Features

### Query Execution Tracking

Every query execution is tracked with:

1. **Execution Metadata**
   - Who executed it (`executor_id`)
   - When it was executed (`started_at`, `finished_at`)
   - How long it took (`running_time`)
   - What was returned (`result_rows`, `result_size`)

2. **Query Definition**
   - Exact SQL executed (`native_query_json`)
   - MBQL query (`json_query`)
   - Query hash for caching

3. **Context Information**
   - Card ID (if from a card)
   - Dashboard ID (if from dashboard)
   - Pulse ID (if from alert)
   - Execution context

### Execution History

Users can view execution history:

```clojure
;; Query execution records provide:
- Timestamp of execution
- User who executed
- Execution time
- Result size
- Success/failure status
- Error information (if failed)
- Actual SQL/MBQL executed
```

### Audit Log (Enterprise)

Enterprise audit log tracks:

```clojure
{:topic :card-query
 :model "Card"
 :model_id 123
 :user_id 42
 :details {
   :card_name "Sales by Category"
   :database_id 1
   :table_id 10
   :query_type :native
   :execution_time_ms 1234
   :result_rows 100
 }
 :timestamp "2024-01-15T10:30:00Z"}
```

## Source Information Display

### Card Detail View

When viewing a card, users can see:

1. **Source Database** - `database_id` → Database name
2. **Source Table** - `table_id` → Table name (if MBQL)
3. **Query Definition** - `dataset_query` → Formatted query
4. **Result Metadata** - `result_metadata` → Column information
5. **Source Card** - `source_card_id` → Parent card (if derived)

### Query Builder Display

In the query builder, sources are displayed:

- **Tables** - Physical tables from database
- **Saved Questions** - Cards available as virtual tables
- **Models** - Models available as virtual tables
- **Metrics** - Metrics available for use

### Execution View

When viewing query execution:

- **Source Information** - Database, table, card
- **Query Text** - Actual SQL or MBQL
- **Execution Details** - Time, rows, performance
- **Error Information** - If execution failed

## Summary

### Source Tracking Levels

1. **Database** - `database_id` on card and query execution
2. **Table** - `table_id` on card, `source-table` in query
3. **Card** - `source_card_id`, `card__<id>` references, template tags
4. **Query** - `dataset_query` stores complete query definition
5. **Execution** - `query_execution` stores execution details

### Lineage Representation

- **Dependencies** - Cards track what they depend on
- **Descendants** - Cards track what depends on them
- **Dependency Graph** - Built for nested queries
- **Circular Detection** - Prevents infinite loops

### Auditability

- **Query Execution** - Every execution recorded
- **Execution Metadata** - Who, when, how long, results
- **Query Definition** - Exact SQL/MBQL stored
- **Audit Log** - Enterprise feature for comprehensive logging
- **View Log** - Tracks who viewed what

### User Access

- **Query Metadata API** - Comprehensive source information
- **Card Metadata** - Source database, table, query
- **Execution History** - Past executions with details
- **Virtual Tables** - Cards appear as queryable sources

This architecture ensures complete traceability of data sources, enabling users to understand where their chart data comes from and providing a comprehensive audit trail for compliance and debugging purposes.


