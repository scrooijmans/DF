# UDF Curve Metadata Mapping - Implementation Summary

## What Was Created

### 1. TypeScript Mapping File
**File**: `src/lib/services/udf-curve-metadata-mapping.ts`

- Default mappings for all 22 registered UDFs
- Maps each UDF to input/output curve metadata types
- Helper functions for querying mappings

### 2. Database Migrations

**Migration 005**: `db/migrations/005-add-missing-curve-metadata-for-udfs.sql`
- Adds missing curve metadata types required by UDFs:
  - `GR_CLEAN`, `GR_SHALE` (for shale volume calculations)
  - `POR`, `VSH`, `SW` (output types)
  - `ANY_NUMERIC`, `PERCENTILE_VALUE`, `STATISTIC` (generic types)
  - `TEMPERATURE` (for unit conversions)
- Updates existing entries if they have wrong configuration

**Migration 006**: `db/migrations/006-create-udf-curve-metadata-mapping-table.sql`
- Creates `udf_curve_metadata_mappings` table
- Inserts default mappings for all 22 UDFs
- Sets up RLS policies and indexes

### 3. Database Service
**File**: `src/lib/services/udf-curve-metadata-mapping-service.ts`

- `getAllUdfCurveMetadataMappings()` - Get all mappings from database
- `getUdfCurveMetadataMapping(udfName)` - Get mapping for specific UDF (with TypeScript fallback)
- `saveUdfCurveMetadataMapping(mapping)` - Save/update mapping
- `deleteUdfCurveMetadataMapping(udfName)` - Delete mapping
- `getUdfCurveMetadataMappings(udfNames)` - Batch fetch mappings

### 4. Curve Query Helper
**File**: `src/lib/services/curve-query-helper.ts`

- `queryCurvesByMetadataType(wellIds, curveMnemonic)` - Get curves of specific type for multiple wells
- `queryCurvesByMetadataTypes(wellIds, curveMnemonics)` - Get multiple curve types at once
- `getAvailableCurveMetadataTypes(wellIds)` - Discover available curve types

### 5. UI Component
**File**: `src/lib/components/pages/home/content-main/content-node-editor/udf-curve-metadata-mapping.svelte`

- AG Grid table for managing UDF mappings
- Edit input/output curve metadata types
- Auto-saves to database on changes
- Similar UI to `curve-mapping.svelte`

## How to Use

### Step 1: Run Migrations

```bash
# Run migrations in order
psql -f db/migrations/005-add-missing-curve-metadata-for-udfs.sql
psql -f db/migrations/006-create-udf-curve-metadata-mapping-table.sql
```

### Step 2: Verify Mappings

```sql
-- Check that all UDFs have mappings
SELECT udf_name, input_curve_metadata_types, output_curve_metadata_type 
FROM udf_curve_metadata_mappings 
ORDER BY udf_name;

-- Verify curve metadata types exist
SELECT curve_mnemonic, main_curve_type, subcurve_name 
FROM curve_metadata 
WHERE curve_mnemonic IN ('GR_CLEAN', 'GR_SHALE', 'VSH', 'POR', 'SW', 'STATISTIC', 'ANY_NUMERIC', 'TEMPERATURE')
ORDER BY curve_mnemonic;
```

### Step 3: Use in Application

```typescript
// Example: Get curves for wells A, B, C with type GR
import { queryCurvesByMetadataType } from '$lib/services/curve-query-helper';

const grCurves = await queryCurvesByMetadataType(
  ['well-123', 'well-456', 'well-789'],
  'GR'
);
// Returns: [{ well_id: 'well-123', well_name: 'Well A', curve_mnemonic: 'GR', ... }, ...]

// Example: Get UDF mapping
import { getUdfCurveMetadataMapping } from '$lib/services/udf-curve-metadata-mapping-service';

const mapping = await getUdfCurveMetadataMapping('linear_vsh');
console.log(mapping.input_curve_metadata_types); // ['GR', 'GR_CLEAN', 'GR_SHALE']
console.log(mapping.output_curve_metadata_type); // 'VSH'
```

## Default Mappings

All 22 registered UDFs have been mapped:

- **5 Shale Volume UDFs**: Input `GR`, `GR_CLEAN`, `GR_SHALE` → Output `VSH`
- **7 Porosity UDFs**: Input `RHOB`/`NPHI`/`DT` → Output `POR`
- **4 Saturation UDFs**: Input `RT`, `POR`, `VSH` → Output `SW`
- **4 Statistical UDFs**: Input `ANY_NUMERIC` → Output `STATISTIC`
- **3 Unit Conversion UDFs**: Input/Output same type (converted units)

## Next Steps

1. **Run migrations** to create database tables and insert default mappings
2. **Access UI component** to review/customize mappings if needed
3. **Integrate into DAG editor** to use mappings for node validation
4. **Use curve query helper** when building ingestion nodes to query curves by type

## Files Created/Modified

### New Files
- `src/lib/services/udf-curve-metadata-mapping.ts`
- `src/lib/services/udf-curve-metadata-mapping-service.ts`
- `src/lib/services/curve-query-helper.ts`
- `src/lib/components/pages/home/content-main/content-node-editor/udf-curve-metadata-mapping.svelte`
- `db/migrations/005-add-missing-curve-metadata-for-udfs.sql`
- `db/migrations/006-create-udf-curve-metadata-mapping-table.sql`
- `markdown-guides/database/supabase/db/udfs/udf-curve-metadata-mapping.md`
- `markdown-guides/database/supabase/db/udfs/udf-curve-metadata-mapping-summary.md`

### Integration Points
- Can be integrated into `content-node-editor.svelte` or accessed as a separate admin page
- Mappings are used by DAG node validation and curve selection UI
- Curve query helper can be used in ingestion node builders

