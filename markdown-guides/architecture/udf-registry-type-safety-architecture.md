# UDF Registry Architecture Analysis: Decoupling UDFs, Curve Metadata, and Mappings

## Executive Summary

**Current Problem**: Tight coupling between UDF definitions (Rust), curve metadata types (PostgreSQL), and mappings (separate system) creates maintenance burden and prevents dynamic updates.

**Recommended Solution**: **Unified Operator Registry** - Use the existing `operator_registry` table as the single source of truth, embedding curve metadata types directly in operator schemas. This provides:

- ✅ Single source of truth (database)
- ✅ Type-safe implementations (Rust code)
- ✅ Dynamic metadata updates (no code changes needed)
- ✅ Unified system (UDFs + native operators)
- ✅ Version control and deprecation support

**Migration Complexity**: Medium (2-3 weeks)
**Long-term Value**: High - eliminates coupling, enables user customization, supports future enhancements

---

## Current Architecture Problems

### Problem 1: Three Separate Systems

```
┌─────────────────────────────────────────────────────────────┐
│                    CURRENT STATE                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Rust Backend (udf_registry.rs)                          │
│     ┌──────────────────────────────────────────────────┐    │
│     │ Hardcoded UdfDefinition structs                  │    │
│     │ - name, description, input_types, output_type    │    │
│     │ - No curve metadata information                  │    │
│     └──────────────────────────────────────────────────┘    │
│                                                              │
│  2. PostgreSQL (curve_metadata table)                       │
│     ┌──────────────────────────────────────────────────┐    │
│     │ curve_mnemonic, main_curve_type, subcurve_name   │    │
│     │ - No knowledge of UDFs                            │    │
│     └──────────────────────────────────────────────────┘    │
│                                                              │
│  3. PostgreSQL (udf_curve_metadata_mappings table)          │
│     ┌──────────────────────────────────────────────────┐    │
│     │ udf_name → input_curve_types, output_curve_type │    │
│     │ - Separate mapping system                        │    │
│     │ - Manual synchronization required                │    │
│     └──────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Issues**:

- Adding a new UDF requires changes in 3 places
- No type safety between systems
- Manual synchronization prone to errors
- No single source of truth
- Hard to query "what UDFs use GR curve?"
- Difficult to version control metadata changes

### Problem 2: Missing Integration Points

When querying "all GR curves for wells A, B, C", you need to:

1. Query `curve_metadata` to find GR curve metadata ID
2. Query `curves` table filtered by `curve_metadata_id`
3. Filter by `well_id` in wells A, B, C
4. Join with `wells` table for well names

This is scattered across multiple tables with no unified interface.

---

## Solution Options Analysis

### Option 1: Unified Operator Registry ⭐ **RECOMMENDED**

**Concept**: Use `operator_registry` table as single source of truth. Embed curve metadata types in `input_schema`/`output_schema` JSONB fields.

#### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│          UNIFIED OPERATOR REGISTRY ARCHITECTURE              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  PostgreSQL (operator_registry) - SINGLE SOURCE OF TRUTH    │
│  ┌────────────────────────────────────────────────────┐     │
│  │ operator_id: "linear_vsh"                         │     │
│  │ operator_name: "Linear Shale Volume"              │     │
│  │ category: "subsurface"                            │     │
│  │ implementation_type: "udf"                        │     │
│  │ version: "1.0.0"                                 │     │
│  │                                                   │     │
│  │ input_schema: {                                  │     │
│  │   schema_id: "shale_volume_input",               │     │
│  │   arrow_schema_json: "{...}",                    │     │
│  │   required_columns: ["gr", "gr_clean", "gr_shale"],│   │
│  │   curve_metadata_types: ["GR", "GR_CLEAN", "GR_SHALE"]│ │
│  │ }                                                 │     │
│  │                                                   │     │
│  │ output_schema: {                                 │     │
│  │   schema_id: "shale_volume_output",              │     │
│  │   arrow_schema_json: "{...}",                    │     │
│  │   required_columns: ["vsh"],                     │     │
│  │   curve_metadata_type: "VSH"                     │     │
│  │ }                                                 │     │
│  │                                                   │     │
│  │ parameters_schema: {...}                         │     │
│  └────────────────────────────────────────────────────┘     │
│           │                                                  │
│           │ (references via foreign key)                      │
│           ▼                                                  │
│  PostgreSQL (curve_metadata)                                │
│  ┌────────────────────────────────────────────────────┐     │
│  │ curve_mnemonic: "GR"                             │     │
│  │ main_curve_type: "GR"                            │     │
│  │ subcurve_name: "GR"                              │     │
│  └────────────────────────────────────────────────────┘     │
│                                                              │
│  Rust Backend (udf_registry.rs)                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │ // Only implementations, no metadata             │     │
│  │ pub fn linear_vsh(gr: f64, gr_clean: f64, ...) -> f64│   │
│  │                                                   │     │
│  │ // Load metadata from operator_registry at startup│     │
│  │ let metadata = load_from_db("linear_vsh").await?;│     │
│  │ register_udf(metadata, linear_vsh_impl);         │     │
│  └────────────────────────────────────────────────────┘     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

#### Implementation Details

**1. Extend SchemaDefinition**:

```rust
// crates/dags/core/src/schema.rs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaDefinition {
    pub schema_id: String,
    pub arrow_schema_json: String,
    pub required_columns: Vec<String>,
    pub optional_columns: Vec<String>,

    // NEW: Curve metadata integration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve_metadata_types: Option<Vec<String>>,  // For input schemas

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve_metadata_type: Option<String>,        // For output schemas

    // NEW: Column-to-curve mapping (more flexible)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_curve_mapping: Option<HashMap<String, String>>,  // column_name -> curve_mnemonic
}
```

**2. Database Migration**:

```sql
-- Migration: Extend operator_registry to support curve metadata
-- No new columns needed - embed in existing JSONB schemas

-- Update existing UDF operators to include curve metadata in schemas
UPDATE operator_registry
SET
    input_schema = jsonb_set(
        input_schema,
        '{curve_metadata_types}',
        '["GR", "GR_CLEAN", "GR_SHALE"]'::jsonb
    ),
    output_schema = jsonb_set(
        output_schema,
        '{curve_metadata_type}',
        '"VSH"'::jsonb
    )
WHERE operator_id = 'linear_vsh' AND implementation_type = 'udf';

-- Add helper function to query operators by curve metadata type
CREATE OR REPLACE FUNCTION get_operators_by_input_curve(curve_mnemonic TEXT)
RETURNS TABLE (
    operator_id TEXT,
    operator_name TEXT,
    category TEXT,
    description TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        op.operator_id,
        op.operator_name,
        op.category,
        op.description
    FROM operator_registry op
    WHERE
        op.implementation_type = 'udf'
        AND op.is_latest = true
        AND (
            op.input_schema->>'curve_metadata_types' LIKE '%' || curve_mnemonic || '%'
            OR op.input_schema->'column_curve_mapping' ? curve_mnemonic
        );
END;
$$ LANGUAGE plpgsql;

-- Add helper function to query operators by output curve type
CREATE OR REPLACE FUNCTION get_operators_by_output_curve(curve_mnemonic TEXT)
RETURNS TABLE (
    operator_id TEXT,
    operator_name TEXT,
    category TEXT,
    description TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        op.operator_id,
        op.operator_name,
        op.category,
        op.description
    FROM operator_registry op
    WHERE
        op.implementation_type = 'udf'
        AND op.is_latest = true
        AND op.output_schema->>'curve_metadata_type' = curve_mnemonic;
END;
$$ LANGUAGE plpgsql;
```

**3. Rust UDF Registry Refactor**:

```rust
// crates/query-engine/udf-core/src/udf_registry.rs

use dag_storage::operator_registry::OperatorRegistryLoader;
use dag_core::schema::SchemaDefinition;

pub struct UdfRegistry {
    db_client: Option<Client>,
    // Cache: operator_id -> (metadata, implementation)
    cache: HashMap<String, (OperatorDefinition, Box<dyn UdfImplementation>)>,
}

impl UdfRegistry {
    /// Initialize registry by loading metadata from database
    pub async fn new_with_database(client: Client) -> Result<Self, UdfRegistryError> {
        let loader = OperatorRegistryLoader::new(client.clone());

        // Load all UDF operators from database
        let rows = client.query(
            r#"
            SELECT operator_id, operator_name, category, version,
                   input_schema, output_schema, parameters_schema
            FROM operator_registry
            WHERE implementation_type = 'udf' AND is_latest = true
            "#,
            &[],
        ).await?;

        let mut registry = Self {
            db_client: Some(client),
            cache: HashMap::new(),
        };

        // Cache metadata (implementations registered separately)
        for row in rows {
            let operator_id: String = row.get("operator_id");
            let input_schema_json: Json<Value> = row.get("input_schema");
            let output_schema_json: Json<Value> = row.get("output_schema");

            let input_schema: SchemaDefinition = serde_json::from_value(input_schema_json.0)?;
            let output_schema: SchemaDefinition = serde_json::from_value(output_schema_json.0)?;

            // Store metadata (implementation will be registered later)
            // Implementation comes from Rust code, metadata from DB
        }

        Ok(registry)
    }

    /// Register UDF implementation (Rust code)
    /// Metadata must already be loaded from database
    pub fn register_implementation(
        &mut self,
        operator_id: &str,
        implementation: Box<dyn UdfImplementation>,
    ) -> Result<(), UdfRegistryError> {
        // Get metadata from cache
        // Combine with implementation
        // Store in cache
        Ok(())
    }

    /// Get curve metadata types for a UDF input
    pub fn get_input_curve_types(&self, operator_id: &str) -> Option<&Vec<String>> {
        self.cache.get(operator_id)
            .and_then(|(metadata, _)| {
                metadata.input_schema.curve_metadata_types.as_ref()
            })
    }

    /// Get curve metadata type for a UDF output
    pub fn get_output_curve_type(&self, operator_id: &str) -> Option<&String> {
        self.cache.get(operator_id)
            .and_then(|(metadata, _)| {
                metadata.output_schema.curve_metadata_type.as_ref()
            })
    }
}
```

**4. Frontend Integration**:

```typescript
// src/lib/services/udf-service.ts

// Load UDFs from operator_registry (not Rust registry)
export async function getUdfsByCategory(): Promise<UdfCategoryInfo[]> {
  const { data, error } = await supabase
    .from("operator_registry")
    .select(
      `
      operator_id,
      operator_name,
      category,
      description,
      version,
      input_schema,
      output_schema,
      parameters_schema
    `,
    )
    .eq("implementation_type", "udf")
    .eq("is_latest", true)
    .order("category", { ascending: true });

  // Extract curve metadata types from input_schema/output_schema
  // Group by category
  // Return same format as before
}

// Helper to get curve metadata types for a UDF
export function getUdfCurveMetadataTypes(udf: UdfInfo): {
  input: string[];
  output: string;
} {
  const inputSchema = udf.input_schema as any;
  const outputSchema = udf.output_schema as any;

  return {
    input: inputSchema.curve_metadata_types || [],
    output: outputSchema.curve_metadata_type || "",
  };
}
```

#### Pros ✅

- **Single source of truth**: All operator metadata in `operator_registry`
- **Unified system**: UDFs and native operators use same registry
- **Type safety**: Foreign keys ensure curve metadata types exist
- **Version control**: Built-in versioning, deprecation, compatibility
- **Dynamic updates**: Change metadata without code changes
- **Queryable**: SQL functions to find operators by curve type
- **Already exists**: `operator_registry` table is designed for this
- **Future-proof**: Supports custom operators, WASM, etc.

#### Cons ❌

- **Migration effort**: Need to migrate existing UDFs (2-3 weeks)
- **Schema complexity**: JSONB schemas become more complex
- **Runtime loading**: Must load from database at startup (caching mitigates)
- **Implementation coupling**: Still need Rust code for implementations

#### Migration Complexity: Medium

- Week 1: Schema extension + migration script
- Week 2: Rust code refactor
- Week 3: Frontend updates + testing
- Week 4: Deprecate old system

---

### Option 2: Schema-First with Code Generation

**Concept**: Define UDFs in YAML/TOML, generate Rust code and SQL from it.

#### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              SCHEMA-FIRST ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Source: udf-definitions.yaml                               │
│  ┌────────────────────────────────────────────────────┐     │
│  │ udfs:                                              │     │
│  │   - name: linear_vsh                               │     │
│  │     category: shale_volume                         │     │
│  │     input_curve_types: [GR, GR_CLEAN, GR_SHALE]   │     │
│  │     output_curve_type: VSH                         │     │
│  │     implementation:                                │     │
│  │       type: rust                                   │     │
│  │       function: linear_vsh                         │     │
│  └────────────────────────────────────────────────────┘     │
│           │                                                  │
│           │ (build-time generation)                         │
│           ├──────────────────────────────┐                  │
│           │                              │                  │
│           ▼                              ▼                  │
│  Generated Rust              Generated SQL                  │
│  ┌──────────────────┐        ┌──────────────────┐          │
│  │ // Auto-gen      │        │ INSERT INTO ... │          │
│  │ pub fn register..│        │                  │          │
│  └──────────────────┘        └──────────────────┘          │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

#### Pros ✅

- **Single source of truth**: YAML file
- **Version control friendly**: Git tracks all changes
- **Type safety**: Generated code ensures consistency
- **No runtime DB dependency**: Code generated at build time
- **Documentation**: Can generate docs from schema

#### Cons ❌

- **Build complexity**: Requires code generation pipeline
- **Less dynamic**: Still requires rebuild for new UDFs
- **Tooling overhead**: Must maintain generation tools
- **High migration effort**: Significant refactoring
- **Developer friction**: Must run build script to add UDFs

#### Migration Complexity: High (4-6 weeks)

---

### Option 3: Fully Dynamic Database-Driven

**Concept**: Store everything in database - metadata AND implementations (as WASM or stored code).

#### Pros ✅

- **Fully dynamic**: Add UDFs without any code changes
- **User-defined UDFs**: Users can create custom UDFs
- **Hot reloading**: Update UDFs without restart

#### Cons ❌

- **Security concerns**: Executing user code requires sandboxing
- **Performance**: Runtime compilation/loading is slower
- **Complexity**: Need WASM runtime or code compilation
- **Type safety**: Harder to ensure at compile time
- **Not suitable for MVP**: Too complex for current needs

#### Migration Complexity: Very High (8+ weeks)

---

### Option 4: Hybrid - Metadata in DB, Implementations in Code

**Concept**: Store metadata in database, keep implementations in Rust. Load metadata at startup.

#### Pros ✅

- **Balanced**: Good compromise
- **Type-safe**: Implementations remain compiled
- **Dynamic metadata**: Can update mappings without code changes
- **Performance**: Implementations are compiled

#### Cons ❌

- **Two sources**: Metadata in DB, implementations in code
- **Sync risk**: Must ensure metadata matches implementation
- **Still requires code**: New UDFs need Rust implementation

#### Migration Complexity: Low-Medium (1-2 weeks)

**Note**: This is essentially what Option 1 does, but Option 1 is better because it uses the existing `operator_registry` infrastructure.

---

## Detailed Comparison

| Criteria                   | Option 1: Unified Registry          | Option 2: Schema-First | Option 3: Fully Dynamic | Option 4: Hybrid     |
| -------------------------- | ----------------------------------- | ---------------------- | ----------------------- | -------------------- |
| **Single Source of Truth** | ✅ Database (`operator_registry`)   | ✅ YAML file           | ✅ Database             | ⚠️ Split (DB + Code) |
| **Type Safety**            | ✅ Strong (FKs + Rust)              | ✅ Strong (generated)  | ⚠️ Runtime only         | ✅ Strong            |
| **Dynamic Updates**        | ✅ Yes (metadata)                   | ❌ Requires rebuild    | ✅ Yes (everything)     | ✅ Yes (metadata)    |
| **Performance**            | ✅ Fast (cached)                    | ✅ Fast (compiled)     | ⚠️ Slower (runtime)     | ✅ Fast (compiled)   |
| **Migration Effort**       | 🟡 Medium (2-3 weeks)               | 🔴 High (4-6 weeks)    | 🔴 Very High (8+ weeks) | 🟢 Low (1-2 weeks)   |
| **User Customization**     | ✅ Yes (DB updates)                 | ❌ No                  | ✅ Yes                  | ⚠️ Limited           |
| **Version Control**        | ✅ DB versioning                    | ✅ Git-friendly        | ✅ DB versioning        | ⚠️ Split             |
| **Complexity**             | 🟡 Medium                           | 🟡 Medium              | 🔴 High                 | 🟢 Low               |
| **Security**               | ✅ Safe                             | ✅ Safe                | ⚠️ Needs sandboxing     | ✅ Safe              |
| **Already Built**          | ✅ Yes (`operator_registry` exists) | ❌ No                  | ❌ No                   | ⚠️ Partial           |
| **Future-Proof**           | ✅ Excellent                        | ⚠️ Good                | ✅ Excellent            | ⚠️ Limited           |
| **Queryability**           | ✅ SQL functions                    | ❌ No                  | ✅ SQL queries          | ⚠️ Limited           |
| **Maintainability**        | ✅ Excellent                        | ⚠️ Good                | ⚠️ Complex              | ⚠️ Good              |

---

## Recommended Solution: Option 1 - Unified Operator Registry

### Why This Is Best

1. **Leverages Existing Infrastructure**
   - `operator_registry` table already exists
   - Designed for this exact purpose (see `dag-execution.md`)
   - Already has versioning, deprecation, compatibility tracking

2. **Single Source of Truth**
   - All operator metadata (UDFs + native) in one table
   - Curve metadata types embedded in schemas
   - No separate mapping tables needed

3. **Type Safety**
   - Foreign keys ensure curve metadata types exist
   - Rust implementations remain type-safe
   - Schema validation at multiple levels

4. **Unified System**
   - UDFs and native operators use same registry
   - Consistent API for all operators
   - Easier to reason about

5. **Future-Proof**
   - Supports custom operators (users can add their own)
   - Supports WASM operators (when needed)
   - Supports operator versioning and deprecation
   - Easy to extend with new fields

6. **Queryability**
   - SQL functions to find operators by curve type
   - Easy to discover "what operators use GR?"
   - Easy to find "what operators produce VSH?"

7. **Migration Path**
   - Can migrate incrementally
   - No breaking changes required
   - Backward compatible during transition

### Implementation Plan

#### Phase 1: Schema Extension (Week 1)

**Step 1.1**: Extend `SchemaDefinition` to include curve metadata

```rust
// crates/dags/core/src/schema.rs
// Add curve_metadata_types and curve_metadata_type fields
```

**Step 1.2**: Create migration to update existing operators

```sql
-- Update operator_registry entries to include curve metadata in schemas
-- Use JSONB functions to add fields to existing schemas
```

**Step 1.3**: Create SQL helper functions

```sql
-- get_operators_by_input_curve(curve_mnemonic)
-- get_operators_by_output_curve(curve_mnemonic)
-- get_curve_metadata_for_operator(operator_id)
```

#### Phase 2: Migrate UDFs (Week 2)

**Step 2.1**: Create migration script to insert all UDFs into `operator_registry`

```sql
-- Read from udf-curve-metadata-mapping.ts
-- Generate INSERT statements for operator_registry
-- Include curve metadata in input_schema/output_schema JSONB
```

**Step 2.2**: Verify all UDFs are in database

```sql
-- Check that all UDFs from Rust registry exist in operator_registry
-- Verify curve metadata is correctly embedded
```

#### Phase 3: Rust Code Refactor (Week 2-3)

**Step 3.1**: Update `UdfRegistry` to load from database

```rust
// Load metadata from operator_registry at startup
// Keep implementations in Rust code
// Combine metadata + implementation when registering
```

**Step 3.2**: Update Tauri commands

```rust
// get_all_udfs() reads from operator_registry
// Extract curve metadata from schemas
// Return same format as before (backward compatible)
```

#### Phase 4: Frontend Updates (Week 3)

**Step 4.1**: Update `udf-service.ts` to read from `operator_registry`

**Step 4.2**: Update components to extract curve metadata from schemas

**Step 4.3**: Update `curve-query-helper.ts` to use new structure

#### Phase 5: Deprecation (Week 4)

**Step 5.1**: Remove `udf_curve_metadata_mappings` table (data merged)

**Step 5.2**: Remove hardcoded `UdfDefinition` structs from Rust

**Step 5.3**: Update all documentation

---

## Enhanced Schema Structure

### Input Schema Example

```json
{
  "schema_id": "shale_volume_input",
  "arrow_schema_json": "{\"fields\":[{\"name\":\"gr\",\"type\":\"Float64\"},{\"name\":\"gr_clean\",\"type\":\"Float64\"},{\"name\":\"gr_shale\",\"type\":\"Float64\"}]}",
  "required_columns": ["gr", "gr_clean", "gr_shale"],
  "optional_columns": [],
  "curve_metadata_types": ["GR", "GR_CLEAN", "GR_SHALE"],
  "column_curve_mapping": {
    "gr": "GR",
    "gr_clean": "GR_CLEAN",
    "gr_shale": "GR_SHALE"
  }
}
```

### Output Schema Example

```json
{
  "schema_id": "shale_volume_output",
  "arrow_schema_json": "{\"fields\":[{\"name\":\"vsh\",\"type\":\"Float64\"}]}",
  "required_columns": ["vsh"],
  "optional_columns": [],
  "curve_metadata_type": "VSH",
  "column_curve_mapping": {
    "vsh": "VSH"
  }
}
```

### Benefits of Enhanced Schema

1. **Self-documenting**: Schema includes curve metadata mapping
2. **Flexible**: Can map specific columns to curve types
3. **Queryable**: Can query operators by curve type using JSONB operators
4. **Backward compatible**: Existing schemas without curve metadata still work

---

## Query Examples

### Find all operators that use GR as input

```sql
SELECT operator_id, operator_name, category
FROM operator_registry
WHERE implementation_type = 'udf'
  AND is_latest = true
  AND (
    input_schema->>'curve_metadata_types' LIKE '%"GR"%'
    OR input_schema->'column_curve_mapping' ? 'GR'
  );
```

### Find all operators that produce VSH

```sql
SELECT operator_id, operator_name, category
FROM operator_registry
WHERE implementation_type = 'udf'
  AND is_latest = true
  AND output_schema->>'curve_metadata_type' = 'VSH';
```

### Get curve metadata types for an operator

```sql
SELECT
    operator_id,
    input_schema->>'curve_metadata_types' as input_curve_types,
    output_schema->>'curve_metadata_type' as output_curve_type
FROM operator_registry
WHERE operator_id = 'linear_vsh';
```

---

## Migration Script Example

```sql
-- Migration: Migrate UDFs from Rust registry to operator_registry
-- This script reads the mappings and creates operator_registry entries

-- Shale Volume UDFs
INSERT INTO operator_registry (
    operator_id, operator_name, category, description, version,
    input_schema, output_schema, parameters_schema,
    implementation_type, is_builtin, is_latest
) VALUES
(
    'linear_vsh',
    'Linear Shale Volume',
    'subsurface',
    'Linear shale volume calculation from gamma ray curves',
    '1.0.0',
    jsonb_build_object(
        'schema_id', 'shale_volume_input',
        'arrow_schema_json', '{"fields":[{"name":"gr","type":"Float64"},{"name":"gr_clean","type":"Float64"},{"name":"gr_shale","type":"Float64"}]}',
        'required_columns', ARRAY['gr', 'gr_clean', 'gr_shale'],
        'optional_columns', ARRAY[]::text[],
        'curve_metadata_types', ARRAY['GR', 'GR_CLEAN', 'GR_SHALE'],
        'column_curve_mapping', jsonb_build_object('gr', 'GR', 'gr_clean', 'GR_CLEAN', 'gr_shale', 'GR_SHALE')
    ),
    jsonb_build_object(
        'schema_id', 'shale_volume_output',
        'arrow_schema_json', '{"fields":[{"name":"vsh","type":"Float64"}]}',
        'required_columns', ARRAY['vsh'],
        'optional_columns', ARRAY[]::text[],
        'curve_metadata_type', 'VSH',
        'column_curve_mapping', jsonb_build_object('vsh', 'VSH')
    ),
    '{"type":"object","properties":{}}'::jsonb,
    'udf',
    true,
    true
)
ON CONFLICT (operator_id, version) DO UPDATE
SET
    input_schema = EXCLUDED.input_schema,
    output_schema = EXCLUDED.output_schema,
    parameters_schema = EXCLUDED.parameters_schema;

-- Repeat for all other UDFs...
```

---

## Risk Assessment

### Low Risk ✅

- **Schema extension**: Adding optional fields to JSONB is safe
- **Backward compatibility**: Existing code works with old schemas
- **Incremental migration**: Can migrate UDFs one at a time

### Medium Risk 🟡

- **Rust code changes**: Need careful refactoring of UdfRegistry
- **Frontend updates**: Must update all UDF-related components
- **Testing**: Need thorough testing of migrated UDFs

### Mitigation Strategies

1. **Feature flag**: Add feature flag to switch between old/new system
2. **Dual write**: Write to both systems during migration
3. **Gradual rollout**: Migrate UDFs incrementally
4. **Comprehensive tests**: Test each migrated UDF thoroughly

---

## Success Criteria

### Phase 1 Complete ✅

- [ ] `SchemaDefinition` extended with curve metadata fields
- [ ] Migration script updates existing operators
- [ ] SQL helper functions created and tested

### Phase 2 Complete ✅

- [ ] All UDFs migrated to `operator_registry`
- [ ] Curve metadata embedded in schemas
- [ ] Verification queries pass

### Phase 3 Complete ✅

- [ ] Rust `UdfRegistry` loads from database
- [ ] Implementations still in Rust code
- [ ] Tauri commands return same format (backward compatible)

### Phase 4 Complete ✅

- [ ] Frontend reads from `operator_registry`
- [ ] Curve metadata extracted from schemas
- [ ] All UI components updated

### Phase 5 Complete ✅

- [ ] `udf_curve_metadata_mappings` table deprecated
- [ ] Hardcoded `UdfDefinition` structs removed
- [ ] Documentation updated

---

## Long-Term Benefits

### 1. User Customization

Users can add custom operators to `operator_registry`:

- Custom UDFs (if we add WASM support)
- Custom curve metadata mappings
- Custom parameter schemas

### 2. Version Management

- Track operator versions over time
- Deprecate old versions
- Ensure schema compatibility

### 3. Discovery

- SQL queries to find operators by curve type
- Discover compatible operators for pipelines
- Find operators that produce specific outputs

### 4. Integration

- Unified API for all operators (UDFs + native)
- Consistent schema structure
- Easier to build tooling

---

## End-to-End Type Safety Architecture

### Type Safety Flow

The UDF registry ensures type safety through multiple layers:

```
┌─────────────────────────────────────────────────────────────┐
│              TYPE SAFETY ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Database Schema (PostgreSQL)                            │
│     ┌──────────────────────────────────────────────────┐    │
│     │ operator_registry.input_schema                  │    │
│     │   → curve_metadata_types: ["GR", "RHOB"]        │    │
│     │   → Foreign key constraint to curve_metadata    │    │
│     │                                                  │    │
│     │ curve_metadata.curve_mnemonic                    │    │
│     │   → UNIQUE constraint                            │    │
│     │   → CHECK constraint for valid types            │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (validated at insert/update)                    │
│           ▼                                                  │
│  2. Runtime Validation (Rust)                              │
│     ┌──────────────────────────────────────────────────┐    │
│     │ SchemaDefinition struct                          │    │
│     │   → curve_metadata_types: Vec<String>            │    │
│     │   → Validated against curve_metadata table      │    │
│     │                                                  │    │
│     │ UDF Implementation                              │    │
│     │   → Type-safe function signatures                │    │
│     │   → Arrow schema validation                      │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (validated at execution)                         │
│           ▼                                                  │
│  3. Frontend Validation (TypeScript)                        │
│     ┌──────────────────────────────────────────────────┐    │
│     │ Curve Options Provider                           │    │
│     │   → Queries curves_view filtered by              │    │
│     │     curve_metadata_types from operator           │    │
│     │   → Returns only valid curve options             │    │
│     │                                                  │    │
│     │ AG Grid Component                                │    │
│     │   → Displays available curves                    │    │
│     │   → User can only select valid options           │    │
│     └──────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Type Safety Mechanisms

#### 1. Database-Level Constraints

- **Foreign Key Constraints**: `operator_registry.input_schema->'curve_metadata_types'` references `curve_metadata.curve_mnemonic`
- **CHECK Constraints**: `curve_metadata.main_curve_type` must be one of valid types
- **UNIQUE Constraints**: `curve_metadata.curve_mnemonic` ensures no duplicates

#### 2. Schema Validation

- **Input Schema**: Operator's `input_schema.curve_metadata_types` array must contain valid curve mnemonics
- **Output Schema**: Operator's `output_schema.curve_metadata_type` must be a valid curve mnemonic
- **Column Mapping**: `column_curve_mapping` maps column names to curve mnemonics (validated)

#### 3. Runtime Type Checking

- **Rust Type System**: UDF implementations use strongly-typed Arrow schemas
- **Schema Matching**: Input/output schemas validated against operator_registry definitions
- **Execution Validation**: Curve data validated against expected schema before execution

#### 4. Frontend Type Safety

- **Curve Options Provider**: Queries only curves matching operator's input requirements
- **Type Filtering**: `getCurveOptionsForOperator()` filters by `curve_metadata_types` from operator schema
- **UI Constraints**: Users can only select from valid curve options displayed in AG Grid

### Curve Options Provider

The `curve-options-provider.ts` service provides type-safe curve selection:

```typescript
// Get curve options for an operator
const options = await getCurveOptionsForOperator("linear_vsh");
// Returns only GR, GR_CLEAN, GR_SHALE curves for current project's wells

// Get curve options for multiple types
const options = await getCurveOptionsByTypes(["GR", "RHOB", "NPHI"]);
// Returns all matching curves grouped by type
```

**Type Safety Features**:

- ✅ Queries `operator_registry` to get operator's required curve types
- ✅ Filters `curves_view` by `curve_mnemonic` matching operator requirements
- ✅ Returns well names (not IDs) for user-friendly display
- ✅ Returns subcurve names (not metadata IDs) for clarity
- ✅ Only returns curves from current project's wells

### AG Grid Integration

The `curve-options-AG-table.svelte` component displays available curves:

- **Columns**: Well Name, Curve Name, Subcurve, Mnemonic, Display Name, Main Type, Units, Parquet Path
- **Filtering**: Users can filter by any column
- **Selection**: Single row selection for curve assignment
- **Type Safety**: Only displays curves matching operator's input requirements

### Benefits of End-to-End Type Safety

1. **Prevents Invalid Configurations**: Users cannot select incompatible curves
2. **Clear Error Messages**: Type mismatches caught early with descriptive errors
3. **Better UX**: Only valid options shown, reducing user confusion
4. **Database Integrity**: Constraints prevent invalid data at the source
5. **Runtime Safety**: Type checking at execution prevents runtime errors

## Conclusion

**Option 1 (Unified Operator Registry)** is the best solution because:

1. ✅ **Leverages existing infrastructure** - `operator_registry` already exists
2. ✅ **Single source of truth** - All metadata in one place
3. ✅ **Type-safe** - Foreign keys + Rust implementations + Frontend validation
4. ✅ **Future-proof** - Supports custom operators, WASM, etc.
5. ✅ **Queryable** - SQL functions for discovery
6. ✅ **Manageable migration** - 2-3 weeks, incremental approach
7. ✅ **End-to-end type safety** - Database → Runtime → Frontend validation

**Implementation Status**:

- ✅ Phase 1: Schema extension + migration script
- ✅ Phase 2: All UDFs migrated to operator_registry
- ✅ Phase 3: Frontend reads from operator_registry
- ✅ Phase 4: Curve options provider + AG Grid component
- ⏳ Phase 5: Rust code refactor (pending)
- ⏳ Phase 6: Deprecate old system (pending)
