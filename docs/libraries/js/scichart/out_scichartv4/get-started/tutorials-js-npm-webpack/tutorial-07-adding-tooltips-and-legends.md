On this page

# Tutorial 07 - Adding Tooltips and Legends

In the [previous tutorial](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends) we explained how to add annotations to a JavaScript Chart using SciChart.js. In this tutorial, we are going to show you how to add tooltips and legends.

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Source code for this tutorial can be found at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0/Tutorials/2D_Chart_Tutorials_JavaScript/Tutorial_7_Adding_Tooltips_and_Legends" rel="noopener noreferrer" target="_blank">SciChart.JS.Examples Github Repository</a>

# Ein Fehler ist aufgetreten.

JavaScript kann nicht ausgeführt werden.

Video tutorial for version 3. SciChart.js JavaScript Chart Tutorial 07 - Legends, Tooltips, Crosshairs and Cursors

  

Lets create a simple chart with 5 dataseries each having 10k points.

- index.js region A
- index.html

``` prism-code
import {
  SciChartSurface,
  NumericAxis,
  XyDataSeries,
  FastLineRenderableSeries,
  RolloverModifier,
  LegendModifier,
  CursorModifier,
} from "scichart";

async function initSciChart() {
  // #region ExampleA
  // Create the SciChartSurface in the div 'scichart-root'
  // The SciChartSurface, and webassembly context 'wasmContext' are paired. This wasmContext
  // instance must be passed to other types that exist on the same surface.
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    "scichart-root"
  );

  // Create an X,Y Axis and add to the chart
  sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

  // Create 5 dataseries, each with 10k points
  for (let seriesCount = 0; seriesCount < 5; seriesCount++) {
    const xyDataSeries = new XyDataSeries(wasmContext);

    const opacity = (1 - seriesCount / 5).toFixed(2);

    // Populate with some data
    for (let i = 0; i < 10000; i++) {
      xyDataSeries.append(
        i,
        Math.sin(i * 0.01) * Math.exp(i * (0.00001 * (seriesCount * 10 + 1)))
      );
    }

    // Add and create a line series with this data to the chart
    // Create a line series
    const lineSeries = new FastLineRenderableSeries(wasmContext, {
      dataSeries: xyDataSeries,
      stroke: `rgba(176,196,222,${opacity})`,
      strokeThickness: 2,
    });
    sciChartSurface.renderableSeries.add(lineSeries);
  }
  // #endregion
}

initSciChart();
```

``` prism-code
<html lang="en-us">
    <head>
        <meta charset="utf-8" />
        <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
        <title>SciChart.js Tutorial 7 - Adding Tooltips and Legends</title>
        <script async type="text/javascript" src="bundle.js"></script>
        <style>
            body { font-family: 'Arial'}
        </style>
    </head>
    <body>
        <h1>Hello SciChart.js world!</h1>
        <p>In this example we add a legend and a tooltip behavior.</p>

        <!-- the Div where the SciChartSurface will reside -->
        <div id="scichart-root" style="width: 800px; height: 600px;"></div>
    </body>
</html>
```

So far we have created a new chart, added both X axis and Y axis andÂ plotted 5Â data series.

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/d8755cde1569c5fcf7a5603e4312e0e6b254642e.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

## Add a Legend<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/#add-a-legend" class="hash-link" aria-label="Direct link to Add a Legend" translate="no" title="Direct link to Add a Legend">â€‹</a>

Now we will add a chart legend.Â In SciChart, a chart legend can be created and configured via theÂ [LegendModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/miscellaneous-modifiers/legend-modifier):

- index.js region B

``` prism-code
  // Add a Legend
  sciChartSurface.chartModifiers.add(
    new LegendModifier({ showCheckboxes: true })
  );
```

Â This gives us the **Legend**, which displays checkboxes to show/hide the series, series markers and series names.

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/7d35e279567a450f671f679eba0ec1936bb3e58b.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

## Add a Cursor (Crosshair)<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/#add-a-cursor-crosshair" class="hash-link" aria-label="Direct link to Add a Cursor (Crosshair)" translate="no" title="Direct link to Add a Cursor (Crosshair)">â€‹</a>

[CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview) adds a crosshair onto aÂ [SciChartSurface](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview). When you place the cursor overÂ the SciChartSurface, it shows X and Y values of the current point in tooltips over the Axes.

- index.js region C

``` prism-code
  // Add axis label tooltips using CursorModifier
  const cursorModifier = new CursorModifier({
    axisLabelFill: "black",
    axisLabelStroke: "#00FF00",
    showAxisLabels: true,
    showTooltip: false,
    showXLine: true,
    showYLine: true,
  });
  sciChartSurface.chartModifiers.add(cursorModifier);
```

Â It gives us the result:

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/a8a2ed8279a3e6ea8e891dcea7139502822ea0d0.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

![](out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

To learn more about the CursorModifier, how to add configurable tooltips and how to show/hide or style parts like the crosshair, axis labels, see theÂ [CursorModifier documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview).

## Add a RolloverModifier Tooltip<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/#add-a-rollovermodifier-tooltip" class="hash-link" aria-label="Direct link to Add a RolloverModifier Tooltip" translate="no" title="Direct link to Add a RolloverModifier Tooltip">â€‹</a>

Tooltips may be added to theÂ [SciChartSurface](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview)Â using theÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier). This is aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase.html" rel="noopener noreferrer" target="_blank">ChartModifierBaseðŸ“˜</a>Â derived type which is attached to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#chartmodifiers" rel="noopener noreferrer" target="_blank">SciChartSurface.chartModifiersðŸ“˜</a>Â property.

Remove the previous CursorModifier from the chart. NowÂ add aÂ [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier)Â by add this code:

- index.js region D

``` prism-code
  // Add a tooltip behavior using the RolloverModifier
  const tooltipModifier = new RolloverModifier({
    showTooltip: true,
    showAxisLabel: true,
    showRolloverLine: true,
    isEnabled: false,
    rolloverLineStroke: "#FF6600",
    rolloverLineStrokeThickness: 2,
  });
  sciChartSurface.chartModifiers.add(tooltipModifier);
```

That givesÂ us the chart with tooltips being displayed for each series:Â 

<img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/0ae6643da8e26ebeca27efb9303856e8ee04e587.png" class="img_ev3q" decoding="async" loading="lazy" width="1012" height="996" />

## Further Reading<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/#further-reading" class="hash-link" aria-label="Direct link to Further Reading" translate="no" title="Direct link to Further Reading">â€‹</a>

To learn more aboutÂ cursors and legendsÂ in SciChart please findÂ links to the corresponding documentation articles below:

- [Cursor Modifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier)
- [Rollover Modifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier)
- [Legend Modifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/miscellaneous-modifiers/legend-modifier)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-js-npm-webpack/tutorial-07-adding-tooltips-and-legends/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
