# Automerge-Repo with IndexedDB

## Overview

Automerge-Repo uses IndexedDB as a persistent storage backend for browser-based applications. The `IndexedDBStorageAdapter` automatically saves Automerge documents to the browser's IndexedDB, enabling data persistence across page reloads and browser sessions.

## Key Benefits

- **Persistence**: Documents survive page reloads and browser restarts
- **Automatic Saving**: Changes are automatically persisted to IndexedDB
- **Offline Support**: Works completely offline once data is loaded
- **Large Storage**: IndexedDB can store much more data than localStorage
- **Efficient**: Only stores document chunks, not full copies

## Installation

```bash
npm install @automerge/automerge-repo @automerge/automerge-repo-storage-indexeddb
```

## Basic Setup

### Simple Configuration

```typescript
import { Repo } from "@automerge/automerge-repo";
import { IndexedDBStorageAdapter } from "@automerge/automerge-repo-storage-indexeddb";

const repo = new Repo({
  storage: new IndexedDBStorageAdapter(),
});

// Documents are automatically saved to IndexedDB
const handle = repo.create({ count: 0 });
await handle.whenReady();

// Data persists across page reloads
const loadedHandle = await repo.find(handle.url);
console.log(loadedHandle.doc()); // { count: 0 }
```

### Custom Database and Store Names

```typescript
import { Repo } from "@automerge/automerge-repo";
import { IndexedDBStorageAdapter } from "@automerge/automerge-repo-storage-indexeddb";

const repo = new Repo({
  storage: new IndexedDBStorageAdapter(
    "my-app-database", // Database name in IndexedDB
    "documents"        // Object store name
  ),
});
```

## How It Works

### Storage Structure

IndexedDB stores Automerge documents as binary chunks:

```
IndexedDB
└── my-app-database (Database)
    └── documents (Object Store)
        ├── <documentId>/snapshot/<hash> → Uint8Array
        ├── <documentId>/incremental/<hash> → Uint8Array
        └── ...
```

### Automatic Persistence

When you create or modify documents, the adapter automatically:

1. **Saves snapshots**: Full document state at certain intervals
2. **Saves increments**: Incremental changes between snapshots
3. **Debounces writes**: Groups multiple changes to reduce I/O
4. **Loads on demand**: Retrieves documents when requested

### Save Debouncing

The repo can be configured with a save debounce rate:

```typescript
const repo = new Repo({
  storage: new IndexedDBStorageAdapter("my-app"),
  saveDebounceRate: 100, // Milliseconds to debounce saves
});
```

## Complete Example

### Setup with Network Adapters

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
  storage: new IndexedDBStorageAdapter(
    "mudrock-app", // Database name
    "automerge-docs" // Object store name
  ),
  saveDebounceRate: 100, // Debounce saves by 100ms
});
```

### Creating and Persisting Documents

```typescript
import { repo } from "./repo";

interface TodoDoc {
  todos: Array<{ id: string; text: string; done: boolean }>;
}

// Create a new document
const handle = await repo.create<TodoDoc>({
  todos: [],
});

console.log(handle.url); // "automerge:4Wjg7..."
console.log(handle.documentId); // "4Wjg7..."

// Document is automatically saved to IndexedDB
await handle.whenReady();

// Make changes
handle.change((doc) => {
  doc.todos.push({
    id: crypto.randomUUID(),
    text: "Learn Automerge",
    done: false,
  });
});

// Changes are automatically persisted to IndexedDB
```

### Loading Persisted Documents

```typescript
// Load document by URL (from IndexedDB if available)
const handle = await repo.find<TodoDoc>(documentUrl);

// Check if document is ready
if (handle.state === "ready") {
  const doc = handle.doc();
  console.log(doc.todos);
} else {
  // Wait for document to load
  await handle.whenReady();
  const doc = handle.doc();
  console.log(doc.todos);
}
```

### Loading with Progress Tracking

```typescript
import { repo } from "./repo";

const progress = repo.findWithProgress<TodoDoc>(documentUrl);

progress.subscribe((p) => {
  console.log(`Loading: ${p.status} - ${p.message}`);
  // Output examples:
  // "Loading: requesting - Requesting from network..."
  // "Loading: loading - Loading from storage..."
  // "Loading: ready - Document ready"
});

const finalHandle = await progress.untilReady(["ready"]);
const doc = finalHandle.doc();
```

## Document Lifecycle with IndexedDB

### 1. Create Document

```typescript
const handle = repo.create({ data: "initial" });
// Document is immediately saved to IndexedDB
```

### 2. Modify Document

```typescript
handle.change((doc) => {
  doc.data = "modified";
});
// Change is queued and saved after debounce period
```

### 3. Reload Page

```typescript
// On page reload, document can be loaded from IndexedDB
const handle = await repo.find(documentUrl);
// If found in IndexedDB, loads immediately
// Otherwise, requests from network
```

### 4. Force Save

```typescript
// Force immediate save to IndexedDB
await repo.flush([handle.documentId]);
```

## Integration with Svelte

### Setup Repo with IndexedDB

```svelte
<!-- src/lib/AutomergeProvider.svelte -->
<script lang="ts">
  import { setContextRepo } from "@automerge/automerge-repo-svelte-store";
  import { Repo } from "@automerge/automerge-repo";
  import { IndexedDBStorageAdapter } from "@automerge/automerge-repo-storage-indexeddb";
  import { BroadcastChannelNetworkAdapter } from "@automerge/automerge-repo-network-broadcastchannel";
  import { onMount } from "svelte";

  let repo = $state<Repo | null>(null);

  onMount(() => {
    repo = new Repo({
      network: [new BroadcastChannelNetworkAdapter()],
      storage: new IndexedDBStorageAdapter("mudrock-app"),
    });

    if (repo) {
      setContextRepo(repo);
    }
  });
</script>

{#if repo}
  <slot />
{:else}
  <p>Initializing...</p>
{/if}
```

### Load Document from IndexedDB

```svelte
<!-- TodoList.svelte -->
<script lang="ts">
  import { document } from "@automerge/automerge-repo-svelte-store";
  import { repo } from "$lib/repo";
  import { onMount } from "svelte";

  interface TodoDoc {
    todos: Array<{ id: string; text: string; done: boolean }>;
  }

  let docUrl = $state<string | null>(null);
  let docStore = $state<ReturnType<typeof document<TodoDoc>> | null>(null);

  onMount(async () => {
    // Try to load from localStorage or create new
    const savedUrl = localStorage.getItem("todoDocUrl");

    if (savedUrl) {
      docUrl = savedUrl;
      docStore = document(savedUrl);
      // Document loads from IndexedDB automatically
    } else {
      const handle = await repo.create<TodoDoc>({ todos: [] });
      docUrl = handle.url;
      localStorage.setItem("todoDocUrl", docUrl);
      docStore = document(docUrl);
    }
  });

  $: todos = docStore?.$doc?.todos || [];

  function addTodo(text: string) {
    docStore?.change((doc) => {
      if (!doc.todos) doc.todos = [];
      doc.todos.push({
        id: crypto.randomUUID(),
        text,
        done: false,
      });
    });
    // Changes automatically saved to IndexedDB
  }
</script>

<div>
  <h1>My Todos</h1>
  <ul>
    {#each todos as todo}
      <li>{todo.text}</li>
    {/each}
  </ul>
</div>
```

## Advanced Usage

### Manual Storage Operations

```typescript
// Force save specific documents
await repo.flush([handle1.documentId, handle2.documentId]);

// Remove document from cache (will reload from IndexedDB if accessed)
await repo.removeFromCache(handle.documentId);

// Export document to binary
const binary = await repo.export(handle.documentId);

// Import document from binary
const importedHandle = repo.import<TodoDoc>(binary, {
  docId: "custom-id", // Optional custom ID
});
```

### Checking Storage Status

```typescript
// Get all cached document handles
const allHandles = repo.handles;

Object.keys(allHandles).forEach((docId) => {
  const handle = allHandles[docId];
  console.log(`Document ${docId}:`, handle.state);
  // States: "idle" | "loading" | "ready" | "unavailable"
});
```

### Graceful Shutdown

```typescript
// Flush all pending writes to IndexedDB before closing
await repo.shutdown();
// This ensures all changes are persisted
```

## Storage Adapter Interface

The `IndexedDBStorageAdapter` implements the `StorageAdapterInterface`:

```typescript
interface StorageAdapterInterface {
  // Load a chunk by key
  load(key: StorageKey): Promise<Uint8Array | undefined>;

  // Save a chunk
  save(key: StorageKey, data: Uint8Array): Promise<void>;

  // Remove a chunk
  remove(key: StorageKey): Promise<void>;

  // Load all chunks with a given prefix
  loadRange(keyPrefix: StorageKey): Promise<Chunk[]>;

  // Remove all chunks with a given prefix
  removeRange(keyPrefix: StorageKey): Promise<void>;
}
```

## IndexedDB Structure

### Database Schema

```typescript
// Database: "my-app-database"
// Object Store: "documents"

// Keys are arrays: [documentId, "snapshot" | "incremental", hash]
// Values are Uint8Array (binary Automerge data)

// Example keys:
["4Wjg7...", "snapshot", "abc123..."]
["4Wjg7...", "incremental", "def456..."]
["4Wjg7...", "incremental", "ghi789..."]
```

### Inspecting IndexedDB

You can inspect the stored data in browser DevTools:

1. Open DevTools (F12)
2. Go to Application tab
3. Expand IndexedDB
4. Find your database name
5. View the object store contents

## Best Practices

### 1. Use Meaningful Database Names

```typescript
// Good: App-specific name
new IndexedDBStorageAdapter("mudrock-app");

// Bad: Generic name
new IndexedDBStorageAdapter("app");
```

### 2. Handle Loading States

```typescript
const handle = await repo.find(docUrl);

if (handle.state === "loading") {
  await handle.whenReady();
}

const doc = handle.doc();
```

### 3. Save Document URLs

```typescript
// Store document URL for later retrieval
const handle = await repo.create({ data: "value" });
localStorage.setItem("myDocUrl", handle.url);

// Later, load from IndexedDB
const savedUrl = localStorage.getItem("myDocUrl");
if (savedUrl) {
  const handle = await repo.find(savedUrl);
}
```

### 4. Clean Up Old Documents

```typescript
// Remove document from storage
await repo.delete(handle.documentId);
// This removes all chunks from IndexedDB
```

### 5. Monitor Storage Usage

```typescript
// Check IndexedDB storage quota
if ("storage" in navigator && "estimate" in navigator.storage) {
  const estimate = await navigator.storage.estimate();
  console.log(`Used: ${estimate.usage} bytes`);
  console.log(`Quota: ${estimate.quota} bytes`);
}
```

## Error Handling

### Handle Storage Errors

```typescript
try {
  const handle = await repo.find(docUrl);
  await handle.whenReady();
} catch (error) {
  if (error.name === "QuotaExceededError") {
    console.error("IndexedDB quota exceeded");
    // Handle quota exceeded
  } else {
    console.error("Storage error:", error);
  }
}
```

### Check Browser Support

```typescript
function isIndexedDBSupported(): boolean {
  return "indexedDB" in window;
}

if (!isIndexedDBSupported()) {
  console.warn("IndexedDB not supported, storage will not persist");
}
```

## Performance Considerations

### Save Debouncing

```typescript
// Higher debounce = fewer writes, but more risk of data loss
const repo = new Repo({
  storage: new IndexedDBStorageAdapter("my-app"),
  saveDebounceRate: 500, // 500ms debounce
});
```

### Batch Operations

```typescript
// Multiple changes in one transaction
handle.change((doc) => {
  doc.items.push(item1);
  doc.items.push(item2);
  doc.items.push(item3);
});
// Only one save operation to IndexedDB
```

### Cache Management

```typescript
// Remove unused documents from memory
await repo.removeFromCache(unusedHandle.documentId);

// Documents can be reloaded from IndexedDB when needed
```

## Comparison with Other Storage

### IndexedDB vs localStorage

| Feature | IndexedDB | localStorage |
|---------|-----------|--------------|
| Storage Limit | ~50MB+ | ~5-10MB |
| Data Type | Binary/Structured | Strings only |
| Async | Yes | No |
| Performance | Better for large data | Better for small data |
| Browser Support | Modern browsers | All browsers |

### IndexedDB vs SessionStorage

- **IndexedDB**: Persists across sessions
- **SessionStorage**: Cleared when tab closes

## Troubleshooting

### Documents Not Persisting

```typescript
// Ensure document is ready before relying on persistence
await handle.whenReady();

// Force save if needed
await repo.flush([handle.documentId]);
```

### Quota Exceeded

```typescript
// Check quota before creating large documents
const estimate = await navigator.storage.estimate();
if (estimate.usage / estimate.quota > 0.9) {
  console.warn("Storage nearly full");
  // Clean up old documents
}
```

### Clearing Storage

```typescript
// Delete specific document
await repo.delete(handle.documentId);

// Clear all IndexedDB data (browser DevTools)
// Application → IndexedDB → Delete database
```

## Summary

Automerge-Repo with IndexedDB provides:

1. **Automatic Persistence**: Documents saved automatically
2. **Offline Support**: Works without network connection
3. **Large Storage**: Can store significant amounts of data
4. **Efficient**: Only stores document chunks, not full copies
5. **Reliable**: Data survives page reloads and browser restarts

The `IndexedDBStorageAdapter` seamlessly integrates with Automerge-Repo, providing a robust storage solution for local-first applications.

## References

- [Automerge-Repo Documentation](https://github.com/automerge/automerge-repo)
- [IndexedDBStorageAdapter](https://github.com/automerge/automerge-repo/tree/main/packages/automerge-repo-storage-indexeddb)
- [MDN IndexedDB Guide](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)

