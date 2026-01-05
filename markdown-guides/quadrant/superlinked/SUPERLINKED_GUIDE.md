# Superlinked Integration Guide

## Requirements
- **Python 3.11** (Superlinked v30.x+ is not compatible with Python 3.12+)
- **Qdrant** running locally (default: http://localhost:6333)
- **Unified well data**: All ingestion uses a shared `wells.json` file for consistency between Rust and Python pipelines.

## Configuration & Workflow

### 1. Data Consistency
- The canonical well dataset is stored in `wells.json` (10 wells, shared by both Rust and Python).
- Both the Rust and Superlinked (Python) ingestion scripts load from this file, ensuring identical data in both `wells` and `superlinked_wells` Qdrant collections.

### 2. Superlinked Ingestion Flow
- **Schema and Index**: Define a Superlinked schema and index using the new API (see `ingest_superlinked_wells.py`).
- **In-Memory Ingestion**: Data is first ingested in-memory using Superlinked's executor.
- **Manual Qdrant Upsert**: Due to a bug in Superlinked's RestExecutor (as of v30.0.0), vectors and metadata are manually upserted to Qdrant using the Python Qdrant client.
- **Collection Management**: The script deletes the old `superlinked_wells` collection before each run to ensure a clean state.
- **Verification**: After upsert, the script verifies the collection and prints the number of points.

### 3. Search & Query
- Both Rust and Python can now query Qdrant for the same well data, using either the `wells` or `superlinked_wells` collection.
- Test queries confirm that Superlinked's multimodal search provides strong differentiation and ranking for relevant wells.

### 4. Best Practices
- Always use the shared `wells.json` as the source of truth for well data.
- Use Python 3.11 for all Superlinked operations until official support for newer versions is released.
- For production, consider running Superlinked as a service (REST API) and automate the ingestion pipeline.
- Monitor Superlinked releases for fixes to the RestExecutor and new features.

## Example: Ingestion Script Highlights
- Loads wells from `wells.json`
- Defines schema and index
- Ingests data in-memory
- Manually upserts to Qdrant
- Verifies collection

## Troubleshooting
- If you see a `UnionType` error, ensure you are using Python 3.11 and the latest Superlinked 30.x release.
- If ingestion fails, check that Qdrant is running and accessible at the configured URL.

## Future Directions
- Once Superlinked's RestExecutor is fixed, the manual upsert step can be replaced with direct Superlinked-to-Qdrant integration.
- For advanced search, use Superlinked's multimodal capabilities in combination with Qdrant's fast vector search.
