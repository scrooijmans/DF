# Svelte Stores and Streams in SvelteKit

This document explains how Svelte Stores and Streams work together in SvelteKit applications, focusing on state management and progressive data loading patterns.

## Overview

SvelteKit combines two powerful patterns for managing reactive data:
1. **Svelte Stores**: Reactive state management for cross-component data sharing
2. **SvelteKit Streams**: Progressive data loading from server load functions

Together, they enable efficient state management and improved perceived performance through streaming data.

## Svelte Stores

### What Are Stores?

Stores are reactive objects that allow components to subscribe to value changes. They provide a way to share state across components without prop drilling.

### Store Types

#### 1. Writable Stores

Writable stores allow both reading and writing values.

```javascript
import { writable } from 'svelte/store';

// Create a writable store
const count = writable(0);

// Subscribe to changes
const unsubscribe = count.subscribe(value => {
  console.log('Count:', value);
});

// Update the value
count.set(5); // Logs: "Count: 5"
count.update(n => n + 1); // Logs: "Count: 6"

// Cleanup
unsubscribe();
```

**Writable Store with Start/Stop Logic:**

```javascript
import { writable } from 'svelte/store';

// Store with lifecycle management (e.g., WebSocket)
const websocketStore = writable(null, (set, update) => {
  const ws = new WebSocket('wss://example.com/socket');

  ws.addEventListener('message', (event) => {
    set(JSON.parse(event.data));
  });

  ws.addEventListener('open', () => {
    console.log('WebSocket connected');
  });

  // Cleanup function runs when last subscriber unsubscribes
  return () => {
    ws.close();
    console.log('WebSocket disconnected');
  };
});
```

#### 2. Readable Stores

Readable stores allow reading but not direct writing from outside.

```javascript
import { readable } from 'svelte/store';

// Current time store
const time = readable(new Date(), (set) => {
  const interval = setInterval(() => {
    set(new Date());
  }, 1000);

  // Cleanup function
  return () => clearInterval(interval);
});

// Geolocation store
const location = readable(null, (set) => {
  if (!navigator.geolocation) {
    set({ error: 'Geolocation not supported' });
    return;
  }

  const watchId = navigator.geolocation.watchPosition(
    (position) => {
      set({
        latitude: position.coords.latitude,
        longitude: position.coords.longitude,
        accuracy: position.coords.accuracy
      });
    },
    (error) => set({ error: error.message })
  );

  return () => navigator.geolocation.clearWatch(watchId);
});
```

#### 3. Derived Stores

Derived stores compute values from other stores, supporting both synchronous and asynchronous derivations.

**Synchronous Derived Store:**

```javascript
import { writable, derived } from 'svelte/store';

const firstName = writable('Alice');
const lastName = writable('Smith');

// Synchronous derived from multiple stores
const fullName = derived(
  [firstName, lastName],
  ([$first, $last]) => `${$first} ${$last}`
);

// Derived from single store
const count = writable(10);
const doubled = derived(count, $count => $count * 2);
```

**Asynchronous Derived Store:**

```javascript
import { writable, derived } from 'svelte/store';

const fullName = writable('Alice Smith');

// Asynchronous derived with set callback
const user = derived(
  fullName,
  ($fullName, set) => {
    // Set loading state immediately
    set({ loading: true, data: null, error: null });
    
    fetch(`/api/users?name=${encodeURIComponent($fullName)}`)
      .then(r => r.json())
      .then(data => set({ loading: false, data, error: null }))
      .catch(err => set({ loading: false, data: null, error: err.message }));

    // Optional: return cleanup function
    return () => {
      console.log('Cleanup for user derived from', $fullName);
    };
  },
  { loading: true, data: null, error: null } // Initial value
);
```

**Complex Derived Store Example:**

```javascript
import { writable, derived } from 'svelte/store';

const cart = writable([
  { id: 1, name: 'Item 1', price: 10, quantity: 2 },
  { id: 2, name: 'Item 2', price: 15, quantity: 1 }
]);
const taxRate = writable(0.08);

const cartTotal = derived(
  [cart, taxRate],
  ([$cart, $taxRate]) => {
    const subtotal = $cart.reduce((sum, item) =>
      sum + item.price * item.quantity, 0);
    const tax = subtotal * $taxRate;
    return {
      subtotal: parseFloat(subtotal.toFixed(2)),
      tax: parseFloat(tax.toFixed(2)),
      total: parseFloat((subtotal + tax).toFixed(2))
    };
  }
);
```

#### 4. Readonly Stores

Convert a writable store to read-only for controlled access.

```javascript
import { writable, readonly } from 'svelte/store';

// Internal writable store
const _count = writable(0);

// Export read-only version
export const count = readonly(_count);

// Internal function to update
export function increment() {
  _count.update(n => n + 1);
}

// External code can subscribe but not set directly
count.subscribe(value => console.log(value)); // Works
// count.set(5); // ERROR: set is not a function
```

### Using Stores in Components

```svelte
<script>
  import { count } from './stores.js';
  
  // Auto-subscribe with $ prefix
  // Automatically unsubscribes when component is destroyed
</script>

<!-- Reactive: updates when store changes -->
<p>Count: {$count}</p>

<button onclick={() => count.update(n => n + 1)}>
  Increment
</button>
```

## SvelteKit Streams

### What Are Streams in SvelteKit?

SvelteKit streams allow progressive data loading by returning unresolved promises from load functions. This enables the page to render immediately with available data while streaming additional data as it becomes ready.

### Streaming from Load Functions

#### Basic Streaming Pattern

Return unresolved promises from server load functions to stream data:

```javascript
/// file: src/routes/blog/[slug]/+page.server.js

/** @type {import('./$types').PageServerLoad} */
export async function load({ params }) {
  // Return promise without awaiting - streams to client
  return {
    // Essential data - await this first
    post: await loadPost(params.slug),
    
    // Non-essential data - stream this
    // Make sure await happens at the end to prevent waterfalls
    comments: loadComments(params.slug)
  };
}
```

**Important**: Place `await` at the end for dependent data to prevent waterfalls. If you await `comments` before `post`, you can't start loading comments until the post is loaded.

#### Displaying Streamed Data

Use `{#await}` blocks to handle streaming promises:

```svelte
<!-- file: src/routes/blog/[slug]/+page.svelte -->
<script>
  /** @type {import('./$types').PageProps} */
  let { data } = $props();
</script>

<!-- Essential data (already resolved) -->
<h1>{data.post.title}</h1>
<div>{@html data.post.content}</div>

<!-- Streamed data (promise) -->
{#await data.comments}
  <p>Loading comments...</p>
{:then comments}
  {#each comments as comment}
    <article>
      <p>{comment.content}</p>
      <small>By {comment.author}</small>
    </article>
  {/each}
{:catch error}
  <p>Error loading comments: {error.message}</p>
{/await}
```

### Handling Promise Rejections

For manually created promises, add a noop `.catch()` to prevent unhandled rejections:

```javascript
/// file: src/routes/+page.server.js

/** @type {import('./$types').PageServerLoad} */
export function load({ fetch }) {
  // Manual promise - needs catch
  const ok_manual = Promise.reject('Some error');
  ok_manual.catch(() => {}); // Mark as handled

  // SvelteKit fetch - automatically handles rejections
  const ok_fetch = fetch('/fetch/that/could/fail');

  // Dangerous - unhandled rejection will crash server
  // const dangerous_unhandled = Promise.reject();

  return {
    ok_manual,
    ok_fetch
  };
}
```

### Multiple Streams Example

```javascript
/// file: src/routes/product/[id]/+page.server.js

/** @type {import('./$types').PageServerLoad} */
export async function load({ params, fetch }) {
  // Load essential product data first
  const product = await fetch(`/api/products/${params.id}`).then(r => r.json());

  // Stream additional data
  return {
    product,
    // Reviews stream separately
    reviews: fetch(`/api/products/${params.id}/reviews`).then(r => r.json()),
    // Recommendations stream separately
    recommendations: fetch(`/api/products/${params.id}/recommendations`).then(r => r.json()),
    // Related products stream separately
    related: fetch(`/api/products/${params.id}/related`).then(r => r.json())
  };
}
```

```svelte
<!-- file: src/routes/product/[id]/+page.svelte -->
<script>
  /** @type {import('./$types').PageProps} */
  let { data } = $props();
</script>

<!-- Product info (immediate) -->
<h1>{data.product.name}</h1>
<p>{data.product.description}</p>
<p>${data.product.price}</p>

<!-- Reviews (streamed) -->
{#await data.reviews}
  <p>Loading reviews...</p>
{:then reviews}
  <section>
    <h2>Reviews</h2>
    {#each reviews as review}
      <article>
        <h3>{review.title}</h3>
        <p>{review.content}</p>
        <p>Rating: {review.rating}/5</p>
      </article>
    {/each}
  </section>
{:catch error}
  <p>Error loading reviews</p>
{/await}

<!-- Recommendations (streamed) -->
{#await data.recommendations}
  <p>Loading recommendations...</p>
{:then recommendations}
  <section>
    <h2>You might also like</h2>
    {#each recommendations as rec}
      <a href="/product/{rec.id}">{rec.name}</a>
    {/each}
  </section>
{/await}
```

## Combining Stores and Streams

### Pattern 1: Store from Streamed Data

Create a store that updates from streamed load function data:

```javascript
/// file: src/lib/stores/user.js
import { writable, derived } from 'svelte/store';

// User store
export const user = writable(null);

// Derived user name
export const userName = derived(
  user,
  $user => $user ? $user.name : 'Guest'
);
```

```javascript
/// file: src/routes/+layout.server.js
import { user } from '$lib/stores/user';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ fetch }) {
  // Load user data
  const userData = await fetch('/api/user').then(r => r.json());
  
  // Update store (on client side)
  if (typeof window !== 'undefined') {
    user.set(userData);
  }
  
  return {
    user: userData
  };
}
```

### Pattern 2: Store with Streaming Updates

Use a store to manage streaming data state:

```javascript
/// file: src/lib/stores/notifications.js
import { writable } from 'svelte/store';

export const notifications = writable([]);
export const unreadCount = writable(0);

export function addNotification(notification) {
  notifications.update(n => [...n, notification]);
  unreadCount.update(c => c + 1);
}

export function markAsRead(id) {
  notifications.update(n => 
    n.map(notif => notif.id === id ? { ...notif, read: true } : notif)
  );
  unreadCount.update(c => Math.max(0, c - 1));
}
```

```javascript
/// file: src/routes/+layout.server.js
import { notifications } from '$lib/stores/notifications';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ fetch }) {
  // Stream notifications
  return {
    notifications: fetch('/api/notifications').then(r => r.json())
  };
}
```

```svelte
<!-- file: src/routes/+layout.svelte -->
<script>
  import { notifications as notificationsStore } from '$lib/stores/notifications';
  import { onMount } from 'svelte';
  
  /** @type {import('./$types').LayoutData} */
  let { data } = $props();
  
  onMount(() => {
    // When streamed notifications arrive, update store
    data.notifications.then(notifs => {
      notificationsStore.set(notifs);
    });
  });
</script>

<!-- Use store in components -->
<NotificationBell count={$unreadCount} />
```

### Pattern 3: Derived Store from Streamed Data

Create derived stores that react to streamed data:

```javascript
/// file: src/routes/dashboard/+page.server.js

/** @type {import('./$types').PageServerLoad} */
export async function load({ fetch }) {
  // Load essential dashboard data
  const dashboard = await fetch('/api/dashboard').then(r => r.json());
  
  return {
    dashboard,
    // Stream additional data
    recentActivity: fetch('/api/activity/recent').then(r => r.json()),
    analytics: fetch('/api/analytics').then(r => r.json())
  };
}
```

```svelte
<!-- file: src/routes/dashboard/+page.svelte -->
<script>
  import { writable, derived } from 'svelte/store';
  
  /** @type {import('./$types').PageProps} */
  let { data } = $props();
  
  // Create store from streamed data
  const recentActivity = writable([]);
  const analytics = writable(null);
  
  // Update stores when promises resolve
  data.recentActivity.then(activity => recentActivity.set(activity));
  data.analytics.then(analyticsData => analytics.set(analyticsData));
  
  // Derived store from analytics
  const totalViews = derived(
    analytics,
    $analytics => $analytics ? $analytics.totalViews : 0
  );
</script>

<h1>Dashboard</h1>

<!-- Essential data -->
<p>Welcome, {data.dashboard.user.name}!</p>

<!-- Streamed activity -->
{#await data.recentActivity}
  <p>Loading activity...</p>
{:then activity}
  <section>
    <h2>Recent Activity</h2>
    {#each $recentActivity as item}
      <p>{item.description}</p>
    {/each}
  </section>
{/await}

<!-- Streamed analytics -->
{#await data.analytics}
  <p>Loading analytics...</p>
{:then analyticsData}
  <section>
    <h2>Analytics</h2>
    <p>Total Views: {$totalViews}</p>
  </section>
{/await}
```

## Best Practices

### 1. Prioritize Essential Data

Always await essential data first, then stream non-essential data:

```javascript
// ✅ Good: Essential data first
export async function load({ params }) {
  return {
    post: await loadPost(params.slug), // Essential
    comments: loadComments(params.slug) // Non-essential
  };
}

// ❌ Bad: Awaiting non-essential first creates waterfall
export async function load({ params }) {
  return {
    comments: await loadComments(params.slug), // Waits unnecessarily
    post: await loadPost(params.slug)
  };
}
```

### 2. Handle Errors Gracefully

Always provide error handling for streamed promises:

```svelte
{#await data.comments}
  <p>Loading comments...</p>
{:then comments}
  <!-- Display comments -->
{:catch error}
  <p>Error: {error.message}</p>
{/await}
```

### 3. Use Stores for Cross-Component State

Use stores when state needs to be shared across multiple components:

```javascript
// ✅ Good: Store for shared state
export const theme = writable('light');

// ❌ Bad: Prop drilling
// Passing theme through 5 component layers
```

### 4. Combine Stores and Streams Strategically

Use stores for client-side reactive state, streams for server data loading:

```javascript
// Store: Client-side reactive state
export const cart = writable([]);

// Stream: Server data loading
export async function load() {
  return {
    products: await fetch('/api/products').then(r => r.json())
  };
}
```

### 5. Clean Up Subscriptions

Stores automatically clean up when components are destroyed, but be mindful of manual subscriptions:

```javascript
// ✅ Good: Auto-cleanup with $ prefix
$count // Automatically unsubscribes

// ⚠️ Manual subscription needs cleanup
const unsubscribe = count.subscribe(value => {
  console.log(value);
});
// Remember to call unsubscribe() when done
```

## Common Patterns

### Real-Time Data with Stores

```javascript
/// file: src/lib/stores/realtime.js
import { writable } from 'svelte/store';

export const messages = writable([], (set, update) => {
  const ws = new WebSocket('wss://api.example.com/messages');
  
  ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    update(msgs => [...msgs, message]);
  };
  
  return () => ws.close();
});
```

### Loading States with Stores

```javascript
/// file: src/lib/stores/loading.js
import { writable, derived } from 'svelte/store';

export const isLoading = writable(false);
export const loadingMessage = writable('');

export const loadingState = derived(
  [isLoading, loadingMessage],
  ([$isLoading, $message]) => ({
    loading: $isLoading,
    message: $message
  })
);
```

### Form State with Stores

```javascript
/// file: src/lib/stores/form.js
import { writable, derived } from 'svelte/store';

export const formData = writable({
  name: '',
  email: '',
  message: ''
});

export const formErrors = writable({});

export const isFormValid = derived(
  [formData, formErrors],
  ([$data, $errors]) => {
    return $data.name && $data.email && Object.keys($errors).length === 0;
  }
);
```

## Summary

**Svelte Stores** provide:
- Reactive state management
- Cross-component data sharing
- Automatic subscription management
- Derived/computed values
- Lifecycle management (start/stop)

**SvelteKit Streams** provide:
- Progressive data loading
- Improved perceived performance
- Non-blocking data fetching
- Better user experience with skeleton UIs

**Together**, they enable:
- Efficient state management
- Progressive page rendering
- Real-time data updates
- Optimized loading patterns
- Better user experience

The combination of stores and streams in SvelteKit allows developers to build highly reactive, performant applications with excellent user experiences through progressive data loading and efficient state management.

