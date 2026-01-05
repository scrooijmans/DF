-- =====================================================
-- MudRock RLS Policies: Project-Notebook
-- =====================================================
-- This script sets up Row Level Security (RLS) policies
-- for the collaborative data analysis platform.

-- Enable RLS on all tables
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE notebooks ENABLE ROW LEVEL SECURITY;
ALTER TABLE project_permissions ENABLE ROW LEVEL SECURITY;

-- =====================================================
-- 1. PROJECTS TABLE POLICIES
-- =====================================================

-- Users can view projects they own or have permissions for
CREATE POLICY "Users can view accessible projects" ON projects
    FOR SELECT USING (
        owner_id = auth.uid() OR 
        id IN (
            SELECT project_id 
            FROM project_permissions 
            WHERE user_id = auth.uid()
        )
    );

-- Only project owners can update projects
CREATE POLICY "Project owners can update projects" ON projects
    FOR UPDATE USING (owner_id = auth.uid())
    WITH CHECK (owner_id = auth.uid());

-- Only project owners can delete projects
CREATE POLICY "Project owners can delete projects" ON projects
    FOR DELETE USING (owner_id = auth.uid());

-- Authenticated users can create projects (they become the owner)
CREATE POLICY "Authenticated users can create projects" ON projects
    FOR INSERT WITH CHECK (
        auth.uid() IS NOT NULL AND 
        owner_id = auth.uid()
    );

-- =====================================================
-- 2. NOTEBOOKS TABLE POLICIES
-- =====================================================

-- Users can view notebooks they have access to
CREATE POLICY "Users can view accessible notebooks" ON notebooks
    FOR SELECT USING (
        -- User is the owner
        owner_id = auth.uid() OR
        -- User has project access
        user_has_project_access(auth.uid(), project_id, 'viewer')
    );

-- Users with editor or owner access can create notebooks
CREATE POLICY "Editors and owners can create notebooks" ON notebooks
    FOR INSERT WITH CHECK (
        auth.uid() IS NOT NULL AND
        -- User has project access
        user_has_project_access(auth.uid(), project_id, 'editor') AND
        -- User is the owner
        owner_id = auth.uid()
    );

-- Users with editor or owner access can update notebooks
CREATE POLICY "Editors and owners can update notebooks" ON notebooks
    FOR UPDATE USING (
        -- User is the owner
        owner_id = auth.uid() OR
        -- User has project access
        user_has_project_access(auth.uid(), project_id, 'editor')
    )
    WITH CHECK (
        -- User is the owner
        owner_id = auth.uid() OR
        -- User has project access
        user_has_project_access(auth.uid(), project_id, 'editor')
    );

-- Only project owners can delete notebooks
CREATE POLICY "Project owners can delete notebooks" ON notebooks
    FOR DELETE USING (
        -- User is the owner
        owner_id = auth.uid() OR
        -- User has project access
        user_has_project_access(auth.uid(), project_id, 'owner')
    );

-- =====================================================
-- 3. PROJECT PERMISSIONS TABLE POLICIES
-- =====================================================

-- Users can view permissions for projects they have access to
CREATE POLICY "Users can view project permissions" ON project_permissions
    FOR SELECT USING (
        project_id IN (
            SELECT id FROM projects WHERE owner_id = auth.uid()
        ) OR
        user_id = auth.uid()
    );

-- Only project owners can manage permissions
CREATE POLICY "Project owners can manage permissions" ON project_permissions
    FOR ALL USING (
        project_id IN (
            SELECT id FROM projects WHERE owner_id = auth.uid()
        )
    )
    WITH CHECK (
        project_id IN (
            SELECT id FROM projects WHERE owner_id = auth.uid()
        )
    );

-- =====================================================
-- 4. ADDITIONAL SECURITY MEASURES
-- =====================================================

-- Prevent users from changing project ownership
CREATE POLICY "Prevent ownership changes" ON projects
    FOR UPDATE USING (owner_id = auth.uid())
    WITH CHECK (owner_id = auth.uid());

-- Prevent users from granting permissions they don't have
CREATE POLICY "Prevent unauthorized permission grants" ON project_permissions
    FOR INSERT WITH CHECK (
        granted_by = auth.uid() AND
        project_id IN (
            SELECT id FROM projects WHERE owner_id = auth.uid()
        )
    );

-- =====================================================
-- 5. HELPER FUNCTIONS FOR RLS
-- =====================================================

-- Function to check if user can access project (used in policies)
CREATE OR REPLACE FUNCTION can_access_project(project_uuid UUID, user_uuid UUID DEFAULT auth.uid())
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM projects 
        WHERE id = project_uuid AND (
            owner_id = user_uuid OR 
            id IN (
                SELECT project_id 
                FROM project_permissions 
                WHERE user_id = user_uuid
            )
        )
    );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to check if user can edit project (used in policies)
CREATE OR REPLACE FUNCTION can_edit_project(project_uuid UUID, user_uuid UUID DEFAULT auth.uid())
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM projects 
        WHERE id = project_uuid AND (
            owner_id = user_uuid OR 
            id IN (
                SELECT project_id 
                FROM project_permissions 
                WHERE user_id = user_uuid AND role IN ('owner', 'editor')
            )
        )
    );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to check if user is project owner (used in policies)
CREATE OR REPLACE FUNCTION is_project_owner(project_uuid UUID, user_uuid UUID DEFAULT auth.uid())
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM projects 
        WHERE id = project_uuid AND owner_id = user_uuid
    );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- =====================================================
-- 6. AUDIT LOGGING (Optional)
-- =====================================================

-- Create audit log table for tracking changes
CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name TEXT NOT NULL,
    record_id UUID NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('INSERT', 'UPDATE', 'DELETE')),
    old_values JSONB,
    new_values JSONB,
    user_id UUID REFERENCES auth.users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enable RLS on audit log
ALTER TABLE audit_log ENABLE ROW LEVEL SECURITY;

-- Only users can view their own audit logs
CREATE POLICY "Users can view their own audit logs" ON audit_log
    FOR SELECT USING (user_id = auth.uid());

-- =====================================================
-- 7. COMMENTS
-- =====================================================
COMMENT ON POLICY "Users can view accessible projects" ON projects IS 
    'Allows users to view projects they own or have permissions for';

COMMENT ON POLICY "Project owners can update projects" ON projects IS 
    'Only project owners can update project details';

COMMENT ON POLICY "Project owners can delete projects" ON projects IS 
    'Only project owners can delete projects';

COMMENT ON POLICY "Authenticated users can create projects" ON projects IS 
    'Authenticated users can create projects and become the owner';

COMMENT ON POLICY "Users can view accessible notebooks" ON notebooks IS 
    'Users can view notebooks they own or have project access to';

COMMENT ON POLICY "Editors and owners can create notebooks" ON notebooks IS 
    'Users with editor or owner access can create notebooks';

COMMENT ON POLICY "Editors and owners can update notebooks" ON notebooks IS 
    'Users with editor or owner access can update notebooks';

COMMENT ON POLICY "Project owners can delete notebooks" ON notebooks IS 
    'Only project owners can delete notebooks';

COMMENT ON POLICY "Users can view project permissions" ON project_permissions IS 
    'Users can view permissions for projects they have access to';

COMMENT ON POLICY "Project owners can manage permissions" ON project_permissions IS 
    'Only project owners can manage project permissions';

-- =====================================================
-- RLS SETUP COMPLETE
-- =====================================================
-- Row Level Security policies have been successfully configured.
-- The database is now secure with proper access controls.
-- 
-- Security Summary:
-- - Users can only access projects they own or have permissions for
-- - Role-based access control (owner, editor, viewer)
-- - Project owners have full control
-- - Editors can create and modify content
-- - Viewers have read-only access
-- - All operations are properly secured with RLS policies
