# AG-Grid for Svelte 5

Demo page: [https://bn-l.github.io/ag-grid-svelte5-extended](https://bn-l.github.io/ag-grid-svelte5-extended). The cell with the thermometer icon is a [svelte component](src/routes/BoldCell.svelte).

This builds on [JohnMaher1/ag-grid-svelte5](https://github.com/JohnMaher1/ag-grid-svelte5) with some additional features.

To use to use ag-grid with a framework you need to create an adaptor that follows this interface: [IFrameworkOverrides](https://github.com/ag-grid/ag-grid/blob/424be7dcadf9b964056ee8c451af9b041ce8877a/packages/ag-grid-community/src/interfaces/iFrameworkOverrides.ts#L7). Ag-grid give no documentation on building a framework integration. This is the adaptor code for svelte 5: [src/lib/SvelteFrameworkOverrides.svelte.ts](src/lib/SvelteFrameworkOverrides.svelte.ts)

## Features

- Fully svelte 5
- **Put any svelte component in a grid cell** (see: [cell renderers](https://www.ag-grid.com/javascript-data-grid/component-cell-renderer/) for context)
- Reactive data updates (based on $state, just update the data prop supplied to the table)
- Cell editing (nothing extra to do, will just work like updating the data).
- Reactive [grid options](https://www.ag-grid.com/javascript-data-grid/grid-options/).

## Note

Always provide a [`getRowId`](https://www.ag-grid.com/javascript-data-grid/grid-options/#reference-rowModels-getRowId) function in [`initialOptions`](https://www.ag-grid.com/javascript-data-grid/grid-options/) for optimal performance

## Installation

```bash
npm install ag-grid-svelte5-extended
```

## Usage

Copy and paste this into a svelte file for a very basic grid. (See [demo page source](src/routes/+page.svelte) for more extended example). The base packages (`@ag-grid-community/*`) are dependencies so will be installed along with this lib.

```svelte
<script lang="ts">
    import { AgGrid } from "ag-grid-svelte5-extended";
    import { ClientSideRowModelModule } from "@ag-grid-community/client-side-row-model";
    import { themeQuartz } from "@ag-grid-community/theming";
    import {type GridOptions} from "@ag-grid-community/core"

    interface Person {
        id: string;
        name: string;
        age: number;
    }

    let rowData = $state<Person[]>([
        { id: "1", name: "Jane", age: 25 },
        { id: "2", name: "Jimbo", age: 32 },
        { id: "3", name: "Jensen", age: 41 },
    ]);

    const gridOptions: GridOptions = {
        columnDefs: [
            { field: "name" },
            { field: "age" }
        ],
        getRowId: (params) => params.data.id,
        theme: themeQuartz
    };

    const modules = [ClientSideRowModelModule];
</script>

<div style="height: 200px; width: 640px; margin: 0 auto;">
    <AgGrid {gridOptions} {rowData} {modules} />
</div>
```

## `AgGrid` Component

| Prop               | Type                 | Required | Description                                                    |
| ------------------ | -------------------- | -------- | -------------------------------------------------------------- |
| `gridOptions`      | `GridOptions<TData>` | Yes      | AG-Grid options                                                |
| `rowData`          | `TData[]`            | No       | Array of data to display                                       |
| `modules`          | `Module[]`           | No       | AG-Grid modules to include                                     |
| `gridClass`        | `string`             | No       | CSS class for grid (defaults to "ag-theme-quartz")             |
| `gridStyle`        | `string`             | No       | Inline styles for grid container (defaults to "height: 100%;") |
| `quickFilterText`  | `string`             | No       | Text for quick filtering                                       |
| `sizeColumnsToFit` | `boolean`            | No       | Auto-size columns (default: true)                              |
| `theme`            | `GridTheme`          | No       | AG-Grid theme object                                           |

<br />

## `makeSvelteCellRendererhelper` function

Can be ignored if you just want text in the grid. A utility function to create [AG-Grid cell renderers](https://www.ag-grid.com/javascript-data-grid/component-cell-renderer/) from Svelte components. Takes a svelte component and optionally the class for the container div. It's given [`ICellRendererParams`](<https://www.ag-grid.com/javascript-data-grid/component-cell-renderer/#:~:text=The%20provided%20props%20(interface%20ICellRendererParams)%20are%3A>) as props (with the cell's value as the `value` prop)

```typescript
function makeSvelteCellRenderer(
  Component: Component<ICellRendererParams>,
  containerDivClass?: string,
): ICellRenderer;
```

#### `makeSvelteCellRenderer` Usage

(See [demo page source](src/routes/+page.svelte) for more extended example)

`CustomBoldCell.svelte`:

```svelte
<div class="font-bold">{value}</div>

<script lang = "ts">
    import type { ICellRendererParams } from "@ag-grid-community/core";
    let { value }: ICellRendererParams = $props();
</script>
```

`+page.svelte`:

```svelte
<script lang="ts" >
    import { makeSvelteCellRenderer } from "ag-grid-svelte5-extended";
    import CustomCell from "./CustomCell.svelte";

    const gridOptions = {
        columnDefs: [
            {
                field: "name",
                cellRenderer: makeSvelteCellRenderer(CustomCell)
            }
        ]
    };

    // etc
</script>
```
