# Database-to-TypeScript Type Safety Strategy

## Problem Statement

MudRock faces several challenges maintaining type safety between PostgreSQL and TypeScript:

1. **Manual Type Definitions**: TypeScript types are manually maintained and can drift from database schema
2. **JSONB Complexity**: Complex JSONB structures (`allowed_data_sources`, `chart_config`, `default_config`) need both compile-time and runtime validation
3. **Dynamic Configurations**: Chart types, UDF mappings, and curve metadata require runtime flexibility with compile-time safety
4. **Multiple Sources of Truth**: Types exist in TypeScript files, database tables, and Rust code
5. **Migration Drift**: Database migrations can be applied without updating TypeScript types

## Current Approach Analysis

### ✅ What Works Well

1. **Dual-Source Mappings** (UDF Curve Metadata):
   - TypeScript defaults with database overrides
   - Provides fallback when database is unavailable
   - Allows offline-first functionality

2. **Hardcoded Schemas** (Rust):
   - Fast, no database queries needed
   - Works well for stable tables

3. **Manual Type Definitions**:
   - Full control over type structure
   - Can add computed properties and methods

### ❌ Pain Points

1. **Type Drift**: Manual types can become outdated after migrations
2. **JSONB Validation**: No runtime validation for complex JSONB structures
3. **No Single Source of Truth**: Types scattered across multiple files
4. **Migration Safety**: No automated checks that types match schema

## Recommended Solution: Hybrid Approach

### Strategy Overview

Use a **three-layer type safety system**:

1. **Generated Base Types** (from database schema)
2. **Manual Extensions** (for JSONB and business logic)
3. **Runtime Validation** (Zod schemas for JSONB)

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Database (PostgreSQL)                     │
│  - Schema definitions                                        │
│  - JSONB structures                                          │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Type Generation Layer                           │
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │ Supabase CLI    │  │ PgTyped         │                  │
│  │ (base types)    │  │ (query types)   │                  │
│  └─────────────────┘  └─────────────────┘                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              TypeScript Type Layer                           │
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │ Generated Types │  │ Manual Extensions│                  │
│  │ (database.ts)   │  │ (extensions.ts) │                  │
│  └─────────────────┘  └─────────────────┘                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Runtime Validation Layer                        │
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │ Zod Schemas     │  │ Type Guards     │                  │
│  │ (JSONB)         │  │ (discriminated) │                  │
│  └─────────────────┘  └─────────────────┘                  │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Plan

### Phase 1: Base Type Generation (Self-Hosted Supabase)

**Goal**: Generate base TypeScript types from database schema automatically.

#### Option A: Direct Database Connection (Recommended for Self-Hosted)

For self-hosted Supabase, use the `--db-url` flag with Supabase CLI:

```bash
# Generate types directly from database connection
supabase gen types typescript \
  --db-url "postgresql://postgres:MudRockSecure2024@91.99.166.223:5432/postgres" \
  --schema public \
  > src/lib/database.types.ts
```

**Note**: If the database is not publicly accessible, use an SSH tunnel:

```bash
# Set up SSH tunnel first (in separate terminal)
ssh -i ~/.ssh/id_rsa_mudrock -L 5433:localhost:5432 root@91.99.166.223

# Then generate types using localhost tunnel
supabase gen types typescript \
  --db-url "postgresql://postgres:MudRockSecure2024@localhost:5433/postgres" \
  --schema public \
  > src/lib/database.types.ts
```

#### Option B: Custom TypeScript Script (✅ WORKING - Recommended for Self-Hosted)

**Status**: ✅ **Tested and Working**

The custom script connects directly to the database without requiring Docker:

```bash
# Run custom type generation script
bun run scripts/database/generate-db-types.ts > src/lib/database.types.ts

# Or with environment variables
DB_HOST=91.99.166.223 \
DB_PORT=5432 \
DB_USER=postgres \
POSTGRES_PASSWORD=MudRockSecure2024 \
DB_NAME=postgres \
bun run scripts/database/generate-db-types.ts > src/lib/database.types.ts
```

**Script Location**: `scripts/database/generate-db-types.ts`

**Advantages**:

- ✅ No Docker required
- ✅ Works with self-hosted Supabase
- ✅ Direct database connection
- ✅ Generates Supabase-compatible types
- ✅ Handles JSONB, arrays, and custom types

**Tested**: Successfully generated types from your database on 2025-12-06

#### Generated Types Structure

```typescript
// src/lib/database.types.ts (AUTO-GENERATED - DO NOT EDIT)
export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[];

export interface Database {
  public: {
    Tables: {
      chart_types: {
        Row: {
          id: string;
          chart_type_id: string;
          display_name: string;
          description: string | null;
          category: string;
          icon_name: string;
          default_config: Json;
          default_data_source_config: Json;
          allowed_data_sources: Json | null;
          is_active: boolean;
          sort_order: number;
          created_at: string;
          updated_at: string;
        };
        Insert: {
          id?: string;
          chart_type_id: string;
          display_name: string;
          // ... insert fields
        };
        Update: {
          id?: string;
          chart_type_id?: string;
          // ... update fields
        };
      };
      // ... other tables
    };
  };
}
```

#### Benefits

- ✅ **Automatic sync**: Types match database schema
- ✅ **Migration safety**: Run after every migration
- ✅ **Type inference**: Supabase client uses these types
- ✅ **No manual maintenance**: Generated from live database

#### Limitations

- ❌ **JSONB is `Json` type**: No structure validation
- ❌ **No business logic**: Just raw database types
- ❌ **No computed properties**: Only database columns

### Phase 2: Manual Type Extensions

**Goal**: Add business logic types and structured JSONB types.

#### Structure

```typescript
// src/lib/database-extensions.ts (MANUAL - EDITABLE)
import type { Database } from './database.types'

// Extend base types with business logic
export type ChartType = Database['public']['Tables']['chart_types']['Row']['chart_type_id']

// Structured JSONB types
export interface AllowedDataSource {
  type: 'curve' | 'well' | 'surface' | 'well_marker' | 'polygon' | 'seismic'
  description: string
  required: boolean
  multiple: boolean
  constraints?: DataSourceConstraints
  ui_label: string
  ui_icon: string
}

export interface ChartConfig {
  tracks?: WellCorrelationTrack[]
  wells?: string[]
  depthRange?: { min: number | null; max: number | null }
  synchronized?: boolean
  // ... chart-specific config
}

// Type-safe chart type
export interface ChartTypeRow extends Database['public']['Tables']['chart_types']['Row'] {
  allowed_data_sources: AllowedDataSource[] | null
  default_config: ChartConfig
}
```

#### Benefits

- ✅ **Type safety**: Structured JSONB types
- ✅ **Business logic**: Add computed properties
- ✅ **Extensibility**: Can add methods and helpers
- ✅ **Backward compatible**: Extends generated types

### Phase 3: Runtime Validation (Zod)

**Goal**: Validate JSONB structures at runtime.

#### Setup

```bash
npm install zod
npm install -D ts-to-zod  # Optional: Generate Zod from TypeScript
```

#### Zod Schemas

```typescript
// src/lib/database-schemas.ts
import { z } from "zod";
import type { AllowedDataSource, ChartConfig } from "./database-extensions";

// Zod schema for AllowedDataSource
export const AllowedDataSourceSchema = z.object({
  type: z.enum([
    "curve",
    "well",
    "surface",
    "well_marker",
    "polygon",
    "seismic",
  ]),
  description: z.string(),
  required: z.boolean(),
  multiple: z.boolean(),
  constraints: z
    .object({
      curve_types: z.union([z.literal("all"), z.array(z.string())]).optional(),
      domain: z.enum(["depth", "time", "all"]).optional(),
    })
    .optional(),
  ui_label: z.string(),
  ui_icon: z.string(),
});

// Zod schema for ChartConfig
export const ChartConfigSchema = z.object({
  tracks: z
    .array(
      z.object({
        id: z.string(),
        title: z.string(),
        curves: z.array(z.string()),
        xAxis: z.object({
          type: z.string(),
          title: z.string(),
          axisTitle: z.string(),
        }),
        yAxis: z.object({
          type: z.string(),
          title: z.string(),
          axisTitle: z.string(),
          isInverted: z.boolean(),
        }),
      }),
    )
    .optional(),
  wells: z.array(z.string()).optional(),
  depthRange: z
    .object({
      min: z.number().nullable(),
      max: z.number().nullable(),
    })
    .optional(),
  synchronized: z.boolean().optional(),
});

// Type-safe parser
export function parseAllowedDataSources(json: unknown): AllowedDataSource[] {
  return z.array(AllowedDataSourceSchema).parse(json);
}

export function parseChartConfig(json: unknown): ChartConfig {
  return ChartConfigSchema.parse(json);
}
```

#### Usage

```typescript
// In your service/component
import { parseAllowedDataSources } from "$lib/database-schemas";
import type { Database } from "$lib/database.types";

async function getChartType(chartTypeId: string) {
  const { data, error } = await supabase
    .from("chart_types")
    .select("*")
    .eq("chart_type_id", chartTypeId)
    .single();

  if (error) throw error;

  // Validate JSONB at runtime
  const allowedDataSources = data.allowed_data_sources
    ? parseAllowedDataSources(data.allowed_data_sources)
    : null;

  return {
    ...data,
    allowed_data_sources: allowedDataSources,
  };
}
```

#### Benefits

- ✅ **Runtime safety**: Catch invalid JSONB at runtime
- ✅ **Type narrowing**: Zod infers types from schemas
- ✅ **Error messages**: Clear validation errors
- ✅ **API safety**: Validate data from database

### Phase 4: Query-Specific Types (PgTyped - Optional)

**Goal**: Generate types for complex SQL queries.

#### When to Use PgTyped

- Complex joins across multiple tables
- Aggregations and computed columns
- Views and materialized views
- Stored procedures

#### Setup

```bash
npm install -D @pgtyped/cli @pgtyped/query
```

#### Example

```sql
-- queries/get-chart-types-with-counts.sql
/* @name getChartTypesWithCounts */
SELECT
  ct.*,
  COUNT(c.id) as chart_count
FROM chart_types ct
LEFT JOIN charts c ON c.chart_type = ct.chart_type_id
GROUP BY ct.id;
```

```typescript
// Generated by PgTyped
export interface IGetChartTypesWithCountsResult {
  id: string;
  chart_type_id: string;
  display_name: string;
  chart_count: number;
  // ... other fields
}
```

#### Benefits

- ✅ **Query-specific types**: Types match exact query results
- ✅ **Complex queries**: Handles joins, aggregations, etc.
- ✅ **Type safety**: Compile-time checking of query results

#### When NOT to Use

- Simple CRUD operations (use Supabase types)
- Dynamic queries (use Supabase query builder)
- JSONB-heavy queries (use Zod validation)

## Recommended Workflow

### 1. After Database Migrations

```bash
# 1. Run migration (via Supabase Studio SQL Editor or psql)
# Or via SSH tunnel:
ssh -i ~/.ssh/id_rsa_mudrock root@91.99.166.223 \
  "docker exec mudrock-db psql -U postgres -d postgres -f /path/to/migration.sql"

# 2. Regenerate types (choose one method):

# Method A: Direct connection (if database is publicly accessible)
supabase gen types typescript \
  --db-url "postgresql://postgres:MudRockSecure2024@91.99.166.223:5432/postgres" \
  --schema public \
  > src/lib/database.types.ts

# Method B: Via SSH tunnel (if database requires VPN/tunnel)
# First, set up tunnel in separate terminal:
# ssh -i ~/.ssh/id_rsa_mudrock -L 5433:localhost:5432 root@91.99.166.223
# Then:
supabase gen types typescript \
  --db-url "postgresql://postgres:MudRockSecure2024@localhost:5433/postgres" \
  --schema public \
  > src/lib/database.types.ts

# Method C: Custom script (if Supabase CLI has Docker issues)
bun run scripts/database/generate-db-types.ts > src/lib/database.types.ts

# 3. Update extensions if needed
# Edit src/lib/database-extensions.ts

# 4. Update Zod schemas if JSONB changed
# Edit src/lib/database-schemas.ts

# 5. Run type check
npm run type-check
```

### 2. CI/CD Integration

```yaml
# .github/workflows/type-check.yml
name: Type Check

on: [push, pull_request]

jobs:
  type-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm install

      # Generate types from database (self-hosted)
      # Note: Requires SSH tunnel or VPN access to database
      - name: Set up SSH tunnel
        run: |
          mkdir -p ~/.ssh
          echo "${{ secrets.SSH_PRIVATE_KEY }}" > ~/.ssh/id_rsa
          chmod 600 ~/.ssh/id_rsa
          ssh-keyscan -H 91.99.166.223 >> ~/.ssh/known_hosts
          ssh -f -N -L 5433:localhost:5432 -i ~/.ssh/id_rsa root@91.99.166.223

      - name: Generate types
        run: |
          supabase gen types typescript \
            --db-url "postgresql://postgres:${{ secrets.DB_PASSWORD }}@localhost:5433/postgres" \
            --schema public \
            > src/lib/database.types.ts

      # Type check
      - run: npm run type-check

      # Verify types are up to date
      - run: git diff --exit-code src/lib/database.types.ts || (echo "Types are out of date. Run: npx supabase gen types typescript" && exit 1)
```

### 3. Pre-commit Hook

```bash
# .husky/pre-commit
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

# Regenerate types
npx supabase gen types typescript --project-id <project-id> > src/lib/database.types.ts

# Add to commit if changed
git add src/lib/database.types.ts
```

## Handling Dynamic Configurations

### Chart Types

**Current Approach**: Manual TypeScript types + database JSONB

**Recommended Approach**:

```typescript
// src/lib/components/pages/home/charts/types/chart-data-sources.ts
import type { Database } from "$lib/database.types";
import { z } from "zod";

// Generated from database
type ChartTypeRow = Database["public"]["Tables"]["chart_types"]["Row"];

// Manual extension for structured JSONB
export interface AllowedDataSource {
  // ... as defined above
}

// Runtime validation
export const AllowedDataSourceSchema = z.object({
  // ... as defined above
});

// Type-safe getter
export async function getChartTypeConfig(chartTypeId: string) {
  const { data } = await supabase
    .from("chart_types")
    .select("*")
    .eq("chart_type_id", chartTypeId)
    .single();

  if (!data) throw new Error(`Chart type ${chartTypeId} not found`);

  // Validate JSONB
  const allowedDataSources = data.allowed_data_sources
    ? z.array(AllowedDataSourceSchema).parse(data.allowed_data_sources)
    : null;

  return {
    ...data,
    allowed_data_sources: allowedDataSources,
  };
}
```

### UDF Mappings

**Current Approach**: TypeScript defaults + database overrides

**Recommended Approach**: Keep current approach, but add Zod validation:

```typescript
// src/lib/services/udf-curve-metadata-mapping-service.ts
import { z } from "zod";
import type { Database } from "$lib/database.types";

const UdfMappingSchema = z.object({
  udf_name: z.string(),
  input_curve_metadata_types: z.array(z.string()),
  output_curve_metadata_type: z.string(),
  description: z.string().optional(),
});

export async function getUdfCurveMetadataMapping(udfName: string) {
  // Try database first
  const { data } = await supabase
    .from("udf_curve_metadata_mappings")
    .select("*")
    .eq("udf_name", udfName)
    .single();

  if (data) {
    // Validate with Zod
    return UdfMappingSchema.parse(data);
  }

  // Fallback to TypeScript defaults
  return getDefaultUdfMapping(udfName);
}
```

## Migration Strategy

### Step 1: Set Up Type Generation (Week 1)

1. Install Supabase CLI
2. Generate initial `database.types.ts`
3. Update imports to use generated types
4. Set up CI/CD type checking

### Step 2: Add Zod Validation (Week 2)

1. Install Zod
2. Create schemas for JSONB structures
3. Add validation to service functions
4. Update error handling

### Step 3: Refactor Existing Types (Week 3-4)

1. Replace manual types with generated + extensions
2. Add Zod validation where needed
3. Update tests
4. Document new patterns

### Step 4: Optional PgTyped (Week 5)

1. Identify complex queries
2. Set up PgTyped for those queries
3. Generate query-specific types
4. Update query code

## Comparison: Current vs. Recommended

| Aspect                 | Current      | Recommended               |
| ---------------------- | ------------ | ------------------------- |
| **Base Types**         | Manual       | Generated (Supabase CLI)  |
| **JSONB Types**        | Manual       | Manual extensions + Zod   |
| **Runtime Validation** | None         | Zod schemas               |
| **Migration Safety**   | Manual       | Automated (CI/CD)         |
| **Type Drift**         | High risk    | Low risk (auto-generated) |
| **Complex Queries**    | Manual types | PgTyped (optional)        |
| **Maintenance**        | High         | Low (mostly automated)    |

## Decision Matrix

### Use Supabase Type Generation When:

- ✅ You have standard tables with simple types
- ✅ You want automatic sync with database
- ✅ You're using Supabase client
- ✅ You want minimal setup

### Use Manual Extensions When:

- ✅ You have complex JSONB structures
- ✅ You need business logic types
- ✅ You need computed properties
- ✅ You need type narrowing

### Use Zod Validation When:

- ✅ You have JSONB columns
- ✅ You receive data from external sources
- ✅ You need runtime validation
- ✅ You want clear error messages

### Use PgTyped When:

- ✅ You have complex SQL queries
- ✅ You need query-specific types
- ✅ You have views/stored procedures
- ❌ NOT for simple CRUD (use Supabase)

## Conclusion

**Recommended Approach**: **Hybrid System**

1. **Supabase CLI** for base table types (automatic, always in sync)
2. **Manual extensions** for JSONB and business logic (structured, type-safe)
3. **Zod schemas** for runtime validation (safe, clear errors)
4. **PgTyped** (optional) for complex queries (when needed)

This approach provides:

- ✅ **Automatic sync** for base types
- ✅ **Type safety** for JSONB structures
- ✅ **Runtime validation** for data integrity
- ✅ **Flexibility** for business logic
- ✅ **Low maintenance** (mostly automated)

The current dual-source approach for UDF mappings is actually good and should be kept, but enhanced with Zod validation for the database portion.
