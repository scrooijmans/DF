On this page

# Synchronizing Vertical Charts

In SciChart.js v3.4, you can now synchronize Vertical Charts, enabling grouped zooming, panning, cursors and tooltips as well as synchronized axis sizes when charts are arranged vertically.

![](out_scichartv4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Before reading this topic, it's worth to familiarize yourself withÂ [What is a Vertical Chart](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/) as well as the topic on Synchronizing Multiple Charts

### Recap onÂ Vertical Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/#recap-onvertical-charts" class="hash-link" aria-label="Direct link to Recap onÂ Vertical Charts" translate="no" title="Direct link to Recap onÂ Vertical Charts">â€‹</a>

Vertical Charts are when a 2D cartesian chart is rotated 90 degrees, or transposed, so that series render from top to bottom as opposed to left to right.

In SciChart, vertical charts are implemented by setting the axis.axisAlignment, e.g.

``` prism-code
xAxis.axisAlignment = EAxisAlignment.Top; // or Bottom
yAxis.axisAlignment = EAxisAlignemnt.Left; // or Right
```

This transposes the entire chart including series rendering, tooltips and annotations and resulting in a vertical chart which renders top to bottom.

![](out_scichartv4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Read more about creatingÂ [Vertical Charts here](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis).

### Recap on Synchronizing Multiple Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/#recap-on-synchronizing-multiple-charts" class="hash-link" aria-label="Direct link to Recap on Synchronizing Multiple Charts" translate="no" title="Direct link to Recap on Synchronizing Multiple Charts">â€‹</a>

In SciChart.js, multiple charts may be synchronized to ensure that zooming/panning operations, tooltips or cursors and even axis sizes are synchronized. This allows you to create multi chart pane applications, orÂ complex dashboards which zoom and pan or allow tooltips/cursors in unison.

The method to synchronize multiple charts involves several steps, which are laid out in the pageÂ [Synchronizing Multiple Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts)

## Creating a pair of Vertical Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/#creating-a-pair-of-vertical-charts" class="hash-link" aria-label="Direct link to Creating a pair of Vertical Charts" translate="no" title="Direct link to Creating a pair of Vertical Charts">â€‹</a>

We've created an example showing how to sync vertical charts below. First we will start off with the codepen, then share the code and walk through how it works.

![](out_scichartv4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Zoom and Pan the above chart to get a feel for how synchronized vertical charts work in a group!

First of all some HTML is required to create two vertical charts. For simplicity this has been included with inline styles showing how to arrange the two divs vertically.

- html

``` prism-code
<div id="scichart-root" style="display: flex">
    <div id="chart0Div" style="width: 50%; height: 100%; flex: 1"></div>
    <div id="chart1Div" style="width: 50%; height: 100%; flex: 1"></div>
</div>
```

After that, some code to initialize the two charts can look like this:

- TS

``` prism-code
// Create Vertical Charts for synchronization
const {
    SciChartSurface,
    NumericAxis,
    FastLineRenderableSeries,
    XyDataSeries,
    SciChartJsNavyTheme,
    EAxisAlignment
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const createSciChartSurface = async (divId, isFirstChart) => {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divId, {
        theme: new SciChartJsNavyTheme()
    });
    // Create some deliberate differences between chart 0 and chart 1
    sciChartSurface.background = isFirstChart ? "#22365B" : "#18304A";
    sciChartSurface.canvasBorder = isFirstChart ? { borderRight: 4, color: "#55698E" } : undefined;

    sciChartSurface.xAxes.add(
        new NumericAxis(wasmContext, {
            axisTitle: isFirstChart ? "XAxis 0" : "XAxis 1",
            axisAlignment: EAxisAlignment.Right
        })
    );
    sciChartSurface.yAxes.add(
        new NumericAxis(wasmContext, {
            // Create some deliberate differences between chart 0 and chart 1
            labelPrecision: isFirstChart ? 1 : 4,
            axisTitle: isFirstChart ? "YAxis 0" : "YAxis 1",
            axisAlignment: EAxisAlignment.Top,
            rotation: -90,
            axisBorder: { borderBottom: 3, color: "#55698E" }
        })
    );

    const xValues = [];
    const yValues = [];
    for (let i = 0; i < 100; i++) {
        const coef = isFirstChart ? 1 : 0.5;
        xValues.push(i);
        yValues.push(0.2 * coef * Math.sin((i * 0.1) / coef) - Math.cos(i * 0.01));
    }

    sciChartSurface.renderableSeries.add(
        new FastLineRenderableSeries(wasmContext, {
            // Create some deliberate differences between chart 0 and chart 1
            stroke: isFirstChart ? "#FF6600" : "#3377FF",
            strokeThickness: 5,
            dataSeries: new XyDataSeries(wasmContext, {
                xValues,
                yValues
            })
        })
    );

    return sciChartSurface;
};

// Create two SciChartSurfaces with separate <div> elements in the DOM

// Create the first chart, Expects a <div id="chart0Div"> in the DOM
const sciChartSurface0 = await createSciChartSurface("chart0Div", true);
// Create the second chart, Expects a <div id="chart1Div"> in the DOM
const sciChartSurface1 = await createSciChartSurface("chart1Div", false);
```

This code initializes a SciChartSurface, creates xAxis and yAxis in the configuration to allow a vertical chart, and adds some data and a line series.

The function `createSciChartSurface` creates a single chart, so this is called twice passing in a different `<div>` ID in order to create the two vertical charts.

## Synchronizing Zooming, Panning and Tooltips on Vertical Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/#synchronizing-zooming-panning-and-tooltips-on-vertical-charts" class="hash-link" aria-label="Direct link to Synchronizing Zooming, Panning and Tooltips on Vertical Charts" translate="no" title="Direct link to Synchronizing Zooming, Panning and Tooltips on Vertical Charts">â€‹</a>

To synchronize the two charts, we have to carry out the following steps:

1.  SynchronizeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange" rel="noopener noreferrer" target="_blank">xAxis.visibleRangeðŸ“˜</a> on the two charts by using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerangechanged" rel="noopener noreferrer" target="_blank">AxisCore.visibleRangeChangedðŸ“˜</a> callback
2.  Synchronize chart axis heights using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scicharthorizontalgroup.html" rel="noopener noreferrer" target="_blank">SciChartHorizontalGroupðŸ“˜</a>
3.  Finally, optionally synchronize chart modifiers (Cursor, Tooltips) using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#modifiergroup" rel="noopener noreferrer" target="_blank">modifierGroupðŸ“˜</a>

- Synchronizing Vertical Charts

``` prism-code
const { SciChartHorizontalGroup, ZoomPanModifier, MouseWheelZoomModifier, RolloverModifier } = SciChart;
// or, for npm, import { SciChartHorizontalGroup, ... } from "scichart"

// Step 1: Synchronize the two chart visibleRanges
sciChartSurface0.xAxes.get(0).visibleRangeChanged.subscribe(data1 => {
    sciChartSurface1.xAxes.get(0).visibleRange = data1.visibleRange;
});
sciChartSurface1.xAxes.get(0).visibleRangeChanged.subscribe(data1 => {
    sciChartSurface0.xAxes.get(0).visibleRange = data1.visibleRange;
});

// Step 2: Synchronize the two chart axis sizes using SciChartHorizontalGroup
// this is useful in case the Y-axis have different sizes (heights) due to text formatting
// or visibleRange differences
const horizontalGroup = new SciChartHorizontalGroup();
horizontalGroup.addSurfaceToGroup(sciChartSurface0);
horizontalGroup.addSurfaceToGroup(sciChartSurface1);

// Step 3: Add some cursors, zooming behaviours and link them with a modifier group
// This ensures mouse events on one chart are sent to the other chart
const group0 = "modifierGroup0";
sciChartSurface0.chartModifiers.add(
    new ZoomPanModifier({ modifierGroup: group0 }),
    new MouseWheelZoomModifier({ modifierGroup: group0 }),
    new RolloverModifier({ modifierGroup: group0 })
);
const group1 = "modifierGroup1";
sciChartSurface1.chartModifiers.add(
    new ZoomPanModifier({ modifierGroup: group1 }),
    new MouseWheelZoomModifier({ modifierGroup: group1 }),
    new RolloverModifier({ modifierGroup: group1 })
);
```

## Some Notes on Chart Synchronization with Vertical Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/#some-notes-on-chart-synchronization-with-vertical-charts" class="hash-link" aria-label="Direct link to Some Notes on Chart Synchronization with Vertical Charts" translate="no" title="Direct link to Some Notes on Chart Synchronization with Vertical Charts">â€‹</a>

Adding aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#modifiergroup" rel="noopener noreferrer" target="_blank">modifierGroupðŸ“˜</a>Â to specific chart modifiers will ensure that mouse events from one chart are passed to the other and vice versa. This will actually cause zooming, panning, mousewheel and tooltip/cursor behaviour to occur on all charts (when one chart is interacted with).

However, from an axis range point of view it is far more accurate to synchronizeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange" rel="noopener noreferrer" target="_blank">xAxis.visibleRangeðŸ“˜</a> on the two charts by using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerangechanged" rel="noopener noreferrer" target="_blank">AxisCore.visibleRangeChangedðŸ“˜</a> callback. Mouse events are only accurate to a pixel and some inconsistencies can be built up with synchronized charts unless you also have the visibleRange synchronization.

Adding aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scicharthorizontalgroup.html" rel="noopener noreferrer" target="_blank">SciChartHorizontalGroupðŸ“˜</a>Â ensures that the yAxis sizes on the two charts are exactly the same. This step is optional but in case of differing sizes of the axis it will give a more consistent look. For horizontal chart groups you can use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartverticalgroup.html" rel="noopener noreferrer" target="_blank">SciChartVerticalGroupðŸ“˜</a>Â helper class.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-synchronization-api/synchronizing-vertical-charts/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
