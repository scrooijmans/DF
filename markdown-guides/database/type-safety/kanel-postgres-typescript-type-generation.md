# Database Type Generation Strategy

## Overview

We use **Kanel** to generate TypeScript types from our PostgreSQL database schema. This ensures type safety and keeps our TypeScript codebase synchronized with the database.

## Why Kanel?

### ✅ Advantages Over Custom Script

1. **Better Type Accuracy**
   - Properly handles array types (`integer[]`, `text[]`, etc.)
   - Detects and generates enum types
   - Handles custom PostgreSQL types correctly

2. **Separate Files Per Table**
   - Easier to navigate with many tables
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

| Feature        | Custom Script           | Kanel                                  |
| -------------- | ----------------------- | -------------------------------------- |
| Array Types    | Simplified (`string[]`) | Accurate (`integer[]`, `text[]`, etc.) |
| Enums          | Not detected            | Properly generated                     |
| File Structure | Single file             | Separate files per table               |
| Maintenance    | Manual updates          | Library maintained                     |
| Type Accuracy  | Basic                   | Advanced                               |

## Setup

### 1. Install Kanel

```bash
bun add -d kanel
```

### 2. Configure Kanel

Create `.kanelrc.cjs` in project root (`.cjs` extension required for ES module projects):

```javascript
// .kanelrc.cjs (must use .cjs extension for ES module projects)
const path = require("path");

module.exports = {
  connection: {
    host: process.env.DB_HOST || "localhost",
    port: parseInt(process.env.DB_PORT || "5432"),
    user: process.env.DB_USER || "postgres",
    password: process.env.POSTGRES_PASSWORD,
    database: process.env.DB_NAME || "postgres",
  },
  outputPath: path.join(__dirname, "src", "lib", "database-types"),
  preDeleteOutputFolder: true,
  schemas: ["public"],
  excludePattern: /^(realtime|pg_catalog|information_schema|auth)/,
  typeMap: {
    jsonb: "Json",
    json: "Json",
    "timestamp with time zone": "string",
    geometry: "string",
  },
  includeEnums: true,
  ignoreForeignKeys: true, // Ignore foreign keys to auth schema
};
```

### 3. Generate Types

```bash
npx kanel
```

Or use the wrapper script:

```bash
bun run scripts/database/generate-db-types-with-kanel.ts
```

## Output Structure

Kanel generates separate files for each table:

```
src/lib/database-types/
├── index.ts              # Main export file
├── Chart.ts              # Chart table types
├── ChartType.ts          # ChartType table types
├── Curve.ts              # Curve table types
└── ...
```

## Usage

### Import Types

```typescript
// Import from generated types
import type { Chart } from "$lib/database-types/Chart";
import type { ChartType } from "$lib/database-types/ChartType";

// Or use the helper type
import type { Database } from "$lib/database-types";
type ChartRow = Database["public"]["Tables"]["charts"]["Row"];
```

### Supabase Compatibility

If you need Supabase-compatible format, create a wrapper:

```typescript
// src/lib/database.types.ts (Supabase-compatible wrapper)
import type { Database as KanelDatabase } from "./database-types";

// Convert Kanel format to Supabase format
export interface Database {
  public: {
    Tables: {
      charts: {
        Row: KanelDatabase["charts"]["Row"];
        Insert: KanelDatabase["charts"]["Insert"];
        Update: KanelDatabase["charts"]["Update"];
      };
      // ... other tables
    };
  };
}
```

## Data Export Script

For debugging and inspection, use the data export script:

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

## Migration from Custom Script

### Step 1: Install Kanel

```bash
bun add -d kanel
```

### Step 2: Generate Types

```bash
npx kanel
```

### Step 3: Update Imports

Replace imports from `database.types.ts` to use new structure:

```typescript
// Before
import type { Database } from "$lib/database.types";
type Chart = Database["public"]["Tables"]["charts"]["Row"];

// After
import type { Chart } from "$lib/database-types/Chart";
```

### Step 4: Create Supabase Wrapper (Optional)

If you need Supabase-compatible format, create the wrapper file.

## Best Practices

1. **Regenerate After Migrations**
   - Always regenerate types after database migrations
   - Add to migration workflow

2. **Commit Generated Types**
   - Commit generated types to git
   - Makes it easy to see schema changes in PRs

3. **Use Helper Types**
   - Create helper types for common patterns
   - Example: `type ChartRow = Database["public"]["Tables"]["charts"]["Row"]`

4. **Separate Concerns**
   - Use generated types for database operations
   - Extend types for business logic in separate files

## Troubleshooting

### Types Not Updating

- Check database connection in `.kanelrc.js`
- Verify schema name matches (`public` by default)
- Clear output directory and regenerate

### Array Types Incorrect

- Kanel should handle arrays correctly
- Check PostgreSQL column type definition
- Verify `udt_name` in `information_schema.columns`

### Enum Types Missing

- Ensure `includeEnums: true` in config
- Check PostgreSQL enum definitions
- Verify enum is in correct schema

## Related Scripts

- `scripts/database/generate-db-types.ts` - Legacy custom script (deprecated)
- `scripts/database/generate-db-types-with-kanel.ts` - Kanel wrapper script
- `scripts/database/export-db-data.ts` - Data export script
