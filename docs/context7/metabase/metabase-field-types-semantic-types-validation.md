# Metabase Field Types, Semantic Types, and Visualization Validation

## Overview

Metabase uses a sophisticated type system to model database fields, infer semantic meaning, and validate visualization compatibility. This document explains how field types, semantic types, and data schemas are modeled, and how they prevent invalid chart configurations.

## Type System Architecture

### Three-Layer Type Model

Metabase uses a three-layer type system:

1. **Database Type** - Native database column type (e.g., `VARCHAR`, `INTEGER`, `TIMESTAMP`)
2. **Base Type** - Metabase's normalized data type (e.g., `type/Text`, `type/Integer`, `type/DateTime`)
3. **Effective Type** - Refined type after coercion/transformation (e.g., `type/Date` from `type/DateTime`)
4. **Semantic Type** - Business meaning of the data (e.g., `type/Email`, `type/Latitude`, `type/CreationDate`)

### Type Hierarchy

All types derive from a common root:

```clojure
(def FieldType
  (mu/with-api-error-message
    [:fn #(isa? % :type/*)]
    (deferred-tru "value must be a valid field type.")))
```

Types form a hierarchy where `isa?` checks determine compatibility:

```clojure
;; Type hierarchy example:
:type/*                    ; Root type
  :type/Text               ; Base text type
  :type/Email              ; Semantic type (isa? :type/Text)
  :type/URL                ; Semantic type (isa? :type/Text)
  :type/Number             ; Base number type
    :type/Integer          ; Specific number type
    :type/Float            ; Specific number type
  :type/Temporal           ; Base temporal type
    :type/Date            ; Specific temporal type
    :type/DateTime        ; Specific temporal type
    :type/Time            ; Specific temporal type
```

## Field Type Modeling

### Base Type

The **base type** represents the fundamental data type from the database:

```clojure
;; Base types include:
:type/Text
:type/Integer
:type/Float
:type/Boolean
:type/Date
:type/DateTime
:type/Time
:type/SerializedJSON
:type/XML
```

Base types are determined during database sync:

```clojure
(defn- insert-new-fields!
  [table new-field-metadatas parent-id]
  "Inserts new Field records with base types from database metadata."
  (t2/insert-returning-pks! :model/Field
    (for [{:keys [base-type database-type field-name] :as field} new-field-metadatas]
      {:table_id      (u/the-id table)
       :name          field-name
       :database_type database-type      ; Native DB type
       :base_type     base-type          ; Metabase base type
       ...})))
```

### Effective Type

The **effective type** is a refined version of the base type after coercion:

```clojure
(defn- insert-new-fields!
  [table new-field-metadatas parent-id]
  (for [{:keys [base-type effective-type coercion-strategy] :as field} new-field-metadatas]
    (do
      (when (and effective-type
                 base-type
                 (not= effective-type base-type)
                 (nil? coercion-strategy))
        (log/warn "WARNING: Field has effective type but no coercion strategy"))
      {:base_type         base-type
       :effective_type    (if (and effective-type coercion-strategy)
                            effective-type
                            base-type)
       :coercion_strategy (when effective-type coercion-strategy)
       ...})))
```

**Example:**
- Database type: `TIMESTAMP`
- Base type: `type/DateTime`
- Effective type: `type/Date` (after date extraction)
- Coercion strategy: `:Coercion/DateTime->Date`

### Type Resolution

When fields are used in queries, types are resolved:

```clojure
(defn- field-type
  "Determines field type from field clause."
  [field-clause]
  (lib.util.match/match-one field-clause
    ;; Integer field ID - lookup from metadata
    [:field (id :guard integer?) _]
    ((some-fn :effective-type :base-type)
     (lib.metadata.protocols/field (qp.store/metadata-provider) id))
    
    ;; String field name - extract from options
    [:field (_ :guard string?) opts]
    (:base-type opts)))
```

### Type Checking

Metabase provides type checking functions:

```clojure
(mu/defn field-is-type?
  "Checks if a field has a specific type."
  [tyype {base-type :base_type, effective-type :effective_type}]
  (some #(isa? % tyype) [base-type effective-type]))
```

This allows checking if a field is compatible with a type:

```clojure
;; Check if field is a number
(field-is-type? :type/Number field)  ; true if base or effective type is number

;; Check if field is temporal
(field-is-type? :type/Temporal field)  ; true if date/time/datetime
```

## Semantic Type System

### Semantic Type Definition

**Semantic types** provide business meaning beyond the base data type:

```clojure
;; Common semantic types:
:type/PK              ; Primary key
:type/FK              ; Foreign key
:type/Email           ; Email address
:type/URL             ; Web URL
:type/Latitude        ; Geographic latitude
:type/Longitude       ; Geographic longitude
:type/City            ; City name
:type/Country         ; Country name
:type/ZipCode         ; Postal code
:type/CreationDate    ; Record creation date
:type/JoinDate        ; User join date
:type/Quantity        ; Numeric quantity
:type/Income          ; Income amount
:type/Discount        ; Discount percentage
:type/Category        ; Categorical value
```

### Semantic Type Inference

Semantic types are inferred from field names and database types:

```clojure
(def ^:private pattern+base-types+semantic-type
  [[#"^id$"                        any-type         :type/PK]
   [#"^lon$"                       float-type       :type/Longitude]
   [#"^.*_lon$"                    float-type       :type/Longitude]
   [#"^lat$"                       float-type       :type/Latitude]
   [#"^.*_lat$"                     float-type       :type/Latitude]
   [#"^.*_url$"                     text-type        :type/URL]
   [#"^city$"                       text-type        :type/City]
   [#"^country"                     text-type        :type/Country]
   [#"^first(?:_?)name$"           text-type        :type/Name]
   [#"^last(?:_?)name$"            text-type        :type/Name]
   [#"^postal(?:_?)code$"           int-or-text-type :type/ZipCode]
   [#"quantity"                     int-type         :type/Quantity]
   [#"count$"                       int-type         :type/Quantity]
   [#"create"                       date-type        :type/CreationDate]
   [#"create"                       timestamp-type   :type/CreationTimestamp]
   [#"join"                         date-type        :type/JoinDate]
   ...])
```

The inference process:

```clojure
(defn infer-semantic-type-by-name
  "Infers semantic type from column name and base type."
  [col]
  (let [name-lower (u/lower-case-en (:name col))
        base-type  (:base_type col)]
    (some (fn [[pattern valid-base-types semantic-type]]
            (when (and (re-find pattern name-lower)
                       (or (nil? valid-base-types)
                           (contains? valid-base-types base-type)))
              semantic-type))
          pattern+base-types+semantic-type)))
```

### Database-Specific Semantic Types

Drivers can provide database-specific semantic type mappings:

```clojure
(defmethod sql-jdbc.sync/column->semantic-type :postgres
  [_driver database-type _column-name]
  "Maps PostgreSQL types to semantic types."
  (case database-type
    "json"  :type/SerializedJSON
    "jsonb" :type/SerializedJSON
    "xml"   :type/XML
    "inet"  :type/IPAddress
    nil))
```

### Result Metadata Semantic Types

Semantic types are also inferred from query results:

```clojure
(mu/defn- maybe-infer-semantic-type
  "Infers semantic type for result column metadata."
  [col]
  (update col :semantic_type
    (fn [original-value]
      (case original-value
        ;; If no semantic type, try to infer from name
        (nil :type/Number) 
        (classifiers.name/infer-semantic-type-by-name col)
        
        ;; Otherwise keep original
        original-value))))
```

## Data Schema Modeling

### Column Metadata Schema

Result columns are modeled with comprehensive metadata:

```clojure
(mr/def ::ResultColumnMetadata
  [:map
   [:name         :string]
   [:display_name :string]
   [:base_type    ms/FieldTypeKeywordOrString]
   [:description        {:optional true} [:maybe :string]]
   [:semantic_type      {:optional true} [:maybe ms/FieldSemanticOrRelationTypeKeywordOrString]]
   [:unit               {:optional true} [:maybe DateTimeUnitKeywordOrString]]
   [:fingerprint        {:optional true} [:maybe fingerprint.schema/Fingerprint]]
   [:id                 {:optional true} [:maybe ::lib.schema.id/field]]
   [:field_ref          {:optional true} [:ref ::MaybeUnnormalizedReference]]
   [:converted_timezone {:optional true} ::lib.schema.expression.temporal/timezone-id]])
```

### Field Metadata Schema

Fields in the database have a similar structure:

```clojure
(mr/def ::column
  "Schema for metadata about a specific Column."
  [:map
   [:lib/type         [:= :metadata/column]]
   [:id               ::lib.schema.id/column]
   [:name             ::lib.schema.common/non-blank-string]
   [:display-name     ::lib.schema.common/non-blank-string]
   [:base-type        ::lib.schema.common/base-type]
   [:effective-type   ::lib.schema.common/base-type]
   [:semantic-type    {:optional true} [:maybe ::lib.schema.common/base-type]]
   [:field-ref        {:optional true} [:maybe :vector]]
   [:table-id         ::lib.schema.id/table]
   [:description      {:optional true} [:maybe ::lib.schema.common/non-blank-string]]
   [:fk-field-id      {:optional true} [:maybe ::lib.schema.id/field]]
   [:coercion-strategy {:optional true} [:maybe :map]]
   [:settings         {:optional true} [:maybe :map]]
   [:visibility-type  ::lib.schema.common/non-blank-string]
   ...])
```

### Type Addition to Fields

When fields are used in queries, types are added if missing:

```clojure
(defn add-types-to-fields
  "Enriches field definitions with base-type and effective-type."
  [x metadata-provider]
  (if-let [field-ids (lib.util.match/match x
                        [:field
                         (options :guard (complement (every-pred :base-type :effective-type)))
                         (id :guard integer? pos?)]
                        id)]
    ;; Pre-warm metadata provider
    (do (lib.metadata/bulk-metadata metadata-provider :metadata/column field-ids)
        (lib.util.match/replace
          x
          [:field (options :guard ...) (id :guard integer? pos?)]
          (update &match 1 merge
                  (-> (lib.metadata/field metadata-provider id)
                      (select-keys [:base-type :effective-type])))))
    x))
```

## Visualization Compatibility Validation

### Chart Type Detection

Metabase detects appropriate chart types based on data structure:

```clojure
(defn detect-pulse-chart-type
  "Determines suitable visualization type for card data."
  [{display-type :display card-name :name} maybe-dashcard {:keys [cols rows] :as data}]
  (letfn [(chart-type [tyype reason & args]
            (log/tracef "Detected chart type %s for Card %s because %s"
                        tyype (pr-str card-name) (apply format reason args))
            tyype)]
    (cond
      ;; Empty results
      (or (empty? rows) (= [[nil]] (-> data :rows)))
      (chart-type :empty "there are no rows in results")
      
      ;; Single value = scalar
      (and (= (count cols) (count (first rows)) 1))
      (chart-type :scalar "result has one row and one column")
      
      ;; Explicit display types
      (#{:scalar :row :progress :gauge :table :funnel} display-type)
      (chart-type display-type "display-type is %s" display-type)
      
      ;; JavaScript visualizations
      (#{:smartscalar :sankey :pie :scatter :waterfall 
         :line :area :bar :combo} display-type)
      (chart-type :javascript_visualization "display-type is javascript_visualization")
      
      ;; Default to table
      :else
      (chart-type :table "no other chart types match"))))
```

### Series Compatibility

When multiple cards are used as series, Metabase validates compatibility:

```clojure
(defmethod series-are-compatible? :area
  [first-card second-card database-id->metadata-provider]
  (area-bar-line-series-are-compatible? first-card second-card database-id->metadata-provider))

(defmethod series-are-compatible? :line
  [first-card second-card database-id->metadata-provider]
  (area-bar-line-series-are-compatible? first-card second-card database-id->metadata-provider))

(defmethod series-are-compatible? :bar
  [first-card second-card database-id->metadata-provider]
  (area-bar-line-series-are-compatible? first-card second-card database-id->metadata-provider))
```

The compatibility check ensures:
- Same number of dimensions
- Compatible dimension types
- Compatible metric types
- Same temporal units (if applicable)

### Temporal Unit Compatibility

For temporal fields, Metabase validates temporal units:

```clojure
(defn compatible-temporal-unit?
  "Checks if temporal unit is valid for a base type."
  [a-type temporal-unit]
  (or (nil? temporal-unit)
      (contains? (valid-units-for-base-type a-type) temporal-unit)))
```

**Valid Units by Type:**
- `type/Date` - `:day`, `:week`, `:month`, `:quarter`, `:year`
- `type/DateTime` - All date units plus `:hour`, `:minute`, `:second`
- `type/Time` - `:hour`, `:minute`, `:second`

### Parameter Type Validation

When parameters are used, Metabase validates types:

```clojure
(mu/defn check-allowed-parameter-value-type
  "Validates parameter value type matches widget type."
  [parameter-name
   widget-type          :- ::lib.schema.template-tag/widget-type
   parameter-value-type :- ::lib.schema.parameter/type]
  (when-not (allowed-parameter-type-for-template-tag-widget-type? 
              parameter-value-type widget-type)
    (let [allowed-types (allowed-parameter-types-for-template-tag-widget-type widget-type)]
      (throw (ex-info 
        (tru "Invalid parameter type {0} for parameter {1}. Parameter type must be one of: {2}"
             parameter-value-type
             (pr-str parameter-name)
             (str/join ", " (sort allowed-types)))
        {:type              qp.error-type/invalid-parameter
         :invalid-parameter parameter-name
         :template-tag-type widget-type
         :allowed-types     allowed-types})))))
```

**Example Validations:**
- Date widget → `:date/single`, `:date/range`
- String widget → `:string/=`, `:string/contains`
- Number widget → `:number/=`, `:number/between`

### Visualization Settings Validation

Visualization settings are validated against field types:

```clojure
(defmulti global-type-settings
  "Retrieves type-specific visualization settings."
  (fn [col _viz-settings] (col-type col)))

(defmethod global-type-settings :type/Temporal 
  [_ {::mb.viz/keys [global-column-settings] :as _viz-settings}]
  (:type/Temporal global-column-settings {}))

(defmethod global-type-settings :type/Number 
  [_ {::mb.viz/keys [global-column-settings] :as _viz-settings}]
  (:type/Number global-column-settings {}))

(defmethod global-type-settings :type/Currency 
  [_ {::mb.viz/keys [global-column-settings] :as _viz-settings}]
  (merge
   {::mb.viz/number-style "currency"}
   (:type/Currency global-column-settings)))
```

## Preventing Invalid Chart Configurations

### Dimension/Metric Validation

Visualizations require specific field types for dimensions and metrics:

**Bar/Line/Area Charts:**
- **Dimensions**: Must be categorical or temporal (not numeric)
- **Metrics**: Must be numeric (aggregated)

**Pie Charts:**
- **Dimensions**: Single categorical dimension
- **Metrics**: Single numeric metric

**Scatter Plots:**
- **X-axis**: Numeric or temporal
- **Y-axis**: Numeric
- Both must be numeric for correlation

**Maps:**
- **Latitude**: `type/Latitude` semantic type or float with lat pattern
- **Longitude**: `type/Longitude` semantic type or float with lon pattern

### Type-Based Filter Validation

When filters are applied, types are validated:

```clojure
(mu/defn- update-filter-for-field-type
  "Updates filter value based on field type."
  [{field :field, {value :value} :as field-filter}]
  (let [effective-type ((some-fn :effective-type :base-type) field)
        new-value (cond
                    (string? value)
                    (parse-value-for-field-type effective-type value)
                    (and (sequential? value)
                         (every? string? value))
                    (mapv (partial parse-value-for-field-type effective-type) value))]
    (cond-> field-filter
      new-value (assoc-in [:value :value] new-value))))
```

**Type-Specific Parsing:**
- UUID fields → Parse as UUID
- Date fields → Parse as date
- Number fields → Parse as number
- Boolean fields → Parse as boolean

### Expression Type Validation

Expressions are validated for type compatibility:

```clojure
(defn type-of?
  "Checks if expression type is compatible with base type."
  [expr base-type]
  (let [expr-type (type-of expr)]
    (assert ((some-fn keyword? set?) expr-type)
            (i18n/tru "type-of {0} returned an invalid type {1}" 
                      (pr-str expr) (pr-str expr-type)))
    (is-type? expr-type base-type)))
```

**Example Validations:**
- `Sum([Quantity])` → Must be numeric
- `Concat([First Name], [Last Name])` → Must be text
- `DateTrunc([Created At], "month")` → Must be temporal

### Visualization Settings Migration

When visualization settings are incompatible, Metabase migrates them:

```clojure
(defn- area-bar-stacked-viz-migration
  "Migrates stacked visualization settings."
  [{display :display viz :visualization_settings :as card}]
  (if (and (#{:area :bar "area" "bar"} display)
           (:stackable.stack_type viz))
    (let [actual-display (or (:stackable.stack_display viz) display)
          new-viz        (m/update-existing viz :series_settings 
                           update-vals (fn [m] (dissoc m :display)))]
      (assoc card
             :display actual-display
             :visualization_settings new-viz))
    card))
```

This ensures that:
- Stacked charts have valid display types
- Series settings are consistent
- Invalid configurations are corrected

## Type System in Practice

### Query Result Metadata

When queries execute, result metadata includes types:

```yaml
result_metadata:
  - base_type: type/Text
    display_name: CATEGORY
    effective_type: type/Text
    semantic_type: null
    name: CATEGORY
  - base_type: type/DateTime
    display_name: Week
    effective_type: type/DateTime
    semantic_type: null
    name: Week
  - base_type: type/BigInteger
    display_name: Count
    effective_type: type/BigInteger
    semantic_type: type/Quantity
    name: Count
```

### Visualization Settings Application

Visualization settings reference fields by type:

```yaml
visualization_settings:
  graph.dimensions:
    - Week        # Temporal dimension
    - CATEGORY    # Text dimension
  graph.metrics:
    - Count       # Numeric metric
```

The frontend validates:
- Dimensions are not numeric (unless explicitly allowed)
- Metrics are numeric
- Temporal dimensions have valid units
- Map visualizations have lat/lon fields

### Type Inference Pipeline

The complete type inference process:

1. **Database Sync** → Base types from database schema
2. **Name Analysis** → Semantic types from field names
3. **Driver Mapping** → Database-specific semantic types
4. **Query Results** → Refined semantic types from data
5. **Visualization** → Type-based validation and formatting

## Summary

### Key Principles

1. **Type Hierarchy** - All types derive from `:type/*` with `isa?` relationships
2. **Three-Layer Model** - Database type → Base type → Effective type
3. **Semantic Enrichment** - Semantic types add business meaning
4. **Validation** - Types prevent invalid configurations
5. **Compatibility** - Type compatibility ensures valid visualizations

### Validation Points

- **Chart Type Selection** - Based on data structure and types
- **Series Compatibility** - Ensures compatible field types
- **Parameter Types** - Validates widget types match parameter types
- **Temporal Units** - Validates units for temporal fields
- **Expression Types** - Validates expression return types
- **Visualization Settings** - Validates dimension/metric assignments

### Benefits

- **Prevents Errors** - Invalid configurations caught before execution
- **Better UX** - Appropriate chart types suggested automatically
- **Type Safety** - Type system ensures data consistency
- **Semantic Understanding** - Business meaning preserved through semantic types
- **Flexible Validation** - Type hierarchy allows flexible matching

This type system ensures that Metabase can intelligently validate visualization configurations and prevent users from creating invalid charts, while providing helpful suggestions based on the data structure and types.


