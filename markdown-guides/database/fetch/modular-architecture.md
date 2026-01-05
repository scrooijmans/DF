# Modular Database Architecture

## Overview

This document describes the modular architecture we've implemented for database operations in the MudRock application. The architecture separates concerns into logical modules on both frontend and backend, making the codebase more maintainable and extensible.

## Backend Structure (Rust)

### Directory Structure

```
src-tauri/src/postgres_query/
├── mod.rs                 # Module exports and re-exports
├── types.rs              # Shared type definitions
├── table_discovery.rs    # Table discovery and listing
├── data_fetching.rs      # Data retrieval operations
└── connection_test.rs    # Connection testing utilities
```

### Module Breakdown

#### 1. `types.rs` - Shared Type Definitions

```rust
// Core data structures
pub struct Well { ... }
pub struct Team { ... }
pub struct Project { ... }
pub struct User { ... }
pub struct DatabaseData { ... }

// Configuration
pub struct ConnectionConfig {
    pub postgrest_url: String,
    pub postgres_url: String,
    pub api_key: String,
}
```

**Purpose**: Centralized type definitions shared across all modules.

#### 2. `table_discovery.rs` - Table Discovery

```rust
pub struct TableDiscovery {
    config: ConnectionConfig,
}

impl TableDiscovery {
    pub fn new() -> Self
    pub fn with_config(config: ConnectionConfig) -> Self
    pub async fn list_tables(&self) -> Result<Vec<String>, Error>
    pub async fn check_tables_exist(&self, table_names: &[&str]) -> Result<Vec<bool>, Error>
}
```

**Purpose**: Handles discovery and listing of database tables using PostgREST.

#### 3. `data_fetching.rs` - Data Retrieval

```rust
pub struct DataFetcher {
    config: ConnectionConfig,
}

impl DataFetcher {
    pub fn new() -> Self
    pub fn with_config(config: ConnectionConfig) -> Self
    pub async fn fetch_wells(&self) -> Result<Vec<Well>, Error>
    pub async fn fetch_teams(&self) -> Result<Vec<Team>, Error>
    pub async fn fetch_projects(&self) -> Result<Vec<Project>, Error>
    pub async fn fetch_users(&self) -> Result<Vec<User>, Error>
    pub async fn fetch_all_data(&self) -> Result<DatabaseData, Error>
}
```

**Purpose**: Handles fetching actual data from database tables using direct PostgreSQL connections.

#### 4. `connection_test.rs` - Connection Testing

```rust
pub struct ConnectionTester {
    config: ConnectionConfig,
}

impl ConnectionTester {
    pub fn new() -> Self
    pub fn with_config(config: ConnectionConfig) -> Self
    pub async fn test_connection(&self) -> Result<bool, Error>
    pub async fn test_connection_postgrest(&self) -> Result<bool, Error>
}
```

**Purpose**: Tests database connections using both PostgreSQL and PostgREST.

### Usage in main.rs

```rust
use crate::postgres_query::{TableDiscovery, DataFetcher, ConnectionTester};

#[tauri::command]
async fn list_database_tables() -> Result<Vec<String>, String> {
    let table_discovery = TableDiscovery::new();
    table_discovery.list_tables().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_database_data() -> Result<serde_json::Value, String> {
    let data_fetcher = DataFetcher::new();
    let data = data_fetcher.fetch_all_data().await?;
    serde_json::to_value(data).map_err(|e| e.to_string())
}
```

## Frontend Structure (TypeScript/Svelte)

### Directory Structure

```
src/lib/tauri-commands/
├── mod.ts                # Module exports
├── types.ts             # TypeScript type definitions
├── table-discovery.ts   # Table discovery operations
├── data-fetching.ts     # Data retrieval operations
└── connection-test.ts   # Connection testing utilities
```

### Module Breakdown

#### 1. `types.ts` - TypeScript Type Definitions

```typescript
export interface Well { ... }
export interface Team { ... }
export interface Project { ... }
export interface User { ... }
export interface DatabaseData { ... }
export interface DatabaseIntegrationResult { ... }
export interface ConnectionTestResult { ... }
```

**Purpose**: TypeScript type definitions that mirror the Rust structs.

#### 2. `table-discovery.ts` - Table Discovery

```typescript
export async function getDatabaseTables(): Promise<string[]>;
export async function testDatabaseIntegration(): Promise<DatabaseIntegrationResult>;
export async function testDummyData(): Promise<string[]>;
export async function testDatabaseConnection(): Promise<boolean>;
export async function testPostgrestConnection(): Promise<boolean>;
```

**Purpose**: Frontend functions for table discovery and comprehensive testing.

#### 3. `data-fetching.ts` - Data Retrieval

```typescript
export async function getAllDatabaseData(): Promise<DatabaseData>;
export async function getWellsData(): Promise<Well[]>;
export async function getTeamsData(): Promise<Team[]>;
export async function getProjectsData(): Promise<Project[]>;
export async function getUsersData(): Promise<User[]>;
```

**Purpose**: Frontend functions for fetching specific data types.

#### 4. `connection-test.ts` - Connection Testing

```typescript
export async function testAllConnections(): Promise<ConnectionTestResult>;
export async function testDatabaseConnection(): Promise<boolean>;
export async function testPostgrestConnection(): Promise<boolean>;
export async function testTauriCommunication(): Promise<boolean>;
```

**Purpose**: Frontend functions for testing various connections.

### Usage in Svelte Components

```svelte
<script lang="ts">
  import { testDatabaseIntegration } from '$lib/tauri-commands/table-discovery';
  import { getWellsData } from '$lib/tauri-commands/data-fetching';
  import { testAllConnections } from '$lib/tauri-commands/connection-test';

  let tables: string[] = $state([]);
  let wells: Well[] = $state([]);

  $effect(async () => {
    const result = await testDatabaseIntegration();
    if (result.connection && result.tables) {
      tables = result.tables;
    }
  });
</script>
```

## Benefits of Modular Architecture

### 1. **Separation of Concerns**

- Each module has a single responsibility
- Easy to understand and maintain
- Clear boundaries between different operations

### 2. **Reusability**

- Modules can be used independently
- Easy to compose different operations
- Reduces code duplication

### 3. **Testability**

- Each module can be tested in isolation
- Mock dependencies easily
- Better error isolation

### 4. **Extensibility**

- Easy to add new modules
- Existing modules can be extended
- Clear patterns for new functionality

### 5. **Type Safety**

- Shared types between frontend and backend
- Compile-time error checking
- Better IDE support

## Migration from Monolithic Structure

### Before (Monolithic)

```rust
// src-tauri/src/postgres_query.rs (376 lines)
pub struct PostgresQuery;
impl PostgresQuery {
    pub async fn list_tables(&self) -> Result<Vec<String>, Error> { ... }
    pub async fn fetch_wells(&self) -> Result<Vec<Well>, Error> { ... }
    pub async fn test_connection(&self) -> Result<bool, Error> { ... }
    // ... many more methods
}
```

### After (Modular)

```rust
// src-tauri/src/postgres_query/table_discovery.rs
pub struct TableDiscovery { ... }
impl TableDiscovery {
    pub async fn list_tables(&self) -> Result<Vec<String>, Error> { ... }
}

// src-tauri/src/postgres_query/data_fetching.rs
pub struct DataFetcher { ... }
impl DataFetcher {
    pub async fn fetch_wells(&self) -> Result<Vec<Well>, Error> { ... }
}

// src-tauri/src/postgres_query/connection_test.rs
pub struct ConnectionTester { ... }
impl ConnectionTester {
    pub async fn test_connection(&self) -> Result<bool, Error> { ... }
}
```

## Best Practices

### 1. **Module Naming**

- Use descriptive names that indicate purpose
- Follow consistent naming conventions
- Group related functionality

### 2. **Error Handling**

- Use consistent error types across modules
- Provide meaningful error messages
- Handle errors at appropriate levels

### 3. **Configuration**

- Centralize configuration in types module
- Use default implementations
- Allow for custom configurations

### 4. **Documentation**

- Document public APIs
- Include usage examples
- Keep documentation up to date

### 5. **Testing**

- Test each module independently
- Mock external dependencies
- Test error conditions

## Future Enhancements

### 1. **Additional Modules**

- `query_builder.rs` - SQL query construction
- `migration_manager.rs` - Database migrations
- `cache_manager.rs` - Data caching
- `analytics.rs` - Usage analytics

### 2. **Performance Optimizations**

- Connection pooling
- Query optimization
- Caching strategies
- Batch operations

### 3. **Monitoring**

- Health checks
- Performance metrics
- Error tracking
- Usage statistics

## Conclusion

The modular architecture provides a solid foundation for the MudRock application's database operations. It improves maintainability, testability, and extensibility while maintaining type safety and performance. The clear separation of concerns makes it easy for developers to understand and contribute to the codebase.
