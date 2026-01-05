Description: Resizable pane components for Svelte and SvelteKit

Keywords: svelte resizable,svelte panels,svelte panes,svelte resizable panels,svelte 5 resizable,sveltekit resizable,sveltekit panels,sveltekit panes,sveltekit resizable panels,svelte 5 panel components

Description: A container for panes or nested pane groups.

Keywords: svelte resizable,svelte panels,svelte panes,svelte resizable panels,svelte 5 resizable,sveltekit resizable,sveltekit panels,sveltekit panes,sveltekit resizable panels,svelte 5 panel components

Title: PaneGroup - PaneForge

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
- Persisted Layouts/Storage
- Data Attributes

# PaneGroup

A container for panes or nested pane groups.

The `PaneGroup` component wraps a collection of panes or nested `PaneGroup`s and is used to initialize and manage the layout of the panes.

## Props

autoSaveId

type: string | null

The id to use when storing the layout of the panes in local storage. If provided, the layout will be saved to local storage when it changes. If not provided, the layout will not be saved.

direction

type: 'horizontal' | 'vertical'

The direction of the panes in the group. If set to `'horizontal'`, the panes will be arranged side by side. If set to `'vertical'`, the panes will be arranged one on top of the other.

required

keyboardResizeBy

type: number | null

The amount of space to add to the pane group when the keyboard resize event is triggered. If not provided, the default value is used.

onLayoutChange

type: (layout: number\[\]) => void | null

A callback that is called when the layout of the panes in the group changes. The layout is an array of numbers representing the size of each pane in pixels.

storage

type: PaneGroupStorage

The storage object to use for saving the layout of the panes in the group.

Show methods

getItem

type: (name: string) => string | null

Retrieves the item from storage.

setItem

type: (name: string, value: string) => void

Sets the item to storage.

ref

type: HTMLElement | null

The underlying DOM element of the pane group. You can `bind` to this prop to get a reference to the element.

this

type: typeof PaneGroup

Imperative API for the pane group. `bind` to this prop to get access to methods for controlling the pane group.

Show methods

getLayout

type: () => number\[\]

Gets the layout of the pane group.

setLayout

type: (newLayout: number\[\]) => void

Sets the layout of the pane group.

getId

type: () => string

Gets the ID of the pane group.

## Persisted Layouts/Storage

When the `PaneGroup` component is provided with an `autoSaveId` prop, it will automatically save the layout of the panes to local storage. If you want to use a different storage mechanism, you can provide a `storage` prop with a custom storage object that implements the `PaneGroupStorage` interface.

```
export type PaneGroupStorage = {
/** Retrieves the item from storage */
getItem(name: string): string | null;
/** Sets the item to storage */
setItem(name: string, value: string): void;
};
```

## Data Attributes

The following data attributes are available for the `PaneGroup` component:

```
export type PaneGroupAttributes = {
/** Applied to every pane group element. */
"data-pane-group": "";
/** The direction of the pane group. */
"data-direction": "horizontal" | "vertical";
/** The ID of the pane group. */
"data-pane-group-id": string;
};
```
