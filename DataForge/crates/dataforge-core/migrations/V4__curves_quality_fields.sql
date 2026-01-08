-- V4__curves_quality_fields.sql
-- Adds quality fields to curves table for data provenance tracking
-- Fields: null_value, quality_flag, acquisition_date, service_company, unit_id

-- Note: This migration is handled by Rust code for conditional execution
-- SQLite's ALTER TABLE doesn't support IF NOT EXISTS for columns

SELECT 1;
