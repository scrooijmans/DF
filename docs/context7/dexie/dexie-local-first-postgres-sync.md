# Dexie.js: Local-First Application with Optional PostgreSQL Sync

## Overview

This guide explains how to set up a type-safe Dexie.js database for a local-first application where users can optionally sync their IndexedDB data to PostgreSQL when online.

## Key Concepts

- **Local-First**: Data is stored in IndexedDB (via Dexie) and works offline
- **Optional Sync**: Users can push local data to PostgreSQL when online
- **Type Safety**: Full TypeScript support with interfaces and typed tables
- **Object Stores**: Tables in Dexie that map to PostgreSQL tables

## 1. Type-Safe Database Setup

### Basic TypeScript Setup

```typescript
import Dexie, { Table } from "dexie";

// Define TypeScript interfaces for your data models
export interface Todo {
  id?: number; // Primary key (auto-increment)
  title: string;
  description?: string;
  done: boolean;
  createdAt: Date;
  updatedAt: Date;
  syncedAt?: Date; // Timestamp when synced to PostgreSQL
  syncStatus: "pending" | "synced" | "error"; // Sync state
  postgresId?: number; // ID in PostgreSQL (if synced)
}

export interface User {
  id?: number;
  email: string;
  name: string;
  createdAt: Date;
  updatedAt: Date;
  syncedAt?: Date;
  syncStatus: "pending" | "synced" | "error";
  postgresId?: number;
}

// Extend Dexie class with typed tables
export class AppDatabase extends Dexie {
  // Declare typed table properties
  todos!: Table<Todo, number>; // Table<TEntity, TKey>
  users!: Table<User, number>;

  constructor() {
    super("MyAppDB"); // Database name

    // Define schema version and stores
    this.version(1).stores({
      // Schema syntax: '++id' = auto-increment primary key
      // Other fields = indexes for querying
      todos: "++id, title, done, syncStatus, postgresId, syncedAt",
      users: "++id, email, name, syncStatus, postgresId, syncedAt",
    });
  }
}

// Export singleton instance
export const db = new AppDatabase();
```

### Advanced Schema with Indexes

```typescript
export class AppDatabase extends Dexie {
  todos!: Table<Todo, number>;
  users!: Table<User, number>;
  projects!: Table<Project, number>;

  constructor() {
    super("MyAppDB");

    this.version(1).stores({
      // Primary key: ++id (auto-increment)
      // Indexes: field names for querying
      // Compound indexes: [field1+field2]
      todos: "++id, title, done, [syncStatus+syncedAt], postgresId",
      users: "++id, email, name, syncStatus",
      projects: "++id, name, userId, [userId+name]",
    });

    // Handle schema migrations
    this.version(2)
      .stores({
        todos: "++id, title, done, [syncStatus+syncedAt], postgresId, priority",
        users: "++id, email, name, syncStatus, avatar",
      })
      .upgrade((tx) => {
        // Migration logic for version 2
        return tx
          .table("todos")
          .toCollection()
          .modify((todo) => {
            todo.priority = "medium"; // Add default priority
          });
      });
  }
}
```

## 2. Object Store Structure for PostgreSQL Sync

### Sync-Aware Data Model

Each entity should include fields to track sync state:

```typescript
export interface SyncableEntity {
  id?: number; // Local IndexedDB ID
  postgresId?: number; // PostgreSQL ID (after sync)
  syncStatus: "pending" | "synced" | "error";
  syncedAt?: Date; // Last successful sync timestamp
  syncError?: string; // Error message if sync failed
  createdAt: Date;
  updatedAt: Date;
}

// Example: Todo with sync fields
export interface Todo extends SyncableEntity {
  title: string;
  description?: string;
  done: boolean;
  userId: number;
}
```

### Mapping to PostgreSQL Schema

Your PostgreSQL table should mirror the Dexie store structure:

```sql
-- PostgreSQL table matching the Dexie 'todos' store
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  done BOOLEAN DEFAULT FALSE,
  user_id INTEGER REFERENCES users(id),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  synced_at TIMESTAMP
);

-- Index for efficient querying
CREATE INDEX idx_todos_user_id ON todos(user_id);
CREATE INDEX idx_todos_sync_status ON todos(synced_at);
```

## 3. CRUD Operations with Type Safety

### Create (Add) Records

```typescript
// Add a new todo
async function addTodo(title: string, description?: string): Promise<number> {
  const now = new Date();
  const todoId = await db.todos.add({
    title,
    description,
    done: false,
    syncStatus: "pending", // Mark as needing sync
    createdAt: now,
    updatedAt: now,
  });
  return todoId;
}

// Bulk add
async function addMultipleTodos(todos: Omit<Todo, "id">[]): Promise<number[]> {
  const now = new Date();
  const todosWithDefaults = todos.map((todo) => ({
    ...todo,
    syncStatus: "pending" as const,
    createdAt: now,
    updatedAt: now,
  }));
  return await db.todos.bulkAdd(todosWithDefaults);
}
```

### Read (Query) Records

```typescript
// Get all todos
async function getAllTodos(): Promise<Todo[]> {
  return await db.todos.toArray();
}

// Get by ID
async function getTodoById(id: number): Promise<Todo | undefined> {
  return await db.todos.get(id);
}

// Query with filters
async function getPendingTodos(): Promise<Todo[]> {
  return await db.todos.where("syncStatus").equals("pending").toArray();
}

// Query with compound index
async function getUnsyncedTodos(): Promise<Todo[]> {
  return await db.todos
    .where("[syncStatus+syncedAt]")
    .between(["pending", undefined], ["pending", new Date()])
    .toArray();
}

// Query with multiple conditions
async function getTodosByUser(userId: number): Promise<Todo[]> {
  return await db.todos.where("userId").equals(userId).toArray();
}
```

### Update Records

```typescript
// Update a todo
async function updateTodo(id: number, updates: Partial<Todo>): Promise<number> {
  const updated = await db.todos.update(id, {
    ...updates,
    updatedAt: new Date(),
    syncStatus: "pending", // Mark as needing re-sync
  });
  return updated; // Returns number of updated records
}

// Update multiple records
async function markTodosAsDone(ids: number[]): Promise<number> {
  return await db.todos
    .where("id")
    .anyOf(ids)
    .modify((todo) => {
      todo.done = true;
      todo.updatedAt = new Date();
      todo.syncStatus = "pending";
    });
}
```

### Delete Records

```typescript
// Delete a todo
async function deleteTodo(id: number): Promise<void> {
  await db.todos.delete(id);
}

// Delete multiple
async function deleteCompletedTodos(): Promise<number> {
  return await db.todos.where("done").equals(true).delete();
}
```

## 4. Sync to PostgreSQL

### Sync Service

```typescript
interface SyncResult {
  success: boolean;
  synced: number;
  errors: number;
}

class SyncService {
  private apiUrl = "/api/sync"; // Your backend endpoint

  // Check if online
  private isOnline(): boolean {
    return navigator.onLine;
  }

  // Sync pending todos to PostgreSQL
  async syncTodos(): Promise<SyncResult> {
    if (!this.isOnline()) {
      throw new Error("Cannot sync: offline");
    }

    // Get all pending todos
    const pendingTodos = await db.todos
      .where("syncStatus")
      .equals("pending")
      .toArray();

    if (pendingTodos.length === 0) {
      return { success: true, synced: 0, errors: 0 };
    }

    let synced = 0;
    let errors = 0;

    // Sync each todo
    for (const todo of pendingTodos) {
      try {
        if (todo.postgresId) {
          // Update existing record
          await this.updateTodoInPostgres(todo);
        } else {
          // Create new record
          await this.createTodoInPostgres(todo);
        }

        // Mark as synced
        await db.todos.update(todo.id!, {
          syncStatus: "synced",
          syncedAt: new Date(),
        });

        synced++;
      } catch (error) {
        // Mark as error
        await db.todos.update(todo.id!, {
          syncStatus: "error",
          syncError: error instanceof Error ? error.message : "Unknown error",
        });
        errors++;
      }
    }

    return { success: errors === 0, synced, errors };
  }

  // Create todo in PostgreSQL
  private async createTodoInPostgres(todo: Todo): Promise<void> {
    const response = await fetch(`${this.apiUrl}/todos`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        title: todo.title,
        description: todo.description,
        done: todo.done,
        userId: todo.userId,
        createdAt: todo.createdAt,
        updatedAt: todo.updatedAt,
      }),
    });

    if (!response.ok) {
      throw new Error(`Failed to create todo: ${response.statusText}`);
    }

    const result = await response.json();

    // Update local record with PostgreSQL ID
    await db.todos.update(todo.id!, {
      postgresId: result.id,
    });
  }

  // Update todo in PostgreSQL
  private async updateTodoInPostgres(todo: Todo): Promise<void> {
    if (!todo.postgresId) {
      throw new Error("Cannot update: no postgresId");
    }

    const response = await fetch(`${this.apiUrl}/todos/${todo.postgresId}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        title: todo.title,
        description: todo.description,
        done: todo.done,
        updatedAt: todo.updatedAt,
      }),
    });

    if (!response.ok) {
      throw new Error(`Failed to update todo: ${response.statusText}`);
    }
  }

  // Sync all pending data
  async syncAll(): Promise<SyncResult> {
    const todosResult = await this.syncTodos();
    // Add other sync methods (users, projects, etc.)

    return {
      success: todosResult.success,
      synced: todosResult.synced,
      errors: todosResult.errors,
    };
  }
}

export const syncService = new SyncService();
```

### Auto-Sync on Online Event

```typescript
// Listen for online/offline events
window.addEventListener("online", async () => {
  console.log("Back online, syncing...");
  try {
    const result = await syncService.syncAll();
    console.log(
      `Sync complete: ${result.synced} synced, ${result.errors} errors`,
    );
  } catch (error) {
    console.error("Sync failed:", error);
  }
});

window.addEventListener("offline", () => {
  console.log("Gone offline");
});
```

### Manual Sync Trigger

```typescript
// Manual sync button handler
async function handleSyncClick() {
  try {
    const result = await syncService.syncAll();
    if (result.success) {
      alert(`Successfully synced ${result.synced} items`);
    } else {
      alert(`Synced ${result.synced} items, ${result.errors} errors`);
    }
  } catch (error) {
    alert(
      `Sync failed: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
}
```

## 5. Complete Example

### Database Setup (`db.ts`)

```typescript
import Dexie, { Table } from "dexie";

export interface Todo {
  id?: number;
  title: string;
  description?: string;
  done: boolean;
  userId: number;
  createdAt: Date;
  updatedAt: Date;
  syncedAt?: Date;
  syncStatus: "pending" | "synced" | "error";
  postgresId?: number;
  syncError?: string;
}

export class AppDatabase extends Dexie {
  todos!: Table<Todo, number>;

  constructor() {
    super("MyAppDB");

    this.version(1).stores({
      todos:
        "++id, title, done, userId, syncStatus, postgresId, syncedAt, [syncStatus+syncedAt]",
    });
  }
}

export const db = new AppDatabase();
```

### Usage in Application

```typescript
import { db } from "./db";
import { syncService } from "./sync-service";

// Add a todo
async function createTodo(title: string) {
  const todoId = await db.todos.add({
    title,
    done: false,
    userId: 1,
    syncStatus: "pending",
    createdAt: new Date(),
    updatedAt: new Date(),
  });

  // Optionally sync immediately if online
  if (navigator.onLine) {
    try {
      await syncService.syncTodos();
    } catch (error) {
      console.error("Auto-sync failed:", error);
    }
  }

  return todoId;
}

// Get all todos
async function getTodos() {
  return await db.todos.toArray();
}

// Update todo
async function toggleTodo(id: number) {
  const todo = await db.todos.get(id);
  if (todo) {
    await db.todos.update(id, {
      done: !todo.done,
      updatedAt: new Date(),
      syncStatus: "pending",
    });
  }
}

// Manual sync
async function syncData() {
  return await syncService.syncAll();
}
```

## 6. Best Practices

### Transaction Safety

```typescript
// Use transactions for multiple operations
async function createTodoWithUser(todo: Omit<Todo, "id">, user: User) {
  return await db.transaction("rw", db.todos, db.users, async () => {
    const userId = await db.users.add(user);
    const todoId = await db.todos.add({
      ...todo,
      userId,
      syncStatus: "pending",
      createdAt: new Date(),
      updatedAt: new Date(),
    });
    return { userId, todoId };
  });
}
```

### Error Handling

```typescript
async function safeAddTodo(todo: Omit<Todo, "id">): Promise<number | null> {
  try {
    return await db.todos.add({
      ...todo,
      syncStatus: "pending",
      createdAt: new Date(),
      updatedAt: new Date(),
    });
  } catch (error) {
    console.error("Failed to add todo:", error);
    return null;
  }
}
```

### Sync Conflict Resolution

```typescript
// Handle conflicts when syncing
async function syncWithConflictResolution(todo: Todo): Promise<void> {
  // Check if local version is newer
  const serverTodo = await fetchTodoFromPostgres(todo.postgresId!);

  if (serverTodo.updatedAt > todo.updatedAt) {
    // Server version is newer, update local
    await db.todos.update(todo.id!, {
      ...serverTodo,
      syncStatus: "synced",
      syncedAt: new Date(),
    });
  } else {
    // Local version is newer, update server
    await syncService.updateTodoInPostgres(todo);
  }
}
```

## 7. PostgreSQL Backend Example

### REST API Endpoint (Node.js/Express)

```typescript
// POST /api/sync/todos
app.post("/api/sync/todos", async (req, res) => {
  const { title, description, done, userId, createdAt, updatedAt } = req.body;

  const result = await db.query(
    `INSERT INTO todos (title, description, done, user_id, created_at, updated_at)
     VALUES ($1, $2, $3, $4, $5, $6)
     RETURNING id`,
    [title, description, done, userId, createdAt, updatedAt],
  );

  res.json({ id: result.rows[0].id });
});

// PUT /api/sync/todos/:id
app.put("/api/sync/todos/:id", async (req, res) => {
  const { id } = req.params;
  const { title, description, done, updatedAt } = req.body;

  await db.query(
    `UPDATE todos 
     SET title = $1, description = $2, done = $3, updated_at = $4
     WHERE id = $5`,
    [title, description, done, updatedAt, id],
  );

  res.json({ success: true });
});
```

## Summary

1. **Type Safety**: Use TypeScript interfaces and typed `Table<TEntity, TKey>` declarations
2. **Schema Definition**: Define stores with `version().stores()` including indexes
3. **Sync Fields**: Add `syncStatus`, `syncedAt`, and `postgresId` to track sync state
4. **CRUD Operations**: Use Dexie's typed API for all database operations
5. **Sync Service**: Create a service to handle syncing pending records to PostgreSQL
6. **Online Detection**: Use `navigator.onLine` and online/offline events
7. **Error Handling**: Track sync errors and retry failed syncs

This architecture provides a robust local-first application with optional PostgreSQL synchronization.

## References

- [Dexie.js Documentation](https://dexie.org)
- [Dexie TypeScript Guide](https://dexie.org/docs/Typescript)
- [Dexie Schema Definition](<https://dexie.org/docs/Version/Version.stores()>)
