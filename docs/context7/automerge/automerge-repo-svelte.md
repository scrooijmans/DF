# Automerge-Repo with Svelte

## Overview

Automerge-Repo is a wrapper for the Automerge CRDT library that provides facilities for working with many documents, pluggable networking, and storage. When combined with Svelte, it enables building local-first, collaborative applications with reactive state management.

## Key Concepts

- **CRDT (Conflict-free Replicated Data Type)**: Enables automatic conflict resolution for collaborative editing
- **Repo**: Manages multiple Automerge documents with networking and storage
- **Document Handle**: Represents a document and provides methods to read/write
- **Reactive Stores**: Svelte stores that automatically update when documents change
- **Network Adapters**: Pluggable networking for syncing between peers
- **Storage Adapters**: Persistent storage for documents

## Installation

```bash
npm install @automerge/automerge-repo @automerge/automerge-repo-svelte-store
```

For networking and storage adapters:

```bash
# Browser storage
npm install @automerge/automerge-repo-storage-indexeddb

# Network adapters
npm install @automerge/automerge-repo-network-broadcastchannel
npm install @automerge/automerge-repo-network-websocket
```

## Basic Setup

### 1. Create Repo Instance

```typescript
// src/lib/repo.ts
import { Repo } from "@automerge/automerge-repo";
import { IndexedDBStorageAdapter } from "@automerge/automerge-repo-storage-indexeddb";
import { BroadcastChannelNetworkAdapter } from "@automerge/automerge-repo-network-broadcastchannel";
import { WebSocketClientAdapter } from "@automerge/automerge-repo-network-websocket";

export const repo = new Repo({
  network: [
    new BroadcastChannelNetworkAdapter(), // Cross-tab sync
    new WebSocketClientAdapter("ws://localhost:3030"), // Server sync
  ],
  storage: new IndexedDBStorageAdapter("my-app"),
  sharePolicy: async (peerId, documentId) => {
    // Control which peers can access which documents
    return true; // Allow all by default
  },
});
```

### 2. Setup Svelte Store

```typescript
// src/lib/automerge-store.ts
import { createAutomergeStore } from "@automerge/automerge-repo-svelte-store";
import { repo } from "./repo";

export const automergeStore = createAutomergeStore(repo);
```

### 3. Use Context API (Recommended)

```svelte
<!-- src/lib/AutomergeProvider.svelte -->
<script lang="ts">
  import { setContextRepo } from "@automerge/automerge-repo-svelte-store";
  import { repo } from "./repo";
  import { onMount } from "svelte";

  // Make repo available to all child components
  setContextRepo(repo);

  // Optional: Create initial document
  let docUrl = $state<string | null>(null);

  onMount(async () => {
    const handle = await repo.create({ count: 0, todos: [] });
    docUrl = handle.url;
  });
</script>

<slot {docUrl} />
```

## Working with Documents

### Creating Documents

```svelte
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";
  import { repo } from "$lib/repo";

  interface TodoDoc {
    todos: Array<{ id: string; text: string; done: boolean }>;
  }

  let docStore = $state<ReturnType<typeof document<TodoDoc>> | null>(null);

  async function createDocument() {
    const handle = await repo.create<TodoDoc>({
      todos: [],
    });
    docStore = document(handle.url);
  }
</script>

<button on:click={createDocument}>Create Document</button>
```

### Loading Documents

```svelte
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";
  import { onMount } from "svelte";

  export let docUrl: string;

  let docStore = $state<ReturnType<typeof document> | null>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      docStore = document(docUrl);
      loading = false;
    } catch (error) {
      console.error("Failed to load document:", error);
      loading = false;
    }
  });
</script>

{#if loading}
  <p>Loading document...</p>
{:else if docStore}
  <!-- Use document -->
{/if}
```

### Reading Document Data

```svelte
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";

  export let docUrl: string;

  const docStore = document(docUrl);

  // Access document data using $ prefix (Svelte store reactivity)
  $: count = $docStore?.count || 0;
  $: todos = $docStore?.todos || [];
</script>

<div>
  <h1>Count: {count}</h1>
  <ul>
    {#each todos as todo}
      <li>{todo.text}</li>
    {/each}
  </ul>
</div>
```

### Updating Documents

```svelte
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";

  export let docUrl: string;

  const docStore = document(docUrl);

  function increment() {
    if (docStore) {
      docStore.change((doc) => {
        doc.count = (doc.count || 0) + 1;
      });
    }
  }

  function addTodo(text: string) {
    if (docStore) {
      docStore.change((doc) => {
        if (!doc.todos) doc.todos = [];
        doc.todos.push({
          id: crypto.randomUUID(),
          text,
          done: false,
        });
      });
    }
  }

  function toggleTodo(id: string) {
    if (docStore) {
      docStore.change((doc) => {
        const todo = doc.todos?.find((t) => t.id === id);
        if (todo) {
          todo.done = !todo.done;
        }
      });
    }
  }
</script>

<button on:click={increment}>Increment</button>
<button on:click={() => addTodo("New todo")}>Add Todo</button>
```

## Svelte 5 Runes Integration

### Using $state and $derived

```svelte
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";

  export let docUrl: string;

  let docStore = $state<ReturnType<typeof document> | null>(null);
  let loading = $state(true);
  let error = $state<Error | null>(null);

  // Derived state from document
  let count = $derived(docStore?.$doc?.count || 0);
  let todos = $derived(docStore?.$doc?.todos || []);

  // Load document
  $effect(() => {
    async function loadDocument() {
      try {
        loading = true;
        docStore = document(docUrl);
        loading = false;
      } catch (err) {
        error = err instanceof Error ? err : new Error(String(err));
        loading = false;
      }
    }

    loadDocument();
  });

  function increment() {
    docStore?.change((doc) => {
      doc.count = (doc.count || 0) + 1;
    });
  }
</script>

{#if loading}
  <p>Loading...</p>
{:else if error}
  <p>Error: {error.message}</p>
{:else}
  <div>
    <h1>Count: {count}</h1>
    <button on:click={increment}>Increment</button>
    <ul>
      {#each todos as todo}
        <li>{todo.text}</li>
      {/each}
    </ul>
  </div>
{/if}
```

## Complete Example: Todo App

```svelte
<!-- TodoApp.svelte -->
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";
  import { repo } from "$lib/repo";
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

  let docUrl = $state<string | null>(null);
  let docStore = $state<ReturnType<typeof document<TodoDoc>> | null>(null);
  let newTodoText = $state("");

  onMount(async () => {
    // Create or load document from URL hash
    const urlFromHash = window.location.hash.substring(1);
    if (urlFromHash && isValidAutomergeUrl(urlFromHash)) {
      docUrl = urlFromHash;
    } else {
      const handle = await repo.create<TodoDoc>({
        todos: [],
        filter: "all",
      });
      docUrl = handle.url;
      window.location.hash = docUrl;
    }

    if (docUrl) {
      docStore = document(docUrl);
    }
  });

  $: todos = docStore?.$doc?.todos || [];
  $: filter = docStore?.$doc?.filter || "all";

  $: filteredTodos = todos.filter((todo) => {
    if (filter === "active") return !todo.done;
    if (filter === "completed") return todo.done;
    return true;
  });

  function addTodo() {
    if (!docStore || !newTodoText.trim()) return;

    docStore.change((doc) => {
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
    if (!docStore) return;

    docStore.change((doc) => {
      const todo = doc.todos?.find((t) => t.id === id);
      if (todo) {
        todo.done = !todo.done;
      }
    });
  }

  function deleteTodo(id: string) {
    if (!docStore) return;

    docStore.change((doc) => {
      if (doc.todos) {
        doc.todos = doc.todos.filter((t) => t.id !== id);
      }
    });
  }

  function setFilter(newFilter: "all" | "active" | "completed") {
    if (!docStore) return;

    docStore.change((doc) => {
      doc.filter = newFilter;
    });
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

  {#if docUrl}
    <div class="share-section">
      <p>Share this URL:</p>
      <input type="text" readonly value={docUrl} />
    </div>
  {/if}
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

  .share-section {
    margin-top: 20px;
    padding: 10px;
    background-color: #f5f5f5;
    border-radius: 4px;
  }
</style>
```

## Network Adapters

### BroadcastChannel (Cross-Tab Sync)

```typescript
import { BroadcastChannelNetworkAdapter } from "@automerge/automerge-repo-network-broadcastchannel";

const repo = new Repo({
  network: [new BroadcastChannelNetworkAdapter()],
  // Documents sync automatically between tabs in the same browser
});
```

### WebSocket (Server Sync)

```typescript
import { WebSocketClientAdapter } from "@automerge/automerge-repo-network-websocket";

const repo = new Repo({
  network: [
    new WebSocketClientAdapter("ws://localhost:3030", 5000), // URL, retry interval
  ],
});
```

### Multiple Adapters

```typescript
const repo = new Repo({
  network: [
    new BroadcastChannelNetworkAdapter(), // Fast local sync
    new WebSocketClientAdapter("ws://sync.example.com"), // Remote sync
  ],
});
```

## Storage Adapters

### IndexedDB (Browser)

```typescript
import { IndexedDBStorageAdapter } from "@automerge/automerge-repo-storage-indexeddb";

const repo = new Repo({
  storage: new IndexedDBStorageAdapter("my-app-db", "documents"),
  // Documents persist across page reloads
});
```

## Document Lifecycle

### Creating Documents

```typescript
// Create with initial state
const handle = await repo.create<MyDoc>({
  count: 0,
  items: [],
});

console.log(handle.url); // "automerge:4Wjg7..."
console.log(handle.documentId); // "4Wjg7..."
```

### Finding Documents

```typescript
// Find by URL
const handle = await repo.find<MyDoc>(docUrl);

// Find with progress tracking
const progress = repo.findWithProgress<MyDoc>(docUrl);

progress.subscribe((p) => {
  console.log(`Loading: ${p.status} - ${p.message}`);
});

const finalHandle = await progress.untilReady(["ready"]);
```

### Document State

```typescript
// Check document state
console.log(handle.state); // "idle" | "loading" | "ready" | "unavailable"

// Wait for document to be ready
await handle.whenReady();

// Access document data
const doc = handle.doc();
console.log(doc.count);
```

## Advanced Features

### Document Cloning

```typescript
// Clone document (creates new document with shared history)
const clonedHandle = repo.clone(handle);
console.log(clonedHandle.url !== handle.url); // true
```

### Document Merging

```typescript
// Merge another document into this one
handle.merge(otherHandle);
```

### Export/Import

```typescript
// Export to binary
const binary = await repo.export(handle.documentId);

// Import from binary
const importedHandle = repo.import<MyDoc>(binary, {
  docId: "custom-id", // Optional
});
```

### Time Travel

```typescript
// Get document history
const history = handle.history();

// View document at specific point in time
const pastView = handle.view(history[1]);
console.log(pastView.doc().count);

// Diff between states
const patches = handle.diff(history[0], history[2]);
```

## Best Practices

### 1. Use Context for Repo

```svelte
<!-- App.svelte -->
<script>
  import { setContextRepo } from "@automerge/automerge-repo-svelte-store";
  import { repo } from "$lib/repo";

  setContextRepo(repo);
</script>

<slot />
```

### 2. Handle Loading States

```svelte
<script>
  let docStore = $state(null);
  let loading = $state(true);

  $effect(() => {
    async function load() {
      loading = true;
      docStore = document(docUrl);
      loading = false;
    }
    load();
  });
</script>
```

### 3. Clean Up Subscriptions

```svelte
<script>
  import { onDestroy } from "svelte";

  const docStore = document(docUrl);

  onDestroy(() => {
    // Automerge stores handle cleanup automatically
    // But you can manually unsubscribe if needed
  });
</script>
```

### 4. Type Safety

```typescript
interface MyDocument {
  count: number;
  items: Array<{ id: string; name: string }>;
}

const docStore = document<MyDocument>(docUrl);
```

### 5. Error Handling

```svelte
<script>
  let error = $state<Error | null>(null);

  async function loadDocument() {
    try {
      docStore = document(docUrl);
    } catch (err) {
      error = err instanceof Error ? err : new Error(String(err));
    }
  }
</script>
```

## Integration with SvelteKit

### Server-Side Rendering

Automerge-Repo is client-side only. Use SvelteKit's `browser` check:

```svelte
<script>
  import { browser } from "$app/environment";
  import { onMount } from "svelte";

  let repo = $state(null);

  onMount(() => {
    if (browser) {
      // Initialize repo only on client
      repo = new Repo({ /* config */ });
    }
  });
</script>
```

### Route-Based Documents

```svelte
<!-- [docId]/+page.svelte -->
<script>
  import { page } from "$app/stores";
  import { document } from "@automerge/automerge-repo-svelte-store";

  $: docUrl = $page.params.docId;
  $: docStore = docUrl ? document(docUrl) : null;
</script>
```

## Summary

Automerge-Repo with Svelte provides:

1. **Reactive State**: Svelte stores automatically update when documents change
2. **Local-First**: Works offline with IndexedDB storage
3. **Collaborative**: Automatic conflict resolution via CRDTs
4. **Type-Safe**: Full TypeScript support
5. **Flexible Networking**: Pluggable adapters for different sync strategies
6. **Persistent Storage**: Documents survive page reloads

This combination enables building powerful collaborative applications with minimal boilerplate, leveraging Svelte's reactivity and Automerge's conflict-free data structures.

## References

- [Automerge-Repo Documentation](https://github.com/automerge/automerge-repo)
- [Automerge-Repo Svelte Store](https://github.com/automerge/automerge-repo/tree/main/packages/automerge-repo-svelte-store)
- [Automerge Documentation](https://automerge.org/)

