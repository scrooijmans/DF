On this page

# Synchronizing Multiple Charts

SciChart.js features a rich set of APIs to synchronize multiple charts. With these APIs you can create dynamic dashboards and chart groups, for example:

- Group charts together with linked behaviours
- Sync chart sizes and axis sizes across chart groups
- Sync tooltips, zooming or panning across chart groups
- Dynamically add or remove chart panes to groups

The <a href="https://www.scichart.com/demo/javascript/sync-multi-chart" rel="noopener noreferrer" target="_blank">JavaScript Sync Multi Chart Example</a> can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

## How to create a Chart Group<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts/#how-to-create-a-chart-group" class="hash-link" aria-label="Direct link to How to create a Chart Group" translate="no" title="Direct link to How to create a Chart Group">â€‹</a>

We've created an example showing how create a synchonrized chart group below. First we will start off with the codepen, then share the code and walk through how it works.

Zoom and Pan the above chart to get a feel for how synchronized charts work in a group!

First of all some HTML is required to create two vertical charts. For simplicity this has been included with inline styles showing how to layout the two charts.

- HTML

``` prism-code
<div id="scichart-root">
  <div id="chart0Div" style="width: 100%; height: 50%"></div>
  <div id="chart1Div" style="width: 100%; height: 50%"></div>
</div>
```

After that, some code to initialize the two charts can look like this:

- TS

``` prism-code
// Create two charts for Synchronization
const { SciChartSurface, NumericAxis, FastLineRenderableSeries, XyDataSeries, SciChartJsNavyTheme } = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const createSciChartSurface = async (divId, isFirstChart) => {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divId, {
        theme: new SciChartJsNavyTheme()
    });
    // Create some deliberate differences between chart 0 and chart 1
    sciChartSurface.background = isFirstChart ? "#22365B" : "#18304A";
    sciChartSurface.canvasBorder = isFirstChart ? { borderBottom: 4, color: "#55698E" } : undefined;

    sciChartSurface.xAxes.add(
        new NumericAxis(wasmContext, {
            axisTitle: isFirstChart ? "XAxis 0" : "XAxis 1"
        })
    );
    sciChartSurface.yAxes.add(
        new NumericAxis(wasmContext, {
            // Create some deliberate differences between chart 0 and chart 1
            labelPrecision: isFirstChart ? 2 : 4,
            axisTitle: isFirstChart ? "YAxis 0" : "YAxis 1"
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

This code initializes a SciChartSurface, creates xAxis and yAxis and adds some data and a line series.

The functionÂ *createSciChartSurface*Â creates a single chart, so this is called twice passing in a different `<div>` ID in order to create the two vertical charts.

## Synchronizing Zooming, Panning and Tooltips on two charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts/#synchronizing-zooming-panning-and-tooltips-on-two-charts" class="hash-link" aria-label="Direct link to Synchronizing Zooming, Panning and Tooltips on two charts" translate="no" title="Direct link to Synchronizing Zooming, Panning and Tooltips on two charts">â€‹</a>

To Synchronize the two charts, we have to carry out the following steps:

1.  SynchronizeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange" rel="noopener noreferrer" target="_blank">xAxis.visibleRangeðŸ“˜</a> on the two charts by using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerangechanged" rel="noopener noreferrer" target="_blank">AxisCore.visibleRangeChangedðŸ“˜</a> callback
2.  Synchronize chart widths using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartverticalgroup.html" rel="noopener noreferrer" target="_blank">SciChartVerticalGroupðŸ“˜</a>
3.  Finally, optionally synchronize chart modifiers (Cursor, Tooltips) using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#modifiergroup" rel="noopener noreferrer" target="_blank">modifierGroupðŸ“˜</a>

- TS

``` prism-code
const { SciChartVerticalGroup, ZoomPanModifier, MouseWheelZoomModifier, RolloverModifier } = SciChart;

// Step1: Synchronize the two chart visibleRanges
sciChartSurface0.xAxes.get(0).visibleRangeChanged.subscribe(data1 => {
    sciChartSurface1.xAxes.get(0).visibleRange = data1.visibleRange;
});
sciChartSurface1.xAxes.get(0).visibleRangeChanged.subscribe(data1 => {
    sciChartSurface0.xAxes.get(0).visibleRange = data1.visibleRange;
});

// Step 2: Synchronize the two chart axis sizes using SciChartVerticalGroup
// this is useful in case the Y-axis have different sizes due to differing visibleRange
// text formatting or size
const verticalGroup = new SciChartVerticalGroup();
verticalGroup.addSurfaceToGroup(sciChartSurface0);
verticalGroup.addSurfaceToGroup(sciChartSurface1);

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

## Some Notes on Chart Synchronization<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts/#some-notes-on-chart-synchronization" class="hash-link" aria-label="Direct link to Some Notes on Chart Synchronization" translate="no" title="Direct link to Some Notes on Chart Synchronization">â€‹</a>

Adding aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html#modifiergroup" rel="noopener noreferrer" target="_blank">modifierGroupðŸ“˜</a>Â to specific chart modifiers will ensure that mouse events from one chart are passed to the other and vice versa. This will actually cause zooming, panning, mousewheel and tooltip/cursor behaviour to occur on all charts (when one chart is interacted with).

However, from an axis range point of view it is far more accurate to synchronizeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange" rel="noopener noreferrer" target="_blank">xAxis.visibleRangeðŸ“˜</a> on the two charts by using aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerangechanged" rel="noopener noreferrer" target="_blank">AxisCore.visibleRangeChangedðŸ“˜</a> callback. Mouse events are only accurate to a pixel and some inconsistencies can be built up with synchronized charts unless you also have the visibleRange synchronization.

Adding aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartverticalgroup.html" rel="noopener noreferrer" target="_blank">SciChartVerticalGroupðŸ“˜</a>Â ensures that the yAxis sizes on the two charts are exactly the same. This step is optional but in case of differing sizes of the axis it will give a more consistent look. For vertical chart groups you can use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scicharthorizontalgroup.html" rel="noopener noreferrer" target="_blank">SciChartHorizontalGroupðŸ“˜</a>Â helper class.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-synchronization-api/synchronizing-multiple-charts/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
