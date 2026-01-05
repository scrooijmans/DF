# UDF to Curve Metadata Mapping System

## Overview

This document describes the system for mapping UDFs (User-Defined Functions) to their required input and output curve metadata types. This mapping enables:

1. **Type-safe DAG node connections**: Validate that operator nodes receive compatible input curves
2. **Curve selection UI**: Show users which curves are available/compatible for each UDF
3. **Query optimization**: Efficiently query curves across multiple wells by metadata type
4. **Data discovery**: Find all curves of a specific type (e.g., "all GR curves for wells A, B, C")

## Architecture

### Components

1. **TypeScript Mapping File** (`udf-curve-metadata-mapping.ts`)
   - Default mappings for all registered UDFs
   - Used as fallback when database mappings don't exist
   - Source of truth for initial mappings

2. **Database Table** (`udf_curve_metadata_mappings`)
   - Stores user-customizable mappings
   - Allows overriding default mappings
   - Persists across sessions

3. **Mapping Service** (`udf-curve-metadata-mapping-service.ts`)
   - CRUD operations for database mappings
   - Falls back to TypeScript mappings if database entry doesn't exist
   - Provides both async (database) and sync (TypeScript) access

4. **Curve Query Helper** (`curve-query-helper.ts`)
   - Unified interface for querying curves by metadata type
   - Handles multi-well queries efficiently
   - Returns structured results with metadata

5. **UI Component** (`udf-curve-metadata-mapping.svelte`)
   - Admin interface for managing mappings
   - Similar to `curve-mapping.svelte` but for UDFs
   - Allows editing input/output curve metadata types

## Database Schema

### `udf_curve_metadata_mappings` Table

```sql
CREATE TABLE public.udf_curve_metadata_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    udf_name TEXT NOT NULL UNIQUE,
    input_curve_metadata_types JSONB NOT NULL DEFAULT '[]'::jsonb,
    output_curve_metadata_type TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES auth.users(id),
    
    CONSTRAINT udf_mapping_output_type_fkey 
        FOREIGN KEY (output_curve_metadata_type) 
        REFERENCES curve_metadata(curve_mnemonic)
        ON DELETE RESTRICT
);
```

**Key Fields**:
- `udf_name`: UDF function name (e.g., `linear_vsh`, `archie_sw`)
- `input_curve_metadata_types`: Array of `curve_mnemonic` values (e.g., `["GR", "GR_CLEAN", "GR_SHALE"]`)
- `output_curve_metadata_type`: Single `curve_mnemonic` value (e.g., `"VSH"`)

## Curve Metadata Types

### Standard Types (from `curve_metadata` table)

- **GR**: Gamma Ray
- **RT**: Resistivity
- **RHOB**: Bulk Density
- **NPHI**: Neutron Porosity
- **DT**: Sonic Transit Time
- **CALI**: Caliper
- **SP**: Spontaneous Potential
- **PE**: Photo-electric Factor

### UDF-Specific Types (added by migration)

- **GR_CLEAN**: Gamma Ray Clean (for shale volume calculations)
- **GR_SHALE**: Gamma Ray Shale (for shale volume calculations)
- **VSH**: Shale Volume (output of shale volume UDFs)
- **POR**: Porosity (output of porosity UDFs)
- **SW**: Water Saturation (output of saturation UDFs)
- **STATISTIC**: Statistical result (output of statistical UDFs)
- **ANY_NUMERIC**: Generic type for any numeric curve (input for statistical UDFs)
- **PERCENTILE_VALUE**: Percentile parameter (0.0 to 1.0)
- **TEMPERATURE**: Temperature measurement (for unit conversion UDFs)

## Usage Examples

### 1. Get Mapping for a UDF

```typescript
import { getUdfCurveMetadataMapping } from '$lib/services/udf-curve-metadata-mapping-service';

// Get mapping from database (with TypeScript fallback)
const mapping = await getUdfCurveMetadataMapping('linear_vsh');
console.log(mapping.input_curve_metadata_types); // ['GR', 'GR_CLEAN', 'GR_SHALE']
console.log(mapping.output_curve_metadata_type); // 'VSH'
```

### 2. Query Curves by Metadata Type

```typescript
import { queryCurvesByMetadataType } from '$lib/services/curve-query-helper';

// Get all GR curves for wells A, B, C
const grCurves = await queryCurvesByMetadataType(
  ['well-123', 'well-456', 'well-789'],
  'GR'
);

// Returns array of curves with metadata:
// [
//   { well_id: 'well-123', well_name: 'Well A', curve_mnemonic: 'GR', ... },
//   { well_id: 'well-456', well_name: 'Well B', curve_mnemonic: 'GR', ... },
//   ...
// ]
```

### 3. Query Multiple Curve Types

```typescript
import { queryCurvesByMetadataTypes } from '$lib/services/curve-query-helper';

// Get GR, RHOB, and NPHI curves for multiple wells
const curves = await queryCurvesByMetadataTypes(
  ['well-123', 'well-456'],
  ['GR', 'RHOB', 'NPHI']
);

// Returns Map:
// {
//   'GR': [...],
//   'RHOB': [...],
//   'NPHI': [...]
// }
```

### 4. Validate UDF Input Compatibility

```typescript
import { canUseCurveAsInput } from '$lib/services/udf-curve-metadata-mapping';

// Check if a curve can be used as input for a UDF
const canUse = canUseCurveAsInput('linear_vsh', 'GR'); // true
const cannotUse = canUseCurveAsInput('linear_vsh', 'RT'); // false
```

### 5. Find UDFs That Produce a Specific Output

```typescript
import { getUdfsForOutputType } from '$lib/services/udf-curve-metadata-mapping';

// Find all UDFs that produce VSH
const vshUdfs = getUdfsForOutputType('VSH');
// Returns: ['linear_vsh', 'larionov_old_vsh', 'larionov_young_vsh', 'clavier_vsh', 'steiber_vsh']
```

## Default Mappings

All registered UDFs have default mappings defined in `udf-curve-metadata-mapping.ts`:

### Shale Volume UDFs
- **Input**: `GR`, `GR_CLEAN`, `GR_SHALE`
- **Output**: `VSH`

### Porosity UDFs
- **Input**: `RHOB` (density), `NPHI` (neutron), `DT` (sonic)
- **Output**: `POR`

### Saturation UDFs
- **Input**: `RT`, `POR`, `VSH` (depending on method)
- **Output**: `SW`

### Statistical UDFs
- **Input**: `ANY_NUMERIC` (any numeric curve)
- **Output**: `STATISTIC`

### Unit Conversion UDFs
- **Input**: `TEMPERATURE`, `DEPT`, `RHOB` (depending on conversion)
- **Output**: Same type as input (converted units)

## Migration Steps

1. **Run SQL migrations**:
   ```bash
   # Add missing curve metadata types
   psql -f db/migrations/005-add-missing-curve-metadata-for-udfs.sql
   
   # Create mapping table and insert defaults
   psql -f db/migrations/006-create-udf-curve-metadata-mapping-table.sql
   ```

2. **Verify mappings**:
   ```sql
   SELECT udf_name, input_curve_metadata_types, output_curve_metadata_type 
   FROM udf_curve_metadata_mappings 
   ORDER BY udf_name;
   ```

3. **Access UI component**:
   - Navigate to Node Editor
   - Access UDF Curve Metadata Mapping component
   - Review and customize mappings as needed

## Integration with DAG Editor

When building DAG nodes:

1. **Node Creation**: User selects a UDF operator
2. **Input Validation**: System checks available curves against UDF's `input_curve_metadata_types`
3. **Curve Selection**: UI shows only compatible curves from selected wells
4. **Output Schema**: System infers output schema from UDF's `output_curve_metadata_type`
5. **Connection Validation**: When connecting nodes, validate that upstream output matches downstream input

## Future Enhancements

1. **Automatic Mapping Detection**: Analyze UDF code to infer input/output types
2. **Multi-Output Support**: Support UDFs that produce multiple output curves
3. **Versioned Mappings**: Track mapping changes over time
4. **Custom Curve Types**: Allow users to define custom curve metadata types
5. **Mapping Templates**: Pre-configured mappings for common workflows

