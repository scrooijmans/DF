On this page

# DataPoint Selection

SciChart now features a native ChartModifier called the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" rel="noopener noreferrer" target="_blank">DataPointSelectionModifierðŸ“˜</a> which allows individual data-points to be clicked or selected via the mouse, or programmatically.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" rel="noopener noreferrer" target="_blank">DataPointSelectionModifierðŸ“˜</a> allows you to do two things:

1.  Be notified via the `onSelectionChanged` event when the user clicks to select one or more points.
2.  Change the rendering of the selected points using a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionpaletteprovider.html" rel="noopener noreferrer" target="_blank">PaletteProviderðŸ“˜</a> to change the fill/stroke of the point-marker when selected.

## Enabling Mouse Click Selection on Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#enabling-mouse-click-selection-on-charts" class="hash-link" aria-label="Direct link to Enabling Mouse Click Selection on Charts" translate="no" title="Direct link to Enabling Mouse Click Selection on Charts">â€‹</a>

To make datapoints clickable in SciChart.js and enable the Data-point selection behaviour, you must do the following:

1.  Add a `DataPointSelectionModifier` to the `SciChartSurface.chartModifier` collection
2.  (Optional) Create and add `IPointMetadata` for each data-point you wish to programmatically select. If you do not do this, `DataPointSelectionModifier` will do it for you.
3.  (Optional) Add a `DataPointSelectionPaletteProvider` to series if you want visual feedback on selection. If you do not do this, points will be selected on click but without visual feedback.

This will make your data-points clickable (selectable) via the mouse or tap (touch).

**Find an example below**

- DataPointSelectionModifier

``` prism-code
import {
    SciChartSurface,
    NumericAxis,
    EllipsePointMarker,
    XyDataSeries,
    DataPointSelectionModifier ,
    DataPointSelectionPaletteProvider,
    FastLineRenderableSeries,
    NumberRange 
} from "scichart";

export async function datapointSelectionExample1() {
    const { sciChartSurface, wasmContext } = await SciChartSurface.create("scichart-div-id");

    sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));

    // Create a chart with line series with a point-marker
    sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, {
        stroke: "SteelBlue",
        strokeThickness: 3,
        pointMarker: new EllipsePointMarker(wasmContext, {
            width: 10,
            height: 10,
            strokeThickness: 2,
            stroke: "SteelBlue",
            fill: "LightSteelBlue"
        }),
        dataSeries: new XyDataSeries(wasmContext, {
            xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
        }),
        // Adding the DataPointSelectionPaletteProvider will change the fill/stroke of the pointmarker on selection
        paletteProvider: new DataPointSelectionPaletteProvider({ fill: "white", stroke: "white" })
    }));

    // Add the DatapointSelectionModifier to the chart
    sciChartSurface.chartModifiers.add(new DataPointSelectionModifier());
}
```

<img src="out_scichartv4/2d-charts/chart-modifier-api/selection/data-point-selection/index_media/6fbf583ec25ae306822c46385b06a76a7073a931.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Getting Callbacks on Click Selection of a DataPoint<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#getting-callbacks-on-click-selection-of-a-datapoint" class="hash-link" aria-label="Direct link to Getting Callbacks on Click Selection of a DataPoint" translate="no" title="Direct link to Getting Callbacks on Click Selection of a DataPoint">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html" rel="noopener noreferrer" target="_blank">DataPointSelectionModifierðŸ“˜</a> has an event, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html#selectionchanged" rel="noopener noreferrer" target="_blank">selectionChangedðŸ“˜</a>, which allows you to subscribe to a callback when points are selected or deselected. The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html#onselectionchanged" rel="noopener noreferrer" target="_blank">onSelectionChangedðŸ“˜</a> function may also be passed into the **constructor options**.

Here are two ways you can be notified when the user clicks a datapoint and selection changes in SciChart.

``` prism-code
import { DataPointSelectionModifier } from "scichart";

// Option 1, pass onSelectionChanged callback when creating the DataPointSelectionModifier
sciChartSurface.chartModifiers.add(new DataPointSelectionModifier({
    onSelectionChanged: (args) => {
        console.log(`${args.selectedDataPoints.length} datapoints selected!`);
    }
}));

// Option 2, multiple subscribers can listen to the selectionChanged event as well
const dataPointSelectionModifier = new DataPointSelectionModifier();
dataPointSelectionModifier.selectionChanged.subscribe((args) => {
    console.log(`${args.selectedDataPoints.length} datapoints selected!`);
});
```

![](out_scichartv4/2d-charts/chart-modifier-api/selection/data-point-selection/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For more info about the arguments to the selectionChanged event or onSelectionChanged callback, please see the following items in our TypeDoc documentation.

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html#selectionchanged" rel="noopener noreferrer" target="_blank">DataPointSelectionModifier.selectionChangedðŸ“˜</a> event
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionchangedargs.html" rel="noopener noreferrer" target="_blank">DataPointSelectionChangedArgsðŸ“˜</a> type
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointinfo.html" rel="noopener noreferrer" target="_blank">DataPointInfoðŸ“˜</a> type

## Multi-select, Invert-selection and Replace-selection<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#multi-select-invert-selection-and-replace-selection" class="hash-link" aria-label="Direct link to Multi-select, Invert-selection and Replace-selection" translate="no" title="Direct link to Multi-select, Invert-selection and Replace-selection">â€‹</a>

The `DataPointSelectionModifier` supports multi-selection by holding the **CTRL** key while clicking on datapoints. This option is available when `DataPointSelectionModifier.allowClickSelect = true`.

Holding the **SHIFT** key inverts a selection. Use this to deselect a single point on the chart.

Without CTRL or SHIFT pressed, the default behaviour is to replace a selection, e.g. a new point clicked will replace a previously clicked point.

To customize this behaviour you can pass a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionmodifier.html#getselectionmode" rel="noopener noreferrer" target="_blank">getSelectionModeðŸ“˜</a> function into the constructor options of `DataPointSelectionModifier`, or, override the `getSelectionMode` function. For example:

``` prism-code
import { DataPointSelectionModifier, ESelectionMode, TModifierKeys } from "scichart";

const dataPointSelectionModifier = new DataPointSelectionModifier({
    // Override getSelectionMode behaviour
    getSelectionMode: (modifierKeys, isAreaSelection) => {
        if (modifierKeys.ctrlKey) {
            // Union when area selection and CTRL else Inverse
            return ESelectionMode.Union;
        } else if (modifierKeys.shiftKey) {
            // When shift Inverse
            return ESelectionMode.Inverse;
        }
        // Default mode is Replace
        return ESelectionMode.Replace;
    },
});
```

## Rectangle Select DataPoints<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#rectangle-select-datapoints" class="hash-link" aria-label="Direct link to Rectangle Select DataPoints" translate="no" title="Direct link to Rectangle Select DataPoints">â€‹</a>

Datapoints may be selected by dragging a rectangle on the chart. This option is available when `DataPointSelectionModifier.allowDragSelect = true`.

Drag to Select rectangle can be customised by setting the properties `DataPointSelectionModifier.selectionStroke`, `DatapointSelectionModifier.selectionFill` and `DataPointSelectionModifier.selectionStrokeThickness` properties. This may also be customizable in the themes by setting `IThemeProvider.rubberBandFillBrush` and `IThemeProvider.rubberBandStrokeBrush` properties.

Multi-select behaviour is also configurable via the `getSelectionMode` function.

<img src="out_scichartv4/2d-charts/chart-modifier-api/selection/data-point-selection/index_media/01c1618eb0e6ff2e5c3d984fe80c109297b0515b.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Customizing the Visual of Datapoint Selection<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#customizing-the-visual-of-datapoint-selection" class="hash-link" aria-label="Direct link to Customizing the Visual of Datapoint Selection" translate="no" title="Direct link to Customizing the Visual of Datapoint Selection">â€‹</a>

By default there is no visual feedback that a datapoint has been clicked (is selected or deselected). To add this behaviour, you can add a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionpaletteprovider.html" rel="noopener noreferrer" target="_blank">PaletteProviderðŸ“˜</a> to each series you want to show visual feedback. We've created one out of the box for you to simplify this process.

``` prism-code
// Create a chart with line series with a point-marker
sciChartSurface.renderableSeries.add(new FastLineRenderableSeries(wasmContext, {
    stroke: "SteelBlue",
    strokeThickness: 3,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 10,
        height: 10,
        strokeThickness: 2,
        stroke: "SteelBlue",
        fill: "LightSteelBlue"
    }),
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8]
    }),
    // Adding the DataPointSelectionPaletteProvider will change the fill/stroke of the pointmarker on selection
    paletteProvider: new DataPointSelectionPaletteProvider({ fill: "white", stroke: "white" })
}));
```

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datapointselectionpaletteprovider.html" rel="noopener noreferrer" target="_blank">DataPointSelectionPaletteProviderðŸ“˜</a> checks for `IPointMetadata.isSelected` to return a fill/stroke for point-markers that are selected. Our implementation looks like this. You can either use our default implementation or create your own based on this.

- DataPointSelectionPaletteProvider

``` prism-code
import { TPalletProviderDefinition } from "../../Builder/buildSeries";
import { EPaletteProviderType } from "../../types/PaletteProviderType";
import { parseArgbToHtmlColor, parseColorToUIntArgb } from "../../utils/parseColor";
import { IRenderableSeries } from "../Visuals/RenderableSeries/IRenderableSeries";
import {
    EFillPaletteMode,
    EStrokePaletteMode,
    IFillPaletteProvider,
    IPointMarkerPaletteProvider,
    IStrokePaletteProvider,
    TPointMarkerArgb
} from "./IPaletteProvider";
import { IPointMetadata } from "./IPointMetadata";

export interface ISelectedPointOptions {
    /**
    * The fill of the point-marker as an HTML color code
    */
    fill?: string;
    /**
    * The stroke of the point-marker as an HTML color code
    */
    stroke?: string;
}

export class DataPointSelectionPaletteProvider implements IPointMarkerPaletteProvider {
    public selectedPointMarker: TPointMarkerArgb;
    public selectedStroke: number;
    public selectedFill: number;
    public strokePaletteMode: EStrokePaletteMode = EStrokePaletteMode.SOLID;
    public fillPaletteMode: EFillPaletteMode = EFillPaletteMode.SOLID;
    constructor(options: ISelectedPointOptions) {
        if (options?.stroke) {
            this.selectedStroke = parseColorToUIntArgb(options?.stroke);
        }
        if (options?.fill) {
            this.selectedFill = parseColorToUIntArgb(options?.fill);
        }
        this.selectedPointMarker = { stroke: this.selectedStroke, fill: this.selectedFill };
    }
    public onAttached(parentSeries: IRenderableSeries): void {}
    public onDetached(): void {}
    public overridePointMarkerArgb(
        xValue: number,
        yValue: number,
        index: number,
        opacity?: number,
        metadata?: IPointMetadata
    ): TPointMarkerArgb {
        if (metadata?.isSelected) {
            return this.selectedPointMarker;
        }
        return undefined;
    }
}
```

For more information on how to style data-points, see the [PaletteProvider Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview).

## Programmatically Selecting Points<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#programmatically-selecting-points" class="hash-link" aria-label="Direct link to Programmatically Selecting Points" translate="no" title="Direct link to Programmatically Selecting Points">â€‹</a>

If you want to programmatically select or deselect datapoints in code, you can do this by setting the `IPointMetadata.isSelected` property. After setting this property don't forget to call `sciChartSurface.invalidateElement()` to force a redraw of the chart!

``` prism-code
// Create a DataSeries with x,y values and metadata
const dataSeries = new XyDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    yValues: [4.3, 5.3, 6, 6.3, 6, 5.2, 4.5, 4.6, 5, 6, 7, 8],
    metadata: [
        { isSelected: false }, { isSelected: false }, { isSelected: false },
        { isSelected: false }, { isSelected: false }, { isSelected: false },
        { isSelected: false }, { isSelected: false }, { isSelected: false },
        { isSelected: false }, { isSelected: false }, { isSelected: false }
    ]
});

// Now set isSelected programmatically on some datapoints
dataSeries.getMetadataAt(3).isSelected = true;
dataSeries.getMetadataAt(4).isSelected = true;
```

This code will programmatically set all points to deselected, except for points at index 3 and 4.

![](out_scichartv4/2d-charts/chart-modifier-api/selection/data-point-selection/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For more information on how to manipulate PointMetadata, see the [PointMetadata API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/point-metadata-api-overview).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Series Selection](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/selection/data-point-selection/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/selection/data-point-selection/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
