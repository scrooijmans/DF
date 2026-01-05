# Remote Functions in Svelte 5 (SvelteKit)

## Overview

Remote functions are a type-safe mechanism for communication between client and server in SvelteKit. They allow you to call functions from anywhere in your app (including the browser), but these functions always execute on the server. This enables safe access to server-only modules containing environment variables, database clients, and other sensitive resources.

**Key Points:**

- Remote functions are exported from `.remote.js` or `.remote.ts` files
- Must be placed in your `src` directory
- On the client, exported functions are transformed to `fetch` wrappers
- They invoke their server counterparts via generated HTTP endpoints
- Available since SvelteKit version 2.27

## Prerequisites

**Important:** Remote functions require SvelteKit version 2.27 or higher. Check your current version:

```bash
npm list @sveltejs/kit
```

To upgrade:

```bash
npm install @sveltejs/kit@latest
```

**Note for Tauri Apps:** Remote functions are for SvelteKit server-side code. For Tauri apps, you'll continue using `invoke()` to call Rust commands. Remote functions can be used for any SvelteKit server endpoints you might have alongside your Tauri backend.

## Four Types of Remote Functions

### 1. `query` - Data Fetching

Used for fetching data that may change. When called from the browser, it's invoked on the server via a fetch call.

**Basic Example:**

```typescript
import { query } from '$app/server';
import * as db from '$lib/server/database';

export const getPosts = query(async () => {
	const posts = await db.sql`
		SELECT title, slug
		FROM post
		ORDER BY published_at DESC
	`;
	return posts;
});

// With input validation
import * as v from 'valibot';

export const getPost = query<v.StringSchema<undefined>, void>(v.string(), async (slug: string) => {
	return await db.getPost(slug);
});
```

**Using in Components:**

```typescript
import { getPosts } from './data.remote.js';

// In your Svelte component
const posts = await getPosts();

// Queries return objects with methods:
// - .refresh() - Manually refresh the query
// - .invalidate() - Mark as stale
// - .subscribe(fn) - Subscribe to updates (Svelte 5 reactivity)
```

**Query Methods:**

```typescript
// Refresh a query manually
await getPosts().refresh();

// Invalidate (mark as stale, will refetch on next access)
getPosts().invalidate();

// Subscribe to query results (Svelte 5)
const unsubscribe = getPosts().subscribe((data) => {
	console.log('Posts updated:', data);
});
```

### 2. `query.batch` - Batched Queries

Solves the n+1 problem by batching requests that happen within the same macrotask. Simultaneous queries are grouped together.

**Example:**

```typescript
import { query } from '$app/server';

export const getUser = query.batch(async (ids: string[]) => {
	// ids is an array of all arguments from simultaneous calls
	const users = await db.getUsersByIds(ids);

	// Return a function that maps each input to its result
	return (id: string, index: number) => users[index];
});
```

### 3. `command` - Server Actions

Used for mutations and side effects. Similar to form actions but more flexible.

**Example:**

```typescript
import { command } from '$app/server';
import * as v from 'valibot';

export const addLike = command<v.StringSchema<undefined>, Promise<void>>(
	v.string(),
	async (id: string) => {
		await db.sql`UPDATE item SET likes = likes + 1 WHERE id = ${id}`;
	}
);
```

**Using in Components:**

```typescript
import { addLike, getLikes } from './actions.remote.js';

// Basic usage
async function handleLike(id: string) {
	await addLike(id);
	// Refresh data if needed
	getLikes(id).refresh();
}

// Optimistic updates with .updates()
async function handleLikeOptimistic(id: string) {
	try {
		await addLike(id).updates(getLikes(id).withOverride((current: number) => current + 1));
	} catch (error) {
		// Rollback on error
		getLikes(id).refresh();
		showToast('Something went wrong!');
	}
}

// Update multiple queries after a command
async function handleLikeMultiple(id: string) {
	await addLike(id).updates(getLikes(id), getPostStats(id), getUserActivity(id));
}
```

**Command Methods:**

- `.updates(...queries)` - Automatically refresh queries after command completes
- `.withOverride(fn)` - Used with queries for optimistic updates

### 4. `prerender` - Build-Time Data

Similar to `query`, but invoked at build time to prerender results. Use for data that changes at most once per redeployment.

**Example:**

```typescript
import { prerender } from '$app/server';
import * as v from 'valibot';
import { error } from '@sveltejs/kit';
import * as db from '$lib/server/database';

export const getPosts = prerender(async () => {
	return await db.getPosts();
});

export const getPost = prerender<v.StringSchema<undefined>, any>(
	v.string(),
	async (slug: string) => {
		const [post] = await db.sql`
			SELECT * FROM post WHERE slug = ${slug}
		`;
		if (!post) {
			error(404, 'Not found');
		}
		return post;
	},
	{
		// Specify inputs for prerendering
		inputs: () => ['first-post', 'second-post', 'third-post'],
		// Allow dynamic calls (not just prerendered ones)
		dynamic: true
	}
);
```

**Note:** Svelte does not yet support asynchronous server-side rendering, so you'll typically call remote functions from the browser. Use `inputs` to specify which arguments should be prerendered.

### 5. `form` - Form Handling

Remote functions can also be used with forms, providing type-safe form handling. Similar to `command` but designed specifically for form submissions.

**Example:**

```typescript
import { form } from '$app/server';
import * as v from 'valibot';
import { fail } from '@sveltejs/kit';

const schema = v.object({
	title: v.string(),
	content: v.string()
});

export const createPost = form(schema, async ({ title, content }) => {
	// Validate and create post
	if (title.length < 3) {
		return fail(400, { error: 'Title too short' });
	}

	await db.createPost({ title, content });
	return { success: true };
});
```

**Using in Components:**

```svelte
<script>
	import { createPost } from './forms.remote.js';

	let formData = $state({ title: '', content: '' });

	async function handleSubmit() {
		const result = await createPost(formData);
		if (result.success) {
			// Handle success
		}
	}
</script>

<form onsubmit={handleSubmit}>
	<input bind:value={formData.title} />
	<textarea bind:value={formData.content}></textarea>
	<button type="submit">Create Post</button>
</form>
```

### 6. Accessing Request Context

You can access the `RequestEvent` (cookies, headers, etc.) in remote functions using `getRequestEvent()`:

```typescript
import { query } from '$app/server';
import { getRequestEvent } from '@sveltejs/kit';

export const getUser = query(async () => {
	const event = getRequestEvent();
	const sessionId = event?.cookies.get('session');

	if (!sessionId) {
		throw new Error('Not authenticated');
	}

	return await db.getUserBySession(sessionId);
});
```

## Setup and Configuration

### 1. File Structure

Create `.remote.js` or `.remote.ts` files in your `src` directory. You can organize them however you like:

```
src/
  lib/
    server/
      database.ts          # Server-only database code
    api/
      posts.remote.ts      # Remote functions for posts
      users.remote.ts      # Remote functions for users
  routes/
    blog/
      data.remote.ts       # Route-specific remote functions
```

### 2. Install Validation Library (Optional but Recommended)

For input validation, install Valibot or Zod:

```bash
npm install valibot
# or
npm install zod
```

### 3. Create Your First Remote Function

```typescript
// src/lib/api/posts.remote.ts
import { query } from '$app/server';
import * as db from '$lib/server/database';

export const getPosts = query(async () => {
	return await db.getPosts();
});
```

### 4. Use in Components

```svelte
<script lang="ts">
	import { getPosts } from '$lib/api/posts.remote.js';

	const posts = await getPosts();
</script>

{#each posts as post}
	<article>
		<h2>{post.title}</h2>
		<p>{post.content}</p>
	</article>
{/each}
```

## How It Works

1. **File Structure**: Create a `.remote.js` or `.remote.ts` file in your `src` directory
2. **Export Functions**: Export functions wrapped with `query`, `command`, `prerender`, or `form`
3. **Client Transformation**: SvelteKit automatically transforms these exports to `fetch` wrappers on the client
4. **HTTP Endpoints**: SvelteKit generates HTTP endpoints for each remote function (e.g., `/api/remote/posts/getPosts`)
5. **Type Safety**: Full TypeScript support with input/output validation using schemas (Valibot, Zod, etc.)
6. **Automatic Serialization**: Arguments and return values are automatically serialized/deserialized

## Benefits

- **Type Safety**: Full TypeScript support with compile-time type checking
- **Server-Only Access**: Functions can safely access server-only modules (env vars, DB clients)
- **Automatic Serialization**: Arguments and return values are automatically serialized
- **No Manual Endpoints**: No need to manually create API routes
- **Batching Support**: Built-in support for batching queries to solve n+1 problems
- **Prerendering**: Support for build-time data fetching

## Migration from Load Functions and Form Actions

Remote functions can replace:

- **Load functions**: Use `query` or `prerender` instead
- **Form actions**: Use `command` or `form` instead

This provides a more unified and type-safe approach to client-server communication.

## Complete Example: Remote Functions File

```typescript
// src/lib/api/posts.remote.ts
import { query, command, prerender, form } from '$app/server';
import { getRequestEvent } from '@sveltejs/kit';
import { error, fail } from '@sveltejs/kit';
import * as v from 'valibot';
import * as db from '$lib/server/database';

// Query - fetch all posts (no arguments)
export const getPosts = query(async () => {
	return await db.sql`
		SELECT title, slug, published_at, likes
		FROM post
		ORDER BY published_at DESC
	`;
});

// Query with input validation - fetch single post
export const getPost = query(
	v.string(), // Input schema
	async (slug: string) => {
		const [post] = await db.sql`
			SELECT * FROM post WHERE slug = ${slug}
		`;
		if (!post) {
			error(404, 'Post not found');
		}
		return post;
	}
);

// Query with access to request context
export const getMyPosts = query(async () => {
	const event = getRequestEvent();
	const userId = event?.cookies.get('user_id');

	if (!userId) {
		error(401, 'Not authenticated');
	}

	return await db.getPostsByUser(userId);
});

// Batched query - solves n+1 problem
export const getUsers = query.batch(async (ids: string[]) => {
	const users = await db.getUsersByIds(ids);
	// Return a function that maps input to result
	return (id: string, index: number) => users[index];
});

// Command - mutation with optimistic updates support
export const addLike = command(v.string(), async (postId: string) => {
	await db.sql`UPDATE post SET likes = likes + 1 WHERE id = ${postId}`;
});

// Command with multiple query updates
export const deletePost = command(v.string(), async (postId: string) => {
	await db.deletePost(postId);
});

// Form - create post with validation
const createPostSchema = v.object({
	title: v.string([v.minLength(3)]),
	content: v.string([v.minLength(10)]),
	slug: v.string()
});

export const createPost = form(createPostSchema, async ({ title, content, slug }) => {
	// Check if slug exists
	const existing = await db.getPostBySlug(slug);
	if (existing) {
		return fail(400, { error: 'Slug already exists' });
	}

	const post = await db.createPost({ title, content, slug });
	return { success: true, post };
});

// Prerender - static data at build time
export const getFeaturedPosts = prerender(
	async () => {
		return await db.getFeaturedPosts();
	},
	{
		inputs: () => ['featured', 'popular'],
		dynamic: true // Allow dynamic calls in addition to prerendered ones
	}
);
```

## Usage in Svelte Components

```svelte
<script lang="ts">
	import { getPosts, getPost, addLike, createPost } from '$lib/api/posts.remote.js';
	import { onMount } from 'svelte';

	// Load posts on mount
	let posts = $state<any[]>([]);
	let loading = $state(true);

	onMount(async () => {
		posts = await getPosts();
		loading = false;
	});

	// Handle like with optimistic update
	async function handleLike(postId: string) {
		try {
			await addLike(postId).updates(
				getPosts().withOverride((currentPosts) => {
					return currentPosts.map(post =>
						post.id === postId
							? { ...post, likes: post.likes + 1 }
							: post
					);
				})
			);
		} catch (error) {
			// Rollback on error
			await getPosts().refresh();
			alert('Failed to like post');
		}
	}

	// Create new post
	async function handleCreatePost(formData: FormData) {
		const result = await createPost({
			title: formData.get('title') as string,
			content: formData.get('content') as string,
			slug: formData.get('slug') as string
		});

		if (result.success) {
			// Refresh posts list
			posts = await getPosts().refresh();
		}
	}
</script>

{#if loading}
	<p>Loading...</p>
{:else}
	{#each posts as post}
		<article>
			<h2>{post.title}</h2>
			<p>{post.content}</p>
			<button onclick={() => handleLike(post.id)}>
				Like ({post.likes})
			</button>
		</article>
	{/each}
{/if}
```

## Advanced Patterns

### Error Handling

```typescript
import { error } from '@sveltejs/kit';

export const getPost = query(v.string(), async (slug: string) => {
	const post = await db.getPost(slug);
	if (!post) {
		error(404, 'Post not found');
	}
	return post;
});
```

### TypeScript Types

```typescript
import type { RemoteQueryFunction, RemoteCommandFunction } from '@sveltejs/kit';

// Explicit typing
export const getPosts: RemoteQueryFunction<void, Post[]> = query(async () => {
	return await db.getPosts();
});

export const addLike: RemoteCommandFunction<string, void> = command(
	v.string(),
	async (id: string) => {
		await db.addLike(id);
	}
);
```

### Reactive Queries (Svelte 5)

```svelte
<script>
	import { getPosts } from '$lib/api/posts.remote.js';

	// Subscribe to query updates
	const posts = $derived.by(() => {
		const query = getPosts();
		return $state.snapshot(query); // Get current value reactively
	});
</script>
```

## Comparison with Tauri Commands

**Important for Tauri Apps:** Remote functions and Tauri commands serve different purposes:

| Feature         | Remote Functions           | Tauri Commands            |
| --------------- | -------------------------- | ------------------------- |
| **Purpose**     | SvelteKit server-side code | Rust backend in Tauri app |
| **Location**    | SvelteKit server           | Rust codebase             |
| **Access**      | HTTP endpoints             | Direct IPC                |
| **Use Case**    | Web server, API endpoints  | Desktop app backend       |
| **Import**      | `$app/server`              | `@tauri-apps/api/core`    |
| **Call Method** | Direct function call       | `invoke('command_name')`  |

**When to Use Each:**

- **Remote Functions**: If you have a SvelteKit server alongside your Tauri app, or for web deployments
- **Tauri Commands**: For desktop app functionality, file system access, native features

You can use both in the same app - remote functions for server-side web logic, Tauri commands for desktop-specific features.

## Troubleshooting

### "Remote functions are not available"

- Ensure SvelteKit version is 2.27 or higher
- Check that files are named `.remote.js` or `.remote.ts`
- Verify files are in the `src` directory

### Type errors

- Install TypeScript types: `npm install -D @types/node`
- Ensure you're using TypeScript 5.0+
- Check that validation schemas match function signatures

### Functions not found on client

- Ensure functions are exported from `.remote.*` files
- Check that imports use `.js` extension (even for `.ts` files)
- Verify SvelteKit is properly configured

## References

- [SvelteKit Remote Functions Documentation](https://svelte.dev/docs/kit/remote-functions)
- [SvelteKit Documentation](https://kit.svelte.dev)
- Available since SvelteKit 2.27
- Part of Svelte 5 ecosystem
- [Valibot Documentation](https://valibot.dev) - Recommended validation library
