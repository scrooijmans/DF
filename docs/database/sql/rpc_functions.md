# Available RPC Functions

## Querying Available RPC Functions

To see all available RPC functions in your Supabase database:

```sql
-- List all RPC functions
SELECT 
    p.proname as function_name,
    pg_get_function_arguments(p.oid) as arguments,
    pg_get_function_result(p.oid) as return_type,
    d.description as description
FROM pg_proc p
LEFT JOIN pg_description d ON p.oid = d.objoid
LEFT JOIN pg_namespace n ON p.pronamespace = n.oid
WHERE n.nspname = 'public'
ORDER BY p.proname;
```

## Recommended Approach: RPC vs Direct PostgreSQL

For the MudRock project, I recommend using **RPC functions** for the following reasons:

### Why RPC is Better:
1. **Security**: RPC functions can have Row Level Security (RLS) policies applied
2. **Performance**: Functions can be optimized and cached
3. **Maintainability**: Business logic is centralized in the database
4. **Type Safety**: Functions have defined input/output types
5. **Reusability**: Functions can be called from multiple clients

### Required RPC Functions for Basin Queries (Updated for Actual Schema):

```sql
-- Function to get wells in a basin using PostGIS spatial queries
-- Based on actual schema: wells.location (POINT) and basins.outline (POLYGON)
CREATE OR REPLACE FUNCTION get_wells_in_basin(
    basin_name TEXT,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    basin_name TEXT
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id as well_id,
        w.name as well_name,
        ST_Y(w.location) as latitude,
        ST_X(w.location) as longitude,
        b.name as basin_name
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    ORDER BY w.name;
END;
$$;

-- Function to get first and last wells in a basin (alphabetically by name)
CREATE OR REPLACE FUNCTION get_first_last_wells_in_basin(
    basin_name TEXT,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    position TEXT -- 'first' or 'last'
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    WITH basin_wells AS (
        SELECT 
            w.id,
            w.name,
            ROW_NUMBER() OVER (ORDER BY w.name) as rn,
            COUNT(*) OVER () as total_count
        FROM wells w
        JOIN basins b ON ST_Contains(b.outline, w.location)
        WHERE b.name ILIKE basin_name
        AND (project_id IS NULL OR w.project_id = project_id)
    )
    SELECT 
        id as well_id,
        name as well_name,
        CASE 
            WHEN rn = 1 THEN 'first'
            WHEN rn = total_count THEN 'last'
        END as position
    FROM basin_wells
    WHERE rn = 1 OR rn = total_count;
END;
$$;

-- Function to get wells with their logs in a basin
CREATE OR REPLACE FUNCTION get_wells_with_logs_in_basin(
    basin_name TEXT,
    project_id UUID DEFAULT NULL,
    log_type_filter TEXT DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    log_id INTEGER,
    log_name TEXT,
    log_type TEXT,
    parquet_file_path TEXT
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id as well_id,
        w.name as well_name,
        l.id as log_id,
        l.name as log_name,
        lt.name as log_type,
        l.parquet_file_path
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    JOIN logs l ON l.well_id = w.id
    JOIN log_types lt ON lt.id = l.log_type_id
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    AND (log_type_filter IS NULL OR lt.name ILIKE '%' || log_type_filter || '%')
    ORDER BY w.name, l.name;
END;
$$;

-- Function to get log data for wells in a basin within depth range
CREATE OR REPLACE FUNCTION get_log_data_for_wells_in_basin(
    basin_name TEXT,
    project_id UUID DEFAULT NULL,
    depth_min NUMERIC DEFAULT NULL,
    depth_max NUMERIC DEFAULT NULL,
    log_type_filter TEXT DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    log_id INTEGER,
    log_name TEXT,
    log_type TEXT,
    depth NUMERIC,
    value NUMERIC
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id as well_id,
        w.name as well_name,
        l.id as log_id,
        l.name as log_name,
        lt.name as log_type,
        ld.depth,
        ld.value
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    JOIN logs l ON l.well_id = w.id
    JOIN log_types lt ON lt.id = l.log_type_id
    JOIN log_data ld ON ld.log_id = l.id
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    AND (depth_min IS NULL OR ld.depth >= depth_min)
    AND (depth_max IS NULL OR ld.depth <= depth_max)
    AND (log_type_filter IS NULL OR lt.name ILIKE '%' || log_type_filter || '%')
    ORDER BY w.name, l.name, ld.depth;
END;
$$;

-- Function to get wells with specific log types in a basin
CREATE OR REPLACE FUNCTION get_wells_by_log_type_in_basin(
    basin_name TEXT,
    log_type_name TEXT,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    log_id INTEGER,
    log_name TEXT,
    log_type TEXT,
    unit TEXT
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id as well_id,
        w.name as well_name,
        l.id as log_id,
        l.name as log_name,
        lt.name as log_type,
        lt.unit
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    JOIN logs l ON l.well_id = w.id
    JOIN log_types lt ON lt.id = l.log_type_id
    WHERE b.name ILIKE basin_name
    AND lt.name ILIKE log_type_name
    AND (project_id IS NULL OR w.project_id = project_id)
    ORDER BY w.name;
END;
$$;

-- ============================================================================
-- ADDITIONAL RPC FUNCTIONS FOR COMPLEX USER QUERIES
-- ============================================================================

-- Function to get well statistics and summary information
CREATE OR REPLACE FUNCTION get_well_statistics_in_basin(
    basin_name TEXT,
    project_id UUID DEFAULT NULL,
    include_log_stats BOOLEAN DEFAULT TRUE
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    total_logs INTEGER,
    log_types TEXT[],
    avg_depth_min NUMERIC,
    avg_depth_max NUMERIC,
    data_points_count BIGINT,
    last_updated TIMESTAMP
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id as well_id,
        w.name as well_name,
        COUNT(DISTINCT l.id) as total_logs,
        ARRAY_AGG(DISTINCT lt.name ORDER BY lt.name) as log_types,
        AVG(l.depth_min) as avg_depth_min,
        AVG(l.depth_max) as avg_depth_max,
        COALESCE(SUM(ld.data_points), 0) as data_points_count,
        MAX(l.updated_at) as last_updated
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    LEFT JOIN logs l ON l.well_id = w.id
    LEFT JOIN log_types lt ON lt.id = l.log_type_id
    LEFT JOIN (
        SELECT log_id, COUNT(*) as data_points 
        FROM log_data 
        GROUP BY log_id
    ) ld ON ld.log_id = l.id
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    GROUP BY w.id, w.name
    ORDER BY w.name;
END;
$$;

-- Function to get log correlation analysis between wells
CREATE OR REPLACE FUNCTION get_log_correlation_analysis(
    basin_name TEXT,
    log_type_name TEXT,
    depth_min NUMERIC DEFAULT NULL,
    depth_max NUMERIC DEFAULT NULL,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    log_id INTEGER,
    log_name TEXT,
    avg_value NUMERIC,
    min_value NUMERIC,
    max_value NUMERIC,
    std_dev NUMERIC,
    data_points INTEGER,
    depth_range TEXT
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id as well_id,
        w.name as well_name,
        l.id as log_id,
        l.name as log_name,
        AVG(ld.value) as avg_value,
        MIN(ld.value) as min_value,
        MAX(ld.value) as max_value,
        STDDEV(ld.value) as std_dev,
        COUNT(*) as data_points,
        CONCAT(MIN(ld.depth), ' - ', MAX(ld.depth)) as depth_range
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    JOIN logs l ON l.well_id = w.id
    JOIN log_types lt ON lt.id = l.log_type_id
    JOIN log_data ld ON ld.log_id = l.id
    WHERE b.name ILIKE basin_name
    AND lt.name ILIKE log_type_name
    AND (project_id IS NULL OR w.project_id = project_id)
    AND (depth_min IS NULL OR ld.depth >= depth_min)
    AND (depth_max IS NULL OR ld.depth <= depth_max)
    GROUP BY w.id, w.name, l.id, l.name
    ORDER BY w.name, l.name;
END;
$$;

-- Function to find wells with missing or incomplete log data
CREATE OR REPLACE FUNCTION get_wells_with_data_gaps(
    basin_name TEXT,
    required_log_types TEXT[] DEFAULT NULL,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    missing_log_types TEXT[],
    incomplete_logs TEXT[],
    data_quality_score NUMERIC
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    WITH well_logs AS (
        SELECT 
            w.id as well_id,
            w.name as well_name,
            ARRAY_AGG(DISTINCT lt.name) as available_log_types,
            COUNT(DISTINCT l.id) as total_logs,
            AVG(ld.data_points) as avg_data_points
        FROM wells w
        JOIN basins b ON ST_Contains(b.outline, w.location)
        LEFT JOIN logs l ON l.well_id = w.id
        LEFT JOIN log_types lt ON lt.id = l.log_type_id
        LEFT JOIN (
            SELECT log_id, COUNT(*) as data_points 
            FROM log_data 
            GROUP BY log_id
        ) ld ON ld.log_id = l.id
        WHERE b.name ILIKE basin_name
        AND (project_id IS NULL OR w.project_id = project_id)
        GROUP BY w.id, w.name
    ),
    missing_analysis AS (
        SELECT 
            well_id,
            well_name,
            available_log_types,
            total_logs,
            avg_data_points,
            CASE 
                WHEN required_log_types IS NULL THEN ARRAY[]::TEXT[]
                ELSE required_log_types - available_log_types
            END as missing_log_types,
            CASE 
                WHEN avg_data_points < 100 THEN ARRAY['Low data density']
                WHEN avg_data_points < 500 THEN ARRAY['Medium data density']
                ELSE ARRAY[]::TEXT[]
            END as incomplete_logs,
            CASE 
                WHEN total_logs = 0 THEN 0.0
                WHEN required_log_types IS NULL THEN 
                    CASE 
                        WHEN avg_data_points >= 1000 THEN 1.0
                        WHEN avg_data_points >= 500 THEN 0.8
                        WHEN avg_data_points >= 100 THEN 0.6
                        ELSE 0.3
                    END
                ELSE 
                    (ARRAY_LENGTH(required_log_types, 1) - ARRAY_LENGTH(required_log_types - available_log_types, 1))::NUMERIC / ARRAY_LENGTH(required_log_types, 1)
            END as data_quality_score
        FROM well_logs
    )
    SELECT 
        well_id,
        well_name,
        missing_log_types,
        incomplete_logs,
        data_quality_score
    FROM missing_analysis
    WHERE ARRAY_LENGTH(missing_log_types, 1) > 0 OR ARRAY_LENGTH(incomplete_logs, 1) > 0
    ORDER BY data_quality_score ASC, well_name;
END;
$$;

-- Function to get basin comparison statistics
CREATE OR REPLACE FUNCTION compare_basins_statistics(
    basin_names TEXT[],
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    basin_name TEXT,
    well_count INTEGER,
    total_logs INTEGER,
    avg_logs_per_well NUMERIC,
    log_types TEXT[],
    avg_depth_min NUMERIC,
    avg_depth_max NUMERIC,
    data_coverage_score NUMERIC
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    SELECT 
        b.name as basin_name,
        COUNT(DISTINCT w.id) as well_count,
        COUNT(DISTINCT l.id) as total_logs,
        ROUND(COUNT(DISTINCT l.id)::NUMERIC / COUNT(DISTINCT w.id), 2) as avg_logs_per_well,
        ARRAY_AGG(DISTINCT lt.name ORDER BY lt.name) as log_types,
        AVG(l.depth_min) as avg_depth_min,
        AVG(l.depth_max) as avg_depth_max,
        ROUND(
            (COUNT(DISTINCT w.id) * COUNT(DISTINCT lt.id))::NUMERIC / 
            (SELECT COUNT(*) FROM log_types), 2
        ) as data_coverage_score
    FROM basins b
    JOIN wells w ON ST_Contains(b.outline, w.location)
    LEFT JOIN logs l ON l.well_id = w.id
    LEFT JOIN log_types lt ON lt.id = l.log_type_id
    WHERE b.name = ANY(basin_names)
    AND (project_id IS NULL OR w.project_id = project_id)
    GROUP BY b.name
    ORDER BY b.name;
END;
$$;

-- Function to get depth-based analysis for specific intervals
CREATE OR REPLACE FUNCTION get_depth_interval_analysis(
    basin_name TEXT,
    depth_intervals NUMERIC[][], -- Array of [min_depth, max_depth] pairs
    log_type_filter TEXT DEFAULT NULL,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    depth_interval TEXT,
    log_type TEXT,
    avg_value NUMERIC,
    min_value NUMERIC,
    max_value NUMERIC,
    data_points INTEGER,
    interval_thickness NUMERIC
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    interval_record NUMERIC[];
BEGIN
    -- Process each depth interval
    FOREACH interval_record IN ARRAY depth_intervals
    LOOP
        RETURN QUERY
        SELECT 
            w.id as well_id,
            w.name as well_name,
            CONCAT(interval_record[1], ' - ', interval_record[2]) as depth_interval,
            lt.name as log_type,
            AVG(ld.value) as avg_value,
            MIN(ld.value) as min_value,
            MAX(ld.value) as max_value,
            COUNT(*) as data_points,
            (interval_record[2] - interval_record[1]) as interval_thickness
        FROM wells w
        JOIN basins b ON ST_Contains(b.outline, w.location)
        JOIN logs l ON l.well_id = w.id
        JOIN log_types lt ON lt.id = l.log_type_id
        JOIN log_data ld ON ld.log_id = l.id
        WHERE b.name ILIKE basin_name
        AND (project_id IS NULL OR w.project_id = project_id)
        AND (log_type_filter IS NULL OR lt.name ILIKE log_type_filter)
        AND ld.depth >= interval_record[1]
        AND ld.depth <= interval_record[2]
        GROUP BY w.id, w.name, lt.name, interval_record[1], interval_record[2]
        ORDER BY w.name, lt.name;
    END LOOP;
END;
$$;

-- Function to get wells with specific log value ranges (for anomaly detection)
CREATE OR REPLACE FUNCTION find_wells_with_log_anomalies(
    basin_name TEXT,
    log_type_name TEXT,
    value_min NUMERIC DEFAULT NULL,
    value_max NUMERIC DEFAULT NULL,
    anomaly_threshold NUMERIC DEFAULT 2.0, -- Standard deviations from mean
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    well_id INTEGER,
    well_name TEXT,
    log_id INTEGER,
    log_name TEXT,
    anomaly_type TEXT,
    anomaly_value NUMERIC,
    normal_range TEXT,
    depth_at_anomaly NUMERIC
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    RETURN QUERY
    WITH log_statistics AS (
        SELECT 
            lt.name as log_type,
            AVG(ld.value) as mean_value,
            STDDEV(ld.value) as std_dev,
            MIN(ld.value) as min_value,
            MAX(ld.value) as max_value
        FROM wells w
        JOIN basins b ON ST_Contains(b.outline, w.location)
        JOIN logs l ON l.well_id = w.id
        JOIN log_types lt ON lt.id = l.log_type_id
        JOIN log_data ld ON ld.log_id = l.id
        WHERE b.name ILIKE basin_name
        AND lt.name ILIKE log_type_name
        AND (project_id IS NULL OR w.project_id = project_id)
        GROUP BY lt.name
    ),
    anomaly_data AS (
        SELECT 
            w.id as well_id,
            w.name as well_name,
            l.id as log_id,
            l.name as log_name,
            ld.value,
            ld.depth,
            stats.mean_value,
            stats.std_dev,
            CASE 
                WHEN ld.value > stats.mean_value + (anomaly_threshold * stats.std_dev) THEN 'High'
                WHEN ld.value < stats.mean_value - (anomaly_threshold * stats.std_dev) THEN 'Low'
                ELSE 'Normal'
            END as anomaly_type,
            CONCAT(
                ROUND(stats.mean_value - (anomaly_threshold * stats.std_dev), 2),
                ' - ',
                ROUND(stats.mean_value + (anomaly_threshold * stats.std_dev), 2)
            ) as normal_range
        FROM wells w
        JOIN basins b ON ST_Contains(b.outline, w.location)
        JOIN logs l ON l.well_id = w.id
        JOIN log_types lt ON lt.id = l.log_type_id
        JOIN log_data ld ON ld.log_id = l.id
        CROSS JOIN log_statistics stats
        WHERE b.name ILIKE basin_name
        AND lt.name ILIKE log_type_name
        AND (project_id IS NULL OR w.project_id = project_id)
        AND (value_min IS NULL OR ld.value >= value_min)
        AND (value_max IS NULL OR ld.value <= value_max)
    )
    SELECT 
        well_id,
        well_name,
        log_id,
        log_name,
        anomaly_type,
        value as anomaly_value,
        normal_range,
        depth as depth_at_anomaly
    FROM anomaly_data
    WHERE anomaly_type != 'Normal'
    ORDER BY well_name, depth;
END;
$$;

-- Function to get comprehensive well summary for a basin
CREATE OR REPLACE FUNCTION get_comprehensive_basin_summary(
    basin_name TEXT,
    project_id UUID DEFAULT NULL
)
RETURNS TABLE (
    summary_type TEXT,
    metric_name TEXT,
    metric_value TEXT,
    details JSONB
) 
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
    -- Well count summary
    RETURN QUERY
    SELECT 
        'Well Summary' as summary_type,
        'Total Wells' as metric_name,
        COUNT(DISTINCT w.id)::TEXT as metric_value,
        jsonb_build_object(
            'wells_with_logs', COUNT(DISTINCT CASE WHEN l.id IS NOT NULL THEN w.id END),
            'wells_without_logs', COUNT(DISTINCT CASE WHEN l.id IS NULL THEN w.id END)
        ) as details
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    LEFT JOIN logs l ON l.well_id = w.id
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    GROUP BY b.name;
    
    -- Log summary
    RETURN QUERY
    SELECT 
        'Log Summary' as summary_type,
        'Total Logs' as metric_name,
        COUNT(DISTINCT l.id)::TEXT as metric_value,
        jsonb_build_object(
            'log_types', ARRAY_AGG(DISTINCT lt.name),
            'avg_logs_per_well', ROUND(COUNT(DISTINCT l.id)::NUMERIC / COUNT(DISTINCT w.id), 2)
        ) as details
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    LEFT JOIN logs l ON l.well_id = w.id
    LEFT JOIN log_types lt ON lt.id = l.log_type_id
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    GROUP BY b.name;
    
    -- Data coverage summary
    RETURN QUERY
    SELECT 
        'Data Coverage' as summary_type,
        'Total Data Points' as metric_name,
        COALESCE(SUM(ld.data_points), 0)::TEXT as metric_value,
        jsonb_build_object(
            'avg_data_points_per_log', ROUND(AVG(ld.data_points), 2),
            'logs_with_data', COUNT(DISTINCT CASE WHEN ld.data_points > 0 THEN l.id END)
        ) as details
    FROM wells w
    JOIN basins b ON ST_Contains(b.outline, w.location)
    LEFT JOIN logs l ON l.well_id = w.id
    LEFT JOIN (
        SELECT log_id, COUNT(*) as data_points 
        FROM log_data 
        GROUP BY log_id
    ) ld ON ld.log_id = l.id
    WHERE b.name ILIKE basin_name
    AND (project_id IS NULL OR w.project_id = project_id)
    GROUP BY b.name;
END;
$$;
```

## API Endpoint Updates

The API server should be updated to include these new endpoints:

```rust
// New endpoints in api_server
POST /api/rpc/get_wells_in_basin
POST /api/rpc/get_first_last_wells_in_basin
POST /api/rpc/get_wells_with_logs_in_basin
POST /api/rpc/get_log_data_for_wells_in_basin
POST /api/rpc/get_wells_by_log_type_in_basin
```

## Testing the RPC Functions

```sql
-- Test the basin query function
SELECT * FROM get_wells_in_basin('Browse Basin');

-- Test first/last wells function
SELECT * FROM get_first_last_wells_in_basin('Browse Basin');

-- Test wells with logs function
SELECT * FROM get_wells_with_logs_in_basin('Browse Basin');

-- Test log data function
SELECT * FROM get_log_data_for_wells_in_basin('Browse Basin', NULL, 1000, 2000);

-- Test wells by log type function
SELECT * FROM get_wells_by_log_type_in_basin('Browse Basin', 'gamma_ray');
```

## Schema Notes

Based on the actual schema.md:
- `wells.location` is a POINT geometry (longitude, latitude)
- `basins.outline` is a POLYGON geometry defining basin boundaries
- `wells.basin_id` is an integer foreign key (alternative to spatial queries)
- All tables have proper project isolation via `project_id` UUID columns
- PostGIS functions like `ST_Contains()` and `ST_X()`, `ST_Y()` are used for spatial operations 