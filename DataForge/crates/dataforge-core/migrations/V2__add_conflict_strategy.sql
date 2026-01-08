-- V2__add_conflict_strategy.sql
-- Adds conflict_strategy column to sync_state table
-- This column stores the strategy for handling sync conflicts

-- Note: This migration is handled by Rust code for conditional execution
-- Refinery SQL migrations are unconditional, so we use a no-op here
-- and let the Rust migration code handle the actual ALTER TABLE

SELECT 1;
