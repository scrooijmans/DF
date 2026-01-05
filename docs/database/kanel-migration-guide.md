# Migrating from Custom Script to Kanel

## Recommendation: Use Kanel

**Yes, I recommend migrating to Kanel** for the following reasons:

### ✅ Advantages

1. **Better Type Accuracy**
   - Properly handles array types (`integer[]`, `text[]`, etc.) vs simplified `string[]`
   - Detects and generates enum types automatically
   - Handles custom PostgreSQL types correctly

2. **Better Organization**
   - Separate files per table (easier to navigate)
   - Better for large codebases
   - Reduces merge conflicts

3. **More Maintainable**
   - Actively maintained library
   - Handles edge cases automatically
   - Regular updates and bug fixes

4. **Extensible**
   - Plugin system for customizations
   - Can generate additional types (Zod schemas, etc.)
   - Customizable output format

### Comparison

| Feature | Current Custom Script | Kanel |
|---------|---------------------|-------|
| Array Types | Simplified (`string[]`) | Accurate (`integer[]`, `text[]`) |
| Enums | Not detected | Properly generated |
| File Structure | Single file | Separate files per table |
| Maintenance | Manual updates | Library maintained |
| Type Accuracy | Basic | Advanced |

## Migration Steps

### Step 1: Install Kanel

```bash
bun add -d kanel
```

### Step 2: Run Kanel

```bash
npx kanel
```

Or use the wrapper script:

```bash
bun run scripts/database/generate-db-types-with-kanel.ts
```

### Step 3: Update Imports

Kanel generates separate files per table:

```typescript
// Before (single file)
import type { Database } from "$lib/database.types";
type Chart = Database["public"]["Tables"]["charts"]["Row"];

// After (separate files)
import type { Chart } from "$lib/database-types/Chart";
```

### Step 4: Create Supabase Wrapper (Optional)

If you need Supabase-compatible format, create a wrapper:

```typescript
// src/lib/database.types.ts (Supabase-compatible wrapper)
import type { Chart as KanelChart } from "./database-types/Chart";
import type { ChartType as KanelChartType } from "./database-types/ChartType";
// ... import all tables

export interface Database {
  public: {
    Tables: {
      charts: {
        Row: KanelChart;
        Insert: Omit<KanelChart, "id" | "created_at" | "updated_at">;
        Update: Partial<Omit<KanelChart, "id">>;
      };
      chart_types: {
        Row: KanelChartType;
        Insert: Omit<KanelChartType, "id" | "created_at" | "updated_at">;
        Update: Partial<Omit<KanelChartType, "id">>;
      };
      // ... other tables
    };
  };
}
```

## Data Export Script

For debugging and inspection, use the separate data export script:

```bash
bun run scripts/database/export-db-data.ts
```

This exports each table to a separate JSON file:

```
scripts/db-exports/
├── charts.json
├── chart_types.json
├── curves.json
└── ...
```

## Decision: Keep Both Approaches?

**Option 1: Full Migration to Kanel** (Recommended)
- Use Kanel for all type generation
- Create Supabase wrapper if needed
- Deprecate custom script

**Option 2: Hybrid Approach**
- Use Kanel for new tables
- Keep custom script for Supabase compatibility
- Gradually migrate

**Recommendation: Option 1** - Full migration provides better long-term maintainability.

