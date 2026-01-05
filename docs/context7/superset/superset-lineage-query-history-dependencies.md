# Apache Superset: Lineage, Query History, and Dataset Dependencies

## Executive Summary

Apache Superset exposes data lineage and traceability through **chart-to-dataset relationships**, **query history tracking**, and **dataset dependency APIs**. Users can trace where a visualization's data originates by following the chain: **Chart → Dataset → Database → Table**. Query history is maintained in SQL Lab with execution metadata, and the chart data API returns the generated SQL query. Dashboard-level lineage shows all datasets used by charts, while dataset details reveal columns, metrics, and relationships. The system provides multiple entry points for tracing data origins: chart metadata, query context, generated SQL, and execution logs.

---

## 1. Data Lineage Overview

### 1.1 Lineage Chain

**Complete data lineage path:**

```
Visualization (Chart)
  ↓
Chart Configuration (datasource_id, datasource_type)
  ↓
Dataset (Table/View)
  ↓
Database Connection
  ↓
Physical Table/View in Database
```

**Example Lineage:**
```
Chart: "Sales by Region" (ID: 42)
  ↓
Dataset: "sales_data" (ID: 15, Type: table)
  ↓
Database: "examples" (ID: 1)
  ↓
Table: "public.sales_data" (PostgreSQL)
```

### 1.2 Lineage Components

**Key Components:**
- **Chart**: Visualization with `datasource_id` and `datasource_type`
- **Dataset**: Table or view definition with columns and metrics
- **Database**: Connection to data source
- **Query Context**: Generated SQL query for chart
- **Query History**: Execution logs with metadata

---

## 2. Chart-to-Dataset Relationships

### 2.1 Chart Metadata

**Charts store datasource information:**

```json
{
  "id": 42,
  "slice_name": "Sales by Region",
  "viz_type": "bar",
  "datasource_id": 15,
  "datasource_type": "table",
  "datasource_name": "sales_data",
  "params": "{\"groupby\":[\"region\"],\"metrics\":[\"sum__sales\"]}",
  "query_context": "{\"datasource\":{\"id\":15,\"type\":\"table\"},\"queries\":[...]}"
}
```

**Key Fields:**
- **datasource_id**: ID of the dataset
- **datasource_type**: Type of datasource (`table`, `query`, etc.)
- **datasource_name**: Name of the dataset
- **params**: Chart configuration (JSON string)
- **query_context**: Query definition (JSON string)

### 2.2 Retrieving Chart Lineage

**API Endpoint:**
```bash
GET /api/v1/chart/{chart_id}
```

**Response:**
```json
{
  "result": {
    "id": 42,
    "datasource_id": 15,
    "datasource_type": "table",
    "datasource_name": "sales_data",
    "query_context": "{\"datasource\":{\"id\":15,\"type\":\"table\"}}"
  }
}
```

**Tracing Steps:**
1. Get chart by ID
2. Extract `datasource_id` and `datasource_type`
3. Query dataset API with `datasource_id`
4. Extract database information from dataset
5. Trace to physical table/view

### 2.3 Chart Query Context

**Query context contains lineage information:**

```json
{
  "datasource": {
    "id": 15,
    "type": "table"
  },
  "queries": [
    {
      "columns": ["region"],
      "metrics": ["sum__sales"],
      "filters": []
    }
  ]
}
```

**Lineage Information:**
- **Datasource ID**: Links to dataset
- **Query Structure**: Shows columns and metrics used
- **Filters**: Applied filters affecting data

---

## 3. Dataset Dependencies

### 3.1 Dataset Details

**Dataset API provides complete lineage:**

```bash
GET /api/v1/dataset/{id}
```

**Response:**
```json
{
  "id": 15,
  "table_name": "sales_data",
  "schema": "public",
  "database": {
    "id": 1,
    "database_name": "examples"
  },
  "columns": [
    {
      "id": 101,
      "column_name": "region",
      "type": "VARCHAR(255)"
    },
    {
      "id": 102,
      "column_name": "sales",
      "type": "NUMERIC(10, 2)"
    }
  ],
  "metrics": [
    {
      "id": 201,
      "metric_name": "sum__sales",
      "expression": "SUM(sales)"
    }
  ],
  "owner_emails": ["admin@superset.com"]
}
```

**Lineage Information:**
- **Database**: Connection to data source
- **Schema**: Database schema name
- **Table Name**: Physical table/view name
- **Columns**: Available columns with types
- **Metrics**: Defined aggregations

### 3.2 Dataset to Database Mapping

**Complete mapping:**

```python
dataset = {
    "id": 15,
    "table_name": "sales_data",
    "schema": "public",
    "database": {
        "id": 1,
        "database_name": "examples"
    }
}

# Full path: examples.public.sales_data
full_path = f"{dataset['database']['database_name']}.{dataset['schema']}.{dataset['table_name']}"
```

### 3.3 Dataset Relationships

**Datasets can reference other datasets:**

- **Views**: SQL views that query other tables
- **Virtual Datasets**: Queries that combine multiple sources
- **Metrics**: Can reference columns from other datasets (via `dataset_id` parameter)

**Example:**
```sql
-- View definition
CREATE VIEW sales_summary AS
SELECT region, SUM(amount) as total
FROM sales_data
GROUP BY region;
```

**Lineage:**
```
sales_summary (view)
  ↓
sales_data (table)
```

---

## 4. Dashboard-Level Lineage

### 4.1 Dashboard Datasets API

**Get all datasets used by dashboard charts:**

```bash
GET /api/v1/dashboard/{dashboard_id_or_slug}/datasets
```

**Response:**
```json
[
  {
    "id": 15,
    "table_name": "sales_data",
    "database": {
      "id": 1,
      "database_name": "examples"
    }
  },
  {
    "id": 16,
    "table_name": "customer_data",
    "database": {
      "id": 1,
      "database_name": "examples"
    }
  }
]
```

**Use Cases:**
- **Impact Analysis**: Which datasets are used by dashboard
- **Dependency Mapping**: All data sources for dashboard
- **Change Management**: Identify affected dashboards when dataset changes

### 4.2 Dashboard to Charts to Datasets

**Complete lineage chain:**

```
Dashboard: "Sales Dashboard" (ID: 10)
  ↓
Charts: [Chart 42, Chart 43, Chart 44]
  ↓
Datasets: [sales_data (15), customer_data (16)]
  ↓
Database: examples (1)
```

**Tracing Process:**
1. Get dashboard by ID
2. Extract chart IDs from `position_json`
3. Get chart details for each chart
4. Extract `datasource_id` from each chart
5. Get dataset details for each datasource
6. Map to database connections

---

## 5. Query History and Execution Logs

### 5.1 SQL Lab Query History

**SQL Lab maintains query execution history:**

```bash
POST /api/v1/sqllab/execute/
```

**Query Response with Metadata:**
```json
{
  "query": {
    "queryId": 42,
    "sql": "SELECT name, SUM(num) as total FROM birth_names WHERE ds >= '1980-01-01' GROUP BY name ORDER BY total DESC LIMIT 10",
    "executedSql": "SELECT name, SUM(num) as total FROM birth_names WHERE ds >= '1980-01-01' GROUP BY name ORDER BY total DESC LIMIT 10",
    "dbId": 1,
    "db": "examples",
    "schema": "public",
    "userId": 1,
    "user": "admin",
    "startDttm": 1705324200.123,
    "endDttm": 1705324200.456,
    "state": "success",
    "rows": 10,
    "limit": 1000
  }
}
```

**Query Metadata:**
- **queryId**: Unique query identifier
- **sql**: Original SQL query
- **executedSql**: Actual executed SQL (may differ due to templating)
- **dbId**: Database ID
- **userId**: User who executed query
- **startDttm/endDttm**: Execution timestamps
- **state**: Execution status (success, failed, running)
- **rows**: Number of rows returned

### 5.2 Query History Storage

**Query history stored in metadata database:**

**Table Structure:**
- **query_id**: Unique identifier
- **sql**: SQL query text
- **database_id**: Database connection
- **user_id**: Executing user
- **start_time**: Query start timestamp
- **end_time**: Query end timestamp
- **status**: Execution status
- **rows**: Rows returned

**Query History Access:**
- **SQL Lab UI**: Recent queries tab
- **API**: Query history endpoints
- **Logs**: Application logs with query details

### 5.3 Chart Query History

**Chart queries also logged:**

**Chart Data API Response:**
```json
{
  "result": [
    {
      "query": "SELECT region, SUM(sales) AS sum__sales FROM sales_data WHERE year = 2024 GROUP BY region ORDER BY sum__sales DESC LIMIT 100",
      "status": "success",
      "rowcount": 10,
      "cache_key": "abc123...",
      "applied_filters": [
        {"column": "year"}
      ]
    }
  ]
}
```

**Chart Query Information:**
- **Generated SQL**: Actual SQL executed
- **Cache Key**: Cache identifier
- **Applied Filters**: Filters that were applied
- **Rejected Filters**: Filters that couldn't be applied

---

## 6. Tracing Data Origins

### 6.1 From Visualization to Source

**Step-by-step tracing:**

**Step 1: Get Chart Details**
```bash
GET /api/v1/chart/42
```

**Response:**
```json
{
  "datasource_id": 15,
  "datasource_type": "table",
  "datasource_name": "sales_data"
}
```

**Step 2: Get Dataset Details**
```bash
GET /api/v1/dataset/15
```

**Response:**
```json
{
  "table_name": "sales_data",
  "schema": "public",
  "database": {
    "id": 1,
    "database_name": "examples"
  }
}
```

**Step 3: Get Database Connection**
```bash
GET /api/v1/database/1
```

**Response:**
```json
{
  "database_name": "examples",
  "sqlalchemy_uri": "postgresql://user:pass@host:5432/examples"
}
```

**Step 4: Physical Table**
```
Database: examples
Schema: public
Table: sales_data
```

### 6.2 From Query to Visualization

**Reverse tracing (query → charts):**

**Step 1: Identify Query**
```sql
SELECT region, SUM(sales) FROM sales_data GROUP BY region
```

**Step 2: Find Dataset**
- Match table name: `sales_data`
- Find dataset with `table_name = 'sales_data'`

**Step 3: Find Charts**
- Query charts with `datasource_id = dataset_id`
- Check `query_context` for matching queries

**Step 4: Find Dashboards**
- Query dashboards containing chart IDs
- Check `position_json` for chart references

### 6.3 Query Context Analysis

**Analyze query context for lineage:**

```json
{
  "datasource": {
    "id": 15,
    "type": "table"
  },
  "queries": [
    {
      "columns": ["region"],
      "metrics": [
        {
          "aggregate": "SUM",
          "column": {"column_name": "sales", "type": "NUMERIC"}
        }
      ],
      "filters": [
        {"col": "year", "op": "==", "val": 2024}
      ]
    }
  ]
}
```

**Lineage Information:**
- **Source Dataset**: ID 15
- **Columns Used**: `region`, `sales`
- **Metrics**: `SUM(sales)`
- **Filters**: `year = 2024`

---

## 7. Generated SQL Queries

### 7.1 Chart Data API SQL

**Chart data API returns generated SQL:**

```bash
POST /api/v1/chart/data
```

**Request:**
```json
{
  "datasource": {"id": 15, "type": "table"},
  "queries": [{
    "columns": ["region"],
    "metrics": ["sum__sales"],
    "filters": [{"col": "year", "op": "==", "val": 2024}]
  }]
}
```

**Response:**
```json
{
  "result": [
    {
      "query": "SELECT region, SUM(sales) AS sum__sales FROM sales_data WHERE year = 2024 GROUP BY region ORDER BY sum__sales DESC LIMIT 100",
      "status": "success",
      "rowcount": 10
    }
  ]
}
```

**SQL Analysis:**
- **Table**: `sales_data` (from dataset)
- **Columns**: `region`, `sales`
- **Aggregation**: `SUM(sales)`
- **Filters**: `year = 2024`
- **Ordering**: `ORDER BY sum__sales DESC`
- **Limit**: `LIMIT 100`

### 7.2 SQL Template Rendering

**SQL may include Jinja templating:**

**Template:**
```sql
SELECT region, {{ metric('sum__sales') }}
FROM {{ table }}
WHERE year = {{ filter_values('year') }}
```

**Rendered SQL:**
```sql
SELECT region, SUM(sales) AS sum__sales
FROM sales_data
WHERE year = 2024
```

**Tracing Templated Queries:**
1. Extract base table from template
2. Resolve metric expressions
3. Resolve filter values
4. Map to dataset columns

### 7.3 Query Execution Details

**Execution metadata in query response:**

```json
{
  "query": "SELECT ...",
  "cache_key": "abc123...",
  "cached_dttm": "2025-01-15T10:30:00",
  "cache_timeout": 86400,
  "applied_filters": [
    {"column": "year"}
  ],
  "rejected_filters": []
}
```

**Execution Information:**
- **Cache Status**: Whether result was cached
- **Cache Key**: Cache identifier
- **Applied Filters**: Filters successfully applied
- **Rejected Filters**: Filters that couldn't be applied

---

## 8. Dataset Dependencies and Relationships

### 8.1 Column Dependencies

**Columns reference dataset:**

```json
{
  "columns": [
    {
      "id": 101,
      "column_name": "region",
      "type": "VARCHAR(255)",
      "dataset_id": 15
    }
  ]
}
```

**Column Lineage:**
- **Column ID**: Unique identifier
- **Dataset ID**: Parent dataset
- **Type**: Data type from database

### 8.2 Metric Dependencies

**Metrics reference columns:**

```json
{
  "metrics": [
    {
      "id": 201,
      "metric_name": "sum__sales",
      "expression": "SUM(sales)",
      "dataset_id": 15
    }
  ]
}
```

**Metric Lineage:**
- **Metric ID**: Unique identifier
- **Expression**: SQL expression
- **Dataset ID**: Parent dataset
- **Column References**: Columns used in expression

### 8.3 Cross-Dataset Metrics

**Metrics can reference other datasets:**

```sql
-- Metric using dataset() macro
{{ metric('total_sales', dataset_id=16) }}
```

**Cross-Dataset Lineage:**
```
Chart
  ↓
Metric (references Dataset 16)
  ↓
Dataset 16 (customer_data)
```

---

## 9. Query History Access Patterns

### 9.1 SQL Lab Query History

**Access recent queries:**

**UI:**
- SQL Lab → Recent Queries tab
- Shows query history with metadata

**API:**
- Query history endpoints
- Filter by user, database, date range

**Query History Entry:**
```json
{
  "queryId": 42,
  "sql": "SELECT ...",
  "database": "examples",
  "user": "admin",
  "startDttm": "2025-01-15T10:30:00",
  "endDttm": "2025-01-15T10:30:05",
  "state": "success",
  "rows": 100
}
```

### 9.2 Chart Query History

**Chart queries logged in execution:**

**Access Points:**
- Chart data API response (includes SQL)
- Application logs
- Cache keys (can trace to queries)

**Chart Query Log:**
```json
{
  "chart_id": 42,
  "query": "SELECT ...",
  "executed_at": "2025-01-15T10:30:00",
  "cache_key": "abc123...",
  "rowcount": 100
}
```

### 9.3 Audit Logging

**Structured audit logs:**

```json
{
  "timestamp": "2025-01-15T10:30:00",
  "user": "admin@example.com",
  "action": "query_dataset",
  "dataset_id": 15,
  "dataset_name": "sales_data",
  "query": "SELECT ...",
  "rows_returned": 100
}
```

**Audit Log Fields:**
- **Timestamp**: When query executed
- **User**: Who executed query
- **Action**: Type of action (query_dataset, create_chart, etc.)
- **Dataset**: Dataset accessed
- **Query**: SQL query executed
- **Rows**: Rows returned

---

## 10. Lineage Visualization Examples

### 10.1 Simple Lineage

**Single chart to table:**

```
Chart: "Sales by Region" (42)
  ↓
Dataset: "sales_data" (15)
  ↓
Database: "examples" (1)
  ↓
Table: "public.sales_data"
```

### 10.2 Dashboard Lineage

**Multiple charts to multiple datasets:**

```
Dashboard: "Sales Dashboard" (10)
  ├─ Chart: "Sales by Region" (42)
  │    └─ Dataset: "sales_data" (15)
  ├─ Chart: "Customer Count" (43)
  │    └─ Dataset: "customer_data" (16)
  └─ Chart: "Revenue Trend" (44)
       └─ Dataset: "sales_data" (15)

Datasets:
  - sales_data (15) → examples.public.sales_data
  - customer_data (16) → examples.public.customer_data
```

### 10.3 Complex Lineage with Views

**Charts using views:**

```
Chart: "Sales Summary" (45)
  ↓
Dataset: "sales_summary" (17) [VIEW]
  ↓
Dataset: "sales_data" (15) [TABLE]
  ↓
Database: "examples" (1)
  ↓
Table: "public.sales_data"
```

---

## 11. API Endpoints for Lineage

### 11.1 Chart Endpoints

**Get chart lineage:**

```bash
# Get chart details
GET /api/v1/chart/{chart_id}

# Get chart data (includes SQL)
POST /api/v1/chart/data
```

**Response includes:**
- `datasource_id`: Dataset ID
- `datasource_type`: Dataset type
- `query_context`: Query definition
- Generated SQL in data response

### 11.2 Dataset Endpoints

**Get dataset lineage:**

```bash
# Get dataset details
GET /api/v1/dataset/{id}

# Update dataset (shows dependencies)
PUT /api/v1/dataset/{id}
```

**Response includes:**
- `database`: Database connection
- `schema`: Database schema
- `table_name`: Physical table name
- `columns`: Column definitions
- `metrics`: Metric definitions

### 11.3 Dashboard Endpoints

**Get dashboard lineage:**

```bash
# Get dashboard datasets
GET /api/v1/dashboard/{dashboard_id}/datasets

# Get dashboard details
GET /api/v1/dashboard/{dashboard_id}
```

**Response includes:**
- List of datasets used by charts
- Chart IDs in `position_json`
- Dashboard metadata

### 11.4 Query History Endpoints

**Get query history:**

```bash
# Execute query (returns query metadata)
POST /api/v1/sqllab/execute/

# Get async query results
GET /api/v1/sqllab/results/?key={key}
```

**Response includes:**
- Query SQL
- Execution metadata
- Database information
- User information

---

## 12. Tracing Workflows

### 12.1 Trace Chart to Source Table

**Complete workflow:**

```python
# 1. Get chart
chart = GET /api/v1/chart/42
datasource_id = chart['datasource_id']

# 2. Get dataset
dataset = GET /api/v1/dataset/{datasource_id}
database_id = dataset['database']['id']
table_name = dataset['table_name']
schema = dataset['schema']

# 3. Get database
database = GET /api/v1/database/{database_id}
database_name = database['database_name']

# 4. Full path
full_path = f"{database_name}.{schema}.{table_name}"
# Result: examples.public.sales_data
```

### 12.2 Trace Query to Charts

**Reverse workflow:**

```python
# 1. Identify table from SQL
sql = "SELECT region, SUM(sales) FROM sales_data GROUP BY region"
table_name = extract_table_name(sql)  # sales_data

# 2. Find dataset
datasets = GET /api/v1/dataset/?q=(table_name: sales_data)
dataset_id = datasets[0]['id']

# 3. Find charts using dataset
charts = GET /api/v1/chart/?q=(datasource_id: {dataset_id})

# 4. Find dashboards with charts
for chart in charts:
    dashboards = GET /api/v1/dashboard/?q=(chart_id: {chart['id']})
```

### 12.3 Impact Analysis

**Find all dependent objects:**

```python
# Given: Dataset ID 15

# 1. Find all charts using dataset
charts = GET /api/v1/chart/?q=(datasource_id: 15)

# 2. Find all dashboards with these charts
dashboard_ids = set()
for chart in charts:
    dashboards = GET /api/v1/dashboard/?q=(chart_id: {chart['id']})
    dashboard_ids.update([d['id'] for d in dashboards])

# 3. Impact summary
impact = {
    "dataset_id": 15,
    "charts_affected": len(charts),
    "dashboards_affected": len(dashboard_ids),
    "chart_ids": [c['id'] for c in charts],
    "dashboard_ids": list(dashboard_ids)
}
```

---

## 13. Metadata Storage

### 13.1 Database Schema

**Lineage stored in metadata database:**

**Tables:**
- **slices**: Charts (with `datasource_id`)
- **tables**: Datasets (with `database_id`)
- **dbs**: Database connections
- **table_columns**: Column definitions (with `dataset_id`)
- **sql_metrics**: Metric definitions (with `dataset_id`)
- **dashboard_slices**: Dashboard-chart relationships

**Relationships:**
```
slices.datasource_id → tables.id
tables.database_id → dbs.id
table_columns.dataset_id → tables.id
sql_metrics.dataset_id → tables.id
dashboard_slices.slice_id → slices.id
```

### 13.2 Query Context Storage

**Query context stored in chart:**

**Storage:**
- **params**: JSON string in `slices.params`
- **query_context**: JSON string in `slices.query_context`

**Example:**
```json
{
  "params": "{\"groupby\":[\"region\"],\"metrics\":[\"sum__sales\"]}",
  "query_context": "{\"datasource\":{\"id\":15,\"type\":\"table\"},\"queries\":[...]}"
}
```

---

## 14. Best Practices for Lineage

### 14.1 Dataset Naming

**Use descriptive names:**
- Clear table names
- Consistent naming conventions
- Include schema information

**Example:**
```
sales_data_q4_2024
customer_orders_staging
revenue_summary_monthly
```

### 14.2 Documentation

**Document dataset lineage:**
- Add descriptions to datasets
- Document column purposes
- Explain metric calculations
- Note data sources

**Example:**
```json
{
  "description": "Sales data aggregated by region. Source: orders table, updated daily.",
  "columns": [
    {
      "column_name": "region",
      "description": "Sales region (US-West, US-East, etc.)"
    }
  ]
}
```

### 14.3 Query Context Preservation

**Preserve query context:**
- Store `query_context` in charts
- Include filter information
- Document metric expressions
- Track query changes

---

## 15. Limitations and Considerations

### 15.1 Current Limitations

**No Automatic Lineage Graph:**
- No built-in lineage visualization
- Must manually trace relationships
- No dependency graph UI

**Limited Cross-Dataset Tracking:**
- Views not automatically tracked
- Virtual datasets lineage not fully captured
- Cross-database relationships not mapped

**Query History Retention:**
- Query history may be purged
- Limited retention period
- No automatic archival

### 15.2 Workarounds

**Manual Lineage Tracking:**
- Use API endpoints to build lineage
- Create custom lineage visualization
- Document relationships manually

**Query History:**
- Export query history regularly
- Use audit logs for long-term tracking
- Implement custom query logging

---

## 16. Conclusion

Superset provides multiple mechanisms for tracing data lineage:

1. **Chart-to-Dataset Mapping**: Charts store `datasource_id` linking to datasets
2. **Dataset Details**: Complete dataset information including database, schema, table
3. **Query History**: SQL Lab maintains execution history with metadata
4. **Generated SQL**: Chart data API returns actual SQL queries
5. **Dashboard Dependencies**: API to get all datasets used by dashboard
6. **Metadata Storage**: Relationships stored in metadata database

**Key Takeaways:**
- **Lineage Chain**: Chart → Dataset → Database → Table
- **API Access**: Multiple endpoints for lineage information
- **Query Tracing**: SQL queries reveal data sources
- **Impact Analysis**: Can trace dependencies in both directions
- **Metadata**: Relationships stored in Superset's metadata database

**Tracing Workflows:**
- **Forward**: Chart → Dataset → Database → Table
- **Reverse**: Query → Dataset → Charts → Dashboards
- **Impact**: Dataset → Charts → Dashboards

**Architecture Strengths:**
- **Clear Relationships**: Well-defined chart-dataset-database hierarchy
- **API Access**: Comprehensive APIs for lineage information
- **Query Visibility**: Generated SQL available for analysis
- **Metadata Storage**: Relationships persisted in database

**Areas for Improvement:**
- Automatic lineage graph visualization
- Better view and virtual dataset tracking
- Enhanced query history retention
- Cross-database relationship mapping

This architecture provides **foundational lineage capabilities** through well-defined relationships and comprehensive APIs, enabling users to trace data origins through multiple entry points and workflows.


