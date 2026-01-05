# Kysely Integration Assessment

## Executive Summary

**Recommendation**: **Gradually adopt Kysely** for type-safe database queries while keeping Supabase client for realtime subscriptions and simple queries. Use Kanel-generated types (`database.types.ts`) for all database operations.

## Current State Analysis

### Current Patterns

1. **Manual Type Definitions**: Types like `Well`, `Curve`, `Chart`, `PipelineRow` are manually defined instead of using generated types
2. **Untyped Data Fetching**: `getTableData()` returns `any[]` requiring manual mapping
3. **Type Assertions**: Supabase queries use manual type assertions (`as PipelineRow`)
4. **Mixed Approaches**: Some queries go through Tauri commands, others use Supabase client directly

### Files Analyzed

- `src/lib/state/postgres/postgres-wells-state.svelte.ts` - Uses `getTableData("wells")` with manual mapping
- `src/lib/state/postgres/postgres-curve-state.svelte.ts` - Uses `getTableData("curves")` with manual mapping
- `src/lib/services/pipeline-service.ts` - Uses Supabase client with manual type assertions
- `src/lib/services/chart-service.ts` - Uses `getTableData("chart_types")` with manual mapping
- `src/lib/tauri-commands/table-data-fetching.ts` - Returns `any[]` from Tauri commands

## Kysely Benefits

### ✅ Advantages

1. **Compile-Time Type Safety**
   - Table and column names checked at compile time
   - Prevents typos in queries
   - Better IDE autocomplete

2. **Integration with Kanel Types**
   - Works seamlessly with `database.types.ts`
   - Automatic type inference from schema
   - No manual type definitions needed

3. **Complex Query Support**
   - Type-safe joins, aggregations, subqueries
   - Better than Supabase client for complex operations
   - SQL-like syntax with type safety

4. **Better Developer Experience**
   - IntelliSense for all query operations
   - Compile-time error detection
   - Self-documenting queries

### ⚠️ Considerations

1. **Migration Effort**
   - Existing code uses Supabase client
   - Need to refactor multiple files
   - Gradual migration recommended

2. **Realtime Subscriptions**
   - Supabase client required for realtime
   - Keep Supabase client for subscriptions
   - Use Kysely for queries, Supabase for realtime

3. **Tauri Commands**
   - Some queries go through Rust backend
   - Kysely is frontend-only
   - Keep Tauri commands for backend operations

## Recommended Approach

### Phase 1: Use Generated Types (Immediate)

**Action**: Replace manual type definitions with Kanel-generated types

```typescript
// ❌ BEFORE: Manual type definition
export interface Well {
  id: number;
  name: string;
  project_id: string | null;
  // ... manually listing all fields
}

// ✅ AFTER: Use generated type
import type { Database } from "$lib/database.types";
type Well = Database["public"]["Tables"]["wells"]["Row"];
```

**Files to Update**:
- `src/lib/state/postgres/postgres-wells-state.svelte.ts`
- `src/lib/state/postgres/postgres-curve-state.svelte.ts`
- `src/lib/services/pipeline-service.ts`
- `src/lib/services/chart-service.ts`

### Phase 2: Add Kysely for New Code (Short-term)

**Action**: Use Kysely for all new database queries

**Setup**:
```bash
bun add kysely
bun add -d kanel-kysely
```

**Create Kysely Client**:
```typescript
// src/lib/database/kysely-client.ts
import { Kysely } from "kysely";
import { PostgresDialect } from "kysely";
import { Pool } from "pg";
import type { Database } from "$lib/database.types";

export const db = new Kysely<Database>({
  dialect: new PostgresDialect({
    pool: new Pool({
      connectionString: process.env.DATABASE_URL,
    }),
  }),
});
```

**Use in New Code**:
```typescript
// ✅ NEW CODE: Use Kysely
import { db } from "$lib/database/kysely-client";

async function getChartsForProject(projectId: string) {
  return await db
    .selectFrom("charts")
    .selectAll()
    .where("project_id", "=", projectId)
    .where("is_active", "=", true)
    .orderBy("updated_at", "desc")
    .execute();
  // Fully typed, no assertions needed!
}
```

### Phase 3: Migrate Existing Code (Long-term)

**Action**: Gradually migrate existing queries to Kysely

**Migration Priority**:
1. **High Priority**: Complex queries (joins, aggregations)
2. **Medium Priority**: Simple queries in service files
3. **Low Priority**: State management files (can keep Supabase for now)

**Keep Supabase Client For**:
- ✅ Realtime subscriptions (`realtime-charts-service.ts`, etc.)
- ✅ Simple queries that work well with Supabase
- ✅ Existing code (migrate gradually)

## Implementation Examples

### Example 1: Wells Fetching

**Current (Manual Types + Tauri)**:
```typescript
// ❌ CURRENT: Manual types, untyped data
const data = await getTableData("wells");
const allWells: Well[] = data.map((item: any) => ({
  id: item.id,
  name: item.name,
  // ... manual mapping
}));
```

**Improved (Generated Types + Kysely)**:
```typescript
// ✅ IMPROVED: Generated types, type-safe query
import type { Database } from "$lib/database.types";
import { db } from "$lib/database/kysely-client";

type WellRow = Database["public"]["Tables"]["wells"]["Row"];

async function loadWells(projectId: string): Promise<WellRow[]> {
  return await db
    .selectFrom("wells")
    .selectAll()
    .where("project_id", "=", projectId)
    .execute();
}
```

### Example 2: Pipeline Queries

**Current (Supabase + Manual Types)**:
```typescript
// ❌ CURRENT: Manual type assertion
export type PipelineRow = {
  id: string;
  name: string;
  // ... manual definition
};

const { data } = await supabase
  .from("pipelines")
  .select("*")
  .eq("id", id)
  .single();
return data as PipelineRow;
```

**Improved (Generated Types + Kysely)**:
```typescript
// ✅ IMPROVED: Generated types, type-safe query
import type { Database } from "$lib/database.types";
import { db } from "$lib/database/kysely-client";

type PipelineRow = Database["public"]["Tables"]["pipelines"]["Row"];

async function getPipelineById(id: string): Promise<PipelineRow | undefined> {
  return await db
    .selectFrom("pipelines")
    .selectAll()
    .where("id", "=", id)
    .executeTakeFirst();
}
```

### Example 3: Complex Query with Joins

**Current (Supabase)**:
```typescript
// ❌ CURRENT: Manual type assertions, no type safety for joins
const { data } = await supabase
  .from("charts")
  .select("*, projects(name)")
  .eq("project_id", projectId);
return data as any[];
```

**Improved (Kysely)**:
```typescript
// ✅ IMPROVED: Type-safe joins
const chartsWithProjects = await db
  .selectFrom("charts")
  .innerJoin("projects", "projects.id", "charts.project_id")
  .select([
    "charts.id",
    "charts.name",
    "charts.chart_type",
    "projects.name as project_name",
  ])
  .where("charts.project_id", "=", projectId)
  .where("charts.is_active", "=", true)
  .execute();
// Fully typed with autocomplete!
```

## Migration Checklist

### Immediate (Use Generated Types)

- [ ] Replace `Well` interface with `Database["public"]["Tables"]["wells"]["Row"]`
- [ ] Replace `Curve` interface with `Database["public"]["Tables"]["curves"]["Row"]`
- [ ] Replace `Chart` interface with `Database["public"]["Tables"]["charts"]["Row"]`
- [ ] Replace `PipelineRow` with `Database["public"]["Tables"]["pipelines"]["Row"]`
- [ ] Update all type assertions to use generated types

### Short-term (Add Kysely)

- [ ] Install Kysely: `bun add kysely`
- [ ] Create `src/lib/database/kysely-client.ts`
- [ ] Use Kysely for all new database queries
- [ ] Update TypeScript rules to recommend Kysely

### Long-term (Migrate Existing Code)

- [ ] Migrate complex queries to Kysely
- [ ] Migrate service files to Kysely
- [ ] Keep Supabase client for realtime subscriptions
- [ ] Document migration patterns

## Conclusion

**Kysely provides significant benefits** for type-safe database queries, especially for complex operations. However, **Supabase client should be retained** for realtime subscriptions and simple queries.

**Recommended Strategy**:
1. ✅ **Immediate**: Use Kanel-generated types (`database.types.ts`) everywhere
2. ✅ **Short-term**: Use Kysely for all new code
3. ✅ **Long-term**: Gradually migrate existing code to Kysely
4. ✅ **Always**: Keep Supabase client for realtime subscriptions

This approach provides **immediate type safety improvements** while allowing **gradual migration** without disrupting existing functionality.

