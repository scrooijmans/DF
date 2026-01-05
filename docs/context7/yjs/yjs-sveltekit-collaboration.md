# Yjs with SvelteKit: Collaborative Applications

This document explains how to use Yjs (CRDT library) in a SvelteKit application to enable real-time collaboration with automatic conflict resolution.

## Overview

Yjs is a high-performance CRDT (Conflict-free Replicated Data Type) library that enables automatic conflict resolution for collaborative editing. When combined with SvelteKit, you can build reactive, collaborative applications where multiple users can edit simultaneously without conflicts.

## Key Concepts

- **Y.Doc**: The main document container that holds all shared data types
- **Y.Map**: Key-value store for collaborative objects
- **Y.Array**: Ordered list for collaborative arrays
- **Y.Text**: Rich text editing with operational transforms
- **Providers**: Network adapters for syncing (WebSocket, WebRTC, etc.)
- **Awareness**: Share user-specific information (cursors, names, colors)
- **Reactive Stores**: Wrap Yjs types in Svelte stores for reactivity

## Installation

```bash
# Core Yjs library
npm install yjs

# WebSocket provider for server sync
npm install y-websocket

# IndexedDB persistence for offline support
npm install y-indexeddb

# Awareness protocol for presence (cursors, users)
npm install y-protocols
```

## Basic Setup

### 1. Create Yjs Document Manager

```typescript
// src/lib/yjs/yjs-doc-manager.ts
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import { IndexeddbPersistence } from 'y-indexeddb';
import * as awarenessProtocol from 'y-protocols/awareness';

export interface YjsDocConfig {
  documentId: string;
  websocketUrl?: string;
  room?: string;
  enablePersistence?: boolean;
}

export class YjsDocManager {
  public doc: Y.Doc;
  public wsProvider?: WebsocketProvider;
  public indexeddbProvider?: IndexeddbPersistence;
  public awareness: awarenessProtocol.Awareness;

  constructor(config: YjsDocConfig) {
    // Create Yjs document
    this.doc = new Y.Doc();

    // Create awareness instance for presence (cursors, users)
    this.awareness = new awarenessProtocol.Awareness(this.doc);

    // Set local user state
    this.awareness.setLocalStateField('user', {
      name: 'Anonymous',
      color: this.generateColor(),
    });

    // Enable IndexedDB persistence (offline support)
    if (config.enablePersistence !== false) {
      this.indexeddbProvider = new IndexeddbPersistence(
        config.documentId,
        this.doc
      );

      this.indexeddbProvider.on('synced', () => {
        console.log('Content loaded from IndexedDB');
      });
    }

    // Connect to WebSocket server for collaboration
    if (config.websocketUrl && config.room) {
      this.wsProvider = new WebsocketProvider(
        config.websocketUrl,
        config.room,
        this.doc,
        {
          awareness: this.awareness,
          params: {
            // Optional: Add authentication token
            // token: getAuthToken(),
          },
        }
      );

      // Monitor connection status
      this.wsProvider.on('status', (event: { status: string }) => {
        console.log('WebSocket status:', event.status);
      });

      this.wsProvider.on('sync', (isSynced: boolean) => {
        console.log('Synced with server:', isSynced);
      });
    }
  }

  // Generate random color for user
  private generateColor(): string {
    const colors = [
      '#ff6b6b', '#4ecdc4', '#45b7d1', '#f9ca24', '#f0932b',
      '#eb4d4b', '#6c5ce7', '#a29bfe', '#fd79a8', '#00b894',
    ];
    return colors[Math.floor(Math.random() * colors.length)];
  }

  // Cleanup
  destroy() {
    this.wsProvider?.destroy();
    this.indexeddbProvider?.destroy();
    this.awareness.destroy();
    this.doc.destroy();
  }
}
```

### 2. Create Reactive Svelte Stores for Yjs Types

```typescript
// src/lib/yjs/yjs-stores.ts
import { writable, derived, type Readable } from 'svelte/store';
import * as Y from 'yjs';
import type { YjsDocManager } from './yjs-doc-manager';

/**
 * Create a reactive store for a Y.Map
 */
export function createYMapStore<T extends Record<string, any>>(
  ymap: Y.Map<T[keyof T]>,
  initialValue: T = {} as T
): Readable<T> & { set: (key: keyof T, value: T[keyof T]) => void; get: () => T } {
  const { subscribe, set } = writable<T>(initialValue);

  // Update store when Y.Map changes
  const observer = (event: Y.YMapEvent<T[keyof T]>) => {
    const current = ymap.toJSON() as T;
    set(current);
  };

  ymap.observe(observer);

  // Initial value
  set(ymap.toJSON() as T);

  return {
    subscribe,
    set: (key: keyof T, value: T[keyof T]) => {
      ymap.set(key as string, value);
    },
    get: () => ymap.toJSON() as T,
  };
}

/**
 * Create a reactive store for a Y.Array
 */
export function createYArrayStore<T>(
  yarray: Y.Array<T>
): Readable<T[]> & {
  push: (items: T[]) => void;
  insert: (index: number, items: T[]) => void;
  delete: (index: number, length: number) => void;
  get: () => T[];
} {
  const { subscribe, set } = writable<T[]>([]);

  // Update store when Y.Array changes
  const observer = (event: Y.YArrayEvent<T>) => {
    set(yarray.toArray());
  };

  yarray.observe(observer);

  // Initial value
  set(yarray.toArray());

  return {
    subscribe,
    push: (items: T[]) => {
      yarray.push(items);
    },
    insert: (index: number, items: T[]) => {
      yarray.insert(index, items);
    },
    delete: (index: number, length: number) => {
      yarray.delete(index, length);
    },
    get: () => yarray.toArray(),
  };
}

/**
 * Create a reactive store for Y.Text
 */
export function createYTextStore(
  ytext: Y.Text
): Readable<string> & {
  insert: (index: number, text: string) => void;
  delete: (index: number, length: number) => void;
  get: () => string;
} {
  const { subscribe, set } = writable<string>('');

  // Update store when Y.Text changes
  const observer = (event: Y.YTextEvent) => {
    set(ytext.toString());
  };

  ytext.observe(observer);

  // Initial value
  set(ytext.toString());

  return {
    subscribe,
    insert: (index: number, text: string) => {
      ytext.insert(index, text);
    },
    delete: (index: number, length: number) => {
      ytext.delete(index, length);
    },
    get: () => ytext.toString(),
  };
}

/**
 * Create a reactive store for awareness (presence)
 */
export function createAwarenessStore(
  awareness: awarenessProtocol.Awareness
): Readable<Map<number, any>> {
  const { subscribe, set } = writable<Map<number, any>>(new Map());

  // Update store when awareness changes
  const observer = () => {
    set(awareness.getStates());
  };

  awareness.on('change', observer);

  // Initial value
  set(awareness.getStates());

  return {
    subscribe,
  };
}
```

### 3. SvelteKit-Specific Setup (Client-Side Only)

```typescript
// src/lib/yjs/yjs-client.ts
import { browser } from '$app/environment';
import { YjsDocManager } from './yjs-doc-manager';

let docManager: YjsDocManager | null = null;

export function getYjsDocManager(config: {
  documentId: string;
  websocketUrl?: string;
  room?: string;
}): YjsDocManager | null {
  // Only create on client-side (browser)
  if (!browser) {
    return null;
  }

  // Reuse existing manager if same document
  if (docManager && docManager.doc.guid === config.documentId) {
    return docManager;
  }

  // Create new manager
  docManager = new YjsDocManager({
    documentId: config.documentId,
    websocketUrl: config.websocketUrl,
    room: config.room || config.documentId,
    enablePersistence: true,
  });

  return docManager;
}

// Cleanup on page unload
if (browser) {
  window.addEventListener('beforeunload', () => {
    docManager?.destroy();
  });
}
```

## Complete Example: Collaborative Todo List

### Component Implementation

```svelte
<!-- src/lib/components/CollaborativeTodoList.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getYjsDocManager } from '$lib/yjs/yjs-client';
  import { createYArrayStore, createAwarenessStore } from '$lib/yjs/yjs-stores';
  import type { YjsDocManager } from '$lib/yjs/yjs-doc-manager';

  interface Todo {
    id: string;
    text: string;
    completed: boolean;
    createdAt: number;
  }

  export let documentId: string = 'todos';
  export let websocketUrl: string = 'ws://localhost:1234';

  let docManager: YjsDocManager | null = null;
  let todosStore: ReturnType<typeof createYArrayStore<Todo>> | null = null;
  let awarenessStore: ReturnType<typeof createAwarenessStore> | null = null;
  let newTodoText = $state('');
  let isConnected = $state(false);

  onMount(() => {
    // Initialize Yjs document manager
    docManager = getYjsDocManager({
      documentId,
      websocketUrl,
      room: `todos-${documentId}`,
    });

    if (!docManager) return;

    // Get or create Y.Array for todos
    const yarray = docManager.doc.getArray<Todo>('todos');

    // Create reactive store
    todosStore = createYArrayStore(yarray);

    // Create awareness store for presence
    awarenessStore = createAwarenessStore(docManager.awareness);

    // Set local user name
    docManager.awareness.setLocalStateField('user', {
      name: 'User ' + Math.floor(Math.random() * 1000),
      color: docManager.awareness.getLocalState()?.user?.color || '#ff6b6b',
    });

    // Monitor connection status
    docManager.wsProvider?.on('status', (event: { status: string }) => {
      isConnected = event.status === 'connected';
    });
  });

  onDestroy(() => {
    // Cleanup is handled by docManager
  });

  function addTodo() {
    if (!newTodoText.trim() || !todosStore) return;

    const todo: Todo = {
      id: crypto.randomUUID(),
      text: newTodoText.trim(),
      completed: false,
      createdAt: Date.now(),
    };

    todosStore.push([todo]);
    newTodoText = '';
  }

  function toggleTodo(index: number) {
    if (!todosStore) return;

    const todos = todosStore.get();
    const todo = todos[index];
    if (!todo) return;

    // Update in Y.Array
    const yarray = docManager!.doc.getArray<Todo>('todos');
    yarray.delete(index, 1);
    yarray.insert(index, [{ ...todo, completed: !todo.completed }]);
  }

  function deleteTodo(index: number) {
    if (!todosStore) return;
    todosStore.delete(index, 1);
  }

  // Get other users from awareness
  $: otherUsers = awarenessStore
    ? Array.from($awarenessStore.entries())
        .filter(([clientId]) => clientId !== docManager?.awareness.clientID)
        .map(([, state]) => state.user)
        .filter(Boolean)
    : [];
</script>

<div class="collaborative-todos">
  <div class="header">
    <h2>Collaborative Todo List</h2>
    <div class="status">
      <span class="status-indicator" class:connected={isConnected}>
        {isConnected ? '🟢 Connected' : '🔴 Disconnected'}
      </span>
      {#if otherUsers.length > 0}
        <span class="users">
          {otherUsers.length} other user{otherUsers.length > 1 ? 's' : ''} online
        </span>
      {/if}
    </div>
  </div>

  <div class="add-todo">
    <input
      type="text"
      bind:value={newTodoText}
      on:keydown={(e) => e.key === 'Enter' && addTodo()}
      placeholder="Add a todo..."
    />
    <button on:click={addTodo}>Add</button>
  </div>

  <div class="todos">
    {#if todosStore}
      {#each $todosStore as todo, index (todo.id)}
        <div class="todo" class:completed={todo.completed}>
          <input
            type="checkbox"
            checked={todo.completed}
            on:change={() => toggleTodo(index)}
          />
          <span class="todo-text">{todo.text}</span>
          <button on:click={() => deleteTodo(index)}>Delete</button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .collaborative-todos {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .status {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  .status-indicator {
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 12px;
  }

  .status-indicator.connected {
    background-color: #d4edda;
    color: #155724;
  }

  .add-todo {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }

  .add-todo input {
    flex: 1;
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }

  .add-todo button {
    padding: 8px 16px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .todos {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .todo {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }

  .todo.completed .todo-text {
    text-decoration: line-through;
    opacity: 0.6;
  }

  .todo-text {
    flex: 1;
  }

  .todo button {
    padding: 4px 8px;
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
</style>
```

## Advanced Example: Collaborative Rich Text Editor

```svelte
<!-- src/lib/components/CollaborativeEditor.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getYjsDocManager } from '$lib/yjs/yjs-client';
  import { createYTextStore, createAwarenessStore } from '$lib/yjs/yjs-stores';
  import type { YjsDocManager } from '$lib/yjs/yjs-doc-manager';

  export let documentId: string = 'editor';
  export let websocketUrl: string = 'ws://localhost:1234';

  let docManager: YjsDocManager | null = null;
  let textStore: ReturnType<typeof createYTextStore> | null = null;
  let awarenessStore: ReturnType<typeof createAwarenessStore> | null = null;
  let textarea: HTMLTextAreaElement;
  let isConnected = $state(false);

  onMount(() => {
    docManager = getYjsDocManager({
      documentId,
      websocketUrl,
      room: `editor-${documentId}`,
    });

    if (!docManager) return;

    // Get or create Y.Text for content
    const ytext = docManager.doc.getText('content');

    // Create reactive store
    textStore = createYTextStore(ytext);

    // Create awareness store
    awarenessStore = createAwarenessStore(docManager.awareness);

    // Set local user state
    docManager.awareness.setLocalStateField('user', {
      name: 'User ' + Math.floor(Math.random() * 1000),
      color: '#ff6b6b',
    });

    // Monitor connection
    docManager.wsProvider?.on('status', (event: { status: string }) => {
      isConnected = event.status === 'connected';
    });

    // Sync textarea with Y.Text
    $effect(() => {
      if (textStore && textarea) {
        const currentValue = textarea.value;
        const ytextValue = textStore.get();

        // Only update if different (avoid cursor jumping)
        if (currentValue !== ytextValue) {
          const cursorPos = textarea.selectionStart;
          textarea.value = ytextValue;
          textarea.setSelectionRange(cursorPos, cursorPos);
        }
      }
    });
  });

  function handleInput(event: Event) {
    if (!textStore || !docManager) return;

    const textarea = event.target as HTMLTextAreaElement;
    const newValue = textarea.value;
    const oldValue = textStore.get();

    // Calculate diff
    const diff = calculateDiff(oldValue, newValue, textarea.selectionStart);

    // Apply changes to Y.Text
    if (diff.inserted) {
      textStore.insert(diff.index, diff.inserted);
    }
    if (diff.deleted > 0) {
      textStore.delete(diff.index, diff.deleted);
    }
  }

  function calculateDiff(
    oldText: string,
    newText: string,
    cursorPos: number
  ): { index: number; inserted: string; deleted: number } {
    // Simple diff calculation (for production, use a proper diff algorithm)
    let index = 0;
    while (index < oldText.length && index < newText.length && oldText[index] === newText[index]) {
      index++;
    }

    let deleted = 0;
    let inserted = '';

    if (newText.length > oldText.length) {
      // Text was inserted
      inserted = newText.slice(index, index + (newText.length - oldText.length));
    } else if (newText.length < oldText.length) {
      // Text was deleted
      deleted = oldText.length - newText.length;
    }

    return { index, inserted, deleted };
  }

  // Get other users' cursors
  $: otherUsers = awarenessStore
    ? Array.from($awarenessStore.entries())
        .filter(([clientId]) => clientId !== docManager?.awareness.clientID)
        .map(([, state]) => state)
        .filter((state) => state.user && state.cursor)
    : [];
</script>

<div class="collaborative-editor">
  <div class="header">
    <h2>Collaborative Editor</h2>
    <div class="status">
      <span class="status-indicator" class:connected={isConnected}>
        {isConnected ? '🟢 Connected' : '🔴 Disconnected'}
      </span>
      {#if otherUsers.length > 0}
        <div class="users">
          {#each otherUsers as userState}
            <span
              class="user-badge"
              style="background-color: {userState.user?.color || '#ccc'}"
            >
              {userState.user?.name || 'Anonymous'}
            </span>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <textarea
    bind:this={textarea}
    on:input={handleInput}
    placeholder="Start typing... Changes sync in real-time!"
    class="editor-textarea"
  >
    {textStore ? $textStore : ''}
  </textarea>
</div>

<style>
  .collaborative-editor {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .status {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  .users {
    display: flex;
    gap: 5px;
  }

  .user-badge {
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 12px;
    color: white;
  }

  .editor-textarea {
    width: 100%;
    min-height: 400px;
    padding: 15px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: monospace;
    font-size: 14px;
    resize: vertical;
  }
</style>
```

## Example: Collaborative Data Object (Y.Map)

```svelte
<!-- src/lib/components/CollaborativeDataObject.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { getYjsDocManager } from '$lib/yjs/yjs-client';
  import { createYMapStore } from '$lib/yjs/yjs-stores';

  interface ProjectData {
    name: string;
    description: string;
    status: 'draft' | 'active' | 'completed';
    members: string[];
  }

  export let documentId: string = 'project';
  export let websocketUrl: string = 'ws://localhost:1234';

  let docManager = $state<YjsDocManager | null>(null);
  let projectStore: ReturnType<typeof createYMapStore<ProjectData>> | null = null;

  onMount(() => {
    docManager = getYjsDocManager({
      documentId,
      websocketUrl,
      room: `project-${documentId}`,
    });

    if (!docManager) return;

    // Get or create Y.Map for project data
    const ymap = docManager.doc.getMap<ProjectData[keyof ProjectData]>('project');

    // Create reactive store
    projectStore = createYMapStore<ProjectData>(ymap, {
      name: '',
      description: '',
      status: 'draft',
      members: [],
    });
  });

  function updateField<K extends keyof ProjectData>(key: K, value: ProjectData[K]) {
    if (!projectStore) return;
    projectStore.set(key, value);
  }
</script>

<div class="collaborative-project">
  <h2>Collaborative Project</h2>

  {#if projectStore}
    <div class="form-group">
      <label>Project Name</label>
      <input
        type="text"
        value={$projectStore.name || ''}
        on:input={(e) => updateField('name', e.currentTarget.value)}
      />
    </div>

    <div class="form-group">
      <label>Description</label>
      <textarea
        value={$projectStore.description || ''}
        on:input={(e) => updateField('description', e.currentTarget.value)}
      />
    </div>

    <div class="form-group">
      <label>Status</label>
      <select
        value={$projectStore.status || 'draft'}
        on:change={(e) => updateField('status', e.currentTarget.value as ProjectData['status'])}
      >
        <option value="draft">Draft</option>
        <option value="active">Active</option>
        <option value="completed">Completed</option>
      </select>
    </div>

    <div class="current-state">
      <h3>Current State (synced in real-time)</h3>
      <pre>{JSON.stringify($projectStore, null, 2)}</pre>
    </div>
  {/if}
</div>
```

## Server-Side Setup (WebSocket Server)

### Option 1: Use y-websocket-server

```bash
# Install y-websocket-server
npm install -g y-websocket-server

# Run server
HOST=localhost PORT=1234 npx y-websocket-server
```

### Option 2: Custom WebSocket Server (Node.js)

```typescript
// server/yjs-websocket-server.ts
import { WebSocketServer } from 'ws';
import * as Y from 'yjs';
import * as encoding from 'lib0/encoding';
import * as decoding from 'lib0/decoding';

const wss = new WebSocketServer({ port: 1234 });

// Store documents in memory (use database in production)
const docs = new Map<string, Y.Doc>();

wss.on('connection', (ws, req) => {
  let doc: Y.Doc | null = null;
  let room: string | null = null;

  ws.on('message', (message: Buffer) => {
    const decoder = decoding.createDecoder(message);
    const messageType = decoding.readVarUint(decoder);

    switch (messageType) {
      case 0: // Sync step 1: Client sends sync message
        room = decoding.readVarString(decoder);
        
        if (!docs.has(room)) {
          docs.set(room, new Y.Doc());
        }
        
        doc = docs.get(room)!;
        
        // Send sync step 2: Server sends document state
        const encoder = encoding.createEncoder();
        encoding.writeVarUint(encoder, 0); // Sync message type
        const syncMessage = Y.encodeStateAsUpdate(doc);
        encoding.writeVarUint8Array(encoder, syncMessage);
        ws.send(encoding.toUint8Array(encoder));
        break;

      case 1: // Update: Client sends document update
        if (doc && room) {
          const update = decoding.readVarUint8Array(decoder);
          Y.applyUpdate(doc, update);
          
          // Broadcast to other clients in same room
          wss.clients.forEach((client) => {
            if (client !== ws && client.readyState === 1) {
              const broadcastEncoder = encoding.createEncoder();
              encoding.writeVarUint(broadcastEncoder, 1); // Update message type
              encoding.writeVarUint8Array(broadcastEncoder, update);
              client.send(encoding.toUint8Array(broadcastEncoder));
            }
          });
        }
        break;
    }
  });

  ws.on('close', () => {
    // Cleanup if needed
  });
});

console.log('Yjs WebSocket server running on ws://localhost:1234');
```

## SvelteKit-Specific Considerations

### 1. Client-Side Only Initialization

```typescript
// src/lib/yjs/yjs-client.ts
import { browser } from '$app/environment';

export function getYjsDocManager(config: YjsDocConfig) {
  // Yjs requires browser APIs (WebSocket, IndexedDB)
  if (!browser) {
    return null;
  }
  
  // ... rest of implementation
}
```

### 2. SSR-Safe Component

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  
  let docManager = $state(null);
  
  onMount(() => {
    // Only initialize on client
    if (browser) {
      docManager = getYjsDocManager({ ... });
    }
  });
</script>

{#if browser && docManager}
  <!-- Yjs components -->
{/if}
```

### 3. Environment Variables

```typescript
// src/lib/yjs/config.ts
import { browser } from '$app/environment';
import { env } from '$env/dynamic/public';

export const YJS_WEBSOCKET_URL = browser
  ? env.PUBLIC_YJS_WEBSOCKET_URL || 'ws://localhost:1234'
  : '';
```

## Best Practices

### 1. Document Lifecycle Management

```typescript
// Cleanup documents when not needed
onDestroy(() => {
  docManager?.destroy();
});
```

### 2. Error Handling

```typescript
docManager.wsProvider?.on('connection-error', (error: Error) => {
  console.error('WebSocket connection error:', error);
  // Implement retry logic or fallback
});
```

### 3. Offline Support

```typescript
// IndexedDB persistence handles offline automatically
// Check sync status before critical operations
if (docManager.wsProvider?.synced) {
  // Safe to assume changes are synced
} else {
  // Changes are queued locally
}
```

### 4. Performance Optimization

```typescript
// Use Yjs transactions for batch updates
doc.transact(() => {
  ymap.set('field1', 'value1');
  ymap.set('field2', 'value2');
  yarray.push([item1, item2]);
});
```

## Comparison with Automerge-Repo

| Feature | Yjs | Automerge-Repo |
|---------|-----|----------------|
| **Bundle Size** | ~50KB | ~200KB |
| **Performance** | Very fast | Fast |
| **Svelte Integration** | Manual stores | Built-in stores |
| **Network Providers** | Many (WebSocket, WebRTC) | Fewer options |
| **Persistence** | IndexedDB plugin | Built-in |
| **TypeScript** | Good | Excellent |
| **Learning Curve** | Moderate | Steeper |

## Summary

Yjs with SvelteKit provides:

- ✅ **Real-time collaboration**: Multiple users can edit simultaneously
- ✅ **Automatic conflict resolution**: CRDT handles conflicts automatically
- ✅ **Offline support**: IndexedDB persistence works offline
- ✅ **Reactive updates**: Svelte stores automatically update UI
- ✅ **Presence awareness**: See other users' cursors and states
- ✅ **Efficient sync**: Only deltas are transmitted

**Key Integration Points**:
1. Create Yjs document manager (client-side only)
2. Wrap Yjs types in Svelte stores for reactivity
3. Use WebSocket provider for server sync
4. Use IndexedDB provider for offline persistence
5. Use Awareness API for presence (cursors, users)

This architecture enables building collaborative applications like Google Docs, Figma, or Notion with SvelteKit.

