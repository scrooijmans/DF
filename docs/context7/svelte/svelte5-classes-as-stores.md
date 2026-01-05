# Classes as Stores in Svelte 5

This document explains how local and global classes can function as stores in Svelte 5, using reactive state (`$state`) to create reactive class instances that work like stores.

## Overview

In Svelte 5, classes can function as stores when they contain reactive properties using `$state`. Unlike Svelte 4, classes are **no longer auto-reactive** - you must explicitly use `$state` to make class properties reactive. This allows you to create class-based stores that can be used locally within components or globally across your application.

## Key Concepts

### Classes Are Not Auto-Reactive in Svelte 5

In Svelte 4, updating a class property would automatically trigger reactivity:

```svelte
<!-- Svelte 4: Auto-reactive -->
<script>
  let foo = new Foo();
</script>

<button on:click={() => (foo.value = 1)}>{foo.value}</button>
```

In Svelte 5, you must explicitly use `$state`:

```svelte
<!-- Svelte 5: Explicit reactivity -->
<script>
  class Foo {
    value = $state(0); // Must use $state
  }
  
  let foo = new Foo();
</script>

<button onclick={() => (foo.value = 1)}>{foo.value}</button>
```

### Wrapping Classes with $state Has No Effect

**Important**: Wrapping a class instance with `$state()` does **not** make it reactive. Only vanilla objects and arrays are made deeply reactive by `$state()`.

```javascript
// ❌ This does NOT work
class Todo {
  text = 'hello';
}

let todo = $state(new Todo()); // No effect - class is not made reactive
```

```javascript
// ✅ This works - use $state on properties
class Todo {
  text = $state('hello'); // Property is reactive
}
```

## Local Classes (Component-Scoped)

Local classes are defined within a component and used only within that component's scope.

### Basic Local Class Store

```svelte
<!-- file: TodoComponent.svelte -->
<script>
  // Define class with reactive properties
  class Todo {
    done = $state(false);
    text = $state('');

    constructor(text) {
      this.text = $state(text); // Can also initialize in constructor
    }

    toggle() {
      this.done = !this.done;
    }

    reset() {
      this.text = '';
      this.done = false;
    }
  }

  // Create instance - automatically reactive
  const todo = new Todo('Learn Svelte 5');
</script>

<div>
  <input bind:value={todo.text} />
  <label>
    <input type="checkbox" bind:checked={todo.done} />
    Done
  </label>
  <button onclick={todo.toggle}>Toggle</button>
  <button onclick={todo.reset}>Reset</button>
</div>

<p>Todo: {todo.text} - {todo.done ? 'Done' : 'Not done'}</p>
```

### Local Class with Methods

```svelte
<script>
  class Counter {
    count = $state(0);

    increment() {
      this.count++;
    }

    decrement() {
      this.count--;
    }

    reset() {
      this.count = 0;
    }

    get doubled() {
      return this.count * 2;
    }
  }

  const counter = new Counter();
</script>

<button onclick={counter.increment}>+</button>
<button onclick={counter.decrement}>-</button>
<button onclick={counter.reset}>Reset</button>

<p>Count: {counter.count}</p>
<p>Doubled: {counter.doubled}</p>
```

### Fixing `this` Context in Class Methods

When passing class methods as callbacks, use arrow functions to preserve `this`:

```javascript
// ❌ Problem: `this` context lost
class Todo {
  done = $state(false);

  reset() {
    this.done = false; // `this` might be undefined
  }
}

// ✅ Solution 1: Arrow function method
class Todo {
  done = $state(false);

  reset = () => {
    this.done = false; // `this` is bound to instance
  }
}

// ✅ Solution 2: Bind in template
<button onclick={() => todo.reset()}>Reset</button>
```

### Local Class with Array State

```svelte
<script>
  class TodoList {
    todos = $state([]);

    add(text) {
      this.todos.push({
        id: Date.now(),
        text: $state(text),
        done: $state(false)
      });
    }

    remove(id) {
      this.todos = this.todos.filter(t => t.id !== id);
    }

    toggle(id) {
      const todo = this.todos.find(t => t.id === id);
      if (todo) {
        todo.done = !todo.done;
      }
    }

    get completedCount() {
      return this.todos.filter(t => t.done).length;
    }
  }

  const todoList = new TodoList();
</script>

<input 
  type="text" 
  onkeydown={(e) => {
    if (e.key === 'Enter') {
      todoList.add(e.target.value);
      e.target.value = '';
    }
  }}
/>

{#each todoList.todos as todo}
  <div>
    <input 
      type="checkbox" 
      checked={todo.done} 
      onchange={() => todoList.toggle(todo.id)}
    />
    <span>{todo.text}</span>
    <button onclick={() => todoList.remove(todo.id)}>Remove</button>
  </div>
{/each}

<p>Completed: {todoList.completedCount} / {todoList.todos.length}</p>
```

## Global Classes (.svelte.js / .svelte.ts Files)

Global classes are defined in `.svelte.js` or `.svelte.ts` files and can be imported and shared across multiple components.

### Creating Global Class Stores

```javascript
/// file: stores/CounterStore.svelte.js
export class CounterStore {
  count = $state(0);

  increment() {
    this.count++;
  }

  decrement() {
    this.count--;
  }

  reset() {
    this.count = 0;
  }

  get doubled() {
    return this.count * 2;
  }
}

// Export singleton instance
export const counter = new CounterStore();
```

### Using Global Class Store

```svelte
<!-- file: Counter.svelte -->
<script>
  import { counter } from './stores/CounterStore.svelte.js';
</script>

<button onclick={counter.increment}>+</button>
<button onclick={counter.decrement}>-</button>
<button onclick={counter.reset}>Reset</button>

<p>Count: {counter.count}</p>
<p>Doubled: {counter.doubled}</p>
```

### Multiple Components Sharing Global Store

```svelte
<!-- file: ComponentA.svelte -->
<script>
  import { counter } from './stores/CounterStore.svelte.js';
</script>

<button onclick={counter.increment}>Increment from A</button>
<p>Count in A: {counter.count}</p>
```

```svelte
<!-- file: ComponentB.svelte -->
<script>
  import { counter } from './stores/CounterStore.svelte.js';
</script>

<button onclick={counter.decrement}>Decrement from B</button>
<p>Count in B: {counter.count}</p>
```

### Global Class Store with Complex State

```javascript
/// file: stores/UserStore.svelte.js
export class UserStore {
  user = $state(null);
  loading = $state(false);
  error = $state(null);

  async login(email, password) {
    this.loading = true;
    this.error = null;
    
    try {
      const response = await fetch('/api/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });
      
      if (!response.ok) {
        throw new Error('Login failed');
      }
      
      this.user = await response.json();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  logout() {
    this.user = null;
    this.error = null;
  }

  get isAuthenticated() {
    return this.user !== null;
  }
}

export const userStore = new UserStore();
```

```svelte
<!-- file: Login.svelte -->
<script>
  import { userStore } from './stores/UserStore.svelte.js';

  let email = '';
  let password = '';

  async function handleLogin() {
    await userStore.login(email, password);
  }
</script>

{#if userStore.loading}
  <p>Logging in...</p>
{:else if userStore.error}
  <p>Error: {userStore.error}</p>
{:else if userStore.isAuthenticated}
  <p>Welcome, {userStore.user.name}!</p>
  <button onclick={userStore.logout}>Logout</button>
{:else}
  <form onsubmit={(e) => { e.preventDefault(); handleLogin(); }}>
    <input type="email" bind:value={email} placeholder="Email" />
    <input type="password" bind:value={password} placeholder="Password" />
    <button type="submit">Login</button>
  </form>
{/if}
```

## Advanced Patterns

### Class Store with Derived State

```javascript
/// file: stores/CartStore.svelte.js
export class CartStore {
  items = $state([]);

  addItem(product) {
    const existing = this.items.find(item => item.id === product.id);
    if (existing) {
      existing.quantity++;
    } else {
      this.items.push({
        ...product,
        quantity: $state(1)
      });
    }
  }

  removeItem(id) {
    this.items = this.items.filter(item => item.id !== id);
  }

  updateQuantity(id, quantity) {
    const item = this.items.find(item => item.id === id);
    if (item) {
      item.quantity = quantity;
    }
  }

  // Computed properties (getters)
  get total() {
    return this.items.reduce((sum, item) => 
      sum + item.price * item.quantity, 0
    );
  }

  get itemCount() {
    return this.items.reduce((sum, item) => sum + item.quantity, 0);
  }

  get isEmpty() {
    return this.items.length === 0;
  }

  clear() {
    this.items = [];
  }
}

export const cart = new CartStore();
```

### Class Store with Effects

```javascript
/// file: stores/ThemeStore.svelte.js
export class ThemeStore {
  mode = $state('light');

  constructor() {
    // Load from localStorage on initialization
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('theme');
      if (saved) {
        this.mode = saved;
      }
    }
  }

  toggle() {
    this.mode = this.mode === 'light' ? 'dark' : 'light';
    
    // Save to localStorage
    if (typeof window !== 'undefined') {
      localStorage.setItem('theme', this.mode);
      document.documentElement.setAttribute('data-theme', this.mode);
    }
  }

  setMode(mode) {
    this.mode = mode;
    if (typeof window !== 'undefined') {
      localStorage.setItem('theme', mode);
      document.documentElement.setAttribute('data-theme', mode);
    }
  }
}

export const theme = new ThemeStore();
```

### Class Store with Subscriptions

```javascript
/// file: stores/WebSocketStore.svelte.js
export class WebSocketStore {
  connected = $state(false);
  messages = $state([]);
  #ws = null;

  connect(url) {
    this.#ws = new WebSocket(url);

    this.#ws.onopen = () => {
      this.connected = true;
    };

    this.#ws.onclose = () => {
      this.connected = false;
    };

    this.#ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.messages.push($state(message));
    };

    this.#ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }

  send(data) {
    if (this.#ws && this.connected) {
      this.#ws.send(JSON.stringify(data));
    }
  }

  disconnect() {
    if (this.#ws) {
      this.#ws.close();
      this.#ws = null;
      this.connected = false;
    }
  }

  clearMessages() {
    this.messages = [];
  }
}

export const wsStore = new WebSocketStore();
```

## Class Stores vs. Traditional Stores

### Traditional Store (svelte/store)

```javascript
import { writable, derived } from 'svelte/store';

const count = writable(0);
const doubled = derived(count, $count => $count * 2);

// Usage
count.subscribe(value => console.log(value));
count.set(1);
count.update(n => n + 1);
```

### Class Store Equivalent

```javascript
export class CounterStore {
  count = $state(0);

  get doubled() {
    return this.count * 2;
  }

  increment() {
    this.count++;
  }
}

export const counter = new CounterStore();

// Usage - direct property access
counter.count; // Read
counter.count = 1; // Write
counter.increment(); // Method
```

**Advantages of Class Stores:**
- ✅ Direct property access (no `.subscribe()` needed)
- ✅ Methods for encapsulated logic
- ✅ Computed properties with getters
- ✅ TypeScript-friendly
- ✅ More object-oriented approach

**Advantages of Traditional Stores:**
- ✅ Explicit subscription model
- ✅ Works with legacy Svelte 4 code
- ✅ `derived` stores for complex computations
- ✅ Better for RxJS integration

## Best Practices

### 1. Use $state for All Reactive Properties

```javascript
// ✅ GOOD: All reactive properties use $state
class Todo {
  text = $state('');
  done = $state(false);
  createdAt = $state(new Date());
}

// ❌ BAD: Non-reactive properties won't trigger updates
class Todo {
  text = ''; // Not reactive
  done = false; // Not reactive
}
```

### 2. Initialize $state in Constructor When Needed

```javascript
class Todo {
  done = $state(false);

  constructor(text) {
    this.text = $state(text); // Initialize in constructor
  }
}
```

### 3. Use Arrow Functions for Methods Passed as Callbacks

```javascript
class Todo {
  done = $state(false);

  // ✅ GOOD: Arrow function preserves `this`
  toggle = () => {
    this.done = !this.done;
  }

  // ⚠️ OK: Regular method (bind in template)
  reset() {
    this.done = false;
  }
}
```

### 4. Export Singleton Instances for Global Stores

```javascript
// ✅ GOOD: Export both class and singleton
export class CounterStore {
  count = $state(0);
}

export const counter = new CounterStore();

// Usage: import { counter } from './store.svelte.js'
```

### 5. Use Getters for Computed Properties

```javascript
class CartStore {
  items = $state([]);

  // ✅ GOOD: Getter for computed value
  get total() {
    return this.items.reduce((sum, item) => sum + item.price, 0);
  }
}
```

### 6. Handle SSR Considerations

```javascript
export class ThemeStore {
  mode = $state('light');

  constructor() {
    // ✅ GOOD: Check for browser environment
    if (typeof window !== 'undefined') {
      const saved = localStorage.getItem('theme');
      if (saved) {
        this.mode = saved;
      }
    }
  }
}
```

## Complete Example: Todo App with Class Stores

```javascript
/// file: stores/TodoStore.svelte.js
export class TodoStore {
  todos = $state([]);
  filter = $state('all'); // 'all', 'active', 'completed'

  add(text) {
    this.todos.push({
      id: Date.now(),
      text: $state(text),
      done: $state(false),
      createdAt: $state(new Date())
    });
  }

  remove(id) {
    this.todos = this.todos.filter(t => t.id !== id);
  }

  toggle(id) {
    const todo = this.todos.find(t => t.id === id);
    if (todo) {
      todo.done = !todo.done;
    }
  }

  setFilter(filter) {
    this.filter = filter;
  }

  clearCompleted() {
    this.todos = this.todos.filter(t => !t.done);
  }

  // Computed properties
  get filteredTodos() {
    if (this.filter === 'active') {
      return this.todos.filter(t => !t.done);
    }
    if (this.filter === 'completed') {
      return this.todos.filter(t => t.done);
    }
    return this.todos;
  }

  get activeCount() {
    return this.todos.filter(t => !t.done).length;
  }

  get completedCount() {
    return this.todos.filter(t => t.done).length;
  }

  get allCompleted() {
    return this.todos.length > 0 && this.completedCount === this.todos.length;
  }
}

export const todoStore = new TodoStore();
```

```svelte
<!-- file: TodoApp.svelte -->
<script>
  import { todoStore } from './stores/TodoStore.svelte.js';

  let input = '';

  function addTodo() {
    if (input.trim()) {
      todoStore.add(input);
      input = '';
    }
  }
</script>

<div class="todo-app">
  <h1>Todos</h1>

  <div class="input-section">
    <input 
      type="text" 
      bind:value={input}
      onkeydown={(e) => e.key === 'Enter' && addTodo()}
      placeholder="What needs to be done?"
    />
    <button onclick={addTodo}>Add</button>
  </div>

  <div class="filters">
    <button 
      class:active={todoStore.filter === 'all'}
      onclick={() => todoStore.setFilter('all')}
    >
      All ({todoStore.todos.length})
    </button>
    <button 
      class:active={todoStore.filter === 'active'}
      onclick={() => todoStore.setFilter('active')}
    >
      Active ({todoStore.activeCount})
    </button>
    <button 
      class:active={todoStore.filter === 'completed'}
      onclick={() => todoStore.setFilter('completed')}
    >
      Completed ({todoStore.completedCount})
    </button>
  </div>

  <ul class="todo-list">
    {#each todoStore.filteredTodos as todo}
      <li class:done={todo.done}>
        <input 
          type="checkbox" 
          checked={todo.done}
          onchange={() => todoStore.toggle(todo.id)}
        />
        <span>{todo.text}</span>
        <button onclick={() => todoStore.remove(todo.id)}>Remove</button>
      </li>
    {/each}
  </ul>

  {#if todoStore.completedCount > 0}
    <button onclick={todoStore.clearCompleted}>
      Clear Completed ({todoStore.completedCount})
    </button>
  {/if}
</div>
```

## Summary

**Local Classes:**
- Defined within components
- Component-scoped
- Good for component-specific state
- Simple and straightforward

**Global Classes:**
- Defined in `.svelte.js` or `.svelte.ts` files
- Shared across components
- Good for application-wide state
- Export singleton instances

**Key Points:**
1. Classes are **not auto-reactive** in Svelte 5
2. Use `$state` for all reactive properties
3. Wrapping class instances with `$state()` has no effect
4. Use arrow functions for methods passed as callbacks
5. Use getters for computed properties
6. Export singleton instances for global stores

Class stores provide a clean, object-oriented approach to state management in Svelte 5, combining the benefits of classes (encapsulation, methods, computed properties) with Svelte's reactivity system.

