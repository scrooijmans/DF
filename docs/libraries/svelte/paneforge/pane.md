Description: Resizable pane components for Svelte and SvelteKit

Keywords: svelte resizable,svelte panels,svelte panes,svelte resizable panels,svelte 5 resizable,sveltekit resizable,sveltekit panels,sveltekit panes,sveltekit resizable panels,svelte 5 panel components

Description: An individual pane within a pane group.

Keywords: svelte resizable,svelte panels,svelte panes,svelte resizable panels,svelte 5 resizable,sveltekit resizable,sveltekit panels,sveltekit panes,sveltekit resizable panels,svelte 5 panel components

Title: Pane - PaneForge

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

- Props
- Data Attributes

# Pane

An individual pane within a pane group.

The `Pane` component is used to create an individual pane within a `PaneGroup`.

## Props

collapsedSize

type: number

The size of the pane when it is in a collapsed state in percentage of the group's size.

collapsible

type: boolean

Whether the pane can be collapsed.

defaultSize

type: number

The default size of the pane in percentage of the group's size.

maxSize

type: number

The maximum size of the pane in percentage of the group's size.

minSize

type: number

The minimum size of the pane in percentage of the group's size.

order

type: number

The order of the pane in the group. Useful for maintaining order when conditionally rendering panes.

onCollapse

type: () => void

A callback that is called when the pane is collapsed.

onExpand

type: () => void

A callback that is called when the pane is expanded.

onResize

type: (size: number, prevSize: number | undefined) => void

A callback that is called when the pane is resized.

ref

type: HTMLElement | null

The underlying DOM element of the pane group. You can `bind` to this prop to get a reference to the element.

this

type: typeof Pane

Imperative API for the pane group. `bind` to this prop to get access to methods for controlling the pane group.

Show methods

collapse

type: () => void

Collapse the pane to its minimum size.

expand

type: () => void

Expand the pane to its previous size.

getId

type: () => string

Gets the ID of the pane.

getSize

type: () => number

Gets the size of the pane.

isCollapsed

type: () => boolean

Checks if the pane is collapsed.

isExpanded

type: () => boolean

Checks if the pane is expanded.

resize

type: (size: number) => void

Resize the pane to the specified size.

## Data Attributes

The following data attributes are available for the `Pane` component:

```
export type PaneAttributes = {
/** Applied to every pane element. */
"data-pane": "";
/** The ID of the pane. */
"data-pane-id": string;
/** The ID of the pane's group. */
"data-pane-group-id": string;
};
```
