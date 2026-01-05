# MudRock DuckDB Analytics Service

High-performance analytics engine for MudRock Enterprise that combines PostgreSQL relational data with Parquet file analytics.

## Features

- **Hybrid Analytics**: Query both PostgreSQL and Parquet files in a single query
- **REST API**: Easy-to-use HTTP endpoints for analytics
- **Real-time Queries**: Fast columnar analytics with DuckDB
- **PostgreSQL Integration**: Direct connection to your Supabase database
- **S3/Storage Ready**: Built-in support for Parquet files in object storage

## API Endpoints

### Health & Info
- `GET /health` - Service health check
- `GET /` - API information and available endpoints

### Data Queries
- `GET /animals` - Get all animals from database
- `GET /animals/dogs` - Get all dogs specifically
- `GET /animals?breed=German Shepherd` - Filter by breed
- `POST /query` - Execute custom DuckDB queries

### Analytics
- `GET /analytics/breed-stats` - Statistics by breed
- `POST /query` - Custom analytics queries

## Example Queries

### Get All Dogs
```bash
curl "http://91.99.166.223:8081/animals/dogs"
```

### Custom Query - Animals by Type
```bash
curl -X POST "http://91.99.166.223:8081/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "SELECT type, COUNT(*) as count FROM postgres_db.dump_animals GROUP BY type ORDER BY count DESC"
  }'
```

### Complex Analytics Query
```bash
curl -X POST "http://91.99.166.223:8081/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "SELECT breed, COUNT(*) as animal_count, AVG(EXTRACT(EPOCH FROM (NOW() - created_at))/86400) as avg_age_days FROM postgres_db.dump_animals WHERE breed IS NOT NULL GROUP BY breed ORDER BY animal_count DESC"
  }'
```

## Configuration

The service connects to your existing Supabase PostgreSQL database and is configured via environment variables:

- `POSTGRES_HOST`: Database host (mudrock-db)
- `POSTGRES_PORT`: Database port (5432)
- `POSTGRES_DB`: Database name (postgres)
- `POSTGRES_USER`: Database user (supabase_admin)
- `POSTGRES_PASSWORD`: Database password

## Testing

Run the test script to verify the service is working:

```bash
./duckdb-scripts/test_duckdb_api.sh
```

## Architecture

```
DuckDB Analytics Service
├── FastAPI (REST API)
├── DuckDB Engine (Analytics)
├── PostgreSQL Extension (Database Access)
└── Parquet Support (Future Analytics)
```

The service provides a bridge between your relational data in PostgreSQL and high-performance analytics capabilities of DuckDB.
