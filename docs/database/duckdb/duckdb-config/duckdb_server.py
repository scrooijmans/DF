#!/usr/bin/env python3
"""
DuckDB Analytics Server for MudRock Enterprise
Provides REST API for analytics queries combining PostgreSQL and Parquet data
"""

import os
import json
import asyncio
import aiohttp
import duckdb
from fastapi import FastAPI, HTTPException, Depends, Query
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Dict, Any, Optional
import logging
from datetime import datetime
import uvicorn

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Initialize FastAPI app
app = FastAPI(
    title="MudRock DuckDB Analytics API",
    description="High-performance analytics engine for geoscience data",
    version="1.0.0"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Configuration
POSTGREST_CONFIG = {
    "base_url": os.getenv("POSTGREST_BASE_URL", "http://supabase-kong:8000/rest/v1"),
    "api_key": os.getenv("SERVICE_ROLE_KEY", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaXNzIjoic3VwYWJhc2UtbXVkcm9jayIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJpYXQiOjE3NTQ2NjE0MTQsImV4cCI6MjA3MDIzNzQxNH0.ipR05YNdP7Ux72VTNqrJHIBKhQW5jvmt20rYNPp_scs"),
    "headers": {
        "apikey": os.getenv("SERVICE_ROLE_KEY", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaXNzIjoic3VwYWJhc2UtbXVkcm9jayIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJpYXQiOjE3NTQ2NjE0MTQsImV4cCI6MjA3MDIzNzQxNH0.ipR05YNdP7Ux72VTNqrJHIBKhQW5jvmt20rYNPp_scs"),
        "Authorization": f"Bearer {os.getenv('SERVICE_ROLE_KEY', 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoic2VydmljZV9yb2xlIiwiaXNzIjoic3VwYWJhc2UtbXVkcm9jayIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJpYXQiOjE3NTQ2NjE0MTQsImV4cCI6MjA3MDIzNzQxNH0.ipR05YNdP7Ux72VTNqrJHIBKhQW5jvmt20rYNPp_scs')}",
        "Content-Type": "application/json"
    }
}

# S3 Storage Configuration for Parquet files
# Based on official Supabase docs: S3 endpoint should be /storage/v1/s3
STORAGE_CONFIG = {
    "bucket": os.getenv("STORAGE_BUCKET", "animal-metrics-parquet"),
    "endpoint": os.getenv("STORAGE_ENDPOINT", "http://supabase-kong:8000"),
    "s3_endpoint": f"{os.getenv('STORAGE_ENDPOINT', 'http://supabase-kong:8000')}/storage/v1/s3",
    "access_key": os.getenv("STORAGE_ACCESS_KEY", os.getenv("SERVICE_ROLE_KEY")),
    "secret_key": os.getenv("STORAGE_SECRET_KEY", os.getenv("SERVICE_ROLE_KEY")),
    "region": os.getenv("STORAGE_REGION", "us-east-1")
}

# Global connections
http_session = None
duckdb_conn = None

class QueryRequest(BaseModel):
    query: str
    parameters: Optional[Dict[str, Any]] = None

class QueryResponse(BaseModel):
    success: bool
    data: List[Dict[str, Any]]
    columns: List[str]
    row_count: int
    execution_time_ms: float
    error: Optional[str] = None

@app.on_event("startup")
async def startup_event():
    """Initialize connections"""
    global http_session, duckdb_conn
    
    try:
        # Initialize HTTP session for PostgREST API calls
        connector = aiohttp.TCPConnector(ssl=False)  # Disable SSL verification
        http_session = aiohttp.ClientSession(connector=connector)
        logger.info("✅ HTTP session created for PostgREST API")
        
        # Initialize DuckDB connection
        duckdb_conn = duckdb.connect()
        logger.info("✅ DuckDB connection established")
        
        # Configure DuckDB for S3 access (Supabase Storage)
        duckdb_conn.execute("INSTALL httpfs")
        duckdb_conn.execute("LOAD httpfs")
        
        # Set S3 credentials for Supabase Storage
        # Use standard S3 authentication with Supabase's S3 compatibility
        duckdb_conn.execute(f"""
            SET s3_endpoint='{STORAGE_CONFIG['endpoint']}';
            SET s3_access_key_id='{STORAGE_CONFIG['access_key']}';
            SET s3_secret_access_key='{STORAGE_CONFIG['secret_key']}';
            SET s3_region='{STORAGE_CONFIG['region']}';
            SET s3_use_ssl=false;
            SET s3_url_style='path';
        """)
        logger.info("✅ DuckDB S3 configuration completed for Supabase Storage")
        
        # Test PostgREST connection (optional - don't fail startup if it fails)
        try:
            async with http_session.get(
                f"{POSTGREST_CONFIG['base_url']}/",
                headers=POSTGREST_CONFIG["headers"],
                timeout=aiohttp.ClientTimeout(total=5)  # 5 second timeout
            ) as response:
                if response.status == 200:
                    logger.info("✅ PostgREST API connection successful")
                else:
                    logger.warning(f"⚠️ PostgREST API returned status {response.status}")
        except Exception as e:
            logger.warning(f"⚠️ PostgREST API connection failed (will retry on first request): {e}")
        
    except Exception as e:
        logger.error(f"❌ Failed to initialize connections: {e}")
        raise

@app.on_event("shutdown")
async def shutdown_event():
    """Clean up connections"""
    global http_session, duckdb_conn
    
    if http_session:
        await http_session.close()
        logger.info("✅ HTTP session closed")
    
    if duckdb_conn:
        duckdb_conn.close()
        logger.info("✅ DuckDB connection closed")

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    try:
        # Test DuckDB connection (required)
        result = duckdb_conn.execute("SELECT 1").fetchone()
        
        # Test PostgREST API connection (optional)
        postgrest_status = "unknown"
        try:
            async with http_session.get(
                f"{POSTGREST_CONFIG['base_url']}/",
                headers=POSTGREST_CONFIG["headers"],
                timeout=aiohttp.ClientTimeout(total=3)
            ) as response:
                postgrest_status = "connected" if response.status == 200 else f"error_{response.status}"
        except Exception as e:
            postgrest_status = f"disconnected: {str(e)[:50]}"
        
        return {
            "status": "healthy",
            "postgrest": postgrest_status,
            "duckdb": "connected",
            "timestamp": datetime.now().isoformat()
        }
    except Exception as e:
        logger.error(f"Health check failed: {e}")
        raise HTTPException(status_code=503, detail=f"Service unhealthy: {e}")

@app.get("/")
async def root():
    """Root endpoint with API information"""
    return {
        "service": "MudRock DuckDB Analytics API",
        "version": "1.0.0",
        "endpoints": {
            "health": "/health",
            "query": "/query",
            "animals": "/animals",
            "analytics": "/analytics"
        }
    }

@app.get("/animals")
async def get_animals(
    breed: Optional[str] = Query(None, description="Filter by breed"),
    limit: int = Query(100, description="Limit number of results")
):
    """Get animals from PostgreSQL database via PostgREST API"""
    try:
        start_time = datetime.now()
        
        # Build PostgREST query URL
        url = f"{POSTGREST_CONFIG['base_url']}/dump_animals"
        params = {
            "select": "id,pet_name,type,breed,created_at",
            "order": "created_at.desc",
            "limit": str(limit)
        }
        
        if breed:
            params["breed"] = f"ilike.*{breed}*"
        
        # Execute query via PostgREST
        async with http_session.get(
            url,
            params=params,
            headers=POSTGREST_CONFIG["headers"]
        ) as response:
            if response.status != 200:
                raise HTTPException(status_code=response.status, detail=f"PostgREST error: {await response.text()}")
            
            data = await response.json()
        
        execution_time = (datetime.now() - start_time).total_seconds() * 1000
        
        return QueryResponse(
            success=True,
            data=data,
            columns=["id", "pet_name", "type", "breed", "created_at"],
            row_count=len(data),
            execution_time_ms=execution_time
        )
        
    except Exception as e:
        logger.error(f"Error fetching animals: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/animals/dogs")
async def get_dogs():
    """Get all dogs from the database via PostgREST API"""
    try:
        start_time = datetime.now()
        
        # Build PostgREST query URL for dogs
        url = f"{POSTGREST_CONFIG['base_url']}/dump_animals"
        params = {
            "select": "id,pet_name,type,breed,created_at",
            "or": "(type.ilike.*dog*,breed.ilike.*dog*)",
            "order": "created_at.desc"
        }
        
        # Execute query via PostgREST
        async with http_session.get(
            url,
            params=params,
            headers=POSTGREST_CONFIG["headers"]
        ) as response:
            if response.status != 200:
                raise HTTPException(status_code=response.status, detail=f"PostgREST error: {await response.text()}")
            
            data = await response.json()
        
        execution_time = (datetime.now() - start_time).total_seconds() * 1000
        
        return QueryResponse(
            success=True,
            data=data,
            columns=["id", "pet_name", "type", "breed", "created_at"],
            row_count=len(data),
            execution_time_ms=execution_time
        )
        
    except Exception as e:
        logger.error(f"Error fetching dogs: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/query")
async def execute_query(request: QueryRequest):
    """Execute custom DuckDB query"""
    try:
        start_time = datetime.now()
        
        # Execute query
        if request.parameters:
            result = duckdb_conn.execute(request.query, list(request.parameters.values())).fetchall()
        else:
            result = duckdb_conn.execute(request.query).fetchall()
        
        # Get column names
        columns = [desc[0] for desc in duckdb_conn.description] if duckdb_conn.description else []
        
        # Convert to list of dictionaries
        data = [dict(zip(columns, row)) for row in result]
        
        execution_time = (datetime.now() - start_time).total_seconds() * 1000
        
        return QueryResponse(
            success=True,
            data=data,
            columns=columns,
            row_count=len(data),
            execution_time_ms=execution_time
        )
        
    except Exception as e:
        logger.error(f"Query execution error: {e}")
        return QueryResponse(
            success=False,
            data=[],
            columns=[],
            row_count=0,
            execution_time_ms=0,
            error=str(e)
        )

@app.get("/analytics/breed-stats")
async def get_breed_stats():
    """Get statistics by breed via PostgREST API"""
    try:
        start_time = datetime.now()
        
        # For complex analytics, we'll use PostgREST's aggregation features
        url = f"{POSTGREST_CONFIG['base_url']}/dump_animals"
        params = {
            "select": "breed,type,created_at",
            "breed": "not.is.null",
            "breed": "neq.",
            "order": "breed"
        }
        
        # Execute query via PostgREST
        async with http_session.get(
            url,
            params=params,
            headers=POSTGREST_CONFIG["headers"]
        ) as response:
            if response.status != 200:
                raise HTTPException(status_code=response.status, detail=f"PostgREST error: {await response.text()}")
            
            raw_data = await response.json()
        
        # Process data in Python for aggregation (since PostgREST has limited aggregation)
        breed_stats = {}
        for item in raw_data:
            breed = item.get('breed', 'Unknown')
            if breed not in breed_stats:
                breed_stats[breed] = {
                    'breed': breed,
                    'animal_count': 0,
                    'type_count': set(),
                    'first_created': None,
                    'last_created': None
                }
            
            breed_stats[breed]['animal_count'] += 1
            breed_stats[breed]['type_count'].add(item.get('type', 'Unknown'))
            
            created_at = item.get('created_at')
            if created_at:
                if not breed_stats[breed]['first_created'] or created_at < breed_stats[breed]['first_created']:
                    breed_stats[breed]['first_created'] = created_at
                if not breed_stats[breed]['last_created'] or created_at > breed_stats[breed]['last_created']:
                    breed_stats[breed]['last_created'] = created_at
        
        # Convert to final format
        data = []
        for breed_data in breed_stats.values():
            data.append({
                'breed': breed_data['breed'],
                'animal_count': breed_data['animal_count'],
                'type_count': len(breed_data['type_count']),
                'first_created': breed_data['first_created'],
                'last_created': breed_data['last_created']
            })
        
        # Sort by animal count descending
        data.sort(key=lambda x: x['animal_count'], reverse=True)
        
        execution_time = (datetime.now() - start_time).total_seconds() * 1000
        
        return QueryResponse(
            success=True,
            data=data,
            columns=["breed", "animal_count", "type_count", "first_created", "last_created"],
            row_count=len(data),
            execution_time_ms=execution_time
        )
        
    except Exception as e:
        logger.error(f"Error getting breed stats: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/analytics/pulse-data")
async def get_pulse_data(
    breed: Optional[str] = Query(None, description="Filter by breed"),
    age_year: Optional[int] = Query(None, description="Filter by age in years"),
    limit: int = Query(100, description="Limit number of results")
):
    """Get pulse data analytics with filtering"""
    try:
        start_time = datetime.now()
        
        # Use Supabase Storage S3 API for Parquet files
        conn = duckdb.connect()
        
        # Configure S3 settings for this connection
        conn.execute("INSTALL httpfs")
        conn.execute("LOAD httpfs")
        conn.execute(f"SET s3_endpoint='{STORAGE_CONFIG['endpoint']}'")
        conn.execute(f"SET s3_access_key_id='{STORAGE_CONFIG['access_key']}'")
        conn.execute(f"SET s3_secret_access_key='{STORAGE_CONFIG['secret_key']}'")
        conn.execute(f"SET s3_region='{STORAGE_CONFIG['region']}'")
        conn.execute("SET s3_use_ssl=false")
        conn.execute("SET s3_url_style='path'")
        
        # Try to read from local Parquet file, fallback to test data
        try:
            # Read from the local Parquet file we copied
            local_file_path = "/app/data/test_pulse_data.parquet"
            
            # Check if the file exists
            import os
            if os.path.exists(local_file_path):
                logger.info(f"📁 Found local Parquet file: {local_file_path}")
                
                # Read from the local file
                conn.execute(f"CREATE TABLE pulse_data AS SELECT * FROM read_parquet('{local_file_path}')")
                
                logger.info("✅ Successfully loaded pulse data from local Parquet file")
            else:
                raise Exception(f"Local file not found: {local_file_path}")
                
        except Exception as e:
            logger.warning(f"⚠️ Could not load from local Parquet file, using test data: {e}")
            # Fallback to test data
            conn.execute("""
                CREATE TABLE pulse_data AS 
                SELECT * FROM (VALUES 
                    (1, '2023-01-01 00:00:00'::timestamp, 80.5, 'sensor_1', 0.95),
                    (1, '2023-01-01 00:01:00'::timestamp, 82.1, 'sensor_1', 0.97),
                    (1, '2023-01-01 00:02:00'::timestamp, 79.8, 'sensor_1', 0.94),
                    (2, '2023-01-01 00:00:00'::timestamp, 75.2, 'sensor_2', 0.96),
                    (2, '2023-01-01 00:01:00'::timestamp, 77.9, 'sensor_2', 0.98)
                ) AS t(animal_id, timestamp, pulse_value, sensor_id, quality_score)
            """)
        
        # Build query with filters
        where_conditions = []
        if breed:
            # For now, just filter by animal_id (would need breed lookup in real implementation)
            where_conditions.append("animal_id = 1")  # Placeholder
        
        if age_year:
            # Placeholder for age filtering
            where_conditions.append("EXTRACT(YEAR FROM timestamp) = 2023")
        
        where_clause = ""
        if where_conditions:
            where_clause = f"WHERE {' AND '.join(where_conditions)}"
        
        # Execute analytics query
        query = f"""
            SELECT 
                animal_id,
                timestamp,
                pulse_value,
                sensor_id,
                quality_score
            FROM pulse_data
            {where_clause}
            ORDER BY timestamp DESC
            LIMIT {limit}
        """
        
        result = conn.execute(query).fetchall()
        
        # Get aggregations
        agg_query = f"""
            SELECT 
                COUNT(*) as total_readings,
                AVG(pulse_value) as avg_pulse,
                MIN(pulse_value) as min_pulse,
                MAX(pulse_value) as max_pulse,
                STDDEV(pulse_value) as std_pulse
            FROM pulse_data
            {where_clause}
        """
        
        agg_result = conn.execute(agg_query).fetchone()
        
        execution_time = (datetime.now() - start_time).total_seconds() * 1000
        
        return {
            "success": True,
            "data": [
                {
                    "animal_id": row[0],
                    "timestamp": row[1].isoformat() if row[1] else None,
                    "pulse_value": row[2],
                    "sensor_id": row[3],
                    "quality_score": row[4]
                }
                for row in result
            ],
            "aggregations": {
                "total_readings": agg_result[0] if agg_result else 0,
                "avg_pulse": round(agg_result[1], 2) if agg_result and agg_result[1] else 0,
                "min_pulse": round(agg_result[2], 2) if agg_result and agg_result[2] else 0,
                "max_pulse": round(agg_result[3], 2) if agg_result and agg_result[3] else 0,
                "std_pulse": round(agg_result[4], 2) if agg_result and agg_result[4] else 0
            },
            "filters": {
                "breed": breed,
                "age_year": age_year,
                "limit": limit
            },
            "execution_time_ms": round(execution_time, 2)
        }
        
    except Exception as e:
        logger.error(f"Error in pulse data analytics: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/storage/test")
async def test_storage_access():
    """Test Supabase Storage bucket access via HTTP API"""
    try:
        start_time = datetime.now()
        
        # Test Supabase Storage bucket access via HTTP API
        bucket_name = STORAGE_CONFIG['bucket']
        
        # Use Supabase Storage API to check if bucket exists (via Kong)
        storage_url = f"{STORAGE_CONFIG['endpoint']}/storage/v1/bucket"
        
        async with http_session.get(
            storage_url,
            headers={
                "Authorization": f"Bearer {STORAGE_CONFIG['access_key']}",
                "Content-Type": "application/json"
            }
        ) as response:
            if response.status == 200:
                buckets = await response.json()
                execution_time = (datetime.now() - start_time).total_seconds() * 1000
                
                # Check if our bucket exists
                our_bucket = next((b for b in buckets if b.get('id') == bucket_name), None)
                
                return {
                    "success": True,
                    "message": f"Successfully connected to Supabase Storage. Bucket '{bucket_name}' {'exists' if our_bucket else 'not found'}",
                    "bucket_name": bucket_name,
                    "bucket_exists": our_bucket is not None,
                    "bucket_details": our_bucket,
                    "total_buckets": len(buckets),
                    "execution_time_ms": execution_time,
                    "storage_config": {
                        "endpoint": STORAGE_CONFIG['endpoint'],
                        "bucket": STORAGE_CONFIG['bucket'],
                        "region": STORAGE_CONFIG['region']
                    }
                }
            else:
                error_text = await response.text()
                return {
                    "success": False,
                    "error": f"HTTP {response.status}: {error_text}",
                    "storage_config": {
                        "endpoint": STORAGE_CONFIG['endpoint'],
                        "bucket": STORAGE_CONFIG['bucket'],
                        "region": STORAGE_CONFIG['region']
                    }
                }
        
    except Exception as e:
        logger.error(f"Error testing storage access: {e}")
        return {
            "success": False,
            "error": str(e),
            "storage_config": {
                "endpoint": STORAGE_CONFIG['endpoint'],
                "bucket": STORAGE_CONFIG['bucket'],
                "region": STORAGE_CONFIG['region']
            }
        }

if __name__ == "__main__":
    port = int(os.getenv("API_PORT", "8080"))
    host = os.getenv("API_HOST", "0.0.0.0")
    
    logger.info(f"🚀 Starting MudRock DuckDB Analytics API on {host}:{port}")
    uvicorn.run(app, host=host, port=port)
