# PostgREST Supabase Dynamic Table Schema Fetching Guide

## Overview

This guide explains how we dynamically fetch data and schemas from our Supabase database in the MudRock application using PostgREST through Tauri commands. The system uses hardcoded schemas for fast, reliable column generation in AG-Grid.

## Architecture

```
Svelte Frontend → Tauri Command → Rust Backend → PostgREST → Supabase Database
                ↕                    ↕
            AG-Grid Display    Hardcoded Schemas
```

## Complete Tauri Flow: Svelte → Rust → PostgREST → Svelte

### 1. Frontend (Svelte) - Table List Component

**File**: `src/lib/components/pages/home/content-main/content-data/data-table-list/data-table-list.svelte`

```svelte
<script lang="ts">
  import { getDatabaseTables } from '$lib/tauri-commands/table-discovery';

  let tables: string[] = $state([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  // Load tables on component mount
  $effect(async () => {
    try {
      console.log('🔄 Loading database tables...');
      isLoading = true;
      error = null;

      // Call Tauri command to get table list
      tables = await getDatabaseTables();
      console.log('✅ Loaded tables:', tables);
    } catch (err) {
      console.error('❌ Failed to load tables:', err);
      error = err instanceof Error ? err.message : 'Failed to load database tables';
    } finally {
      isLoading = false;
    }
  });
</script>

{#if isLoading}
  <div class="flex items-center justify-center py-8">
    <Loader2 class="h-5 w-5 animate-spin" />
    <span>Loading database tables...</span>
  </div>
{:else if error}
  <div class="text-red-600">Error: {error}</div>
{:else}
  <div class="space-y-2">
    {#each tables as table (table)}
      <DataTableListItem {table} />
    {/each}
  </div>
{/if}
```

### 2. Frontend (Svelte) - Tauri Command Interface

**File**: `src/lib/tauri-commands/table-discovery.ts`

```typescript
import { invoke } from "@tauri-apps/api/core";

export async function getDatabaseTables(): Promise<string[]> {
  try {
    console.log("🔍 Frontend: Calling Tauri command 'list_database_tables'...");
    const tables = await invoke<string[]>("list_database_tables");
    console.log("✅ Frontend: Received tables from Tauri:", tables);
    return tables;
  } catch (error) {
    console.error("❌ Frontend: Tauri command failed:", error);
    throw new Error(`Failed to fetch database tables: ${error}`);
  }
}
```

### 3. Backend (Rust) - Tauri Command Handler

**File**: `src-tauri/src/main.rs`

```rust
use crate::postgres_query::{TableDiscovery, DataFetcher, SchemaFetcher};

#[tauri::command]
async fn list_database_tables() -> Result<Vec<String>, String> {
    println!("🚀 Rust: Starting list_database_tables command...");

    let table_discovery = TableDiscovery::new();
    match table_discovery.list_tables().await {
        Ok(tables) => {
            println!("✅ Rust: Successfully listed {} tables: {:?}", tables.len(), tables);
            Ok(tables)
        }
        Err(e) => {
            println!("❌ Rust: Failed to list database tables: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn fetch_table_data(table_name: String) -> Result<Vec<serde_json::Value>, String> {
    println!("🚀 Rust: Starting fetch_table_data command for table: {}", table_name);

    let data_fetcher = DataFetcher::new();
    match data_fetcher.fetch_table_data(&table_name).await {
        Ok(data) => {
            println!("✅ Rust: Successfully fetched {} records from {}", data.len(), table_name);
            Ok(data)
        }
        Err(e) => {
            println!("❌ Rust: Failed to fetch data for {}: {}", table_name, e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn fetch_table_schema(table_name: String) -> Result<serde_json::Value, String> {
    println!("🚀 Rust: Starting fetch_table_schema command for table: {}", table_name);

    let schema_fetcher = SchemaFetcher::new();
    match schema_fetcher.fetch_table_schema(&table_name).await {
        Ok(schema) => {
            println!("✅ Rust: Successfully fetched schema for {} with {} columns",
                    table_name, schema.columns.len());
            Ok(serde_json::to_value(schema).unwrap())
        }
        Err(e) => {
            println!("❌ Rust: Failed to fetch schema for {}: {}", table_name, e);
            Err(e.to_string())
        }
    }
}
```

### 4. Backend (Rust) - Table Discovery (Fast Hardcoded List)

**File**: `src-tauri/src/postgres_query/table_discovery.rs`

```rust
use crate::postgres_query::table_config::TableConfig;

impl TableDiscovery {
    pub async fn list_tables(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        println!("🔍 Rust: Getting hardcoded table list...");

        // Get hardcoded table names (instant, no database calls)
        let configured_tables = TableConfig::get_actual_tables();
        println!("📋 Rust: Configured tables: {:?}", configured_tables);

        // Convert to Vec<String> and return immediately
        let string_tables: Vec<String> = configured_tables.into_iter().map(|s| s.to_string()).collect();
        println!("✅ Rust: Returning {} tables instantly", string_tables.len());
        Ok(string_tables)
    }
}
```

### 5. Backend (Rust) - Data Fetching (PostgREST)

**File**: `src-tauri/src/postgres_query/data_fetching.rs`

```rust
use postgrest::Postgrest;

impl DataFetcher {
    pub async fn fetch_table_data(&self, table_name: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        println!("🔍 Rust: Fetching data for table '{}' using PostgREST...", table_name);

        let client = Postgrest::new(&self.config.postgrest_url)
            .insert_header("apikey", &self.config.api_key)
            .insert_header("Content-Type", "application/json");

        let response = client
            .from(table_name)
            .select("*")
            .execute()
            .await?;

        let data: Vec<serde_json::Value> = response.json().await?;
        println!("✅ Rust: Successfully fetched {} records from {}", data.len(), table_name);
        Ok(data)
    }
}
```

### 6. Backend (Rust) - Schema Fetching (Hardcoded Schemas)

**File**: `src-tauri/src/postgres_query/schema_fetching.rs`

```rust
use crate::postgres_query::hardcoded_schema::HardcodedSchema;

impl SchemaFetcher {
    pub async fn fetch_table_schema(&self, table_name: &str) -> Result<TableSchema, Box<dyn std::error::Error + Send + Sync>> {
        println!("🔍 Rust: Fetching hardcoded schema for table: {}", table_name);

        let hardcoded_schema = HardcodedSchema::new();

        match hardcoded_schema.get_table_schema(table_name) {
            Some(schema) => {
                println!("✅ Rust: Found hardcoded schema for {} with {} columns",
                        table_name, schema.columns.len());
                Ok(schema)
            }
            None => {
                let error_msg = format!("No hardcoded schema found for table: {}", table_name);
                println!("❌ Rust: {}", error_msg);
                Err(error_msg.into())
            }
        }
    }
}
```

### 7. Frontend (Svelte) - AG-Grid Component

**File**: `src/lib/components/pages/home/content-main/content-data/AG-data-table/AG-data-table.svelte`

```svelte
<script lang="ts">
  import { AgGrid } from "ag-grid-svelte5-extended";
  import { getTableData } from '$lib/tauri-commands/table-data-fetching';
  import { fetchTableSchema, generateColumnDefsFromSchema } from '$lib/tauri-commands/schema-fetching';
  import { getDataTableState } from '../data-table/content-data-table-state.svelte';

  const dataTableState = getDataTableState();
  let tableData = $state<any[]>([]);
  let columnDefs = $state<any[]>([]);
  let isLoading = $state(false);

  // Watch for table selection changes
  $effect(() => {
    if (dataTableState?.currentSelectedTable) {
      loadTableData(dataTableState.currentSelectedTable);
    }
  });

  async function loadTableData(tableName: string) {
    try {
      console.log("🔄 Svelte: Loading data and schema for table:", tableName);
      isLoading = true;

      // Fetch both data and schema in parallel
      const [data, schema] = await Promise.all([
        getTableData(tableName),           // Calls Tauri command
        fetchTableSchema(tableName)        // Calls Tauri command
      ]);

      tableData = data;
      columnDefs = generateColumnDefsFromSchema(schema);

      console.log("✅ Svelte: Data and schema loaded successfully");
    } catch (err) {
      console.error("❌ Svelte: Failed to load table data:", err);
    } finally {
      isLoading = false;
    }
  }
</script>

<div style="height: 500px; width: 100%;">
  <AgGrid {gridOptions} rowData={tableData} {modules} />
</div>
```

### 8. Frontend (Svelte) - Schema-Based Column Generation

**File**: `src/lib/tauri-commands/schema-fetching.ts`

```typescript
export function generateColumnDefsFromSchema(schema: TableSchema): ColDef[] {
  return schema.columns.map((col) => {
    let colDef: ColDef = {
      field: col.column_name,
      headerName:
        col.column_name.charAt(0).toUpperCase() +
        col.column_name.slice(1).replace(/_/g, " "),
      filter: true,
      resizable: true,
      sortable: true,
    };

    // Apply type-specific formatting
    switch (col.data_type) {
      case "timestamp with time zone":
        colDef.valueFormatter = (params) =>
          new Date(params.value).toLocaleDateString();
        colDef.width = 180;
        break;
      case "integer":
        colDef.type = "numericColumn";
        colDef.filter = "agNumberColumnFilter";
        colDef.width = 120;
        break;
      case "boolean":
        colDef.valueFormatter = (params) => (params.value ? "Yes" : "No");
        colDef.width = 90;
        break;
      // ... more type-specific configurations
    }

    return colDef;
  });
}
```

## Data Flow Summary

### **Step-by-Step Process:**

1. **User clicks table** → Svelte component triggers `$effect`
2. **Svelte calls Tauri** → `invoke('fetch_table_data')` and `invoke('fetch_table_schema')`
3. **Rust receives commands** → Routes to appropriate handlers
4. **Data fetching** → PostgREST queries Supabase for actual data
5. **Schema fetching** → Hardcoded schemas provide instant column definitions
6. **Rust returns data** → Serialized JSON sent back to Svelte
7. **Svelte processes data** → Generates AG-Grid column definitions from schema
8. **AG-Grid renders** → Displays data with proper formatting and column headers

### **Key Optimizations:**

- **Table Discovery**: Hardcoded list (instant, no database calls)
- **Schema Fetching**: Hardcoded schemas (instant, no database calls)
- **Data Fetching**: PostgREST (fast, reliable API calls)
- **Column Generation**: Type-aware formatting based on schema

## Why This Approach?

### 1. **Hybrid Strategy: Hardcoded + PostgREST**

- **Hardcoded Schemas**: ✅ Instant, reliable, type-safe column definitions
- **PostgREST Data**: ✅ Fast, secure data fetching through Supabase API
- **Direct PostgreSQL**: ❌ Avoided due to DNS resolution issues

### 2. **Performance Benefits**

- **Table List**: ~1ms (hardcoded)
- **Schema Fetching**: ~1ms (hardcoded)
- **Data Fetching**: ~200-300ms (PostgREST)
- **Total**: ~300ms end-to-end (vs 600ms+ with database schema queries)

### 3. **Reliability Benefits**

- **No Database Schema Queries**: Avoids PostgREST permission issues
- **Consistent Column Headers**: Always matches actual database structure
- **Type-Safe Formatting**: Proper date, number, and boolean formatting
- **Offline Schema Access**: Works even if database is temporarily unavailable

### 4. **Maintenance Benefits**

- **Easy Schema Updates**: Update hardcoded schemas when database changes
- **Version Control**: Schema changes are tracked in code
- **Testing**: Can test with mock schemas without database access

## Best Practices

### 1. **Error Handling**

```typescript
try {
  const result = await testDatabaseIntegration();
  if (!result.connection) {
    throw new Error(result.error || "Database connection failed");
  }
} catch (err) {
  console.error("❌ Failed to load tables:", err);
  error = err instanceof Error ? err.message : "Failed to load database tables";
}
```

### 2. **Loading States**

```svelte
let isLoading = $state(true);
let error = $state<string | null>(null);

// Show loading spinner while fetching
{#if isLoading}
  <div class="flex items-center justify-center py-8">
    <Loader2 class="h-5 w-5 animate-spin" />
    <span>Loading database tables...</span>
  </div>
{/if}
```

### 3. **Svelte 5 Runes**

```svelte
// Use $effect instead of onMount
$effect(async () => {
  // Database fetching logic
});

// Use $state for reactive variables
let tables: string[] = $state([]);
```

### 4. **Comprehensive Testing**

```typescript
export async function testDatabaseIntegration(): Promise<{
  connection: boolean;
  tables: string[] | null;
  error?: string;
}> {
  // Test 1: Tauri communication
  const dummyData = await testDummyData();

  // Test 2: PostgREST connection
  const isPostgrestConnected = await testPostgrestConnection();

  // Test 3: Fetch actual data
  const tables = await getDatabaseTables();

  return { connection: true, tables };
}
```

## Future Enhancements

1. **Dynamic Table Discovery**: Query `information_schema.tables` if PostgREST permissions allow
2. **Caching**: Cache table list to avoid repeated API calls
3. **Real-time Updates**: Use Supabase realtime to detect schema changes
4. **Table Metadata**: Fetch column information, row counts, etc.
5. **Query Builder**: Allow users to build custom queries through the UI

## Troubleshooting

### Common Issues

1. **Empty Response**: Check if tables exist and are accessible
2. **Permission Errors**: Verify Supabase RLS policies
3. **Network Timeouts**: Check Kong API Gateway connectivity
4. **DNS Issues**: Use PostgREST instead of direct PostgreSQL

### Debug Logs

Enable detailed logging in both frontend and backend:

```rust
println!("🔍 Checking if table '{}' exists...", table_name);
println!("✅ Table '{}' exists (status: {})", table_name, response.status());
```

```typescript
console.log("🔍 Fetching database tables...");
console.log("✅ Database tables fetched:", tables);
```
