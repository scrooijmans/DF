# PostgREST Supabase All Data Fetch - End-to-End Process

This document describes the complete data fetching flow from frontend Svelte components to backend Rust unified database service and back to the UI.

## 🏗️ Architecture Overview

```
Svelte Component → Tauri Command → UnifiedDatabaseService → PostgREST + OpenDAL → Supabase Database + MinIO Storage
     ↑                                                              ↓
     ←─────────── JSON Response ←─────────── HTTP Response ←────────
```

## 📋 Complete Flow Breakdown

### 1. **Frontend State Management (Svelte 5 Runes)**

#### **Global State Class** (`content-data-table-state.svelte.ts`)

```typescript
export class DataTableState {
  currentSelectedTable = $state<string | null>(null);
  tableData = $state<any[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);

  constructor() {
    // Reactive effect that triggers data fetching when table changes
    $effect(() => {
      if (this.currentSelectedTable) {
        this.fetchDataForSelectedTable(this.currentSelectedTable);
      }
    });
  }
}
```

**Key Features:**

- Uses Svelte 5 `$state` for reactive state management
- Context API for global state sharing across components
- Automatic data fetching when `currentSelectedTable` changes

#### **State Initialization** (`+layout.svelte`)

```svelte
<script lang="ts">
  import { setDataTableState } from '$lib/components/pages/home/content-main/content-data/data-table/content-data-table-state.svelte';

  // Initialize global data table state
  setDataTableState();
</script>
```

### 2. **User Interaction Flow**

#### **Table Selection** (`data-table-list-item.svelte`)

```svelte
<script lang="ts">
  import { getDataTableState } from '../data-table/content-data-table-state.svelte';

  const dataTableState = getDataTableState();

  function handleClick() {
    // Direct property access (Svelte 5 pattern)
    dataTableState.currentSelectedTable = tableName;
    dataTableState.error = null; // Clear previous errors
  }
</script>

<button onclick={handleClick}>
  {tableName}
</button>
```

**What Happens:**

1. User clicks on a table name
2. `currentSelectedTable` state is updated
3. `$effect` in `DataTableState` automatically triggers
4. Data fetching process begins

### 3. **Frontend Tauri Command Invocation**

#### **Command Interface** (`table-data-fetching.ts`)

```typescript
export async function getTableData(tableName: string): Promise<any[]> {
  try {
    console.log(
      `🔍 [UnifiedDatabaseService] Fetching data for table: ${tableName}...`,
    );

    // Direct Tauri command invocation to unified database service
    const tableData = await invoke<any[]>("get_database_table_data", {
      tableName,
    });

    console.log(
      `✅ [UnifiedDatabaseService] Table data fetched for ${tableName}:`,
      tableData,
    );
    return tableData;
  } catch (error) {
    console.error(
      `❌ [UnifiedDatabaseService] Failed to fetch data for table ${tableName}:`,
      error,
    );
    throw new Error(`Failed to fetch data for table ${tableName}: ${error}`);
  }
}
```

**Key Features:**

- Type-safe Tauri command invocation
- Comprehensive error handling and logging
- Direct table-specific data fetching through unified service
- **NEW**: Routes through `UnifiedDatabaseService` with caching

### 4. **Backend Tauri Command Handler**

#### **Command Registration** (`main.rs`)

```rust
#[tauri::command]
async fn get_database_table_data(table_name: String) -> Result<Vec<serde_json::Value>, String> {
    println!("🚀 [Tauri] Getting data for table: {}", table_name);
    let unified_service = UnifiedDatabaseService::get_instance().map_err(|e| e.to_string())?;
    unified_service.get_table_data(&table_name).await.map_err(|e| e.to_string())
}
```

**Command Registration:**

```rust
.invoke_handler(tauri::generate_handler![
    // ... other commands
    get_database_table_data,
    // ... more commands
])
```

### 5. **Unified Database Service Integration**

#### **UnifiedDatabaseService Implementation** (`unified_database_service.rs`)

```rust
impl UnifiedDatabaseService {
    /// Get data for a specific table with caching
    pub async fn get_table_data(&self, table_name: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        println!("🔍 [UnifiedDatabaseService] Fetching data for table: {}", table_name);

        // Check cache first
        if let Some(cached_data) = self.table_data_cache.get(table_name).await {
            println!("✅ [UnifiedDatabaseService] Cache hit for table: {} ({} records)", table_name, cached_data.len());
            return Ok(cached_data);
        }

        println!("🔄 [UnifiedDatabaseService] Cache miss for table: {}, fetching from database...", table_name);

        // Fetch from database
        let data = self.database_fetcher.fetch_table_data(table_name).await?;

        // Store in cache
        self.table_data_cache.insert(table_name.to_string(), data.clone()).await;

        println!("✅ [UnifiedDatabaseService] Fetched {} records from {} and cached", data.len(), table_name);
        Ok(data)
    }
}
```

**Key Features:**

- **Caching Layer**: Moka-based in-memory caching for database queries
- **PostgREST Integration**: Uses `DataFetcher` for PostgREST operations
- **Storage Integration**: Includes `MudRockStorageManager` for file operations
- **Singleton Pattern**: Ensures shared cache across all requests

### 6. **PostgREST Integration (DataFetcher)**

#### **Data Fetcher Implementation** (`data_fetching.rs`)

```rust
use postgrest::Postgrest;
use crate::postgres_query::types::ConnectionConfig;

impl DataFetcher {
    /// Fetch data for a specific table using PostgREST
    pub async fn fetch_table_data(&self, table_name: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        println!("🔍 Fetching data for table '{}' using PostgREST...", table_name);

        // Create PostgREST client with authentication
        let client = Postgrest::new(&self.config.postgrest_url)
            .insert_header("apikey", &self.config.api_key)
            .insert_header("Content-Type", "application/json");

        // Execute query
        let response = client
            .from(table_name)
            .select("*")
            .execute()
            .await?;

        // Parse response
        let body = response.text().await?;
        println!("📊 Raw {} response: {}", table_name, body);

        let data: Vec<serde_json::Value> = serde_json::from_str(&body)?;
        println!("✅ Successfully fetched {} records from {}", data.len(), table_name);

        Ok(data)
    }
}
```

#### **Connection Configuration** (`types.rs`)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub postgrest_url: String,
    pub postgres_url: String,
    pub api_key: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            postgrest_url: "http://91.99.166.223:8000/rest/v1".to_string(),
            postgres_url: "postgresql://postgres:MudRockSecure2024!@#@91.99.166.223:5432/postgres?connect_timeout=10".to_string(),
            api_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string(),
        }
    }
}
```

### 6. **Supabase PostgREST API**

#### **HTTP Request Details**

```
POST /rest/v1/{table_name}
Headers:
  - apikey: {supabase_anon_key}
  - Content-Type: application/json
  - Authorization: Bearer {jwt_token} (if RLS enabled)

Query: ?select=*
```

#### **Response Format**

```json
[
  {
    "id": 1,
    "name": "Well-001",
    "team_id": "uuid-here",
    "x": 123.45,
    "y": 67.89,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
  // ... more records
]
```

### 7. **Data Flow Back to Frontend**

#### **Response Processing**

1. **Rust**: PostgREST response → `Vec<serde_json::Value>`
2. **Tauri**: Serialized to JSON for transport
3. **Frontend**: Deserialized to `any[]` TypeScript array
4. **Svelte**: Reactive state update triggers UI re-render

#### **State Update in DataTable Component** (`data-table.svelte`)

```svelte
<script lang="ts">
  import { getDataTableState } from './content-data-table-state.svelte';
  import { getTableData } from '$lib/tauri-commands/table-data-fetching';

  const dataTableState = getDataTableState();

  // Reactive derived state
  let selectedTable = $derived(dataTableState.currentSelectedTable);
  let tableData = $derived(dataTableState.tableData);
  let isLoading = $derived(dataTableState.isLoading);
  let error = $derived(dataTableState.error);

  // Effect that triggers data fetching
  $effect(() => {
    if (selectedTable) {
      loadTableData(selectedTable);
    }
  });

  async function loadTableData(tableName: string) {
    try {
      dataTableState.isLoading = true;
      dataTableState.error = null;

      const data = await getTableData(tableName);
      dataTableState.tableData = data;

    } catch (err) {
      dataTableState.error = err instanceof Error ? err.message : 'Failed to load table data';
    } finally {
      dataTableState.isLoading = false;
    }
  }
</script>
```

### 8. **UI Rendering**

#### **Dynamic Table Generation**

```svelte
{#if tableData.length > 0}
  <div class="overflow-auto border rounded-md flex-1">
    <Table.Root>
      <Table.Header class="sticky top-0 bg-white z-10">
        <Table.Row>
          {#each columns as column (column)}
            <Table.Head class="capitalize">{column.replace(/_/g, ' ')}</Table.Head>
          {/each}
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each tableData as row (JSON.stringify(row))}
          <Table.Row>
            {#each columns as column (column)}
              <Table.Cell>{row[column]}</Table.Cell>
            {/each}
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </div>
{/if}
```

**Dynamic Features:**

- Columns auto-generated from first row's keys
- Responsive table with horizontal scrolling
- Sticky header for large datasets
- Loading and error states

## 🔄 Complete Data Flow Sequence

1. **User Action**: Clicks table name in sidebar
2. **State Update**: `currentSelectedTable` updated in global state
3. **Effect Trigger**: `$effect` in `DataTableState` detects change
4. **Tauri Call**: `invoke("get_database_table_data", { tableName })`
5. **Unified Service**: `UnifiedDatabaseService::get_table_data()` called
6. **Cache Check**: Check in-memory cache first
7. **PostgREST Query**: HTTP request to Supabase API (if cache miss)
8. **Database Query**: PostgreSQL query executed on Supabase
9. **Cache Storage**: Store result in Moka cache
10. **Response**: JSON data returned through all layers
11. **State Update**: `tableData` updated in global state
12. **UI Re-render**: Svelte components reactively update
13. **Table Display**: Data rendered in shadcn-svelte Table component

## 🔄 Realtime Integration & Cache Invalidation

### **Automatic Cache Invalidation**

When Supabase Realtime detects changes to the `wells` table (INSERT, UPDATE, DELETE), the cache is automatically invalidated to ensure fresh data on the next fetch:

**Flow**:

```
Realtime detects well change → RealtimeWellsService handles event
                                      ↓
                            Updates global state (PostgresWellsState)
                                      ↓
                            Invalidates UnifiedDatabaseService cache
                                      ↓
                    Next fetch will get fresh data from database
```

**Implementation**:

1. **Realtime Service** (`src/lib/services/realtime-wells-service.ts`):

   ```typescript
   private async handleWellInsert(newWell: any) {
     // Update global state
     this.wellsState.wells = [...this.wellsState.wells, well];

     // Invalidate cache for "wells" table
     await invalidateTableCache("wells");
   }
   ```

2. **Cache Invalidation Function** (`src/lib/tauri-commands/table-data-fetching.ts`):

   ```typescript
   export async function invalidateTableCache(
     tableName: string,
   ): Promise<void> {
     await invoke<void>("invalidate_table_cache", {
       tableName,
     });
   }
   ```

3. **Backend Cache Invalidation** (`src-tauri/src/database/unified_database_service.rs`):
   ```rust
   pub async fn invalidate_table_cache(&self, table_name: &str) {
       self.table_data_cache.invalidate(table_name).await;
   }
   ```

**Benefits**:

- ✅ **Fresh Data**: PostgREST fetches always return up-to-date data after Realtime updates
- ✅ **Consistency**: Frontend global state and PostgREST cache stay in sync
- ✅ **Performance**: Cache still provides benefits for unchanged tables
- ✅ **Automatic**: No manual cache clearing required

**Cache Behavior**:

- **Cache Hit**: Returns cached data instantly (if cache not invalidated)
- **Cache Miss**: Fetches fresh data from database and caches it
- **After Realtime Update**: Cache is invalidated, next fetch gets fresh data
- **TTL**: Cache entries expire after 5 minutes (configured in `UnifiedDatabaseService`)

## 🚀 Performance Benefits

- **Caching**: Subsequent requests served from memory cache (when valid)
- **Smart Invalidation**: Automatic cache invalidation on Realtime changes
- **Direct Queries**: No unnecessary data fetching or filtering
- **PostgREST Efficiency**: Optimized HTTP API calls
- **Reactive Updates**: Only affected components re-render
- **Type Safety**: End-to-end TypeScript/Rust type checking
- **Error Handling**: Comprehensive error propagation and display

## 🔧 Debugging Features

- **Extensive Logging**: Console logs at every step
- **Error States**: User-friendly error messages
- **Loading States**: Visual feedback during data fetching
- **Raw Response Logging**: Full PostgREST response visibility
- **Cache Statistics**: Monitor cache hit/miss rates

## 🆕 **New Unified Architecture Benefits**

### **Caching Layer**:

- **Memory Efficiency**: Moka cache with configurable TTL/TTI
- **Performance**: 10-100x faster for cached data
- **Cache Management**: Frontend commands for cache statistics and invalidation
- **Realtime Integration**: Automatic cache invalidation when Realtime detects database changes

### **Unified Service**:

- **Single Entry Point**: All database operations through one service
- **Storage Integration**: Combines PostgREST with OpenDAL storage
- **Singleton Pattern**: Shared cache across all requests
- **Type Safety**: Compile-time validation of data types

### **Enhanced Monitoring**:

- **Cache Metrics**: Hit/miss rates, entry counts, memory usage
- **Performance Tracking**: Query execution times and optimization
- **Error Handling**: Detailed error types and recovery strategies

This architecture provides a robust, performant, and maintainable data fetching solution that scales well with the application's growth while providing significant performance improvements through intelligent caching.
