# Metabase Dashboard Synchronization: Cross-Filtering, Parameter Binding, and Global Filter Propagation

## Overview

Metabase provides sophisticated mechanisms for synchronizing multiple visualizations on a dashboard through parameter binding, cross-filtering, and global filter propagation. This document explains how these systems work together to create interactive, synchronized dashboards.

## Dashboard Parameter System

### Parameter Architecture

Dashboard parameters serve as the central coordination mechanism for synchronizing visualizations:

1. **Dashboard Parameters** - Global filter widgets at the dashboard level
2. **Parameter Mappings** - Links between dashboard parameters and card parameters
3. **Parameter Values** - Current filter values applied to the dashboard
4. **Linked Filters** - Parameters that filter other parameters (chain filtering)

### Parameter Structure

```clojure
;; Dashboard parameter definition
{:id "param-123"
 :name "Category"
 :slug "category"
 :type :string/=
 :widget-type :string/=
 :default ["Gizmo"]
 :filteringParameters ["param-456"]  ; Linked to another parameter
 :values_source_type "card"           ; Values from a card
 :values_source_config {:card_id 789
                        :value_field [:field 10 nil]}}

;; Parameter mapping on dashboard card
{:parameter_id "param-123"
 :target [:dimension [:template-tag "category_filter"]]
 :card_id 456}
```

### Resolved Parameters

Dashboard parameters are resolved with their mappings:

```clojure
(mu/defn- dashboard->resolved-params
  "Resolves dashboard parameters with their mappings to cards."
  [dashboard]
  (let [param-key->mappings (apply
                             merge-with set/union
                             (for [dashcard (:dashcards dashboard)
                                   param    (:parameter_mappings dashcard)]
                               {(:parameter_id param) #{(assoc param :dashcard dashcard)}}))]
    (into {} (for [{param-key :id, :as param} (:parameters dashboard)]
               [(u/qualified-name param-key) 
                (assoc param :mappings (get param-key->mappings param-key))]))))
```

**Result Structure:**
```clojure
{"param-123" {:id "param-123"
              :name "Category"
              :type :string/=
              :mappings #{{:parameter_id "param-123"
                          :target [:dimension [:template-tag "category_filter"]]
                          :card_id 456
                          :dashcard {:id 789 ...}}
                         {:parameter_id "param-123"
                          :target [:dimension [:field 10 nil]]
                          :card_id 123
                          :dashcard {:id 456 ...}}}}
```

## Parameter Binding

### Binding Process

When a dashboard is loaded, parameters are bound to cards through parameter mappings:

#### 1. Parameter Mapping Resolution

Each dashboard card has `parameter_mappings` that link dashboard parameters to card parameters:

```clojure
;; Dashboard card structure
{:id 789
 :card_id 456
 :parameter_mappings [
   {:parameter_id "param-123"
    :target [:dimension [:template-tag "category_filter"]]
    :card_id 456}]
 :visualization_settings {...}}
```

#### 2. Parameter Resolution for Queries

When a card query is executed, parameters are resolved:

```clojure
(defn- resolve-param-for-card
  "Resolves a dashboard parameter for a specific card."
  [card-id dashcard-id param-id->param request-param]
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
      
      ;; 4. Return merged parameter info with target
      (merge {:type (:type matching-param)}
             request-param
             {:id     (:id request-param)
              :target (:target matching-mapping)}))))
```

#### 3. Query Parameter Application

Resolved parameters are applied to card queries:

```clojure
(mu/defn- resolve-params-for-query
  "Resolves all parameters for a card query."
  [dashboard-id card-id dashcard-id request-params]
  (let [dashboard (-> (t2/select-one :model/Dashboard :id dashboard-id)
                      (t2/hydrate :resolved-params))
        dashboard-param-id->param (into {} 
                                        (map (fn [[param-id param]]
                                               [param-id (dissoc param :default)]))
                                        (:resolved-params dashboard))
        ;; Merge defaults with request parameters
        merged-parameters (vals (merge 
                                  (dashboard-param-defaults dashboard-param-id->param card-id)
                                  request-param-id->param))]
    ;; Resolve each parameter for this card
    (into [] (comp (map (partial resolve-param-for-card 
                                  card-id dashcard-id dashboard-param-id->param))
                   (filter some?))
          merged-parameters)))
```

### Parameter Target Types

Parameters can target different card elements:

#### 1. Template Tags (Native SQL)

```clojure
{:target [:dimension [:template-tag "category_filter"]]
 :card_id 456}
```

Maps to native SQL template tag:
```sql
WHERE category = {{category_filter}}
```

#### 2. MBQL Field References

```clojure
{:target [:dimension [:field 10 nil]]
 :card_id 456}
```

Maps to MBQL filter:
```clojure
{:query {:filter [:= [:field 10 nil] "Gizmo"]}}
```

#### 3. Expression References

```clojure
{:target [:dimension [:expression "Custom Field"]]
 :card_id 456}
```

Maps to expression-based filters.

## Global Filter Propagation

### Propagation Mechanism

When a dashboard parameter value changes, it propagates to all cards with mappings:

#### 1. Parameter Value Update

```clojure
;; Dashboard parameter values structure
{:param_values {"param-123" {:values ["Gizmo" "Widget"]
                             :default ["Gizmo"]}
                "param-456" {:values [100 200]}}}
```

#### 2. Query Execution with Parameters

Each card query includes resolved parameters:

```clojure
POST /api/dashboard/:dashboard-id/dashcard/:dashcard-id/card/:card-id/query
{
  "parameters": [
    {
      "id": "param-123",
      "type": "string/=",
      "value": ["Gizmo"],
      "target": ["dimension", ["template-tag", "category_filter"]]
    }
  ]
}
```

#### 3. Parameter Substitution

Parameters are substituted into queries:

**Native SQL:**
```sql
-- Original query
SELECT * FROM products WHERE {{category_filter}}

-- After substitution
SELECT * FROM products WHERE category = 'Gizmo'
```

**MBQL:**
```clojure
;; Original query
{:query {:filter [:= [:field 10 nil] nil]}}

;; After substitution
{:query {:filter [:= [:field 10 nil] "Gizmo"]}}
```

### Synchronized Query Execution

All cards on a dashboard execute with the same parameter values:

```clojure
;; Dashboard load process
1. Load dashboard with resolved parameters
2. For each dashboard card:
   a. Resolve parameters for this card
   b. Execute query with resolved parameters
   c. Return results
3. Frontend renders all cards with synchronized data
```

## Cross-Filtering

### Click Behavior Configuration

Cross-filtering is enabled through click behaviors on visualizations:

```clojure
;; Click behavior structure
{:click-behavior {:type :crossfilter
                  :parameter-mapping [
                    {:id "param-123"
                     :source {:field "category"
                              :column "CATEGORY"}
                     :target {:id "param-123"
                              :type "string/="}}]}}
```

### Cross-Filter Implementation

When a user clicks on a chart element:

#### 1. Click Event Capture

Frontend captures click event with data:
```javascript
{
  column: "CATEGORY",
  value: "Gizmo",
  data: {...}
}
```

#### 2. Parameter Update

Parameter value is updated:
```javascript
// Update dashboard parameter
dashboard.setParameterValue("param-123", ["Gizmo"]);
```

#### 3. Query Refresh

All cards with parameter mappings refresh:
```javascript
// For each card with param-123 mapping
cards.forEach(card => {
  if (card.hasParameterMapping("param-123")) {
    card.refreshQuery({
      parameters: [{
        id: "param-123",
        value: ["Gizmo"]
      }]
    });
  }
});
```

#### 4. Backend Query Execution

Each card query executes with updated parameters:

```clojure
;; Card 1: Has param-123 mapping
POST /api/dashboard/1/dashcard/10/card/20/query
{
  "parameters": [{"id": "param-123", "value": ["Gizmo"]}]
}

;; Card 2: Has param-123 mapping
POST /api/dashboard/1/dashcard/11/card/21/query
{
  "parameters": [{"id": "param-123", "value": ["Gizmo"]}]
}

;; Card 3: No param-123 mapping - not affected
POST /api/dashboard/1/dashcard/12/card/22/query
{
  "parameters": []
}
```

### Click Behavior Types

#### 1. Cross-Filter

Updates dashboard parameters based on clicked value:
```clojure
{:type :crossfilter
 :parameter-mapping [{:id "param-123"
                      :source {:field "category"}
                      :target {:id "param-123"}}]}
```

#### 2. Link to Dashboard

Navigates to another dashboard with parameters:
```clojure
{:type :link
 :link-type :dashboard
 :link-target-id 789
 :parameter-mapping [{:id "param-123"
                      :source {:field "category"}
                      :target {:id "child-param-456"}}]}
```

#### 3. Link to Question

Opens a question with parameters:
```clojure
{:type :link
 :link-type :card
 :link-target-id 456
 :parameter-mapping [{:id "param-123"
                      :source {:field "category"}
                      :target {:id "card-param-789"}}]}
```

## Chain Filtering (Linked Filters)

### Linked Filter Concept

Chain filtering allows one parameter to filter the available values of another parameter:

```clojure
;; Parameter 1: Country
{:id "param-country"
 :name "Country"
 :type :string/=}

;; Parameter 2: City (linked to Country)
{:id "param-city"
 :name "City"
 :type :string/=
 :filteringParameters ["param-country"]}  ; Linked to Country
```

### Chain Filter Implementation

#### 1. Constraint Generation

When fetching values for a linked parameter:

```clojure
(mu/defn- chain-filter-constraints
  "Generates constraints for chain filtering."
  [dashboard constraint-param-key->value]
  (vec (for [[param-key value] constraint-param-key->value
             :let [param (get-in dashboard [:resolved-params param-key])]
             :when param
             field (param->fields param)]
         (assoc field :value value))))
```

#### 2. Value Fetching with Constraints

```clojure
(mu/defn param-values
  "Fetches parameter values with optional constraints."
  ([dashboard param-key constraint-param-key->value query]
   (let [dashboard (t2/hydrate dashboard :resolved-params)
         param     (get (:resolved-params dashboard) param-key)]
     ;; Generate chain filter constraints
     (custom-values/parameter->values
      param
      query
      (fn [] (chain-filter dashboard param-key constraint-param-key->value query))))))
```

#### 3. Constraint Application

Constraints filter the available values:

```clojure
;; Fetch cities when country = "USA"
GET /api/dashboard/1/params/param-city/values?param-country=USA

;; Backend applies constraint
(defn chain-filter
  [dashboard param-key constraint-param-key->value query]
  (let [constraints (chain-filter-constraints dashboard constraint-param-key->value)
        fields      (param->fields (get-in dashboard [:resolved-params param-key]))]
    ;; Query with constraints applied
    (apply-constraints-to-query fields constraints)))
```

### Linked Filter Flow

1. **User selects Country** → `param-country = "USA"`
2. **City parameter updates** → Fetches cities filtered by country
3. **City dropdown updates** → Shows only cities in USA
4. **User selects City** → `param-city = "New York"`
5. **All cards refresh** → With both country and city filters

## Parameter Value Sources

### Static List

```clojure
{:values_source_type "static-list"
 :values_source_config {:values [["Gizmo"] ["Widget"] ["Doohickey"]]}}
```

### Card-Based Values

```clojure
{:values_source_type "card"
 :values_source_config {:card_id 456
                        :value_field [:field 10 nil]}}
```

Values are fetched by executing the card query and extracting the specified field.

### Field-Based Values

```clojure
{:values_source_type nil  ; Default
 :mappings [{:target [:dimension [:field 10 nil]]}]}
```

Values are fetched directly from the field using field values or a query.

### Chain-Filtered Values

When a parameter has `filteringParameters`, values are filtered:

```clojure
;; Fetch values with constraints
(defn chain-filter
  [dashboard param-key constraint-param-key->value query]
  (let [constraints (chain-filter-constraints dashboard constraint-param-key->value)
        param       (get-in dashboard [:resolved-params param-key])]
    ;; Apply constraints to value fetching
    (fetch-parameter-values param constraints query)))
```

## Synchronization Flow

### Complete Flow Diagram

```
1. User Interaction
   ├─ Dashboard filter widget change
   ├─ Chart element click (cross-filter)
   └─ URL parameter change

2. Parameter Update
   ├─ Update dashboard parameter value
   ├─ Check for linked filters (filteringParameters)
   └─ Update linked parameter values if needed

3. Query Resolution
   ├─ For each dashboard card:
   │  ├─ Resolve parameters for this card
   │  ├─ Build query with parameters
   │  └─ Execute query
   └─ Return results

4. Frontend Update
   ├─ Update all affected visualizations
   ├─ Update filter widgets
   └─ Update linked filter dropdowns
```

### Example: Complete Synchronization

**Initial State:**
- Dashboard has 3 cards
- Parameter "Category" with value ["Gizmo"]
- All cards have parameter mappings to "Category"

**User Action:**
1. User clicks "Widget" in Card 1's chart

**Backend Processing:**
```clojure
;; 1. Update parameter value
{:param_values {"param-category" {:values ["Widget"]}}}

;; 2. Resolve parameters for each card
;; Card 1
(resolve-params-for-query dashboard-id card-1-id dashcard-1-id 
  [{:id "param-category" :value ["Widget"]}])
;; => [{:id "param-category" 
;;      :value ["Widget"]
;;      :target [:dimension [:template-tag "category_filter"]]}]

;; Card 2
(resolve-params-for-query dashboard-id card-2-id dashcard-2-id 
  [{:id "param-category" :value ["Widget"]}])
;; => [{:id "param-category"
;;      :value ["Widget"]
;;      :target [:dimension [:field 10 nil]]}]

;; Card 3 (no mapping - not affected)
(resolve-params-for-query dashboard-id card-3-id dashcard-3-id [])
;; => []
```

**Query Execution:**
```clojure
;; Card 1 query
POST /api/dashboard/1/dashcard/10/card/20/query
{
  "parameters": [{"id": "param-category", "value": ["Widget"]}]
}
;; Executes: SELECT * FROM products WHERE category = 'Widget'

;; Card 2 query
POST /api/dashboard/1/dashcard/11/card/21/query
{
  "parameters": [{"id": "param-category", "value": ["Widget"]}]
}
;; Executes: MBQL query with filter [:= [:field 10 nil] "Widget"]

;; Card 3 query (no parameters)
POST /api/dashboard/1/dashcard/12/card/22/query
{
  "parameters": []
}
;; Executes: Original query unchanged
```

**Frontend Update:**
- Card 1 refreshes with new data
- Card 2 refreshes with new data
- Card 3 remains unchanged
- Filter widget shows "Widget" selected

## Advanced Features

### Parameter Defaults

Parameters can have default values:

```clojure
{:id "param-category"
 :default ["Gizmo"]}
```

Defaults are applied when:
- Dashboard first loads
- Parameter value is cleared
- No explicit value provided

### Parameter Merging

Default values are merged with request values:

```clojure
(defn- dashboard-param-defaults
  "Constructs default parameter values for a card."
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

Request values take precedence over defaults.

### User Parameter Values

Metabase stores user-specific parameter values:

```clojure
(defn- resolve-params-for-query
  [dashboard-id card-id dashcard-id request-params]
  ;; Store user parameter values
  (when-let [user-id api/*current-user-id*]
    (when (seq request-params)
      (user-parameter-value/store! user-id dashboard-id request-params)))
  ;; ... resolve parameters
)
```

This allows:
- Remembering user filter preferences
- Restoring filters on dashboard load
- Per-user filter state

### Parameter Validation

Parameters are validated before application:

```clojure
(defn- resolve-param-for-card
  [card-id dashcard-id param-id->param request-param]
  ;; Validate parameter type
  (when (:type request-param)
    (check-allowed-parameter-value-type
     param-id
     (or (when (and (= (:type matching-param) :dimension)
                    (not= (:widget-type matching-param) :none))
           (:widget-type matching-param))
         (:type matching-param))
     (:type request-param)))
  ;; ... resolve parameter
)
```

## Performance Optimizations

### Batch Query Execution

Dashboard cards can execute queries in parallel:

```javascript
// Frontend batches queries
const queries = dashboard.cards
  .filter(card => card.hasParameterMappings())
  .map(card => ({
    url: `/api/dashboard/${dashboardId}/dashcard/${card.dashcardId}/card/${card.cardId}/query`,
    body: { parameters: resolvedParameters }
  }));

const results = await Promise.all(
  queries.map(q => fetch(q.url, { method: 'POST', body: JSON.stringify(q.body) }))
);
```

### Parameter Caching

Parameter values can be cached:

```clojure
;; Cache parameter field values
(defn param-values
  [dashboard param-key constraint-param-key->value query]
  ;; Check cache first
  (if-let [cached (get-cached-param-values param-key constraint-param-key->value)]
    cached
    ;; Fetch and cache
    (let [values (fetch-parameter-values ...)]
      (cache-param-values param-key constraint-param-key->value values)
      values)))
```

### Incremental Updates

Only affected cards refresh:

```javascript
// Determine which cards need refresh
const affectedCards = dashboard.cards.filter(card => {
  const cardParams = card.getParameterMappings();
  return changedParameters.some(changed => 
    cardParams.some(cp => cp.parameter_id === changed.id)
  );
});

// Refresh only affected cards
affectedCards.forEach(card => card.refreshQuery());
```

## Summary

### Key Components

1. **Dashboard Parameters** - Global filter widgets
2. **Parameter Mappings** - Links parameters to card targets
3. **Parameter Values** - Current filter state
4. **Linked Filters** - Chain filtering relationships
5. **Click Behaviors** - Cross-filtering triggers

### Synchronization Mechanisms

1. **Parameter Binding** - Dashboard parameters bound to cards via mappings
2. **Global Propagation** - Parameter changes propagate to all mapped cards
3. **Cross-Filtering** - Chart clicks update parameters and refresh cards
4. **Chain Filtering** - Parameters filter other parameters' values
5. **Query Resolution** - Parameters resolved and applied to queries

### Flow

1. **User Action** → Parameter value changes
2. **Parameter Resolution** → Values resolved for each card
3. **Query Execution** → Cards execute with resolved parameters
4. **Result Update** → Visualizations refresh with new data
5. **UI Synchronization** → Filter widgets and charts update

This architecture ensures that all dashboard visualizations stay synchronized through a centralized parameter system, enabling rich interactive experiences with cross-filtering and linked filters.


