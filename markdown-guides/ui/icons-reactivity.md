# Icon Reactivity Best Practices

## Overview

This guide covers best practices for ensuring proper reactivity with Lucide Svelte icons and other interactive elements in Svelte 5.

## ✅ What TO Do

### 1. Use Proper Svelte 5 State Management

```svelte
<script lang="ts">
  import { Download, Loader2 } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';

  // ✅ Use $state for reactive variables
  let isExporting = $state(false);
  let isLoading = $state(false);
  let isDisabled = $state(false);
</script>

<!-- ✅ Use reactive state in templates -->
<Button
  onclick={handleClick}
  disabled={isExporting || isDisabled}
  title={isExporting ? "Exporting..." : "Export file"}
>
  {#if isExporting}
    <Loader2 class="h-4 w-4 animate-spin" />
  {:else}
    <Download class="h-4 w-4" />
  {/if}
</Button>
```

### 2. Use Shadcn-svelte Components for Interactive Elements

```svelte
<!-- ✅ Use Button component instead of plain HTML -->
<Button
  size="sm"
  variant="ghost"
  onclick={handleExportClick}
  disabled={isExporting}
  title={isExporting ? "Exporting..." : "Export file"}
>
  {#if isExporting}
    <Loader2 class="h-4 w-4 animate-spin" />
  {:else}
    <Download class="h-4 w-4" />
  {/if}
</Button>

<!-- ❌ Avoid plain HTML buttons for interactive elements -->
<button onclick={handleClick}>
  <Download class="h-4 w-4" />
</button>
```

### 3. Proper Event Handling

```svelte
<script lang="ts">
  async function handleExportClick(event: Event) {
    // ✅ Prevent event bubbling
    event.stopPropagation();

    // ✅ Check loading state
    if (isExporting) {
      console.log('⚠️ Export already in progress, ignoring click');
      return;
    }

    try {
      isExporting = true;
      // ... perform async operation
    } catch (error) {
      console.error('❌ Export error:', error);
    } finally {
      isExporting = false;
    }
  }
</script>
```

### 4. Use Conditional Rendering for State Changes

```svelte
<!-- ✅ Clear conditional rendering -->
{#if isExporting}
  <Loader2 class="h-4 w-4 animate-spin" />
{:else if isDisabled}
  <Download class="h-4 w-4 opacity-50" />
{:else}
  <Download class="h-4 w-4" />
{/if}
```

### 5. Proper State Updates

```svelte
<script lang="ts">
  // ✅ Update state before async operations
  async function performAction() {
    isExporting = true; // Set immediately

    try {
      await someAsyncOperation();
    } finally {
      isExporting = false; // Always reset
    }
  }
</script>
```

## ❌ What NOT to Do

### 1. Don't Use Plain HTML Buttons

```svelte
<!-- ❌ Plain HTML buttons lack proper event handling -->
<button onclick={handleClick}>
  <Download class="h-4 w-4" />
</button>
```

### 2. Don't Forget Event Propagation

```svelte
<!-- ❌ Missing stopPropagation can cause double events -->
<button onclick={handleClick}>
  <Download class="h-4 w-4" />
</button>
```

### 3. Don't Use let for Reactive State

```svelte
<script lang="ts">
  // ❌ Using let instead of $state in Svelte 5
  let isExporting = false;
  let isLoading = false;
</script>
```

### 4. Don't Forget Loading State Checks

```svelte
<!-- ❌ No protection against multiple clicks -->
<Button onclick={handleClick}>
  <Download class="h-4 w-4" />
</Button>
```

### 5. Don't Mix State Management Patterns

```svelte
<script lang="ts">
  // ❌ Mixing $state and let
  let isExporting = $state(false);
  let isLoading = false; // Should also be $state
</script>
```

## 🔧 Common Issues and Solutions

### Issue: Icon Not Updating

**Problem**: Icon doesn't change when state changes
**Solution**: Ensure you're using `$state` and proper conditional rendering

### Issue: Multiple Event Fires

**Problem**: Click handler fires multiple times
**Solution**: Add `event.stopPropagation()` and loading state checks

### Issue: Button Not Disabled

**Problem**: Button remains clickable during async operations
**Solution**: Use `disabled={isLoading}` and proper state management

### Issue: Visual Feedback Missing

**Problem**: No visual indication of loading state
**Solution**: Use conditional rendering with different icons/states

## 📋 Checklist for Icon Reactivity

- [ ] Use `$state` for reactive variables
- [ ] Use Shadcn-svelte Button component
- [ ] Add `event.stopPropagation()` to prevent bubbling
- [ ] Check loading state before executing actions
- [ ] Use conditional rendering for different states
- [ ] Always reset loading state in `finally` block
- [ ] Provide visual feedback (loading spinner, disabled state)
- [ ] Add proper accessibility attributes (`title`, `aria-label`)

## 🎯 Example: Complete Reactive Icon Implementation

```svelte
<script lang="ts">
  import { Download, Loader2, Check } from '@lucide/svelte';
  import { Button } from '$lib/components/ui/button';

  let isExporting = $state(false);
  let isSuccess = $state(false);

  async function handleExportClick(event: Event) {
    event.stopPropagation();

    if (isExporting) return;

    try {
      isExporting = true;
      isSuccess = false;

      await performExport();

      isSuccess = true;
      setTimeout(() => { isSuccess = false; }, 2000);
    } catch (error) {
      console.error('Export failed:', error);
    } finally {
      isExporting = false;
    }
  }

  function getIcon() {
    if (isSuccess) return Check;
    if (isExporting) return Loader2;
    return Download;
  }

  function getIconClass() {
    if (isSuccess) return "h-4 w-4 text-green-600";
    if (isExporting) return "h-4 w-4 animate-spin";
    return "h-4 w-4";
  }
</script>

<Button
  size="sm"
  variant="ghost"
  onclick={handleExportClick}
  disabled={isExporting}
  title={isExporting ? "Exporting..." : isSuccess ? "Export complete!" : "Export file"}
  aria-label={isExporting ? "Exporting..." : isSuccess ? "Export complete!" : "Export file"}
>
  <svelte:component this={getIcon()} class={getIconClass()} />
</Button>
```

This approach ensures:

- ✅ Proper reactivity with Svelte 5
- ✅ Visual feedback for all states
- ✅ Accessibility compliance
- ✅ Protection against multiple clicks
- ✅ Clean, maintainable code
