# Apache Superset: Query Execution, Caching, and Refresh Mechanisms

## Executive Summary

When a chart or dashboard is refreshed in Apache Superset, the system builds a **QueryContext** from the chart's configuration, checks the cache using a deterministically generated cache key, executes SQL queries against data sources if cache misses occur, and stores results for future use. Dashboard refreshes trigger parallel queries for all charts, with optional staggering to prevent database overload. Cache invalidation occurs based on timeouts, dataset changes, or explicit cache clearing.

---

## 1. Query Context Building

### 1.1 What is a QueryContext?

A **QueryContext** is a structured object that encapsulates all information needed to execute a query:

```python
query_context_dict = {
    "datasource": {"id": 1, "type": "table"},
    "queries": [
        {
            "columns": ["product_category"],
            "metrics": ["sum__sales"],
            "filters": [
                {"col": "year", "op": "==", "val": 2024}
            ],
            "row_limit": 100,
            "time_range": "2024-01-01 : 2024-12-31",
            "granularity": "ds",
            "orderby": [["sum__sales", False]],
            "extras": {
                "having": "",
                "where": "",
                "time_grain_sqla": "P1D"
            }
        }
    ],
    "result_type": "full",
    "result_format": "json",
    "force": False
}
```

**Key Components:**
- **datasource**: Identifies the dataset (table or SQL query)
- **queries**: Array of query specifications (can be multiple for complex charts)
- **result_type**: Format of results (`"full"`, `"query"`, `"samples"`)
- **result_format**: Data format (`"json"`, `"csv"`, etc.)
- **force**: Whether to bypass cache

### 1.2 Building QueryContext from Chart Configuration

**Chart Storage:**
- Charts store configuration in `params` (JSON string) and `query_context` (JSON string)
- `params` contains visualization-specific settings (colors, chart type, etc.)
- `query_context` contains query execution parameters

**Conversion Flow:**
```
Chart.params (JSON) 
  → Parse to formData
  → Extract query parameters
  → Build QueryContext object
  → Merge with dashboard filters
  → Add native filter values
```

**Example: Chart params to QueryContext**

```python
# Chart.params contains:
{
  "groupby": ["region"],
  "metrics": ["sum__sales"],
  "adhoc_filters": [
    {"col": "year", "op": "==", "val": 2024}
  ],
  "row_limit": 50,
  "time_range": "Last 30 days",
  "viz_type": "dist_bar"
}

# Converted to QueryContext:
{
  "datasource": {"id": 5, "type": "table"},
  "queries": [{
    "columns": ["region"],
    "metrics": ["sum__sales"],
    "filters": [
      {"col": "year", "op": "==", "val": 2024}
    ],
    "row_limit": 50,
    "time_range": "Last 30 days"
  }],
  "result_type": "full",
  "result_format": "json"
}
```

### 1.3 Merging Dashboard Context

When a chart is displayed in a dashboard, additional context is merged:

**Dashboard-Level Filters:**
- Native filters applied to all charts
- URL parameters (e.g., `?native_filters=...`)
- Cross-filter selections from other charts

**Merged QueryContext:**
```python
# Original chart query
chart_query = {
    "filters": [{"col": "year", "op": "==", "val": 2024}]
}

# Dashboard native filter
dashboard_filter = {
    "col": "region",
    "op": "IN",
    "val": ["US-West", "US-East"]
}

# Merged query
merged_query = {
    "filters": [
        {"col": "year", "op": "==", "val": 2024},
        {"col": "region", "op": "IN", "val": ["US-West", "US-East"]}
    ]
}
```

### 1.4 QueryContext Validation

Before execution, QueryContext is validated:

```python
from superset.common.query_context import QueryContext
from superset.commands.chart.data.get_data_command import ChartDataCommand

query_context = QueryContext(**query_context_dict)
command = ChartDataCommand(query_context)
command.validate()  # Checks:
# - Datasource exists and is accessible
# - User has permission to query datasource
# - Query parameters are valid
# - Metrics and columns exist
```

---

## 2. Cache Key Generation

### 2.1 Deterministic Cache Key

Cache keys are generated deterministically from query parameters:

```python
from superset.utils.cache import generate_cache_key

cache_params = {
    "datasource_id": 1,
    "datasource_type": "table",
    "time_range": "Last week",
    "metrics": ["sum__sales"],
    "filters": [{"col": "region", "op": "==", "val": "US"}],
    "columns": ["product_category"],
    "row_limit": 100
}

cache_key = generate_cache_key(cache_params, key_prefix="chart_data_")
# Returns: "chart_data_a1b2c3d4e5f6..."
```

**Key Properties:**
- **Deterministic**: Same parameters → same cache key
- **Unique**: Different parameters → different cache key
- **Includes**: Datasource, filters, metrics, time range, user context (if row-level security)
- **Excludes**: Visualization settings (colors, chart type don't affect cache)

### 2.2 Cache Key Components

**Included in Cache Key:**
- Datasource ID and type
- Query columns
- Metrics (with expressions)
- Filters (column, operator, value)
- Time range
- Row limit
- Granularity
- User ID (for row-level security)
- Custom Jinja variables (if `cache_key_wrapper()` used)

**Excluded from Cache Key:**
- Chart visualization type
- Color schemes
- Chart title/description
- Dashboard layout
- UI state

### 2.3 User Context in Cache Keys

For row-level security, user context is included:

```python
# Cache key includes user_id for security
cache_params = {
    "datasource_id": 1,
    "user_id": 42,  # Included for RLS
    "filters": [...]
}
```

**Jinja Template Variables:**
```sql
-- In dataset SQL
SELECT * FROM sales 
WHERE region = '{{ current_user_id() }}'
-- current_user_id() is automatically included in cache key
```

---

## 3. Cache Lookup and Execution Flow

### 3.1 Complete Query Execution Flow

```
┌─────────────────────────────────────┐
│ 1. Chart/Dashboard Refresh Trigger │
├─────────────────────────────────────┤
│ 2. Build QueryContext               │
│    - Parse chart.params             │
│    - Merge dashboard filters        │
│    - Add native filters             │
├─────────────────────────────────────┤
│ 3. Generate Cache Key              │
│    - Hash query parameters          │
│    - Include user context           │
├─────────────────────────────────────┤
│ 4. Check Cache                      │
│    - Lookup by cache_key            │
│    - Check expiration               │
├─────────────────────────────────────┤
│ 5a. Cache Hit                       │
│     - Return cached result          │
│     - Skip database query           │
├─────────────────────────────────────┤
│ 5b. Cache Miss                      │
│     - Generate SQL from QueryContext│
│     - Execute against datasource    │
│     - Store result in cache         │
│     - Return result                 │
└─────────────────────────────────────┘
```

### 3.2 Cache Lookup Implementation

```python
from superset.commands.chart.data.get_data_command import ChartDataCommand

query_context = QueryContext(**query_context_dict)
command = ChartDataCommand(query_context)
command.validate()

# Execute with caching
result = command.run(cache=True, force_cached=False)

# Result contains:
result = {
    "queries": [
        {
            "query": "SELECT ...",  # Generated SQL
            "data": [...],          # Query results
            "rowcount": 100,
            "cache_key": "abc123...",
            "cached_dttm": "2025-01-15T10:30:00",  # If from cache
            "cache_timeout": 3600
        }
    ]
}
```

**Cache Parameters:**
- `cache=True`: Enable caching (check cache, store results)
- `force_cached=False`: Allow cache misses (execute query if not cached)
- `force=True`: Bypass cache entirely (always execute query)

### 3.3 SQL Generation

When cache misses, Superset generates SQL from QueryContext:

**QueryContext → SQL Conversion:**
```python
# QueryContext
{
    "queries": [{
        "columns": ["region"],
        "metrics": ["sum__sales"],
        "filters": [{"col": "year", "op": "==", "val": 2024}],
        "row_limit": 50,
        "orderby": [["sum__sales", False]]
    }]
}

# Generated SQL
SELECT 
    region,
    SUM(sales) AS sum__sales
FROM sales_data
WHERE year = 2024
GROUP BY region
ORDER BY sum__sales DESC
LIMIT 50
```

**SQL Generation Process:**
1. **Select Clause**: Columns + metrics
2. **From Clause**: Dataset table/view
3. **Where Clause**: Filters converted to SQL conditions
4. **Group By**: Non-metric columns
5. **Having Clause**: Metric-based filters
6. **Order By**: Sorting criteria
7. **Limit**: Row limit

**Database-Specific Adaptations:**
- Uses database engine specs for dialect-specific SQL
- Handles unsupported features (e.g., subqueries, JOINs)
- Applies database-specific optimizations

---

## 4. Query Execution

### 4.1 Synchronous Execution

**Default Behavior:**
- Queries execute synchronously in the request thread
- Results returned immediately
- Suitable for fast queries (< 60 seconds default timeout)

```python
# Synchronous execution
result = command.run(cache=True, force_cached=False)
# Blocks until query completes
data = result["queries"][0]["data"]
```

**Timeout Configuration:**
```python
SUPERSET_WEBSERVER_TIMEOUT = 60  # seconds
```

### 4.2 Asynchronous Execution

**For Long-Running Queries:**
- Queries can run asynchronously via Celery
- Returns job ID immediately
- Client polls for results

```python
# Asynchronous execution
query_result = {
    "status": "running",
    "query": {
        "queryId": 43,
        "resultsKey": "async_result_key_abc123"
    }
}

# Client polls for results
GET /api/v1/sqllab/results/?key=async_result_key_abc123
```

**Celery Configuration:**
```python
class CeleryConfig(object):
    broker_url = "redis://localhost:6379/0"
    result_backend = "redis://localhost:6379/0"
    imports = (
        "superset.sql_lab",
        "superset.tasks.scheduler",
    )

CELERY_CONFIG = CeleryConfig
```

**Results Backend:**
- Redis: Fast, in-memory storage
- S3: For large result sets
- Database: Persistent storage

### 4.3 Query Execution Against Data Sources

**Connection Pooling:**
- SQLAlchemy connection pools per database
- Reuses connections for efficiency
- Handles connection failures gracefully

**Execution Steps:**
1. **Get Database Connection**: From connection pool
2. **Prepare SQL**: Apply database-specific formatting
3. **Execute Query**: Using SQLAlchemy engine
4. **Fetch Results**: Stream or fetch all rows
5. **Transform Data**: Convert to requested format (JSON, CSV)
6. **Return Results**: With metadata (columns, rowcount, etc.)

**Error Handling:**
- Connection errors: Retry or fail gracefully
- SQL errors: Return error message in response
- Timeout errors: Return partial results or error

---

## 5. Cache Storage and Management

### 5.1 Cache Backends

**Supported Backends:**
- **Redis**: Recommended for production
- **Memcached**: Alternative in-memory cache
- **SupersetMetastoreCache**: Uses metadata database (not recommended for production)
- **Filesystem**: Local file-based cache

**Configuration:**
```python
DATA_CACHE_CONFIG = {
    "CACHE_TYPE": "RedisCache",
    "CACHE_REDIS_URL": "redis://localhost:6379/2",
    "CACHE_KEY_PREFIX": "superset_results",
    "CACHE_DEFAULT_TIMEOUT": 3600,  # 1 hour
}
```

### 5.2 Cache Storage

**Stored Data:**
```python
cache_value = {
    "data": [...],           # Query results
    "query": "SELECT ...",   # Generated SQL
    "rowcount": 100,
    "columns": [...],        # Column metadata
    "cache_key": "abc123...",
    "cached_dttm": "2025-01-15T10:30:00"
}
```

**Storage with Logging:**
```python
from superset.utils.cache import set_and_log_cache

set_and_log_cache(
    cache_instance=cache_manager.data_cache,
    cache_key="chart_data_abc123",
    cache_value=query_result,
    cache_timeout=3600,
    datasource_uid="1__table"
)
# Automatically:
# - Stores in cache
# - Logs metadata to database
# - Tracks cache metrics
```

### 5.3 Cache Timeout

**Timeout Hierarchy:**
1. **Chart-level**: `chart.cache_timeout` (if set)
2. **Dataset-level**: `dataset.cache_timeout` (if set)
3. **Global default**: `DATA_CACHE_CONFIG["CACHE_DEFAULT_TIMEOUT"]`

**Timeout Examples:**
```python
# Chart with 1 hour cache
chart.cache_timeout = 3600

# Dataset with 24 hour cache
dataset.cache_timeout = 86400

# Global default: 1 hour
DATA_CACHE_CONFIG = {
    "CACHE_DEFAULT_TIMEOUT": 3600
}
```

---

## 6. Cache Invalidation

### 6.1 Time-Based Expiration

**Automatic Expiration:**
- Cache entries expire after `cache_timeout` seconds
- Expired entries are not returned (cache miss)
- Expired entries may be cleaned up by cache backend

**Expiration Check:**
```python
# On cache lookup
cached_result = cache.get(cache_key)
if cached_result:
    cached_dttm = cached_result.get("cached_dttm")
    age = (now() - cached_dttm).total_seconds()
    if age < cache_timeout:
        return cached_result  # Cache hit
    else:
        # Expired, treat as cache miss
        pass
```

### 6.2 Manual Cache Invalidation

**Force Refresh:**
```python
# Force bypass cache
result = command.run(cache=True, force_cached=False, force=True)
# Always executes query, ignores cache
```

**Clear Cache:**
- No built-in API for clearing specific cache entries
- Cache backends may support manual clearing
- Dataset/chart changes don't automatically invalidate cache

### 6.3 Cache Invalidation Strategies

**Current Limitations:**
- **No automatic invalidation** on dataset changes
- **No dependency tracking** (changing dataset doesn't clear related caches)
- **Time-based only** (expiration-based invalidation)

**Workarounds:**
1. **Short timeouts**: Use short cache_timeout for frequently changing data
2. **Force refresh**: Use `force=True` parameter to bypass cache
3. **Manual clearing**: Clear cache backend manually when data changes
4. **Versioned cache keys**: Include dataset version in cache key (custom implementation)

---

## 7. Dashboard Refresh Mechanisms

### 7.1 Dashboard Refresh Triggers

**Manual Refresh:**
- User clicks "Refresh" button
- All charts refresh simultaneously (or staggered)

**Automatic Refresh:**
- Timed refresh based on `refresh_frequency` in dashboard metadata
- Configurable per dashboard

**Dashboard Metadata:**
```json
{
  "refresh_frequency": 30,  // seconds
  "timed_refresh_immune_slices": [324],  // Charts to skip
  "stagger_refresh": false,
  "stagger_time": 2500
}
```

### 7.2 Chart Refresh Flow

**Dashboard → Charts:**
1. **Load Dashboard**: Fetch dashboard configuration
2. **Extract Chart IDs**: Parse `position_json` for all `chartId` values
3. **Load Charts**: Fetch chart configurations
4. **Build QueryContexts**: One per chart
5. **Execute Queries**: Parallel or staggered execution
6. **Render Charts**: Display results

**Frontend Flow:**
```typescript
// Dashboard refresh triggers chart queries
async function refreshDashboard(dashboardId: number) {
  const dashboard = await fetchDashboard(dashboardId);
  const chartIds = extractChartIds(dashboard.position_json);
  
  // Execute queries for all charts
  const chartQueries = chartIds.map(chartId => 
    fetchChartData(chartId, formData, dispatch)
  );
  
  // Wait for all queries (or stagger)
  await Promise.all(chartQueries);
}
```

### 7.3 Staggered Refresh

**Purpose**: Prevent database overload when refreshing many charts

**Configuration:**
```json
{
  "stagger_refresh": true,
  "stagger_time": 2500  // milliseconds between chart refreshes
}
```

**Behavior:**
- Charts refresh sequentially with delay
- Reduces concurrent database connections
- Improves stability for large dashboards

**Example:**
```
Time  Chart
----  -----
T0    Chart 1 starts
T2.5  Chart 2 starts (Chart 1 may still be running)
T5.0  Chart 3 starts
T7.5  Chart 4 starts
```

### 7.4 Immune Slices

**Exclude Charts from Refresh:**
```json
{
  "timed_refresh_immune_slices": [324, 325]
}
```

**Use Cases:**
- Charts with very slow queries
- Charts that don't need frequent updates
- Charts that change infrequently

**Behavior:**
- Immune charts are skipped during automatic refresh
- Can still be manually refreshed
- Useful for expensive queries

---

## 8. Cache Warming

### 8.1 Pre-populating Cache

**Purpose**: Improve dashboard load performance by pre-caching queries

**Chart Cache Warming:**
```python
from superset.commands.chart.warm_up_cache import ChartWarmUpCacheCommand

command = ChartWarmUpCacheCommand(
    chart_id=42,
    dashboard_id=10,
    extra_filters=[
        {"col": "region", "op": "==", "val": "US-West"}
    ]
)
result = command.run()

if result["success"]:
    print(f"✓ Cache warmed for chart 42")
    print(f"  Queries cached: {len(result['queries'])}")
```

**Dataset Cache Warming:**
```python
from superset.commands.dataset.warm_up_cache import DatasetWarmUpCacheCommand

command = DatasetWarmUpCacheCommand(
    dataset_id=1,
    dashboard_id=10
)
result = command.run()
print(f"Cached {result['chart_count']} charts using dataset")
```

### 8.2 Warm-Up Strategies

**Scheduled Warm-Up:**
- Use Celery Beat to schedule cache warming
- Warm cache before peak usage times
- Pre-cache common filter combinations

**On-Demand Warm-Up:**
- Warm cache when dashboard is published
- Warm cache after dataset schema changes
- Warm cache for embedded dashboards

---

## 9. Query Context Examples

### 9.1 Simple Bar Chart

**Chart Configuration:**
```json
{
  "groupby": ["region"],
  "metrics": ["sum__sales"],
  "viz_type": "dist_bar"
}
```

**QueryContext:**
```json
{
  "datasource": {"id": 1, "type": "table"},
  "queries": [{
    "columns": ["region"],
    "metrics": ["sum__sales"],
    "row_limit": 100
  }]
}
```

**Generated SQL:**
```sql
SELECT 
    region,
    SUM(sales) AS sum__sales
FROM sales_data
GROUP BY region
ORDER BY sum__sales DESC
LIMIT 100
```

### 9.2 Complex Query with Filters

**Chart Configuration:**
```json
{
  "groupby": ["product_category", "region"],
  "metrics": ["sum__sales", "avg__price"],
  "adhoc_filters": [
    {"col": "year", "op": "==", "val": 2024},
    {"col": "status", "op": "IN", "val": ["active", "pending"]}
  ],
  "time_range": "Last 90 days",
  "row_limit": 500
}
```

**QueryContext:**
```json
{
  "datasource": {"id": 1, "type": "table"},
  "queries": [{
    "columns": ["product_category", "region"],
    "metrics": ["sum__sales", "avg__price"],
    "filters": [
      {"col": "year", "op": "==", "val": 2024},
      {"col": "status", "op": "IN", "val": ["active", "pending"]}
    ],
    "time_range": "Last 90 days",
    "row_limit": 500
  }]
}
```

**Generated SQL:**
```sql
SELECT 
    product_category,
    region,
    SUM(sales) AS sum__sales,
    AVG(price) AS avg__price
FROM sales_data
WHERE 
    year = 2024
    AND status IN ('active', 'pending')
    AND order_date >= DATE_SUB(NOW(), INTERVAL 90 DAY)
GROUP BY product_category, region
ORDER BY sum__sales DESC
LIMIT 500
```

### 9.3 Dashboard with Native Filters

**Dashboard Native Filter:**
```json
{
  "id": "FILTER_REGION",
  "targets": [{"datasetId": 1, "column": {"name": "region"}}],
  "filterState": {"value": ["US-West", "US-East"]}
}
```

**Chart QueryContext (with filter applied):**
```json
{
  "datasource": {"id": 1, "type": "table"},
  "queries": [{
    "columns": ["product_category"],
    "metrics": ["sum__sales"],
    "filters": [
      {"col": "region", "op": "IN", "val": ["US-West", "US-East"]}
    ]
  }]
}
```

---

## 10. Performance Optimizations

### 10.1 Cache Hit Optimization

**Benefits:**
- **Zero database load**: Cached queries don't hit database
- **Fast response**: Sub-millisecond cache retrieval
- **Reduced costs**: Fewer database queries = lower costs

**Cache Hit Rate:**
- Monitor cache hit rate via metrics
- Optimize cache timeout based on data freshness requirements
- Use cache warming for predictable queries

### 10.2 Query Optimization

**Database-Specific:**
- Uses database engine specs for optimal SQL generation
- Applies database-specific optimizations
- Handles unsupported features gracefully

**Connection Pooling:**
- Reuses database connections
- Reduces connection overhead
- Handles connection failures

### 10.3 Parallel Execution

**Dashboard Charts:**
- Charts can execute in parallel (if not staggered)
- Limited by database connection pool
- Staggered refresh prevents overload

**Async Queries:**
- Long-running queries use Celery
- Doesn't block web server
- Better for queries > 60 seconds

---

## 11. Error Handling

### 11.1 Query Execution Errors

**Error Response:**
```json
{
  "result": [{
    "status": "error",
    "error": "Connection failed: ...",
    "query": "SELECT ...",
    "data": null
  }]
}
```

**Error Types:**
- **Connection errors**: Database unavailable
- **SQL errors**: Invalid SQL, syntax errors
- **Permission errors**: User lacks access
- **Timeout errors**: Query exceeded timeout

### 11.2 Cache Errors

**Cache Miss Handling:**
- Cache miss → execute query (expected behavior)
- No error, just slower response

**Cache Backend Errors:**
- If cache backend fails, queries still execute
- Degrades gracefully (no caching, but still functional)
- Logs errors for monitoring

---

## 12. Best Practices

### 12.1 Cache Configuration

**Recommended Settings:**
```python
# Production cache configuration
DATA_CACHE_CONFIG = {
    "CACHE_TYPE": "RedisCache",
    "CACHE_REDIS_URL": "redis://localhost:6379/2",
    "CACHE_KEY_PREFIX": "superset_results",
    "CACHE_DEFAULT_TIMEOUT": 3600,  # 1 hour
}
```

**Cache Timeout Guidelines:**
- **Real-time data**: 60-300 seconds
- **Hourly updates**: 1800-3600 seconds
- **Daily updates**: 86400 seconds
- **Static data**: 604800 seconds (1 week)

### 12.2 Query Optimization

**Reduce Query Complexity:**
- Limit row counts appropriately
- Use appropriate time ranges
- Avoid unnecessary columns
- Use efficient filters

**Database Optimization:**
- Index columns used in filters
- Partition tables by time (if applicable)
- Use materialized views for complex queries

### 12.3 Dashboard Design

**Staggered Refresh:**
- Enable for dashboards with > 10 charts
- Set appropriate stagger_time (2-5 seconds)
- Mark expensive charts as immune

**Cache Warming:**
- Warm cache for published dashboards
- Pre-cache common filter combinations
- Schedule warm-up before peak usage

---

## 13. Monitoring and Debugging

### 13.1 Cache Metrics

**Track:**
- Cache hit rate
- Cache miss rate
- Average cache age
- Cache size/eviction rate

**Logging:**
- Cache operations logged to metadata database
- Track cache_key, datasource_uid, timestamps
- Monitor for cache performance issues

### 13.2 Query Performance

**Monitor:**
- Query execution time
- Database connection pool usage
- Query timeout rate
- Error rate

**Debugging:**
- Check generated SQL in query results
- Review cache keys for correctness
- Monitor database query logs
- Use Celery Flower for async queries

---

## 14. Conclusion

Superset's query execution system provides:

1. **Efficient Caching**: Deterministic cache keys, multiple backends, configurable timeouts
2. **Flexible Query Building**: QueryContext abstraction, dashboard filter merging, SQL generation
3. **Scalable Execution**: Synchronous and asynchronous modes, connection pooling, staggered refresh
4. **Performance Optimization**: Cache warming, parallel execution, database-specific optimizations

**Key Takeaways:**
- QueryContext is the central abstraction for all queries
- Cache keys are deterministic and include all query parameters
- Dashboard refreshes trigger parallel chart queries (or staggered)
- Cache invalidation is time-based only (no automatic invalidation on data changes)
- Asynchronous execution available for long-running queries via Celery

**Limitations:**
- No automatic cache invalidation on dataset changes
- No dependency tracking between datasets and cached queries
- Cache warming requires manual setup
- Staggered refresh is all-or-nothing (can't stagger specific charts)

This architecture prioritizes **performance and simplicity** over **fine-grained cache control**, which aligns with Superset's goal of providing fast, responsive dashboards for business intelligence use cases.


