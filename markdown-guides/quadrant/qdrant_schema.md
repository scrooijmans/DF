# Qdrant Schema Design for MudRock

## Core Collections

### 1. wells
- id: "well_123"
- vector: [OpenAI/code-aware embedding]
- payload: {
    well_id, name, basin_id, basin_name, location, team_id, project_id, created_at, metadata (operator, spud_date, total_depth, status)
  }

### 2. logs
- id: "log_456"
- vector: [embedding of log description]
- payload: {
    log_id, well_id, well_name, log_type_id, log_type_name, log_type_aliases, name, unit,
    parquet_file_path, depth_range, data_points, created_at, metadata (tool, run_number, quality)
  }

### 3. basins
- id: "basin_1"
- vector: [embedding of basin description]
- payload: {
    basin_id, name, geometry, country, region, created_at, metadata (area_km2, water_depth_range)
  }

### 4. functions
- id: "func_slowness_to_velocity"
- vector: [embedding of function description]
- payload: {
    function_id, name, category, description, module, function_path, parameters, return_type, return_description, tags, aliases, created_at
  }

### 5. projects
- id: "project_789"
- vector: [embedding of project description]
- payload: {
    project_id, name, description, team_id, created_at, updated_at, metadata (status, client, budget)
  }

## Notes
- All log data is referenced by `parquet_file_path` and loaded via the parquet loader.
- All semantic search, filtering, and function registry is handled in Qdrant.
- No log_data or embeddings tables in Postgres.
- Function registry is now semantic/code-searchable via Qdrant.

## Example Query Flow
- User query → Qdrant (semantic search/filter) → Get metadata, file paths, function info
- App loads Parquet data as needed, runs scientific functions in app code 