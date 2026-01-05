# Testing Kanel Types

## Quick Test

Run the test script to verify Kanel types are working:

```bash
bun run scripts/database/test-kanel-types.ts
```

This will verify:
- ✅ Type imports work
- ✅ Type checking works
- ✅ JSONB types handled correctly
- ✅ Array types work
- ✅ Nullable types work
- ✅ Initializer types (Insert) work
- ✅ Mutator types (Update) work

## Compare with Existing Types

Compare Kanel types with existing Supabase types:

```bash
bun run scripts/database/compare-types.ts
```

This shows:
- Key differences between formats
- Type compatibility
- Known differences (Date vs string for timestamps)

## Manual Testing

### 1. Import and Use Types

```typescript
// Import Kanel types
import type Charts from "$lib/database-types/public/Charts";
import type { ChartsInitializer, ChartsMutator } from "$lib/database-types/public/Charts";

// Use in your code
const chart: Charts = {
  id: "chart-id",
  project_id: "project-id",
  name: "My Chart",
  chart_type: "line",
  // ... other required fields
};

// For inserts
const newChart: ChartsInitializer = {
  project_id: "project-id",
  name: "New Chart",
  chart_type: "line",
  created_by: "user-id",
  // id, created_at, updated_at are optional (have defaults)
};

// For updates
const chartUpdate: ChartsMutator = {
  name: "Updated Name",
  // All fields optional
};
```

### 2. Check TypeScript Compilation

```bash
bun run check
```

This will catch any type errors if Kanel types don't match your usage.

### 3. Verify in IDE

Open any file that uses database types and check:
- ✅ IntelliSense works
- ✅ Type hints show correct properties
- ✅ Errors highlight incorrect usage

## Expected Results

### ✅ Success Indicators

- Test script runs without errors
- TypeScript compilation succeeds
- IDE shows correct type hints
- No runtime type errors

### ⚠️ Known Differences

1. **Timestamp Types**
   - Kanel: `Date` objects
   - Supabase: `string` (ISO format)
   - **Solution**: Convert when needed: `date.toISOString()` or `new Date(string)`

2. **JSONB Types**
   - Kanel: `unknown` (needs type assertion)
   - Supabase: `Json` type (same)
   - **Solution**: Use type assertions or create typed interfaces

3. **Format**
   - Kanel: Separate interfaces (`Charts`, `ChartsInitializer`, `ChartsMutator`)
   - Supabase: Nested structure (`Database['public']['Tables']['charts']['Row']`)
   - **Solution**: Use Kanel types directly or create wrapper

## Next Steps

1. ✅ Run test script: `bun run scripts/database/test-kanel-types.ts`
2. ✅ Run comparison: `bun run scripts/database/compare-types.ts`
3. ⏳ Update imports in one component to use Kanel types
4. ⏳ Verify everything works
5. ⏳ Gradually migrate remaining code

## Troubleshooting

### Types Not Found

```bash
# Regenerate types
npx kanel
```

### Type Errors

Check if types match database schema:
```bash
# Compare with database
bun run scripts/database/compare-types.ts
```

### Import Errors

Make sure path is correct:
```typescript
// ✅ Correct
import type Charts from "$lib/database-types/public/Charts";

// ❌ Wrong
import type Charts from "$lib/database-types/Charts";
```

