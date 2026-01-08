-- V8__osdu_schema_alignment.sql
-- Full OSDU schema alignment migration
--
-- This migration adds:
-- 1. wellbores table (OSDU master-data--Wellbore)
-- 2. OSDU columns to wells table (operator, spud_date, surface location, etc.)
-- 3. wellbore_id and OSDU columns to log_runs table
-- 4. index_type column to curves table
-- 5. wellbore_id column to survey_runs table
-- 6. OSDU columns to marker_sets table (wellbore_id, interpretation_date, confidence_level)
-- 7. wellbore_id column to checkshot_runs table
--
-- Note: The actual ALTER TABLE operations are handled by Rust code
-- because SQLite's ALTER TABLE doesn't support IF NOT EXISTS for columns.
-- This SQL file serves as documentation and version tracking.

SELECT 1;
