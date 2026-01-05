# Metabase Unit of Truth Architecture: Questions, Datasets, and Cards

## Overview

In Metabase, the **Card** (also referred to as a "Question" or "Dataset") serves as the fundamental unit of truth for queries and visualizations. This document explains how these entities are structured, stored, and referenced, and how multiple visualizations can share the same underlying query definition.

## Core Entities: Cards, Questions, and Datasets

### Terminology

Metabase uses three related terms that refer to essentially the same entity:

1. **Card** - The internal database model name (`report_card` table)
2. **Question** - The user-facing term for a saved query (`type: "question"`)
3. **Dataset/Model** - A special type of card that serves as a reusable data model (`type: "model"`)

All three are stored in the same `report_card` table, differentiated by the `type` column:

```clojure
(def card-types
  #{:model :question :metric})
```

### Card Types

- **`:question`** - A standard saved question/query that can be visualized
- **`:model`** - A saved question that has been promoted to a model, making it reusable as a data source
- **`:metric`** - A V2 metric definition (newer feature)

The `type` field replaced the deprecated boolean `dataset` column in Metabase 0.49.0.

## Storage Architecture

### Database Schema: `report_card` Table

Cards are stored in the `report_card` table with the following key fields:

#### Core Identity Fields
- `id` - Primary key, unique identifier
- `name` - Display name
- `description` - Optional description
- `type` - Card type (`"question"`, `"model"`, or `"metric"`)
- `entity_id` - Stable entity identifier for serialization

#### Query Definition (The Unit of Truth)
- **`dataset_query`** - The core query definition stored as JSON/JSONB. This is the **primary unit of truth** for what data the card retrieves. It contains:
  - `type` - Query type (`"query"` for MBQL, `"native"` for SQL)
  - `database` - Database ID reference
  - `query` - MBQL query structure (for MBQL queries)
  - `native` - SQL query with template tags (for native queries)

#### Result Metadata
- **`result_metadata`** - JSON array describing the columns returned by the query. Each column metadata includes:
  - `name` - Column name
  - `display_name` - Human-readable name
  - `base_type` - Data type (e.g., `type/Text`, `type/DateTime`, `type/BigInteger`)
  - `effective_type` - Refined type after processing
  - `field_ref` - Field reference for MBQL queries
  - `semantic_type` - Business meaning (e.g., `type/Quantity`)

#### Visualization Configuration
- **`display`** - Default visualization type (e.g., `"table"`, `"bar"`, `"line"`, `"area"`, `"pie"`)
- **`visualization_settings`** - JSON object containing visualization-specific settings:
  - `graph.dimensions` - Which columns to use as dimensions
  - `graph.metrics` - Which columns to use as metrics
  - `graph.series_labels` - Custom labels for series
  - `column_settings` - Per-column display settings
  - `graph.x_axis.title_text` - X-axis title
  - `graph.y_axis.title_text` - Y-axis title

#### Organization
- `collection_id` - Parent collection (if any)
- `collection_position` - Position within collection
- `database_id` - Reference to the source database
- `table_id` - Reference to the source table (for MBQL queries)

#### Metadata
- `created_at` - Creation timestamp
- `creator_id` - User who created the card
- `archived` - Whether the card is archived
- `cache_ttl` - Cache time-to-live in seconds

### Example Card Structure

From the serialization format, a complete card looks like:

```yaml
name: Products by week
description: Area chart of products created by week
entity_id: r6vC_vLmo9zG6_r9sAuYG
type: question
dataset_query:
  database: Sample Database
  native:
    query: |-
      SELECT
        category,
        date_trunc ('week', created_at) AS "Week",
        count(*) AS "Count"
      FROM
        products
      WHERE
        {{category_filter}}
      GROUP BY
        category,
        "Week"
    template-tags:
      category_filter:
        type: dimension
        # ... dimension configuration
  type: native
result_metadata:
  - name: CATEGORY
    base_type: type/Text
    display_name: CATEGORY
  - name: Week
    base_type: type/DateTime
    display_name: Week
  - name: Count
    base_type: type/BigInteger
    display_name: Count
    semantic_type: type/Quantity
display: area
visualization_settings:
  graph.dimensions:
    - Week
    - CATEGORY
  graph.metrics:
    - Count
```

## How Cards Are Referenced

### 1. Direct References by ID

Cards are primarily referenced by their numeric `id`:

```clojure
(t2/select-one :model/Card :id card-id)
```

### 2. Entity ID References

For serialization and stable references across environments, cards use `entity_id`:

```yaml
entity_id: r6vC_vLmo9zG6_r9sAuYG
```

### 3. In SQL Queries (CTE References)

Saved questions/models can be referenced in native SQL queries using the `{#ID-name}` syntax:

```sql
WITH gizmo_orders AS {{#5-gizmo-orders-in-2019}}
SELECT count(*)
FROM gizmo_orders
```

Metabase expands this at runtime to include the referenced card's SQL query as a CTE.

### 4. In Dashboard Cards

Dashboard cards reference the underlying card via `card_id`:

```clojure
(def ^:private NewDashboardCard
  [:map
   [:dashboard_id ms/PositiveInt]
   [:card_id {:optional true} [:maybe ms/PositiveInt]]
   ;; ... other fields
   ])
```

## Multiple Visualizations Sharing the Same Query

Metabase enables multiple visualizations to share the same underlying query definition through two mechanisms:

### Mechanism 1: Dashboard Card Series

The primary way multiple visualizations share a query is through **Dashboard Card Series**. A single dashboard card can display multiple series, each using a different card's query, but they can all reference the same underlying card.

#### Database Schema

The relationship is managed through:

1. **`report_dashboardcard`** table - The dashboard card instance
   - `id` - Dashboard card ID
   - `dashboard_id` - Parent dashboard
   - `card_id` - Primary card (the main visualization)
   - `visualization_settings` - Visualization configuration for this dashboard card instance
   - `parameter_mappings` - How dashboard parameters map to card parameters

2. **`dashboardcard_series`** table - Additional series cards
   - `dashboardcard_id` - Reference to the dashboard card
   - `card_id` - Reference to an additional card to display as a series
   - `position` - Order of the series

#### How It Works

```clojure
(defn dashcard->multi-cards
  [dashcard]
  "Returns the cards which have been added to this dashcard using the 
   'add series' dashboard feature. Dashboards allow Line, Area, and Bar 
   dashcards to have other questions (series) added to them so that 
   several Questions are displayed in a single dashcard."
  (mdb.query/query {:select    [:newcard.*]
                    :from      [[:report_dashboardcard :dashcard]]
                    :left-join [[:dashboardcard_series :dashcardseries]
                                [:= :dashcard.id :dashcardseries.dashboardcard_id]
                                [:report_card :newcard]
                                [:= :dashcardseries.card_id :newcard.id]]
                    :where     [:and
                                [:= :newcard.archived false]
                                [:= :dashcard.id (:id dashcard)]]}))
```

#### Key Points

1. **Same Query, Different Visualizations**: Multiple dashboard cards can reference the same `card_id`, each with different `visualization_settings`:
   ```clojure
   ;; Dashboard Card 1: Bar chart
   {:card_id 123
    :visualization_settings {:display "bar" ...}}
   
   ;; Dashboard Card 2: Line chart (same query)
   {:card_id 123
    :visualization_settings {:display "line" ...}}
   ```

2. **Series Overlay**: A single dashboard card can display multiple series, each from different cards:
   ```clojure
   ;; Dashboard Card with series
   {:card_id 123  ; Primary card
    :series [456, 789]  ; Additional cards as series
    :visualization_settings {:display "line" ...}}
   ```

3. **Visualization Settings Override**: Each dashboard card instance can override the card's default visualization settings:
   ```clojure
   ;; Card default
   {:display "table"
    :visualization_settings {:graph.dimensions [...]}}
   
   ;; Dashboard card override
   {:card_id 123
    :visualization_settings {:display "bar"
                            :graph.dimensions [...]}}
   ```

### Mechanism 2: Card Default vs. Dashboard Override

Even when a single card is used once on a dashboard, the dashboard card can override visualization settings:

```clojure
(defn- save-card!
  [card]
  "Saves a card with its default visualization settings"
  ;; ... saves card with :display and :visualization_settings
  )

;; Later, when adding to dashboard:
(defn- save-card!
  [dashboard-card]
  "Dashboard card can override card's visualization settings"
  {:card_id 123
   :visualization_settings {:display "bar"}  ; Overrides card's default "table"
   })
```

## The Unit of Truth Hierarchy

### Primary Unit of Truth: `dataset_query`

The **`dataset_query`** field in the `report_card` table is the absolute unit of truth for:
- What data to retrieve
- Which database to query
- What filters/aggregations to apply
- What columns to return

### Secondary Unit of Truth: `result_metadata`

The **`result_metadata`** describes the structure of the query results:
- Column names and types
- Field references for MBQL queries
- Semantic types

### Visualization Configuration: Instance-Level

Visualization settings exist at multiple levels with inheritance:

1. **Card Level** (`report_card.visualization_settings`) - Default visualization
2. **Dashboard Card Level** (`report_dashboardcard.visualization_settings`) - Override for this dashboard instance
3. **Series Level** - Each series card can have its own visualization settings

### Query Execution Flow

1. **Card Query** - The `dataset_query` defines what to execute
2. **Result Processing** - Results are processed according to `result_metadata`
3. **Visualization Rendering** - Visualization settings determine how to display:
   - Card's default `visualization_settings` (if on card detail page)
   - Dashboard card's `visualization_settings` (if on dashboard)
   - Series cards' settings (for multi-series visualizations)

## Key Architectural Principles

### 1. Single Source of Truth for Queries

The `dataset_query` in `report_card` is the single source of truth. When a card is updated:
- All dashboard cards referencing it get the new query
- All series using it get the new query
- The query is never duplicated

### 2. Visualization Settings Are Instance-Specific

Visualization settings can be customized per dashboard card instance:
- Same query, different chart types
- Same query, different axis configurations
- Same query, different column settings

### 3. Series Enable Query Composition

Series allow combining multiple queries in a single visualization:
- Overlay multiple metrics on the same chart
- Compare different time periods
- Combine related queries visually

### 4. Models Enable Query Reuse

Cards with `type: "model"` can be referenced as data sources:
- Used in the "Saved Questions" virtual database
- Referenced in other queries via CTE syntax
- Serve as reusable data transformations

## Code Examples

### Fetching a Card with Its Query

```clojure
(t2/select-one :model/Card 
               :id card-id
               {:select [:id :name :dataset_query :result_metadata 
                         :display :visualization_settings :type]})
```

### Creating a Dashboard Card with Series

```clojure
(def ^:private NewDashboardCard
  [:map
   [:dashboard_id ms/PositiveInt]
   [:card_id {:optional true} [:maybe ms/PositiveInt]]
   [:visualization_settings {:optional true} [:maybe map?]]
   [:series {:optional true} [:maybe [:sequential ms/PositiveInt]]]])
```

### Batch Updating Series

```clojure
(defn update-dashboard-cards-series!
  [dashcard-id->card-ids]
  "Batch update DashboardCardSeries. Each card-ids list should be 
   a definitive collection of *all* IDs of cards for the dashboard 
   card in the desired order."
  (t2/delete! :model/DashboardCardSeries 
               :dashboardcard_id [:in (keys dashcard-id->card-ids)])
  (when-let [card-series (seq (for [[dashcard-id card-ids] dashcard-id->card-ids
                                     [i card-id] (map-indexed vector card-ids)]
                                 {:dashboardcard_id dashcard-id
                                  :card_id card-id
                                  :position i}))]
    (t2/insert! :model/DashboardCardSeries card-series)))
```

## Summary

1. **Unit of Truth**: The `dataset_query` field in the `report_card` table is the single source of truth for what data a card retrieves.

2. **Storage**: Cards are stored in `report_card` with:
   - `dataset_query` - The query definition (MBQL or SQL)
   - `result_metadata` - Column structure and types
   - `visualization_settings` - Default visualization configuration
   - `type` - Distinguishes questions, models, and metrics

3. **Multiple Visualizations**: Multiple visualizations can share the same query through:
   - **Dashboard Card Series** - Multiple cards displayed as series in one visualization
   - **Dashboard Card Overrides** - Same card, different visualization settings per dashboard instance
   - **Card Reuse** - The same `card_id` referenced by multiple dashboard cards

4. **Architecture Benefits**:
   - Query changes propagate to all visualizations using the card
   - Visualization settings can be customized per dashboard instance
   - Series enable rich multi-query visualizations
   - Models enable query composition and reuse

This architecture ensures that queries remain the single source of truth while allowing flexible visualization customization at the dashboard level.


