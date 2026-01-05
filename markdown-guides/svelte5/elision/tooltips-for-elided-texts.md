# Tooltips for Elided Text in Svelte Components

## Overview

When text is truncated using `STYLE_CONSTANTS.TEXT_TRUNCATE`, we use Shadcn Svelte Tooltip components to display the full text on hover. This ensures users can always see the complete content even when space constraints require text elision.

## Pattern

### Component Structure

```svelte
<script lang="ts">
  import * as Tooltip from '$lib/components/ui/tooltip';
  import { STYLE_CONSTANTS } from '$lib/styles/constants';
</script>

<Tooltip.Provider delayDuration={0}>
  <div class="flex items-center gap-1">
    <div class="flex-1 min-w-0">
      <Tooltip.Root>
        <Tooltip.Trigger class="w-full text-left">
          <div class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} {STYLE_CONSTANTS.TEXT_TRUNCATE}">
            {text}
          </div>
        </Tooltip.Trigger>
        <Tooltip.Content>
          <p>{text}</p>
        </Tooltip.Content>
      </Tooltip.Root>
    </div>
    <!-- Icon or other fixed-width element -->
    <button class="flex-shrink-0 ...">
      <Icon />
    </button>
  </div>
</Tooltip.Provider>
```

### Key Components

1. **Tooltip.Provider**: Wraps the component to provide tooltip context
   - `delayDuration={0}`: Instant tooltip appearance (no delay)

2. **Tooltip.Root**: Container for each tooltip instance
   - One `Tooltip.Root` per elided text element

3. **Tooltip.Trigger**: The element that triggers the tooltip
   - Wraps the truncated text element directly
   - Use `class="w-full text-left"` to make it span full width and align text left
   - The Trigger creates a button element, so style it appropriately

4. **Tooltip.Content**: The tooltip content displayed on hover
   - Contains the full text that was truncated

### Layout Considerations

**Flex Layout Pattern:**

- Container: `flex items-center gap-1` (or `STYLE_CONSTANTS.SPACING.GAP.MIN`)
- Text container: `flex-1 min-w-0` (allows shrinking and truncation)
- Icon/button: `flex-shrink-0` (prevents shrinking)

**Preventing Overlap:**

- When text is next to an icon or button, add `max-w-[calc(100%-Xch)]` to the text container
- `X` should account for icon width + padding + gap + buffer (typically 4-6 characters)
- Example: `max-w-[calc(100%-4ch)]` reserves space for icon and prevents overlap

## Example: content-library-udf-category-item.svelte

```svelte
<Tooltip.Provider delayDuration={0}>
  <div class="flex items-center {STYLE_CONSTANTS.SPACING.GAP.MIN}">
    <div class="flex-1 min-w-0 max-w-[calc(100%-4ch)]">
      <!-- Name with tooltip -->
      <Tooltip.Root>
        <Tooltip.Trigger class="w-full text-left">
          <div class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} font-medium {STYLE_CONSTANTS.COLORS.TEXT.PRIMARY} {STYLE_CONSTANTS.TEXT_TRUNCATE}">
            {udf.name}
          </div>
        </Tooltip.Trigger>
        <Tooltip.Content>
          <p>{udf.name}</p>
        </Tooltip.Content>
      </Tooltip.Root>

      <!-- Description with tooltip -->
      {#if udf.description}
        <Tooltip.Root>
          <Tooltip.Trigger class="w-full text-left">
            <div class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} {STYLE_CONSTANTS.COLORS.TEXT.MUTED} {STYLE_CONSTANTS.TEXT_TRUNCATE}">
              {udf.description}
            </div>
          </Tooltip.Trigger>
          <Tooltip.Content>
            <p>{udf.description}</p>
          </Tooltip.Content>
        </Tooltip.Root>
      {/if}
    </div>

    <!-- Icon button (fixed width) -->
    <button class="flex-shrink-0 {STYLE_CONSTANTS.SPACING.PADDING.MIN} ...">
      <HelpCircle class="h-3 w-3 ..." />
    </button>
  </div>
</Tooltip.Provider>
```

## Rules

1. **Always wrap elided text with Tooltip**: When using `STYLE_CONSTANTS.TEXT_TRUNCATE`, wrap the element with `Tooltip.Root` > `Tooltip.Trigger` > `Tooltip.Content`

2. **Style Trigger appropriately**: Use `class="w-full text-left"` on `Tooltip.Trigger` to make it span full width and align text left

3. **Single Provider per component**: Wrap the entire component in one `Tooltip.Provider` for efficiency

4. **Prevent overlap**: Add `max-w-[calc(100%-Xch)]` to text containers when they're next to fixed-width elements (icons, buttons)

5. **Instant tooltips**: Use `delayDuration={0}` for immediate tooltip appearance

6. **Full text in Content**: Always show the complete original text in `Tooltip.Content`, not a truncated version

## Benefits

- ✅ **Accessibility**: Users can always see full text via hover
- ✅ **Consistent UX**: All elided text behaves the same way
- ✅ **No layout shifts**: Tooltips don't affect layout
- ✅ **Clean design**: Text truncation keeps layouts tight while tooltips provide details on demand
