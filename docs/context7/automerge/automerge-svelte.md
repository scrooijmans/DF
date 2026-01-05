# Automerge with Svelte

This document explains how to use the core Automerge CRDT library with Svelte for building collaborative, reactive applications.

## Overview

Automerge is a CRDT (Conflict-free Replicated Data Type) library that enables automatic conflict resolution for collaborative editing. When combined with Svelte, you can build reactive, collaborative applications where data changes automatically trigger UI updates.

## Key Concepts

- **CRDT**: Conflict-free Replicated Data Type - automatically resolves conflicts
- **Immutable Updates**: Automerge documents are immutable; changes create new document states
- **Change Function**: All modifications happen within a `change()` callback
- **Merge**: Combine changes from different document copies
- **Reactive Stores**: Wrap Automerge documents in Svelte stores for reactivity

## Installation

```bash
npm install @automerge/automerge
```

For Vite projects, you may need WASM plugins:

```bash
npm install vite-plugin-wasm vite-plugin-top-level-await
```

## Basic Usage

### Creating and Updating Documents

```typescript
import * as Automerge from "@automerge/automerge";

// Create initial document
let doc = Automerge.from({
  count: 0,
  todos: [],
});

// Update document (creates new document)
doc = Automerge.change(doc, (d) => {
  d.count = 1;
  d.todos.push({ id: "1", text: "Learn Automerge", done: false });
});
```

### Making Automerge Reactive in Svelte

Since Automerge documents are immutable, you need to wrap them in a Svelte store to make them reactive:

```typescript
// src/lib/automerge-store.ts
import { writable } from "svelte/store";
import * as Automerge from "@automerge/automerge";

export function createAutomergeStore<T>(initialDoc: T) {
  let doc = Automerge.from<T>(initialDoc);
  const { subscribe, set, update } = writable(doc);

  return {
    subscribe,
    // Update document using change function
    change: (changeFn: (doc: T) => void) => {
      doc = Automerge.change(doc, changeFn);
      set(doc);
    },
    // Get current document
    get: () => doc,
    // Merge another document
    merge: (otherDoc: typeof doc) => {
      doc = Automerge.merge(doc, otherDoc);
      set(doc);
    },
  };
}
```

## Svelte Component Example

### Basic Counter

```svelte
<!-- Counter.svelte -->
<script lang="ts">
  import { createAutomergeStore } from "$lib/automerge-store";

  interface CounterDoc {
    count: number;
  }

  const store = createAutomergeStore<CounterDoc>({ count: 0 });

  function increment() {
    store.change((doc) => {
      doc.count += 1;
    });
  }

  function decrement() {
    store.change((doc) => {
      doc.count -= 1;
    });
  }
</script>

<div>
  <h1>Count: {$store.count}</h1>
  <button on:click={increment}>+</button>
  <button on:click={decrement}>-</button>
</div>
```

## Svelte 5 Runes Integration

### Using $state with Automerge

```svelte
<script lang="ts">
  import * as Automerge from "@automerge/automerge";

  interface TodoDoc {
    todos: Array<{ id: string; text: string; done: boolean }>;
  }

  // Create document state
  let doc = $state(Automerge.from<TodoDoc>({ todos: [] }));

  // Derived state
  let todos = $derived(doc.todos || []);
  let activeCount = $derived(todos.filter((t) => !t.done).length);

  function addTodo(text: string) {
    doc = Automerge.change(doc, (d) => {
      if (!d.todos) d.todos = [];
      d.todos.push({
        id: crypto.randomUUID(),
        text,
        done: false,
      });
    });
  }

  function toggleTodo(id: string) {
    doc = Automerge.change(doc, (d) => {
      const todo = d.todos?.find((t) => t.id === id);
      if (todo) {
        todo.done = !todo.done;
      }
    });
  }

  function deleteTodo(id: string) {
    doc = Automerge.change(doc, (d) => {
      if (d.todos) {
        d.todos = d.todos.filter((t) => t.id !== id);
      }
    });
  }
</script>

<div>
  <h1>Todo App ({activeCount} active)</h1>
  <input
    type="text"
    on:keydown={(e) => {
      if (e.key === "Enter" && e.currentTarget.value) {
        addTodo(e.currentTarget.value);
        e.currentTarget.value = "";
      }
    }}
    placeholder="Add todo..."
  />
  <ul>
    {#each todos as todo}
      <li>
        <input
          type="checkbox"
          checked={todo.done}
          on:change={() => toggleTodo(todo.id)}
        />
        <span class:done={todo.done}>{todo.text}</span>
        <button on:click={() => deleteTodo(todo.id)}>Delete</button>
      </li>
    {/each}
  </ul>
</div>

<style>
  .done {
    text-decoration: line-through;
    opacity: 0.6;
  }
</style>
```

## Advanced Store Implementation

### Full-Featured Automerge Store

```typescript
// src/lib/automerge-store.ts
import { writable, derived } from "svelte/store";
import * as Automerge from "@automerge/automerge";

export interface AutomergeStore<T> {
  subscribe: (callback: (value: T) => void) => () => void;
  change: (changeFn: (doc: T) => void) => void;
  get: () => T;
  merge: (otherDoc: T) => void;
  fork: () => AutomergeStore<T>;
  save: () => Uint8Array;
  load: (data: Uint8Array) => void;
  getHistory: () => Automerge.DecodedChange[];
}

export function createAutomergeStore<T>(
  initialDoc: T,
): AutomergeStore<T> {
  let doc = Automerge.from<T>(initialDoc);
  const { subscribe, set } = writable(doc);

  return {
    subscribe,
    change: (changeFn: (doc: T) => void) => {
      doc = Automerge.change(doc, changeFn);
      set(doc);
    },
    get: () => doc,
    merge: (otherDoc: T) => {
      doc = Automerge.merge(doc, otherDoc);
      set(doc);
    },
    fork: () => {
      const forkedDoc = Automerge.clone(doc);
      return createAutomergeStore(forkedDoc);
    },
    save: () => {
      return Automerge.save(doc);
    },
    load: (data: Uint8Array) => {
      doc = Automerge.load<T>(data);
      set(doc);
    },
    getHistory: () => {
      return Automerge.getHistory(doc);
    },
  };
}
```

## Collaborative Editing

### Merging Changes from Multiple Sources

```svelte
<script lang="ts">
  import * as Automerge from "@automerge/automerge";
  import { createAutomergeStore } from "$lib/automerge-store";

  interface Doc {
    text: string;
  }

  const store = createAutomergeStore<Doc>({ text: "" });

  // Simulate receiving changes from another peer
  function receiveChanges(remoteDoc: typeof store.get()) {
    store.merge(remoteDoc);
  }

  // Simulate sending changes to another peer
  function sendChanges() {
    return store.get();
  }

  // Example: Merge changes from WebSocket
  function handleWebSocketMessage(data: Uint8Array) {
    const remoteDoc = Automerge.load<Doc>(data);
    store.merge(remoteDoc);
  }
</script>

<textarea
  bind:value={$store.text}
  on:input={(e) => {
    // Update on every keystroke
    store.change((doc) => {
      doc.text = e.currentTarget.value;
    });
  }}
></textarea>
```

## Persistence

### Saving and Loading Documents

```typescript
// Save to localStorage
function saveToLocalStorage(store: AutomergeStore<any>, key: string) {
  const data = store.save();
  localStorage.setItem(key, JSON.stringify(Array.from(data)));
}

// Load from localStorage
function loadFromLocalStorage<T>(
  key: string,
  defaultDoc: T,
): AutomergeStore<T> {
  const stored = localStorage.getItem(key);
  if (stored) {
    const data = new Uint8Array(JSON.parse(stored));
    const doc = Automerge.load<T>(data);
    return createAutomergeStore(doc);
  }
  return createAutomergeStore(defaultDoc);
}
```

### Auto-Save Component

```svelte
<!-- AutoSave.svelte -->
<script lang="ts">
  import { createAutomergeStore, type AutomergeStore } from "$lib/automerge-store";
  import { onMount } from "svelte";

  export let store: AutomergeStore<any>;
  export let storageKey: string;

  let saving = $state(false);
  let lastSaved = $state<Date | null>(null);

  // Auto-save on changes
  $effect(() => {
    // Subscribe to store changes
    const unsubscribe = store.subscribe(() => {
      saving = true;
      const data = store.save();
      localStorage.setItem(storageKey, JSON.stringify(Array.from(data)));
      lastSaved = new Date();
      saving = false;
    });

    return unsubscribe;
  });

  // Load on mount
  onMount(() => {
    const stored = localStorage.getItem(storageKey);
    if (stored) {
      const data = new Uint8Array(JSON.parse(stored));
      store.load(data);
    }
  });
</script>

<div class="save-status">
  {#if saving}
    <span>Saving...</span>
  {:else if lastSaved}
    <span>Saved at {lastSaved.toLocaleTimeString()}</span>
  {/if}
</div>
```

## Conflict Resolution

### Handling Conflicts

```typescript
// Check for conflicts
function hasConflicts(doc: any, path: string[]): boolean {
  const conflicts = Automerge.getConflicts(doc, path);
  return Object.keys(conflicts).length > 0;
}

// Resolve conflicts by choosing a value
function resolveConflict(
  store: AutomergeStore<any>,
  path: string[],
  value: any,
) {
  store.change((doc) => {
    // Set the chosen value
    Automerge.set(doc, path, value);
  });
}
```

## Document History and Time Travel

### Viewing History

```svelte
<script lang="ts">
  import * as Automerge from "@automerge/automerge";
  import { createAutomergeStore } from "$lib/automerge-store";

  interface Doc {
    count: number;
  }

  const store = createAutomergeStore<Doc>({ count: 0 });

  let history = $derived(Automerge.getHistory(store.get()));

  function viewAtTime(index: number) {
    const historicalDoc = Automerge.viewAt(store.get(), history[index].hash);
    console.log("Document at that time:", historicalDoc);
  }
</script>

<div>
  <h2>History ({history.length} changes)</h2>
  <ul>
    {#each history as change, index}
      <li>
        <button on:click={() => viewAtTime(index)}>
          View at change {index}
        </button>
        <span>Hash: {change.hash.slice(0, 8)}...</span>
      </li>
    {/each}
  </ul>
</div>
```

## Complete Example: Collaborative Todo App

```svelte
<!-- CollaborativeTodoApp.svelte -->
<script lang="ts">
  import * as Automerge from "@automerge/automerge";
  import { createAutomergeStore } from "$lib/automerge-store";
  import { onMount } from "svelte";

  interface Todo {
    id: string;
    text: string;
    done: boolean;
    createdAt: number;
  }

  interface TodoDoc {
    todos: Todo[];
    filter: "all" | "active" | "completed";
  }

  // Initialize store
  const STORAGE_KEY = "collaborative-todos";
  let store = $state(
    createAutomergeStore<TodoDoc>({
      todos: [],
      filter: "all",
    }),
  );

  // Load from localStorage on mount
  onMount(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      try {
        const data = new Uint8Array(JSON.parse(stored));
        const doc = Automerge.load<TodoDoc>(data);
        store = createAutomergeStore(doc);
      } catch (e) {
        console.error("Failed to load:", e);
      }
    }

    // Auto-save
    const unsubscribe = store.subscribe(() => {
      const data = store.save();
      localStorage.setItem(STORAGE_KEY, JSON.stringify(Array.from(data)));
    });

    return unsubscribe;
  });

  // Derived state
  let todos = $derived($store.todos || []);
  let filter = $derived($store.filter || "all");

  let filteredTodos = $derived(
    todos.filter((todo) => {
      if (filter === "active") return !todo.done;
      if (filter === "completed") return todo.done;
      return true;
    }),
  );

  let newTodoText = $state("");

  function addTodo() {
    if (!newTodoText.trim()) return;

    store.change((doc) => {
      if (!doc.todos) doc.todos = [];
      doc.todos.push({
        id: crypto.randomUUID(),
        text: newTodoText.trim(),
        done: false,
        createdAt: Date.now(),
      });
    });

    newTodoText = "";
  }

  function toggleTodo(id: string) {
    store.change((doc) => {
      const todo = doc.todos?.find((t) => t.id === id);
      if (todo) {
        todo.done = !todo.done;
      }
    });
  }

  function deleteTodo(id: string) {
    store.change((doc) => {
      if (doc.todos) {
        doc.todos = doc.todos.filter((t) => t.id !== id);
      }
    });
  }

  function setFilter(newFilter: "all" | "active" | "completed") {
    store.change((doc) => {
      doc.filter = newFilter;
    });
  }

  // Export/import for sharing
  function exportDoc() {
    const data = store.save();
    const blob = new Blob([data], { type: "application/octet-stream" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "todos.automerge";
    a.click();
    URL.revokeObjectURL(url);
  }

  function importDoc(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      const data = new Uint8Array(e.target?.result as ArrayBuffer);
      const doc = Automerge.load<TodoDoc>(data);
      store.merge(doc);
    };
    reader.readAsArrayBuffer(file);
  }
</script>

<div class="todo-app">
  <h1>Collaborative Todo App</h1>

  <div class="input-section">
    <input
      type="text"
      bind:value={newTodoText}
      on:keydown={(e) => e.key === "Enter" && addTodo()}
      placeholder="Add a todo..."
    />
    <button on:click={addTodo}>Add</button>
  </div>

  <div class="filters">
    <button
      class:active={filter === "all"}
      on:click={() => setFilter("all")}
    >
      All
    </button>
    <button
      class:active={filter === "active"}
      on:click={() => setFilter("active")}
    >
      Active
    </button>
    <button
      class:active={filter === "completed"}
      on:click={() => setFilter("completed")}
    >
      Completed
    </button>
  </div>

  <ul class="todo-list">
    {#each filteredTodos as todo}
      <li class:done={todo.done}>
        <input
          type="checkbox"
          checked={todo.done}
          on:change={() => toggleTodo(todo.id)}
        />
        <span>{todo.text}</span>
        <button on:click={() => deleteTodo(todo.id)}>Delete</button>
      </li>
    {/each}
  </ul>

  <div class="actions">
    <button on:click={exportDoc}>Export</button>
    <label>
      Import
      <input type="file" accept=".automerge" on:change={importDoc} />
    </label>
  </div>
</div>

<style>
  .todo-app {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
  }

  .input-section {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }

  .filters {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }

  .filters button.active {
    background-color: #007bff;
    color: white;
  }

  .todo-list {
    list-style: none;
    padding: 0;
  }

  .todo-list li {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-bottom: 1px solid #eee;
  }

  .todo-list li.done span {
    text-decoration: line-through;
    opacity: 0.6;
  }

  .actions {
    margin-top: 20px;
    display: flex;
    gap: 10px;
  }
</style>
```

## Best Practices

### 1. Always Use change() for Updates

```typescript
// ✅ Correct
doc = Automerge.change(doc, (d) => {
  d.count += 1;
});

// ❌ Wrong - won't work
doc.count += 1;
```

### 2. Handle Immutability in Svelte

```typescript
// ✅ Correct - reassign the document
let doc = $state(Automerge.from({ count: 0 }));

function increment() {
  doc = Automerge.change(doc, (d) => {
    d.count += 1;
  });
}
```

### 3. Use Stores for Complex State

```typescript
// ✅ Better for complex apps
const store = createAutomergeStore({ todos: [] });
```

### 4. Save Incrementally for Large Documents

```typescript
// For large documents, use incremental saves
const incremental = Automerge.saveIncremental(doc);
// Append to existing save data
```

### 5. Handle Conflicts Gracefully

```typescript
// Check for conflicts before displaying
const conflicts = Automerge.getConflicts(doc, ["key"]);
if (Object.keys(conflicts).length > 0) {
  // Show conflict resolution UI
}
```

## Differences from Automerge-Repo

| Feature | Automerge (Core) | Automerge-Repo |
|---------|------------------|----------------|
| Document Management | Manual | Automatic |
| Networking | Manual sync | Built-in adapters |
| Storage | Manual persistence | Built-in adapters |
| API Style | Functional (`change()`, `merge()`) | Object-oriented (handles) |
| Svelte Integration | Custom stores needed | Built-in store package |
| Use Case | Simple apps, custom sync | Complex apps, multi-document |

## Summary

Using Automerge with Svelte provides:

1. **Reactive Collaborative Data**: Changes automatically trigger UI updates
2. **Conflict Resolution**: Automatic merging of concurrent edits
3. **Immutable Updates**: Safe, predictable state changes
4. **History**: Built-in time travel and document history
5. **Persistence**: Easy save/load for offline support

The core Automerge library gives you full control over document management, syncing, and storage, making it ideal for custom collaborative applications or when you need fine-grained control over the sync process.

