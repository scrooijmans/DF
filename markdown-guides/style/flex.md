# Responsive Grid Layouts with Tailwind CSS

## Overview

Responsive grid layouts automatically rearrange components based on viewport size, providing optimal display across different screen sizes. This pattern uses Tailwind CSS's responsive grid utilities to create flexible, adaptive layouts.

## How It Works

### Grid Container

The responsive grid is created using Tailwind's `grid` utility combined with responsive column breakpoints:

```svelte
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
  {#each items as item (item.id)}
    <ItemComponent {item} />
  {/each}
</div>
```

### Breakpoint Behavior

- **`grid-cols-1`** (default): Single column on mobile devices (< 640px)
- **`sm:grid-cols-2`**: Two columns on small screens (≥ 640px)
- **`lg:grid-cols-3`**: Three columns on large screens (≥ 1024px)
- **`xl:grid-cols-4`**: Four columns on extra-large screens (≥ 1280px)

### Automatic Rearrangement

As the viewport width changes:
1. **Narrow screens**: Items stack vertically in a single column
2. **Medium screens**: Items flow into 2 columns
3. **Large screens**: Items flow into 3 columns
4. **Extra-large screens**: Items flow into 4 columns

The browser automatically rearranges items to fit the available space, wrapping to new rows as needed.

## Example Implementation

```svelte
<script lang="ts">
  import { STYLE_CONSTANTS } from '$lib/styles/constants';
  import ItemComponent from './item-component.svelte';
  
  const items = $state([...]);
</script>

<div class="flex-1 overflow-y-auto {STYLE_CONSTANTS.SPACING.PADDING.SMALL}">
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 {STYLE_CONSTANTS.SPACING.GAP.SMALL}">
    {#each items as item (item.id)}
      <ItemComponent {item} />
    {/each}
  </div>
</div>
```

## Key Principles

1. **Mobile-First**: Start with single column (`grid-cols-1`), then add columns for larger screens
2. **Consistent Gaps**: Use `STYLE_CONSTANTS.SPACING.GAP.SMALL` for consistent spacing between items
3. **Flexible Items**: Grid items automatically resize to fit available space
4. **No Manual Calculations**: Browser handles all layout calculations automatically

## Benefits

- **Responsive**: Adapts to any screen size automatically
- **Maintainable**: No media queries or JavaScript needed
- **Performance**: CSS-only solution, no runtime calculations
- **Accessible**: Maintains proper reading order and focus flow

