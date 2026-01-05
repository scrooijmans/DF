# Selections Schema Design

## Overview

The selections schema allows users to create and store arbitrary selections of data from various tables (wells, curves, nodes, pipelines, projects). This enables users to save frequently-used filters and groupings for quick access.

## Schema Design

### Core Tables

#### 1. `selections` (Metadata Table)
Stores selection metadata:
- **id**: UUID primary key
- **name**: Selection name (1-200 chars)
- **description**: Optional description (max 1000 chars)
- **data_type**: Enum (`wells`, `curves`, `nodes`, `pipelines`, `projects`)
- **owner_id**: User who created the selection
- **project_id**: Optional project association
- **is_public**: Whether visible to all project members
- **is_favorite**: User's favorite selections
- **tags**: Array of tags for organization
- **created_at**, **updated_at**: Timestamps

#### 2. `selection_items` (Items Table)
Stores the actual IDs in the selection:
- **id**: UUID primary key
- **selection_id**: Foreign key to `selections`
- **well_id**: INTEGER (nullable, for well selections)
- **curve_id**: UUID (nullable, for curve selections)
- **node_id**: UUID (nullable, for node selections)
- **pipeline_id**: UUID (nullable, for pipeline selections)
- **project_id**: UUID (nullable, for project selections)
- **item_order**: INTEGER for maintaining user-defined order
- **created_at**: Timestamp

### Design Decisions

#### Why Separate Tables?
- **Normalization**: Keeps selection metadata separate from item references
- **Performance**: Efficient queries with proper indexes
- **Flexibility**: Easy to add new data types without schema changes
- **Scalability**: Can handle large selections efficiently

#### Why Polymorphic References?
Instead of separate tables per data type (`well_selections`, `curve_selections`), we use:
- **Single table** with nullable columns for each type
- **Check constraints** ensure exactly one ID is set
- **Triggers** validate that IDs match the selection's `data_type`
- **Indexes** on each ID column for fast lookups

**Benefits:**
- Single query interface for all selection types
- Consistent API across all data types
- Easier to maintain and extend

#### Why Not JSONB?
While JSONB could store IDs as arrays, we chose separate columns because:
- **Type Safety**: Proper foreign key constraints
- **Query Performance**: Indexed columns are faster than JSONB queries
- **Referential Integrity**: Cascade deletes work automatically
- **Query Flexibility**: Easier to join with source tables

### Key Features

#### 1. Type Safety
- Enum for `data_type` ensures only valid types
- Check constraints ensure exactly one ID column is set
- Trigger validates IDs match selection's data_type

#### 2. Uniqueness
- Unique constraints prevent duplicate items in a selection
- Separate unique constraints per data type (using partial indexes)

#### 3. Ordering
- `item_order` column maintains user-defined order
- Indexed for efficient sorting

#### 4. Sharing & Privacy
- `is_public` flag for project-wide visibility
- RLS policies enforce access control
- Project association for team collaboration

#### 5. Organization
- `tags` array for flexible categorization
- `is_favorite` for quick access
- Views for common query patterns

### Helper Functions

#### `get_selection_item_count(selection_id)`
Returns the number of items in a selection.

#### `add_selection_items(selection_id, ...)`
Helper function to add multiple items at once:
```sql
-- Add wells to a selection
SELECT add_selection_items(
    'selection-uuid',
    p_well_ids := ARRAY[1, 2, 3, 4, 5]
);

-- Add curves to a selection
SELECT add_selection_items(
    'selection-uuid',
    p_curve_ids := ARRAY['uuid1', 'uuid2', 'uuid3']
);
```

### Views

#### `selections_view`
Selections with item counts:
```sql
SELECT * FROM selections_view
WHERE owner_id = auth.uid()
ORDER BY created_at DESC;
```

#### `well_selections_view`
Well selections with well details joined:
```sql
SELECT * FROM well_selections_view
WHERE selection_id = 'selection-uuid'
ORDER BY item_order;
```

#### `curve_selections_view`
Curve selections with curve and well details joined:
```sql
SELECT * FROM curve_selections_view
WHERE selection_id = 'selection-uuid'
ORDER BY item_order;
```

## Usage Examples

### Create a Well Selection

```sql
-- 1. Create the selection
INSERT INTO selections (name, description, data_type, owner_id, project_id)
VALUES (
    'Permian Basin Wells',
    'All wells in the Permian Basin region',
    'wells',
    auth.uid(),
    'project-uuid'
)
RETURNING id;

-- 2. Add wells to the selection
INSERT INTO selection_items (selection_id, well_id, item_order)
SELECT 
    'selection-uuid',
    id,
    ROW_NUMBER() OVER (ORDER BY name)
FROM wells
WHERE x BETWEEN -103 AND -102  -- Permian Basin coordinates
  AND y BETWEEN 31 AND 32;

-- Or use the helper function
SELECT add_selection_items(
    'selection-uuid',
    p_well_ids := ARRAY[1, 2, 3, 4, 5]
);
```

### Create a Curve Selection

```sql
-- 1. Create the selection
INSERT INTO selections (name, data_type, owner_id)
VALUES ('Gamma Ray Curves', 'curves', auth.uid())
RETURNING id;

-- 2. Add curves to the selection
INSERT INTO selection_items (selection_id, curve_id, item_order)
SELECT 
    'selection-uuid',
    c.id,
    ROW_NUMBER() OVER (ORDER BY w.name, c.curve_name)
FROM curves c
JOIN curve_metadata cm ON c.curve_metadata_id = cm.id
JOIN wells w ON c.well_id = w.id
WHERE cm.main_curve_type = 'GR';
```

### Query Selections

```sql
-- Get all well selections for current user
SELECT 
    s.*,
    COUNT(si.id) as item_count
FROM selections s
LEFT JOIN selection_items si ON s.id = si.selection_id
WHERE s.owner_id = auth.uid()
  AND s.data_type = 'wells'
GROUP BY s.id
ORDER BY s.created_at DESC;

-- Get items in a selection with details
SELECT 
    w.id,
    w.name,
    w.x,
    w.y
FROM selection_items si
JOIN wells w ON si.well_id = w.id
WHERE si.selection_id = 'selection-uuid'
ORDER BY si.item_order;
```

### Update Selection

```sql
-- Add a well to existing selection
INSERT INTO selection_items (selection_id, well_id)
VALUES ('selection-uuid', 42)
ON CONFLICT (selection_id, well_id) DO NOTHING;

-- Remove a well from selection
DELETE FROM selection_items
WHERE selection_id = 'selection-uuid'
  AND well_id = 42;

-- Reorder items
UPDATE selection_items
SET item_order = 1
WHERE selection_id = 'selection-uuid'
  AND well_id = 42;
```

## Performance Considerations

### Indexes
- **Selections**: Indexed on `owner_id`, `project_id`, `data_type`, `created_at`
- **Selection Items**: Indexed on `selection_id`, each ID column, and `item_order`
- **Composite**: Indexes on `(owner_id, data_type)` and `(project_id, data_type)`

### Query Optimization
- Use views for common query patterns
- Partial indexes on ID columns (WHERE id IS NOT NULL) reduce index size
- GIN index on `tags` array for fast tag searches

## Security (RLS)

- **Read**: Users can read their own selections and public selections in accessible projects
- **Write**: Users can only modify their own selections
- **Delete**: Cascade deletes ensure data consistency

## Future Extensions

The schema is designed to be extensible:
- Add new data types by extending the enum
- Add new columns to `selection_items` for new types
- Maintain backward compatibility with existing selections

## Migration Notes

When executing this schema:
1. Ensure `auth.users` table exists (Supabase provides this)
2. Ensure referenced tables exist (`wells`, `curves`, `nodes`, `pipelines`, `projects`)
3. The schema uses `gen_random_uuid()` - ensure `uuid-ossp` extension is available
4. RLS policies reference `auth.uid()` - ensure Supabase Auth is configured

