# Kanel vs Custom Script: Decision Summary

## ✅ Recommendation: Use Kanel

After testing both approaches, **Kanel is the better choice** for the following reasons:

### Issues Fixed

1. **Config File Format**: Fixed `.kanelrc.js` → `.kanelrc.cjs` for ES module compatibility
2. **Type Mappings**: Added proper mappings for `jsonb`, `timestamp with time zone`, `geometry`
3. **Schema Filtering**: Configured to only generate types for `public` schema
4. **Foreign Keys**: Set `ignoreForeignKeys: true` to skip auth schema references

### Generated Output

Kanel successfully generated types in `src/lib/database-types/public/`:

```
src/lib/database-types/public/
├── Charts.ts              # Chart table types
├── ChartTypes.ts          # ChartType table types
├── Curves.ts              # Curve table types
├── Nodes.ts               # Node table types
└── ... (one file per table)
```

### Format Comparison

**Kanel Format:**
```typescript
// Charts.ts
export default interface Charts {
  id: string;
  project_id: string;
  name: string;
  chart_type: string;
  chart_config: unknown; // JSONB
  // ...
}

export interface ChartsInitializer { /* Insert type */ }
export interface ChartsMutator { /* Update type */ }
```

**Supabase Format (Current):**
```typescript
// database.types.ts
export interface Database {
  public: {
    Tables: {
      charts: {
        Row: { /* ... */ };
        Insert: { /* ... */ };
        Update: { /* ... */ };
      };
    };
  };
}
```

### Migration Strategy

**Option 1: Keep Both** (Recommended for gradual migration)
- Use Kanel types for new code
- Keep `database.types.ts` for Supabase client compatibility
- Gradually migrate imports

**Option 2: Create Supabase Wrapper**
- Generate Kanel types
- Create wrapper that converts Kanel format to Supabase format
- Single source of truth (Kanel)

**Option 3: Full Migration**
- Migrate all code to use Kanel types directly
- Remove `database.types.ts`
- Update all imports

### Next Steps

1. ✅ Kanel installed and configured
2. ✅ Types generated successfully
3. ⏳ Decide on migration strategy
4. ⏳ Update imports in codebase (if migrating)
5. ⏳ Create Supabase wrapper (if needed)

### Data Export Script

The separate `scripts/database/export-db-data.ts` script exports table data to JSON files for debugging:

```bash
bun run scripts/database/export-db-data.ts
```

Output: `scripts/db-exports/*.json`

