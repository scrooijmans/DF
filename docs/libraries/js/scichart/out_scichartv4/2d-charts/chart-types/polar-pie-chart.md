On this page

# The Pie Chart (using Polar Columns)

The Polar Pie Chart type is achieved using SciChart's `PolarColumnRenderableSeries` and some data manipulation to mimic pie segments while using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html" rel="noopener noreferrer" target="_blank">SciChartPolarSurfaceðŸ“˜</a> class. It is the native way to create pie charts in SciChart.js, as opposed to using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" rel="noopener noreferrer" target="_blank">SciChartPieSurfaceðŸ“˜</a>

![](out_scichartv4/2d-charts/chart-types/polar-pie-chart/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The Polar Pie Chart is not yet able to support all features a regular pie chart would have, such as certain animation effects or selection behaviors, but it is a useful way to visualize data in a pie format using polar coordinates.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-pie-chart" target="_blank">Polar Pie Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Create a Basic Polar Pie Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-pie-chart/#create-a-basic-polar-pie-chart" class="hash-link" aria-label="Direct link to Create a Basic Polar Pie Chart" translate="no" title="Direct link to Create a Basic Polar Pie Chart">â€‹</a>

To create a Javascript Polar Pie Chart with SciChart.js, use the following code:

``` prism-code
// Demonstrates how to create a basic polar pie chart using SciChart.js
const {
    SciChartPolarSurface,
    PolarNumericAxis,
    PolarColumnRenderableSeries,
    EPolarAxisMode,
    NumberRange,
    XyxDataSeries,
    Thickness,
    EColumnMode,
    MetadataPaletteProvider,
    SciChartJsNavyTheme,
    EColumnDataLabelPosition,
    EPolarLabelMode,
    EMultiLineAlignment
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    padding: Thickness.fromNumber(20),
    theme: new SciChartJsNavyTheme(),
});

const radialYAxis = new PolarNumericAxis(wasmContext, {
    visibleRangeLimit: new NumberRange(0, 1),
    polarAxisMode: EPolarAxisMode.Radial,
    startAngleDegrees: 90, // start at 12 o'clock
    isVisible: false
});
sciChartSurface.yAxes.add(radialYAxis);

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular,
    startAngleDegrees: 90, // start at 12 o'clock
    flippedCoordinates: true, // set to true to go clockwise (biggest values first, starting from 12 o'clock, clockwise)
    isVisible: false
});
sciChartSurface.xAxes.add(angularXAxis);

const DATA = [
    { xValue: 60, label: "First", color: "dodgerblue" },
    { xValue: 50, label: "Second", color: "orangered" },
    { xValue: 40, label: "Third", color: "darkorchid" },
    { xValue: 30, label: "Fourth", color: "salmon" },
    { xValue: 20, label: "Fifth", color: "darkolivegreen" },
    { xValue: 10, label: "Sixth", color: "indianred" }
];

const metadata = [];
const xValues = [];
const x1Values = [];
const yValues = [];
let xSum = 0;

DATA.forEach((item, i) => {
    xValues.push(xSum);
    x1Values.push(item.xValue);
    yValues.push(1);

    xSum += item.xValue;

    metadata.push({
        isSelected: false,
        fill: item.color, // each column will have a different color, handled by the MetadataPaletteProvider
        customLabel: item.label,
        value: item.xValue // add value for optional data-labels
    });
});

const polarColumn = new PolarColumnRenderableSeries(wasmContext, {
    dataSeries: new XyxDataSeries(wasmContext, {
        xValues,
        x1Values,
        yValues,
        metadata
    }),
    stroke: "#110533",
    strokeThickness: 2,
    columnXMode: EColumnMode.StartWidth, // makes columns span from x to x1 value
    paletteProvider: new MetadataPaletteProvider(), // use colors from the metadata for each column value
    dataLabels: {
        metaDataSelector: (metadata: IPointMetadata) => {
            // @ts-ignore
            if (metadata.xValue < 35) { 
                // @ts-ignore
                return metadata.customLabel + ' - ' + metadata.value + '%'; // keep smaller segments' label single-line
            } else {
                // @ts-ignore
                return metadata.customLabel + '\n' + metadata.value + '%';
            }
            // you can avoid the ts-ignore's with a custom point metadata interface casting with all values you'll use.
        },
        style: {
            fontSize: 16,
            multiLineAlignment: EMultiLineAlignment.Center,
            lineSpacing: 12
        },
        color: "#EEE",
        labelYPositionMode: EColumnDataLabelPosition.Inside,
        polarLabelMode: EPolarLabelMode.Perpendicular,
    }
});
sciChartSurface.renderableSeries.add(polarColumn);
```

In the code above:

- We prepare the data from an array of values to 2 arrays: `xValues` and `x1Values`, showing the start and end angles of each "segment", while the `yValues` array is just filled with 1s.
- The `X` and `X1` value arrays along with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html#columnxmode" rel="noopener noreferrer" target="_blank">columnXModeðŸ“˜</a>: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html#setupstart" rel="noopener noreferrer" target="_blank">EColumnMode.StartWidthðŸ“˜</a>, are used to create different-width columns, mimicking pie segments.
- The coloring for each segment must be different, we cannot set "fill", and have columns be the same color. Instead, we use the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html" rel="noopener noreferrer" target="_blank">MetadataPaletteProviderðŸ“˜</a> so that each segment may have its own color.

------------------------------------------------------------------------

## How Does It Work?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-pie-chart/#how-does-it-work" class="hash-link" aria-label="Direct link to How Does It Work?" translate="no" title="Direct link to How Does It Work?">â€‹</a>

- Data is preprocessed into arrays of:
  - `xValues` â€” start angle of each segment (in degrees or radians)
  - `x1Values` â€” end angle of each segment
  - `yValues` â€” (typically all 1) for equal segment radius
  - A metadata array for coloring
- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank"><code>PolarColumnRenderableSeries</code>ðŸ“˜</a> is set to <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecolumnmode.html#startwidth" rel="noopener noreferrer" target="_blank"><code>columnXMode: EColumnXMode.StartWidth</code>ðŸ“˜</a>, which means each column gets its own custom angular width (i.e., pie slice).
- A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html" rel="noopener noreferrer" target="_blank"><code>MetadataPaletteProvider</code>ðŸ“˜</a> is used to apply a unique color to each pie segment.
- Using the metadata, you can als

## Tips & Best Practices<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-pie-chart/#tips--best-practices" class="hash-link" aria-label="Direct link to Tips &amp; Best Practices" translate="no" title="Direct link to Tips &amp; Best Practices">â€‹</a>

- Use the **metadata** array to assign tooltip data, custom colors, labels, or selection state to each segment
- Adjust the **radial axis**â€™ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarnumericaxis.html#innerradius" rel="noopener noreferrer" target="_blank">innerRadiusðŸ“˜</a> for donut/ring-style pie charts
- Set the **start angle** to rotate your entire pie as needed
- Add annotations or labels for segment values or percentage displays if needed
- Combine with other polar series (lines, scatter) for hybrid visualizations!

## Related API and Demos<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-pie-chart/#related-api-and-demos" class="hash-link" aria-label="Direct link to Related API and Demos" translate="no" title="Direct link to Related API and Demos">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarcolumnrenderableseries.html" rel="noopener noreferrer" target="_blank">PolarColumnRenderableSeriesðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/metadatapaletteprovider.html" rel="noopener noreferrer" target="_blank">MetadataPaletteProviderðŸ“˜</a>
- <a href="https://www.scichart.com/demo/react/polar-pie-chart" rel="noopener noreferrer" target="_blank">Polar Column &amp; Pie Demo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" rel="noopener noreferrer" target="_blank">SciChartPieSurface for Classic PieðŸ“˜</a>

The **Polar Pie Chart** is a powerful hybrid, bringing all the flexibility of scientific polar charts to the familiar and insightful pie chart format!

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-pie-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-pie-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
