Description: Resizable pane components for Svelte and SvelteKit

Keywords: svelte resizable,svelte panels,svelte panes,svelte resizable panels,svelte 5 resizable,sveltekit resizable,sveltekit panels,sveltekit panes,sveltekit resizable panels,svelte 5 panel components

Description: A draggable handle between two panes that allows the user to resize them.

Keywords: svelte resizable,svelte panels,svelte panes,svelte resizable panels,svelte 5 resizable,sveltekit resizable,sveltekit panels,sveltekit panes,sveltekit resizable panels,svelte 5 panel components

Title: PaneResizer - PaneForge

Skip to main content

- Paneforge

- Introduction
- Getting Started
- Styling
- Releases

Components

- PaneGroup
- PaneResizer
- Pane

Examples

- Collapsible Panes
- Conditional Panes
- Horizontal Groups
- Nested Groups
- Overflowing Panes
- Persistent Layouts
- Reactive Size
- Vertical Groups

On this page

- Usage
- Props
- Data Attributes

# PaneResizer

A draggable handle between two panes that allows the user to resize them.

The `PaneResizer` component is used to create a draggable handle between two panes that allows the user to resize them.

## Usage

```
<script lang="ts">
import { PaneGroup, Pane, PaneResizer } from "svelte-pane";
</script>
<PaneGroup direction="horizontal">
<Pane defaultSize={50}>Pane 1</Pane>
<PaneResizer />
<Pane defaultSize={50}>Pane 2</Pane>
</PaneGroup>
```

## Props

disabled

type: boolean

Whether the resize handle is disabled.

onDraggingChange

type: (isDragging: boolean) => void

A callback that is called when the resize handle's dragging state changes.

ref

type: HTMLElement | null

The underlying DOM element of the pane group. You can `bind` to this prop to get a reference to the element.

## Data Attributes

The following data attributes are available for the `PaneResizer` component:

```
export type PaneResizerAttributes = {
/** The direction of the pane group the handle belongs to. */
"data-direction": "horizontal" | "vertical";
/** The ID of the pane group the handle belongs to. */
"data-pane-group-id": string;
/** Whether the resize handle is active or not. */
"data-active"?: "pointer" | "keyboard";
/** Whether the resize handle is enabled or not. */
"data-enabled"?: boolean;
/** The ID of the resize handle. */
"data-pane-resizer-id": string;
/** Present on all resizer elements */
"data-pane-resizer": "";
};
```
