On this page

# The Treemap Chart Type

Treemap Charts can be within with SciChart.js by using our <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html" rel="noopener noreferrer" target="_blank">FastRectangleRenderableSeriesðŸ“˜</a> series type, with the added calculations of the: position / size / coloring of each rectangle to create a treemap-like appearance.

![](out_scichartv4/2d-charts/chart-types/tree-map-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/javascript-treemap-chart" rel="noopener noreferrer" target="_blank">JavaScript Treemap Chart Example</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/release_v4.0/Examples/src/components/Examples/Charts2D/BasicChartTypes/TreemapChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Treemap Chart</a>Â on Github

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/treemap-chart" target="_blank">Treemap Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

This demo creates an interactive treemap chart visualization to display stock market data where companies are represented as rectangles sized by their market value and colored by their performance percentage.

## Data Structure and Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/#data-structure-and-types" class="hash-link" aria-label="Direct link to Data Structure and Types" translate="no" title="Direct link to Data Structure and Types">â€‹</a>

The code defines several TypeScript interfaces to structure the treemap data:

``` prism-code
type TTreemapDataItem = {
    name: string;           // Company name
    shortName?: string;     // Abbreviated name (e.g., "AAPL")
    parent: string;         // Parent category
    value: number;          // Market value in billions
    progress?: number;      // Percentage gain/loss
};
```

The raw data contains technology companies with their market values and performance metrics, organized hierarchically under a "Technology" parent category.

## Color Management with Palette Provider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/#color-management-with-palette-provider" class="hash-link" aria-label="Direct link to Color Management with Palette Provider" translate="no" title="Direct link to Color Management with Palette Provider">â€‹</a>

The StockTreemapPaletteProvider class implements dynamic coloring based on stock performance:

- Green shades for positive performance (gains)
- Red shades for negative performance (losses)
- Gray for neutral (0% change)

The color intensity corresponds to the magnitude of the percentage change, with interpolation between gray and the target color based on the performance relative to the min/max values in the dataset.

## Dynamic Label Display<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/#dynamic-label-display" class="hash-link" aria-label="Direct link to Dynamic Label Display" translate="no" title="Direct link to Dynamic Label Display">â€‹</a>

The TreemapDataLabelProvider creates adaptive text labels that show different levels of detail based on rectangle size:

- Large rectangles: Full company name + percentage change
- Medium rectangles: Short name + percentage change
- Small rectangles: Just the first letter of the name
- Tiny rectangles: No label at all

This prevents text overcrowding while maximizing information display.

## D3.js Integration for Layout<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/#d3js-integration-for-layout" class="hash-link" aria-label="Direct link to D3.js Integration for Layout" translate="no" title="Direct link to D3.js Integration for Layout">â€‹</a>

The code uses D3.js's hierarchical layout algorithms to calculate rectangle positions and sizes:

``` prism-code
...
import { stratify, treemap } from "d3-hierarchy";
...
function prepareDataUsingD3External(data: TTreemapDataItem[]): RectangluarNode[] {
    const root = stratify()
        .id((d) => (d as TTreemapDataItem).name)
        .parentId((d) => (d as TTreemapDataItem).parent)(data);
    
    root.sum((d) => +(d as TTreemapDataItem).value);
    treemap().size([WIDTH, HEIGHT]).padding(0.1)(root);
    
    return root.leaves() as unknown as RectangluarNode[];
}
```

This transforms the flat data structure into a hierarchical tree and calculates optimal rectangle dimensions using D3's treemap algorithm.

## Chart Configuration and Rendering<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/#chart-configuration-and-rendering" class="hash-link" aria-label="Direct link to Chart Configuration and Rendering" translate="no" title="Direct link to Chart Configuration and Rendering">â€‹</a>

The main drawExample function sets up the SciChart surface with:

- Hidden axes (since treemaps don't need traditional x/y axes)
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastrectanglerenderableseries.html" rel="noopener noreferrer" target="_blank">FastRectangleRenderableSeriesðŸ“˜</a> for high-performance rectangle rendering
- Interactive modifiers for zooming and panning

The chart uses <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" rel="noopener noreferrer" target="_blank">XyxyDataSeriesðŸ“˜</a> to define rectangles with start/end coordinates for both X and Y dimensions, along with metadata for each company's information.

## Interactive Features<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/#interactive-features" class="hash-link" aria-label="Direct link to Interactive Features" translate="no" title="Direct link to Interactive Features">â€‹</a>

The visualization includes several user interaction capabilities:

- Mouse wheel zoom for detailed examination
- Pan functionality to navigate large datasets
- Zoom to extents to reset the view
- Dynamic label scaling that adapts as users zoom in/out

This creates a comprehensive financial data visualization tool that effectively communicates both company size (through rectangle area) and performance (through color coding) in an intuitive, interactive format.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/tree-map-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/tree-map-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
