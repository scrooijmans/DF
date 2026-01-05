# Svelte 5 `render_fn is not a function` Error

## Issue

When running the application, the following error occurred:

```
Unhandled Promise Rejection: TypeError: render_fn is not a function. (In 'render_fn(anchor)', 'render_fn' is undefined)

	in <unknown>
	in +page.svelte
	in +layout.svelte
	in +layout.svelte
	in root.svelte
```

## Root Cause

The `children` prop in the root layout (`src/routes/+layout.svelte`) was incorrectly typed as `any` instead of the proper `Snippet` type from Svelte 5.

### Before (Incorrect):

```typescript
let { data, children }: { data: LayoutData; children: any } = $props();
```

### After (Correct):

```typescript
import type { Snippet } from "svelte";
let { data, children }: { data: LayoutData; children: Snippet } = $props();
```

## Explanation

In Svelte 5, when using the `{@render children()}` snippet syntax, the `children` prop **must** be typed as `Snippet` from Svelte. Using `any` or other types causes the Svelte compiler to fail to properly generate the render function, resulting in the `render_fn is not a function` error at runtime.

## Solution

1. **Import the `Snippet` type** from Svelte:

   ```typescript
   import type { Snippet } from "svelte";
   ```

2. **Type the `children` prop correctly**:
   ```typescript
   let { data, children }: { data: LayoutData; children: Snippet } = $props();
   ```

## Files Affected

- `src/routes/+layout.svelte` - Root layout with children prop

## Resolution

✅ Fixed by adding the `Snippet` import and correctly typing the `children` prop.

## Related Issues

This is a common Svelte 5 migration issue when upgrading from Svelte 4 or when not properly typing snippet props.

## Prevention

Always ensure that any prop used with `{@render prop()}` is typed as `Snippet` from Svelte:

```typescript
import type { Snippet } from "svelte";

// Correct typing
let { children }: { children: Snippet } = $props();
```

## Date Fixed

2025-01-28
