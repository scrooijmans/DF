-- V3__curves_dual_storage.sql
-- Handles the schema change from single blob_hash to native/gridded dual storage
-- This recreates the curves table with the new schema

-- Note: This migration is handled by Rust code for conditional execution
-- SQLite doesn't support conditional DDL, so we use a no-op here
-- and let the Rust migration code detect old schema and recreate table

SELECT 1;
