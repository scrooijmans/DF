# Kysely Usage Guide

## Overview

Kysely is a type-safe SQL query builder for TypeScript that integrates seamlessly with our Kanel-generated database types. It provides compile-time type checking, autocompletion, and prevents SQL injection attacks.

## Setup

### Installation

Kysely is already installed in the project:

```bash
bun add kysely
```

### Client Setup

The Kysely client is configured in `src/lib/database/kysely-client.ts`:

```typescript
import { db } from '$lib/database/kysely-client';

// Use db for queries
const charts = await db
  .selectFrom('charts')
  .selectAll()
  .where('project_id', '=', projectId)
  .execute();
```

## Type System

### Using Generated Types

Kysely uses our Kanel-generated types from `database.types.ts`. Helper types are provided for convenience:

```typescript
import { type RowType, type InsertType, type UpdateType } from '$lib/database/kysely-client';

// Get Row type for a table
type ChartRow = RowType<'charts'>;

// Get Insert type for a table
type ChartInsert = InsertType<'charts'>;

// Get Update type for a table
type ChartUpdate = UpdateType<'charts'>;
```

## Common Patterns

### 1. Simple SELECT Query

```typescript
import { db, type RowType } from '$lib/database/kysely-client';

type ChartRow = RowType<'charts'>;

async function getChartById(chartId: string): Promise<ChartRow | undefined> {
  return await db
    .selectFrom('charts')
    .selectAll()
    .where('id', '=', chartId)
    .where('is_active', '=', true)
    .executeTakeFirst();
}
```

### 2. SELECT with Filtering and Ordering

```typescript
async function getChartsForProject(projectId: string): Promise<ChartRow[]> {
  return await db
    .selectFrom('charts')
    .selectAll()
    .where('project_id', '=', projectId)
    .where('is_active', '=', true)
    .orderBy('updated_at', 'desc')
    .execute();
}
```

### 3. SELECT with JOIN

```typescript
async function getChartsWithProjects(projectId: string) {
  return await db
    .selectFrom('charts')
    .innerJoin('projects', 'projects.id', 'charts.project_id')
    .select([
      'charts.id as chart_id',
      'charts.name as chart_name',
      'charts.chart_type',
      'projects.name as project_name',
    ])
    .where('charts.project_id', '=', projectId)
    .where('charts.is_active', '=', true)
    .execute();
}
```

### 4. INSERT Query

```typescript
import { db, type InsertType } from '$lib/database/kysely-client';

type ChartInsert = InsertType<'charts'>;

async function createChart(data: ChartInsert): Promise<ChartRow> {
  const result = await db
    .insertInto('charts')
    .values(data)
    .returningAll()
    .executeTakeFirstOrThrow();

  return result;
}
```

### 5. UPDATE Query

```typescript
import { db, type UpdateType } from '$lib/database/kysely-client';

type ChartUpdate = UpdateType<'charts'>;

async function updateChart(
  chartId: string,
  updates: ChartUpdate,
): Promise<ChartRow> {
  const result = await db
    .updateTable('charts')
    .set({
      ...updates,
      updated_at: new Date().toISOString(),
    })
    .where('id', '=', chartId)
    .returningAll()
    .executeTakeFirstOrThrow();

  return result;
}
```

### 6. Soft DELETE

```typescript
async function deleteChart(chartId: string): Promise<void> {
  await db
    .updateTable('charts')
    .set({ is_active: false })
    .where('id', '=', chartId)
    .execute();
}
```

### 7. Transactions

```typescript
import { transaction } from '$lib/database/kysely-client';

async function createChartWithDefaults(
  projectId: string,
  name: string,
): Promise<ChartRow> {
  return await transaction(async (trx) => {
    // First query
    const chartType = await trx
      .selectFrom('chart_types')
      .selectAll()
      .where('chart_type_id', '=', 'line')
      .executeTakeFirstOrThrow();

    // Second query (uses same transaction)
    const chart = await trx
      .insertInto('charts')
      .values({
        project_id: projectId,
        name,
        chart_type: chartType.chart_type_id,
        chart_config: chartType.default_config || {},
        data_source_config: chartType.default_data_source_config || {},
        created_by: null,
      })
      .returningAll()
      .executeTakeFirstOrThrow();

    return chart;
  });
}
```

### 8. Complex Queries with Aggregations

```typescript
async function getWellsWithCurveCount(projectId: string) {
  return await db
    .selectFrom('wells')
    .leftJoin('curves', 'curves.well_id', 'wells.id')
    .select([
      'wells.id as well_id',
      'wells.name as well_name',
    ])
    .select((eb) => eb.fn.count('curves.id').as('curve_count'))
    .where('wells.project_id', '=', projectId)
    .groupBy(['wells.id', 'wells.name'])
    .execute();
}
```

## Migration from Supabase Client

### Before (Supabase Client)

```typescript
import { supabase } from '$lib/supabase';

async function getChart(id: string) {
  const { data, error } = await supabase
    .from('charts')
    .select('*')
    .eq('id', id)
    .single();

  if (error) throw new Error(error.message);
  return data as ChartRow; // Manual type assertion
}
```

### After (Kysely)

```typescript
import { db, type RowType } from '$lib/database/kysely-client';

type ChartRow = RowType<'charts'>;

async function getChart(id: string): Promise<ChartRow | undefined> {
  return await db
    .selectFrom('charts')
    .selectAll()
    .where('id', '=', id)
    .executeTakeFirst();
  // Fully typed, no assertions needed!
}
```

## When to Use Kysely vs Supabase Client

### ✅ Use Kysely For:

- Complex queries (joins, aggregations, subqueries)
- New code and features
- Type-safe queries with compile-time checking
- Transactions
- Better developer experience with autocomplete

### ✅ Keep Supabase Client For:

- Realtime subscriptions (`realtime-charts-service.ts`, etc.)
- Simple queries that work well with Supabase
- Existing code (migrate gradually)
- When Supabase-specific features are needed

## Examples

See `src/lib/database/kysely-examples.ts` for comprehensive examples of all query patterns.

## Type Safety Benefits

1. **Compile-Time Checking**: Table and column names are checked at compile time
2. **Autocomplete**: Full IntelliSense support for all queries
3. **Type Inference**: Return types are automatically inferred
4. **Prevents Errors**: Typos in table/column names are caught before runtime

## Best Practices

1. **Always use type helpers**: Use `RowType`, `InsertType`, `UpdateType` for consistency
2. **Prefer Kysely for new code**: Use Kysely for all new database queries
3. **Keep Supabase for realtime**: Use Supabase client for realtime subscriptions
4. **Use transactions**: Wrap multi-step operations in transactions
5. **Handle errors**: Use `neverthrow` for error handling with Kysely queries

## Troubleshooting

### Connection Issues

If you encounter connection errors, check:
1. Environment variables (`DB_HOST`, `DB_PORT`, `DB_USER`, `POSTGRES_PASSWORD`)
2. Database is accessible from your network
3. Connection pool settings in `kysely-client.ts`

### Type Errors

If you see type errors:
1. Regenerate types: `bun run scripts/database/generate-db-types-with-kanel.ts`
2. Check that table names match exactly (case-sensitive)
3. Verify column names match database schema

## Resources

- [Kysely Documentation](https://kysely.dev)
- [Kysely API Reference](https://kysely-org.github.io/kysely-apidoc/)
- [Kanel Documentation](https://kristiandupont.github.io/kanel)

