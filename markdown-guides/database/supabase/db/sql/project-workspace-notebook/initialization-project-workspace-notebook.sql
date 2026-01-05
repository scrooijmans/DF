-- =====================================================
-- MudRock Database Schema: Project-Notebook
-- =====================================================
-- This script creates the core tables for the collaborative
-- data analysis platform with projects and notebooks.

-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- 1. PROJECTS TABLE
-- =====================================================
-- Top-level project containers for collaborative data analysis
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL CHECK (length(name) >= 1 AND length(name) <= 100),
    description TEXT CHECK (length(description) <= 500),
    owner_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settings JSONB DEFAULT '{}'::jsonb,
    
    -- Constraints
    CONSTRAINT projects_name_not_empty CHECK (trim(name) != ''),
    CONSTRAINT projects_description_length CHECK (description IS NULL OR length(trim(description)) > 0)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_projects_owner_id ON projects(owner_id);
CREATE INDEX IF NOT EXISTS idx_projects_created_at ON projects(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);
CREATE INDEX IF NOT EXISTS idx_projects_settings ON projects USING GIN(settings);

-- =====================================================
-- 2. NOTEBOOKS TABLE
-- =====================================================
-- Interactive analysis notebooks within projects
CREATE TABLE IF NOT EXISTS notebooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (length(name) >= 1 AND length(name) <= 100),
    
    -- Notebook metadata
    title TEXT,
    description TEXT CHECK (length(description) <= 500),
    tags TEXT[] DEFAULT '{}',
    
    -- Content structure (JSONB for flexibility)
    content JSONB DEFAULT '{
        "cells": [],
        "metadata": {
            "kernel": "mudrock-sql",
            "language": "sql",
            "version": "1.0"
        },
        "version": 1,
        "created_at": null,
        "updated_at": null
    }'::jsonb,
    
    -- Execution state
    execution_state JSONB DEFAULT '{
        "status": "idle",
        "current_cell": null,
        "last_executed": null,
        "execution_count": 0
    }'::jsonb,
    
    -- Collaboration
    is_public BOOLEAN DEFAULT false,
    owner_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT notebooks_name_not_empty CHECK (trim(name) != ''),
    CONSTRAINT notebooks_unique_name_per_project UNIQUE (project_id, name),
    CONSTRAINT notebooks_content_valid CHECK (
        content ? 'cells' AND
        content ? 'metadata' AND
        content ? 'version'
    )
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_notebooks_project_id ON notebooks(project_id);
CREATE INDEX IF NOT EXISTS idx_notebooks_owner_id ON notebooks(owner_id);
CREATE INDEX IF NOT EXISTS idx_notebooks_created_at ON notebooks(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notebooks_updated_at ON notebooks(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_notebooks_name ON notebooks(name);
CREATE INDEX IF NOT EXISTS idx_notebooks_content ON notebooks USING GIN(content);
CREATE INDEX IF NOT EXISTS idx_notebooks_execution_state ON notebooks USING GIN(execution_state);
CREATE INDEX IF NOT EXISTS idx_notebooks_tags ON notebooks USING GIN(tags);

-- =====================================================
-- 3. PROJECT PERMISSIONS TABLE
-- =====================================================
-- User access control for projects
CREATE TABLE IF NOT EXISTS project_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('owner', 'editor', 'viewer')),
    granted_by UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT project_permissions_unique_user_project UNIQUE (project_id, user_id),
    CONSTRAINT project_permissions_no_self_grant CHECK (user_id != granted_by)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_project_permissions_project_id ON project_permissions(project_id);
CREATE INDEX IF NOT EXISTS idx_project_permissions_user_id ON project_permissions(user_id);
CREATE INDEX IF NOT EXISTS idx_project_permissions_role ON project_permissions(role);
CREATE INDEX IF NOT EXISTS idx_project_permissions_granted_by ON project_permissions(granted_by);

-- =====================================================
-- 4. UPDATE TRIGGERS
-- =====================================================
-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_projects_updated_at 
    BEFORE UPDATE ON projects 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notebooks_updated_at 
    BEFORE UPDATE ON notebooks 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =====================================================
-- 5. HELPER FUNCTIONS
-- =====================================================

-- Function to check if user has access to project
CREATE OR REPLACE FUNCTION user_has_project_access(user_id UUID, project_id UUID, required_role TEXT DEFAULT 'viewer')
RETURNS BOOLEAN AS $$
DECLARE
    user_role TEXT;
BEGIN
    -- Check if user is the owner
    SELECT 'owner' INTO user_role
    FROM projects 
    WHERE id = project_id AND owner_id = user_id;
    
    -- If not owner, check permissions table
    IF user_role IS NULL THEN
        SELECT role INTO user_role
        FROM project_permissions 
        WHERE project_id = project_id AND user_id = user_id;
    END IF;
    
    -- Check if user has required role or higher
    IF user_role = 'owner' THEN
        RETURN TRUE;
    ELSIF user_role = 'editor' AND required_role IN ('editor', 'viewer') THEN
        RETURN TRUE;
    ELSIF user_role = 'viewer' AND required_role = 'viewer' THEN
        RETURN TRUE;
    END IF;
    
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to get user's role in project
CREATE OR REPLACE FUNCTION get_user_project_role(user_id UUID, project_id UUID)
RETURNS TEXT AS $$
DECLARE
    user_role TEXT;
BEGIN
    -- Check if user is the owner
    SELECT 'owner' INTO user_role
    FROM projects 
    WHERE id = project_id AND owner_id = user_id;
    
    -- If not owner, check permissions table
    IF user_role IS NULL THEN
        SELECT role INTO user_role
        FROM project_permissions 
        WHERE project_id = project_id AND user_id = user_id;
    END IF;
    
    RETURN COALESCE(user_role, 'none');
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- =====================================================
-- 6. INITIAL DATA
-- =====================================================
-- Note: Initial data will be populated by the application
-- when users create their first projects

-- =====================================================
-- 7. COMMENTS
-- =====================================================
COMMENT ON TABLE projects IS 'Top-level project containers for collaborative data analysis';
COMMENT ON TABLE notebooks IS 'Interactive analysis notebooks within projects with enhanced functionality';
COMMENT ON TABLE project_permissions IS 'User access control for projects with role-based permissions';

COMMENT ON COLUMN projects.settings IS 'JSONB field for storing project-specific settings and configuration';
COMMENT ON COLUMN notebooks.content IS 'JSONB field storing notebook cells, metadata, and version information';
COMMENT ON COLUMN notebooks.execution_state IS 'JSONB field storing current execution state and progress';
COMMENT ON COLUMN notebooks.tags IS 'Array of tags for categorizing and filtering notebooks';

-- =====================================================
-- SCHEMA CREATION COMPLETE
-- =====================================================
-- The database schema has been successfully created.
-- Next steps:
-- 1. Set up Row Level Security (RLS) policies
-- 2. Create API functions for CRUD operations
-- 3. Implement frontend integration
-- 4. Add storage management functions
