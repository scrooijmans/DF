On this page

# The SciChartSurface Type

![](out_scichartv4/2d-charts/surface/scichart-surface-type-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Are you new to SciChart? Check out our most popular pages to get started below:

- [SciChart.js Vanilla JavaScript Tutorials](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-01-setting-up-npm-project-with-scichart-js)
- [Series or Chart Types](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview)
- [Axis Types and Configuration](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview)
- [Zooming & Panning](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/overview)
- [Tooltips](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier)
- [Annotations and Markers](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)
- [Styling and Themeing](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api)
- [Chart Synchronization APIs](https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts)
- [3D charts Basics](https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview)

## The basics of the SciChartSurface Type<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview/#the-basics-of-the-scichartsurface-type" class="hash-link" aria-label="Direct link to The basics of the SciChartSurface Type" translate="no" title="Direct link to The basics of the SciChartSurface Type">â€‹</a>

The root 2D chart view is called the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a>. This is the JavaScript chart control you will be adding to your applications wherever you need a chart. You can add more than one SciChartSurface to an HTML page, you can configure them independently, and you can link them together.

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> has a number of properties and functions which allow you to configure and control the chart.

![](out_scichartv4/2d-charts/surface/scichart-surface-type-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Info about the properties and functions available can be found at the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" rel="noopener noreferrer" target="_blank">TypeDoc API Documentation for SciChartðŸ“˜</a>.

## Series or Chart Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview/#series-or-chart-types" class="hash-link" aria-label="Direct link to Series or Chart Types" translate="no" title="Direct link to Series or Chart Types">â€‹</a>

A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> has collections of RenderableSeries (see <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">sciChartSurface.renderableSeriesðŸ“˜</a>), which form the series or chart types on the chart. Each RenderableSeries must have a DataSeries (see [DataSeries types](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview)) which defines the data for that chart type.

Several RenderableSeries types are available in SciChart, including

- [Line Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series)
- [Scatter Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series)
- [Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type)
- [Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series)
- [Band Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series)
- [Candlestick Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series)
- [Ohlc Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series)
- [Polar Line Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-line-renderable-series)
- [Polar Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series) and more.

> For more information about chart types in SciChart, head over to the [RenderableSeries API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/renderable-series-api-overview) or see our <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">Examples</a>.

## Axis and Axis Types<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview/#axis-and-axis-types" class="hash-link" aria-label="Direct link to Axis and Axis Types" translate="no" title="Direct link to Axis and Axis Types">â€‹</a>

A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> has a collection of X-Axis and Y-Axis (see <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#xaxes" rel="noopener noreferrer" target="_blank">sciChartSurface.xAxesðŸ“˜</a> collection). Each SciChartSurface can have unlimited, multiple X and Y Axis. SciChart is unique in that you can place axis on the left, right, top, bottom, but [you can also rotate the chart](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis) to have an XAxis on the left and YAxis on the top. It is this configurability which gives SciChart it's edge over other charting libraries.

A few axis types are available in SciChart, such as [Value Axis and Category Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis). Axis may be configured by setting [gridline interval](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval), gridline styling, titles, labels and more.

> For more information about Axis types and configuration in SciChart, head over to the [Axis API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-api-overview) or see our <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">Examples</a>.

## Annotations and Markers<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview/#annotations-and-markers" class="hash-link" aria-label="Direct link to Annotations and Markers" translate="no" title="Direct link to Annotations and Markers">â€‹</a>

A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurfaceðŸ“˜</a> has a collection of Annotations (see <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html#annotations" rel="noopener noreferrer" target="_blank">sciChartSurface.annotationsðŸ“˜</a> collection). Annotations are markers (text, labels, lines, custom shapes) which can be placed arbitrarily over the chart ([see types of annotation here](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview)), independent of chart types (series) or data. As the chart zooms and pans, the annotations move with the chart, however there is also an <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#xcoordinatemode" rel="noopener noreferrer" target="_blank">xCoordinateModeðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#ycoordinatemode" rel="noopener noreferrer" target="_blank">yCoordinateModeðŸ“˜</a> property on Annotations which allows you to place watermarks, or dock annotations to the left, right, top, bottom or centre of a chart.

> For more information about Annotations types in SciChart, head over to the [Annotations API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview) or see our <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">Examples</a>.

## Interaction - Zooming, Panning and Tooltips<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview/#interaction---zooming-panning-and-tooltips" class="hash-link" aria-label="Direct link to Interaction - Zooming, Panning and Tooltips" translate="no" title="Direct link to Interaction - Zooming, Panning and Tooltips">â€‹</a>

Zooming, Panning, interaction such as Tooltips and Legends are provided by [ChartModifiers](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview). Several modifiers exist out of the box such as the [ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier), [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier) and [MouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier). Tooltips and Legends are also provided by modifiers - see the [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier) and [LegendModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/miscellaneous-modifiers/legend-modifier). You can also create your own modifiers and handle mouse interactions and provide custom behaviours to the chart using this powerful and flexible API.

> For more information about Chart Modifier types in SciChart, head over to the [ChartModifier API documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview) or see our <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">Examples</a>.

## Declaring a SciChartSurface Instance<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview/#declaring-a-scichartsurface-instance" class="hash-link" aria-label="Direct link to Declaring a SciChartSurface Instance" translate="no" title="Direct link to Declaring a SciChartSurface Instance">â€‹</a>

In [Tutorial \#1 - Setting up a Project with SciChart.js](https://www.scichart.com/documentation/js/v4/get-started/tutorials-js-npm-webpack/tutorial-01-setting-up-npm-project-with-scichart-js), we walk you through how to setup a new project with npm, webpack and install SciChart. In particular we show you how to copy the wasm files to the build output, a step that is necessary for our WebAssembly WebGL powered charts.

If you haven't already, go and watch that tutorial quickly so that you can understand the basics of setting up a project with SciChart. The tutorial series is a great way to learn SciChart.

We also have a comprehensive <a href="https://www.scichart.com/demo/react" rel="noopener noreferrer" target="_blank">Examples Suite</a> with many examples of how to use our Fast, JavaScript Charts.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/surface/scichart-surface-type-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/surface/scichart-surface-type-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
