# TanStack Start vs SvelteKit: Side-by-Side Comparison

This document analyzes the [tanstack-start-vs-sveltekit](https://github.com/bmdavis419/tanstack-start-vs-sveltekit.git) repository, which demonstrates how TanStack Start and SvelteKit can be used to build similar applications with different architectural approaches.

## Repository Overview

The repository contains **three separate implementations** of the same todo application:

1. **TanStack Start** (`tanstack-app/`) - React-based full-stack framework
2. **SvelteKit** (`sveltekit-example/`) - Svelte-based full-stack framework
3. **Next.js** (`nextjs-example/`) - For additional comparison

Both TanStack Start and SvelteKit implementations:

- Use the same todo functionality
- Deploy to Cloudflare Workers
- Use Tailwind CSS for styling
- Implement server-side data fetching
- Support type-safe server-client communication

## Architecture Comparison

### TanStack Start Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              TanStack Start Application                      │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  TanStack Router (File-based routing)                │  │
│  │  - routes/index.tsx                                  │  │
│  │  - routes/__root.tsx                                 │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                          │                                  │
│  ┌───────────────────────▼─────────────────────────────┐  │
│  │  Route Loader (Server-side data fetching)            │  │
│  │  - loader: async () => await getServerTodos()       │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                          │                                  │
│  ┌───────────────────────▼─────────────────────────────┐  │
│  │  Server Functions (createServerFn)                   │  │
│  │  - getServerTodos()                                 │  │
│  │  - addServerTodo()                                  │  │
│  │  - toggleServerTodo()                               │  │
│  └──────────────────────┬──────────────────────────────┘  │
│                          │                                  │
│  ┌───────────────────────▼─────────────────────────────┐  │
│  │  React Component                                     │  │
│  │  - useQuery (TanStack Query)                         │  │
│  │  - useMutation (TanStack Query)                      │  │
│  │  - useServerFn (Client-side server function calls)   │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### SvelteKit Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              SvelteKit Application                          │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  File-based Routing                                   │  │
│  │  - routes/+page.svelte                                │  │
│  │  - routes/+layout.svelte                              │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                          │                                  │
│  ┌───────────────────────▼─────────────────────────────┐  │
│  │  Remote Functions ($app/server)                      │  │
│  │  - query() for data fetching                         │  │
│  │  - command() for mutations                           │  │
│  │  - form() for form submissions                       │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                          │                                  │
│  ┌───────────────────────▼─────────────────────────────┐  │
│  │  Svelte Component                                     │  │
│  │  - $derived for reactive data                        │  │
│  │  - await remote functions                            │  │
│  │  - Form actions with progressive enhancement         │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## File Structure Comparison

### TanStack Start Structure

```
tanstack-app/
├── src/
│   ├── routes/
│   │   ├── __root.tsx          # Root route with layout
│   │   └── index.tsx           # Home page route
│   ├── data/
│   │   └── demo.tq-todos.ts    # Server functions
│   ├── router.tsx              # Router configuration
│   └── integrations/
│       └── tanstack-query/     # TanStack Query setup
├── vite.config.ts              # Vite + TanStack Start plugin
└── package.json
```

### SvelteKit Structure

```
sveltekit-example/
├── src/
│   ├── routes/
│   │   ├── +layout.svelte      # Layout component
│   │   └── +page.svelte        # Home page
│   ├── lib/
│   │   ├── components/
│   │   │   └── Todos.svelte    # Todo component
│   │   └── remote/
│   │       └── todos.remote.ts # Remote functions
│   └── app.html                # HTML template
├── vite.config.ts              # Vite + SvelteKit plugin
└── package.json
```

## Data Fetching Patterns

### TanStack Start: Server Functions + Loaders

**Server Functions** (`src/data/demo.tq-todos.ts`):

```typescript
import { createServerFn } from '@tanstack/react-start';
import z from 'zod';

// GET - Fetch todos
export const getServerTodos = createServerFn({ method: 'GET' }).handler(() => {
	if (todos.length === 0) {
		const defaultTodos = getDefaultTodos();
		todos = [...defaultTodos];
	}
	return { todos };
});

// POST - Add todo with validation
const addTodoInputSchema = z.object({
	name: z.string()
});

export const addServerTodo = createServerFn({ method: 'POST' })
	.inputValidator(addTodoInputSchema)
	.handler(async ({ data }) => {
		const todo = {
			id: crypto.randomUUID(),
			name: data.name,
			completed: false
		};
		todos.push(todo);
		return todo;
	});

// POST - Toggle todo
const toggleTodoInputSchema = z.object({
	id: z.string()
});

export const toggleServerTodo = createServerFn({ method: 'POST' })
	.inputValidator(toggleTodoInputSchema)
	.handler(async ({ data }) => {
		const todo = todos.find((t) => t.id === data.id);
		if (!todo) {
			throw new Error('Todo not found');
		}
		todo.completed = !todo.completed;
		return todo;
	});
```

**Route Loader** (`src/routes/index.tsx`):

```typescript
export const Route = createFileRoute('/')({
	component: TanStackQueryDemo,
	loader: async () => await getServerTodos() // Server-side data fetching
});
```

**Component Usage**:

```typescript
function TanStackQueryDemo() {
	// Get initial data from loader
	const initTodos = Route.useLoaderData();

	// Create client-side server function wrappers
	const getTodos = useServerFn(getServerTodos);
	const addTodo = useServerFn(addServerTodo);
	const toggleTodo = useServerFn(toggleServerTodo);

	// TanStack Query for data management
	const { data, refetch } = useQuery({
		queryKey: ['todos'],
		queryFn: () => getTodos(), // Calls server function
		initialData: initTodos // Uses loader data
	});

	// Mutations with automatic refetch
	const { mutate: mutateAddTodo } = useMutation({
		mutationFn: (todo: string) => addTodo({ data: { name: todo } }),
		onSuccess: () => refetch()
	});

	// ... component JSX
}
```

**Call Flow**:

```
1. User navigates to route
   ↓
2. Route loader executes (server-side)
   - Calls getServerTodos() server function
   - Returns initial data
   ↓
3. Component renders with loader data
   ↓
4. TanStack Query initializes with initialData
   ↓
5. User interactions trigger mutations:
   - mutateAddTodo() → addTodo() server function
   - mutateToggleTodo() → toggleTodo() server function
   ↓
6. Mutations succeed → refetch() updates query
   ↓
7. Component re-renders with new data
```

### SvelteKit: Remote Functions

**Remote Functions** (`src/lib/remote/todos.remote.ts`):

```typescript
import { command, form, query } from '$app/server';
import z from 'zod';

// Query - Fetch todos (reactive, auto-refreshes)
export const remoteGetTodos = query(async () => {
	if (todos.length === 0) {
		const defaultTodos = getDefaultTodos();
		todos = [...defaultTodos];
	}
	return { todos };
});

// Command - Toggle todo with validation
const toggleTodoInputSchema = z.object({
	id: z.string()
});

export const remoteToggleTodo = command(toggleTodoInputSchema, async ({ id }) => {
	const todo = todos.find((t) => t.id === id);
	if (!todo) {
		throw new Error('Todo not found');
	}
	todo.completed = !todo.completed;

	// Manually refresh the query
	await remoteGetTodos().refresh();

	return todo;
});

// Form - Add todo with progressive enhancement
const addTodoInputSchema = z.object({
	name: z.string().min(1, 'name is required').max(100, 'name is too long')
});

export const remoteAddTodo = form(addTodoInputSchema, async ({ name }) => {
	const todo: Todo = {
		id: crypto.randomUUID(),
		name,
		completed: false
	};
	todos.push(todo);

	// Manually refresh the query
	await remoteGetTodos().refresh();

	return todo;
});
```

**Component Usage** (`src/lib/components/Todos.svelte`):

```svelte
<script lang="ts">
  import {
    remoteAddTodo,
    remoteGetTodos,
    remoteResetTodos,
    remoteToggleTodo
  } from '$lib/remote/todos.remote';

  // Reactive query - automatically updates when refreshed
  const todosQuery = $derived(await remoteGetTodos());

  // Derived reactive values
  const totalCount = $derived(todosQuery.todos.length);
  const completedCount = $derived(
    todosQuery.todos.filter((t) => t.completed).length
  );

  // Command handlers
  const fireToggleTodo = async (id: string) => {
    await remoteToggleTodo({ id });
  };

  const fireResetTodos = async () => {
    await remoteResetTodos();
  };
</script>

<!-- Form with progressive enhancement -->
<form class="flex flex-col gap-2" {...remoteAddTodo}>
  <input
    {...remoteAddTodo.fields.name.as('text')}
    placeholder="Enter a new todo..."
  />
  <!-- Validation errors -->
  {#each remoteAddTodo.fields.name.issues() as issue}
    <p class="text-sm text-red-500">{issue.message}</p>
  {/each}
  <button type="submit">Add todo</button>
</form>

<!-- Render todos -->
{#each todosQuery.todos as t}
  <button onclick={() => fireToggleTodo(t.id)}>
    {t.name} - {t.completed ? 'Completed' : 'Not completed'}
  </button>
{/each}
```

**Call Flow**:

```
1. Component loads
   ↓
2. $derived(await remoteGetTodos()) executes
   - Calls remoteGetTodos() query function
   - Returns promise that resolves with todos
   ↓
3. Component renders with todos data
   ↓
4. User interactions:
   a. Form submission → remoteAddTodo() form function
      - Validates input
      - Adds todo
      - Calls remoteGetTodos().refresh()
      - Component reactively updates

   b. Button click → remoteToggleTodo() command function
      - Validates input
      - Toggles todo
      - Calls remoteGetTodos().refresh()
      - Component reactively updates
   ↓
5. Reactive $derived values recalculate
   ↓
6. Component re-renders automatically
```

## API Usage Patterns

### TanStack Start API Patterns

#### 1. Server Functions (`createServerFn`)

**Purpose**: Type-safe RPC functions that run on the server

**Features**:

- HTTP method specification (GET, POST)
- Input validation with Zod
- Type-safe client-server communication
- Automatic serialization

**Example**:

```typescript
// Server function definition
export const getServerTodos = createServerFn({ method: 'GET' }).handler(() => {
	return { todos };
});

// Client-side usage
const getTodos = useServerFn(getServerTodos);
const { data } = useQuery({
	queryKey: ['todos'],
	queryFn: () => getTodos() // Type-safe call
});
```

#### 2. Route Loaders

**Purpose**: Pre-fetch data before component renders

**Features**:

- Server-side execution
- Can call server functions
- Data available via `Route.useLoaderData()`
- Supports SSR

**Example**:

```typescript
export const Route = createFileRoute('/')({
	loader: async () => await getServerTodos(),
	component: MyComponent
});

function MyComponent() {
	const initData = Route.useLoaderData(); // Type-safe
}
```

#### 3. TanStack Query Integration

**Purpose**: Client-side data management and caching

**Features**:

- Automatic caching
- Background refetching
- Optimistic updates
- DevTools support

**Example**:

```typescript
const { data, refetch } = useQuery({
	queryKey: ['todos'],
	queryFn: () => getTodos(),
	initialData: initTodos
});

const { mutate } = useMutation({
	mutationFn: (todo: string) => addTodo({ data: { name: todo } }),
	onSuccess: () => refetch()
});
```

### SvelteKit API Patterns

#### 1. Remote Functions (`$app/server`)

**Purpose**: Type-safe server functions with different semantics

**Types**:

- `query()` - Data fetching with reactivity
- `command()` - Mutations that can refresh queries
- `form()` - Form submissions with progressive enhancement

**Example**:

```typescript
// Query - reactive data fetching
export const remoteGetTodos = query(async () => {
	return { todos };
});

// Command - mutation with validation
export const remoteToggleTodo = command(z.object({ id: z.string() }), async ({ id }) => {
	// ... mutation logic
	await remoteGetTodos().refresh(); // Manual refresh
	return todo;
});

// Form - progressive enhancement
export const remoteAddTodo = form(z.object({ name: z.string() }), async ({ name }) => {
	// ... form logic
	await remoteGetTodos().refresh();
	return todo;
});
```

#### 2. Reactive Data Access

**Purpose**: Automatic reactivity with Svelte's runes

**Features**:

- `$derived` for computed values
- `await` expressions for async data
- Automatic re-rendering on data changes

**Example**:

```svelte
<script>
  const todosQuery = $derived(await remoteGetTodos());
  const totalCount = $derived(todosQuery.todos.length);
</script>

{#each todosQuery.todos as todo}
  {todo.name}
{/each}
```

#### 3. Form Progressive Enhancement

**Purpose**: Forms work without JavaScript

**Features**:

- Server-side validation
- Automatic error display
- Works with JavaScript disabled
- Enhanced with JavaScript enabled

**Example**:

```svelte
<form {...remoteAddTodo}>
  <input {...remoteAddTodo.fields.name.as('text')} />
  {#each remoteAddTodo.fields.name.issues() as issue}
    <p>{issue.message}</p>
  {/each}
  <button type="submit">Submit</button>
</form>
```

## Architectural Design Choices

### TanStack Start Design Choices

#### 1. **Separation: Server Functions vs. Loaders**

**Choice**: Separate server functions from route loaders

**Benefits**:

- **Reusability**: Server functions can be called from multiple places
- **Type safety**: Full TypeScript inference
- **Flexibility**: Loaders for SSR, server functions for RPC

**Tradeoffs**:

- **Complexity**: Two patterns to learn
- **Boilerplate**: More setup required

#### 2. **TanStack Query Integration**

**Choice**: Use TanStack Query for client-side state management

**Benefits**:

- **Caching**: Automatic request deduplication
- **Background sync**: Automatic refetching
- **DevTools**: Excellent debugging experience
- **Optimistic updates**: Better UX

**Tradeoffs**:

- **Bundle size**: Additional dependency
- **Learning curve**: Must understand Query patterns

#### 3. **File-based Routing**

**Choice**: TanStack Router with file-based routes

**Benefits**:

- **Type safety**: Generated route types
- **Code splitting**: Automatic route-based splitting
- **Nested routes**: Complex routing structures

**Tradeoffs**:

- **Convention**: Must follow file naming conventions
- **Generated files**: Route tree generation step

### SvelteKit Design Choices

#### 1. **Unified Remote Functions**

**Choice**: Single API (`query`, `command`, `form`) for all server operations

**Benefits**:

- **Simplicity**: One pattern to learn
- **Semantic clarity**: Function type indicates purpose
- **Progressive enhancement**: Forms work without JS

**Tradeoffs**:

- **Less flexibility**: Fewer customization options
- **Manual refresh**: Must explicitly refresh queries

#### 2. **Reactive by Default**

**Choice**: Svelte's reactivity system with runes

**Benefits**:

- **Automatic updates**: No manual state management
- **Performance**: Fine-grained reactivity
- **Simplicity**: Less boilerplate

**Tradeoffs**:

- **Framework lock-in**: Tied to Svelte
- **Learning curve**: Must understand Svelte reactivity

#### 3. **Progressive Enhancement**

**Choice**: Forms work without JavaScript

**Benefits**:

- **Accessibility**: Works for all users
- **SEO**: Better search engine support
- **Resilience**: Graceful degradation

**Tradeoffs**:

- **Complexity**: Must handle both JS and no-JS cases
- **Testing**: More scenarios to test

## Side-by-Side Comparison

### Data Fetching

| Aspect               | TanStack Start             | SvelteKit                                |
| -------------------- | -------------------------- | ---------------------------------------- |
| **Server Functions** | `createServerFn()`         | `query()`, `command()`, `form()`         |
| **Initial Data**     | Route loaders              | Can use load functions or direct queries |
| **Client State**     | TanStack Query             | Svelte reactivity (`$derived`)           |
| **Caching**          | Automatic (TanStack Query) | Manual refresh required                  |
| **Type Safety**      | Full TypeScript inference  | Full TypeScript inference                |

### Mutations

| Aspect                 | TanStack Start                | SvelteKit                |
| ---------------------- | ----------------------------- | ------------------------ |
| **Pattern**            | `useMutation` + `useServerFn` | `command()` or `form()`  |
| **Validation**         | Zod in server function        | Zod in remote function   |
| **Refetch**            | Automatic via `onSuccess`     | Manual `.refresh()` call |
| **Optimistic Updates** | Built-in (TanStack Query)     | Manual implementation    |

### Forms

| Aspect            | TanStack Start                  | SvelteKit                             |
| ----------------- | ------------------------------- | ------------------------------------- |
| **Pattern**       | Manual form handling            | `form()` with progressive enhancement |
| **Validation**    | Client-side + server validation | Server-side with auto-display         |
| **Error Display** | Manual error handling           | Automatic via `fields.issues()`       |
| **No-JS Support** | Requires manual implementation  | Built-in progressive enhancement      |

### Routing

| Aspect              | TanStack Start        | SvelteKit             |
| ------------------- | --------------------- | --------------------- |
| **Router**          | TanStack Router       | SvelteKit Router      |
| **File Structure**  | `routes/index.tsx`    | `routes/+page.svelte` |
| **Type Generation** | Automatic route types | Automatic route types |
| **Nested Routes**   | Supported             | Supported             |

## Use Cases

### When to Use TanStack Start

1. **React Ecosystem**: Already using React or prefer React
2. **Complex State Management**: Need TanStack Query's advanced features
3. **Large Teams**: Familiar with React patterns
4. **Optimistic Updates**: Need built-in optimistic update support
5. **DevTools**: Want extensive debugging tools

### When to Use SvelteKit

1. **Performance**: Need smaller bundle sizes and faster runtime
2. **Progressive Enhancement**: Forms must work without JavaScript
3. **Simplicity**: Prefer less boilerplate and simpler patterns
4. **Reactivity**: Want fine-grained reactivity without manual management
5. **Developer Experience**: Prefer Svelte's component model

## Integration Possibilities

While these are separate applications in the repository, they could potentially coexist:

### 1. **Micro-frontend Architecture**

```
┌─────────────────────────────────────────────────────────────┐
│              Main Application                                │
│                                                             │
│  ┌──────────────────┐      ┌──────────────────┐          │
│  │  TanStack Start  │      │    SvelteKit      │          │
│  │  (Admin Panel)   │      │  (Public Site)    │          │
│  └──────────────────┘      └──────────────────┘          │
│           │                          │                      │
│           └──────────┬─────────────────┘                      │
│                     │                                        │
│           ┌─────────▼─────────┐                            │
│           │  Shared API       │                            │
│           │  (Backend)        │                            │
│           └───────────────────┘                            │
└─────────────────────────────────────────────────────────────┘
```

### 2. **Shared Server Functions**

Both frameworks could share backend logic:

```typescript
// shared/server/todos.ts
export async function getTodos() {
	// Shared logic
	return { todos };
}

// TanStack Start
export const getServerTodos = createServerFn().handler(() => getTodos());

// SvelteKit
export const remoteGetTodos = query(() => getTodos());
```

### 3. **Monorepo Structure**

```
monorepo/
├── apps/
│   ├── tanstack-app/      # TanStack Start app
│   └── sveltekit-app/     # SvelteKit app
├── packages/
│   ├── shared-types/      # Shared TypeScript types
│   ├── shared-server/    # Shared server logic
│   └── shared-ui/         # Shared UI components
└── package.json
```

## Deployment

Both applications deploy to **Cloudflare Workers**:

### TanStack Start

```json
// wrangler.jsonc
{
	"name": "tanstack-app",
	"compatibility_date": "2024-01-01",
	"pages_build_output_dir": ".output"
}
```

### SvelteKit

```json
// wrangler.jsonc
{
	"name": "sveltekit-example",
	"compatibility_date": "2024-01-01"
}
```

**Deployment Commands**:

```bash
# TanStack Start
cd tanstack-app
bun run build && wrangler deploy

# SvelteKit
cd sveltekit-example
bun run build && wrangler deploy
```

## Summary

**TanStack Start** and **SvelteKit** both provide:

- ✅ Type-safe server-client communication
- ✅ File-based routing
- ✅ Server-side rendering
- ✅ Cloudflare Workers deployment
- ✅ Modern developer experience

**Key Differences**:

1. **Framework**: React vs. Svelte
2. **State Management**: TanStack Query vs. Svelte reactivity
3. **Forms**: Manual vs. Progressive enhancement
4. **Caching**: Automatic vs. Manual refresh
5. **Bundle Size**: Larger (React + Query) vs. Smaller (Svelte)

**Architectural Philosophy**:

- **TanStack Start**: Separation of concerns, explicit state management, powerful tooling
- **SvelteKit**: Simplicity, reactivity by default, progressive enhancement

Both frameworks excel at building full-stack applications with type safety and modern patterns. The choice depends on your team's preferences, existing ecosystem, and specific requirements.
