On this page

# Features and Chart Types of SciChart.js

The following page is a comprehensive list of all features in SciChart.js. From v4 onwards, SciChart.js features:

- over 70 2D & 3D chart types including Polar, 2D Charts, 3D Charts and Maps
- specialist chart types for the financial industry, scientific, semiconductor, industrial and medical applications
- has many interaction behaviours such as zoom, pan, selection, and hit-test
- features programmatic zoom, and zoom-changed callbacks
- features unlimited, multiple axis, many axis types, multiple axis layout & formatting options
- has configurable legends, scrollbars, chart synchronization and dashboard layout options
- interactive annotations and overlays,
- supports fast, real-time updates of data, efficient structure of arrays data formats, optional Js objects at X,y data-points (metadata API), data-labels rendered on the chart
- supports per-point colouring (individual data-point colouring) via the PaletteProvider API
- supports full GPU hardware acceleration via WebGL and fast pointer-based arithmetic and memory operations via WebAssembly
- a programmatic API (JavaScript) or optional side-by-side JSON Builder API allowing creation of charts via JSON or JavaScript configuration objects
- serialization/deserialization of charts to JSON
- accessibility
- retina DPI support
- built-in themes, custom themeing and animations APIs
- data filters, render transforms, and polygon/triangle series types for custom drawing
- a React wrapper (scichart-react), Angular wrapper (scichart-angular) and full TypeScript support

For a full list of features in SciChart.js, browse the FAQs below.

## What Chart Types does SciChart.js Support?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-chart-types-does-scichartjs-support" class="hash-link" aria-label="Direct link to What Chart Types does SciChart.js Support?" translate="no" title="Direct link to What Chart Types does SciChart.js Support?">â€‹</a>

**SciChart.js version 4 features over 70 2D & 3D chart types**, including cartesian charts, polar charts, pie/donut charts, radial & linear gauges, heatmaps, JavaScript GeoJSON maps, 3d bubble, 3d scatter, 3d point-cloud, 3d lines, 3d surface plots and more.

Most chart types support dashed line & solid stroke, solid and gradient colour fills, customisable [Text Data Labels](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/) rendered at the X,y value, optional [Metadata](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/point-metadata-api-overview/) (Js Objects at X,y points) and per-point colouring (custom data-point colours) via the [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/).

SciChart.js charts are hardware accelerated using WebGL and WebAssembly for outstanding performance in big-data, financial, scientific and real-time monitoring applications.

A detailed list of all supported chart types with links to documentation / demos can be found below.

### 2D Cartesian Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#2d-cartesian-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to 2D Cartesian Chart Types Supported by SciChart.js" translate="no" title="Direct link to 2D Cartesian Chart Types Supported by SciChart.js">â€‹</a>

#### Line Chart Types Supported<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#line-chart-types-supported" class="hash-link" aria-label="Direct link to Line Chart Types Supported" translate="no" title="Direct link to Line Chart Types Supported">â€‹</a>

1.  [Polyline Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-renderable-series/)
2.  [Line Segment Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-segment-renderable-series/)
3.  [Bezier Spline Line Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-line-renderable-series/)
4.  [Digital/Step Line Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-digital-renderable-series/)

#### Scatter and Bubble Chart Types Supported<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#scatter-and-bubble-chart-types-supported" class="hash-link" aria-label="Direct link to Scatter and Bubble Chart Types Supported" translate="no" title="Direct link to Scatter and Bubble Chart Types Supported">â€‹</a>

5.  [Scatter Charts with configurable point-marker](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/xy-scatter-renderable-series/)
6.  [Bubble Charts with variable size bubbles](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-bubble-renderable-series/)
7.  [Per-point coloured Scatter / Bubble Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-bubble-renderable-series/)

#### Column and Bar Charts Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#column-and-bar-charts-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Column and Bar Charts Supported by SciChart.js" translate="no" title="Direct link to Column and Bar Charts Supported by SciChart.js">â€‹</a>

8.  [Column / Bar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-column-renderable-series/column-series-type/)
9.  <a href="https://www.scichart.com/demo/react/histogram-chart" rel="noopener noreferrer" target="_blank">Histogram Charts</a>
10. [Rectangle Range Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-rectangle-renderable-series/)
11. <a href="https://www.scichart.com/demo/react/animated-columns" rel="noopener noreferrer" target="_blank">Horizontal Bar Charts</a>
12. [Stacked Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-column-renderable-series/)
13. [100% Stacked Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-column-renderable-series/)
14. [Grouped Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-column-renderable-series/)

#### Mountain and Area Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#mountain-and-area-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Mountain and Area Chart Types Supported by SciChart.js" translate="no" title="Direct link to Mountain and Area Chart Types Supported by SciChart.js">â€‹</a>

15. [Mountain (Area) Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-mountain-area-renderable-series/)
16. [Spline Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-mountain-renderable-series/)
17. [Stacked Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/)
18. [Smoothed (Spline) Stacked Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/smooth-stacked-mountain-renderable-series/)
19. [100% Stacked Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/stacked-mountain-renderable-series/)

#### Statistical Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#statistical-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Statistical Chart Types Supported by SciChart.js" translate="no" title="Direct link to Statistical Chart Types Supported by SciChart.js">â€‹</a>

20. [Box Plots](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-box-plot-renderable-series/)
21. [Vertical Error Bars](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/)
22. [Horizontal Error Bars](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-error-bars-renderable-series/)
23. <a href="https://www.scichart.com/demo/react/histogram-chart" rel="noopener noreferrer" target="_blank">Histogram Charts</a>

#### Misc Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#misc-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Misc Chart Types Supported by SciChart.js" translate="no" title="Direct link to Misc Chart Types Supported by SciChart.js">â€‹</a>

24. [Heatmap Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type/) with optional [HeatmapColorMap](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/) legend
25. [Non-uniform Heatmap Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/non-uniform-heatmap-renderable-series/) with optional [HeatmapColorMap](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/) legend
26. [Contours Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-contours-renderable-series/)
27. [Band Charts or High-Low Area Fill Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series/)
28. [Spline Band Charts / Area Fill](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/spline-band-renderable-series/)
29. [Fan Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fan-charts-type/)
30. [Text / Word Cloud Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-text-renderable-series/)
31. [Impulse, Lollipop or Stem Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-impulse-renderable-series/)
32. <a href="https://www.scichart.com/demo/react/population-pyramid" rel="noopener noreferrer" target="_blank">Population Pyramid Charts</a>
33. <a href="https://www.scichart.com/demo/react/chart-background-annotations" rel="noopener noreferrer" target="_blank">Quadrant Charts</a>
34. <a href="https://www.scichart.com/demo/react/waterfall-chart" rel="noopener noreferrer" target="_blank">Waterfall Charts</a>
35. <a href="https://www.scichart.com/demo/react/vector-field" rel="noopener noreferrer" target="_blank">Vector Field Charts</a>
36. [Treemap Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/tree-map-type/)
37. <a href="https://www.scichart.com/demo/react/gantt-chart" rel="noopener noreferrer" target="_blank">Gantt Charts</a>
38. [Rectangle Series Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-rectangle-renderable-series/)
39. [Triangle Series Charts (Polygons)](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-triangle-renderable-series/)

#### Financial Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#financial-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Financial Chart Types Supported by SciChart.js" translate="no" title="Direct link to Financial Chart Types Supported by SciChart.js">â€‹</a>

40. [Candlestick Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-candlestick-renderable-series/)
41. [OHLC Bars Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series/)
42. [Volume Bars Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-ohlc-renderable-series/)
43. <a href="https://www.scichart.com/blog/how-to-create-a-volume-profile-in-a-javascript-financial-chart/" rel="noopener noreferrer" target="_blank">Market Profile / Volume Profile Chart</a>
44. <a href="https://www.scichart.com/demo/react/depth-chart" rel="noopener noreferrer" target="_blank">Market Depth Chart</a>

#### Pie / Donut Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#pie--donut-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Pie / Donut Chart Types Supported by SciChart.js" translate="no" title="Direct link to Pie / Donut Chart Types Supported by SciChart.js">â€‹</a>

45. [Pie Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/pie-chart-type/)
46. [Donut Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/donut-chart-type/)

#### Gauges Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#gauges-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Gauges Supported by SciChart.js" translate="no" title="Direct link to Gauges Supported by SciChart.js">â€‹</a>

47. <a href="https://www.scichart.com/demo/react/linear-gauges" rel="noopener noreferrer" target="_blank">Linear Gauges</a>
48. [Radial Gauges](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-partial-chart-type/#1-gauge-charts)

### 2D Polar or Radial Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#2d-polar-or-radial-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to 2D Polar or Radial Chart Types Supported by SciChart.js" translate="no" title="Direct link to 2D Polar or Radial Chart Types Supported by SciChart.js">â€‹</a>

49. [Polar Line Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-line-renderable-series/)
50. <a href="https://www.scichart.com/demo/react/polar-spline-line-chart" rel="noopener noreferrer" target="_blank">Polar Spline Line Charts</a>
51. [Polar Column Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/)
52. <a href="https://www.scichart.com/demo/react/polar-reange-column-chart" rel="noopener noreferrer" target="_blank">Polar Range Column Chart</a>
53. [Windrose Chart or Stacked Polar Column Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-column-renderable-series/)
54. [Radial Sunburst Chart](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-sunburst-chart/)
55. [Radial Column Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-column-renderable-series/#polar-radial-polar-column-series)
56. <a href="https://www.scichart.com/demo/react/polar-stacked-radial-column-chart" rel="noopener noreferrer" target="_blank">Radial Stacked Column Charts</a>
57. [Polar Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-mountain-renderable-series/)
58. [Polar Stacked Mountain Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-stacked-mountain-renderable-series/)
59. [Polar Band (High Low Fill) Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-band-renderable-series/)
60. [Polar Scatter Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-xy-scatter-renderable-series/)
61. [Radar Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-radar-chart/)
62. [Polar Heatmap Charts](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/)
63. <a href="https://www.scichart.com/demo/react/polar-map-example" rel="noopener noreferrer" target="_blank">Polar Map Chart</a>
64. <a href="https://www.scichart.com/demo/react/phasor-diagram-chart" rel="noopener noreferrer" target="_blank">Phasor Diagrams</a>

### 2D Maps Charts Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#2d-maps-charts-supported-by-scichartjs" class="hash-link" aria-label="Direct link to 2D Maps Charts Supported by SciChart.js" translate="no" title="Direct link to 2D Maps Charts Supported by SciChart.js">â€‹</a>

65. <a href="https://www.scichart.com/demo/react/map-example" rel="noopener noreferrer" target="_blank">Chloropleth Maps</a>
66. <a href="https://www.scichart.com/demo/react/multi-map-example" rel="noopener noreferrer" target="_blank">GeoJson Maps</a>

### 3D Chart Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#3d-chart-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to 3D Chart Types Supported by SciChart.js" translate="no" title="Direct link to 3D Chart Types Supported by SciChart.js">â€‹</a>

67. [3D Bubble Chart](https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/bubble-3d-chart/)
68. [3D Surface Mesh (Surface Plots) Chart](https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/surface-mesh-3d/) with optional [HeatmapColorMap](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/) legend
69. [3D Point Line Charts](https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/lines-3d-chart/)
70. <a href="https://www.scichart.com/demo/react/3d-lidar-visualization" rel="noopener noreferrer" target="_blank">3D Point Cloud Charts</a>
71. [3D Scatter Charts](https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/scatter-3d-chart/)
72. [3D Column / Bar Charts](https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/column-3d-chart/)

## What Axis Types does SciChart.js Support?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-axis-types-does-scichartjs-support" class="hash-link" aria-label="Direct link to What Axis Types does SciChart.js Support?" translate="no" title="Direct link to What Axis Types does SciChart.js Support?">â€‹</a>

SciChart.js features 10 Axis types including cartesian value axis ([NumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis/)) date axis ([DateTimeNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/)), Category Axis ([CategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis/)), polar (radial) axis ([PolarNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis/), [PolarCategoryAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis/)), solutions for [Text Label axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/) and non-linear axis types such [LogarithmicAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/). A full list of axis types and links to respective documentation pages can be found below:

### Linear Axis Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#linear-axis-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Linear Axis Types Supported by SciChart.js" translate="no" title="Direct link to Linear Axis Types Supported by SciChart.js">â€‹</a>

1.  [Linear Numeric Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/numeric-axis/)
2.  [Linear Date Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/)
3.  [Category Numeric Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis/)
4.  [Category Date Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis/)
5.  [Text / String Labels Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/text-string-axis/)
6.  (3D Charts) [Linear Numeric 3D Axis](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/numeric-and-date-axis-in-scichart-3d/)
7.  (3D Charts) [Linear Numeric 3D Date Axis](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/numeric-and-date-axis-in-scichart-3d/)

### Logarithmic Axis Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#logarithmic-axis-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Logarithmic Axis Types Supported by SciChart.js" translate="no" title="Direct link to Logarithmic Axis Types Supported by SciChart.js">â€‹</a>

8.  [Logarithmic Numeric Axis (Log2, LogE, Log10)](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/logarithmic-axis/)

### Radial (Polar) Axis Types Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#radial-polar-axis-types-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Radial (Polar) Axis Types Supported by SciChart.js" translate="no" title="Direct link to Radial (Polar) Axis Types Supported by SciChart.js">â€‹</a>

9.  [Polar Numeric Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-numeric-axis/)
10. [Polar Category Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/polar-category-axis/)

### What Axis Configuration Options and Axis Layout Options does SciChart.js Support?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-axis-configuration-options-and-axis-layout-options-does-scichartjs-support" class="hash-link" aria-label="Direct link to What Axis Configuration Options and Axis Layout Options does SciChart.js Support?" translate="no" title="Direct link to What Axis Configuration Options and Axis Layout Options does SciChart.js Support?">â€‹</a>

Axis in SciChart are infinitely configurable for everything from labels to layout and more. Axes support label formatting ([NumericFormats](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/numeric-formats/)), date formatting ([DateTimeNumericAxis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/date-time-numeric-axis/)), customisation of label values and decimal places ([LabelProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/)), [image labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/image-labels/), fine-grained gridline/label interval or spacing customisation ([Gridline Spacing](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/gridline-and-label-spacing-interval/), [TickProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-tick-label-interval/tick-provider-api/)), dynamic label formatting on zoom ([Custom LabelProviders](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/custom-label-providers-readable-numbers/), alignment or docking of the axis on the left, right, top, bottom, unlimited multiple X and Y axis ([Multiple Axis docs](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/secondary-and-multiple-axis-overview/), [rotating axis labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/rotating-axis-labels/) and [Multi Line Axis Labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/text-and-multi-line-labels/)).

Axis layout options include [Vertically Stacked Axis Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/), [horizontal stacking](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/), [central axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/central-axis-layout/), [inner or outer axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/), [drawing series behind axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/inner-axis-layout/#drawseriesbehindaxis-property), rotation of charts by 90 degrees ([vertical charts](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertical-charts-rotate-transpose-axis/)), and [complex custom layouts](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/advanced-options-custom-layout-managers/) of axis including stacked, stretched layouts of multi-axis charts via the LayoutManager API.

Axis can be dragged to scale, dragged to pan, and custom markers or images can be placed over axis via the [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/). Using the [SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview/), it's possible to draw a chart on an axis.

All axis elements may be styled including gridlines, tick lines, labels, bands (shading between gridlines) and axis borders and background ([Axis styling docs](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/title-labels-gridlines-axis-band-style/)). Axis also may be hidden or visible ([Axis Visibility](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/visibility-of-axis-elements/)). Axis support [Zoom to Fit and AutoRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/) on a per-axis basis and callbacks exist for axis range changed ([VisibleRangeChanged](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/listen-to-visible-range-changes/)))

## What Zoom, Pan, Selection and Interaction Behaviours does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-zoom-pan-selection-and-interaction-behaviours-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Zoom, Pan, Selection and Interaction Behaviours does SciChart.js Feature?" translate="no" title="Direct link to What Zoom, Pan, Selection and Interaction Behaviours does SciChart.js Feature?">â€‹</a>

SciChart.js features a myriad of built-in interaction behaviours, such as zooming, panning, drag-rectangle zoom, mousewheel zoom, pinch zoom for touch screens, mouse-wheel zoom or pan, x-axis drag to zoom or pan, y-axis drag, zoom to fit and scrollbars provided by the SciChartOverview control. Tooltips, selection, hover are also supported via the modifiers listed below. For 3D Charts, SciChart.js supports Orbiting, Pinch zoom, Mousewheel zoom in-out of the 3D scene, Tooltips in 3D and Camera viewport reset modifiers.

### Zooming Behaviours Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#zooming-behaviours-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Zooming Behaviours Supported by SciChart.js" translate="no" title="Direct link to Zooming Behaviours Supported by SciChart.js">â€‹</a>

1.  [ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/) - Drag to pan in X or Y, plus built-in touch screen support
2.  [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/rubber-band-xy-zoom-modifier/) - Drag rectangle to zoom
3.  [PinchZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/) - Two finger pinch zoom on touchscreen devices
4.  [ZoomExtentsModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-extents-modifier/) - double click to Zoom Extents (zoom to fit)
5.  [YAxisDragModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/y-axis-drag-modifier/) - drag Y-Axis to zoom or pan
6.  [XAxisDragModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/x-axis-drag-modifier/) - drag X-Axis to zoom or pan
7.  [MouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier/) - mousewheel to zoom or pan
8.  [SciChartOverview](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/overview/) - scrollbars with configurable chart view behind
9.  (3D Charts) [OrbitModifier3D](https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/orbit-modifier-3d/) - Rotates a 3D chart in 2-axis on mouse-drag by orbiting the 3D scene. This is achieved by updating the Camera position/target
10. (3D Charts) [PinchZoomModifier3D](https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/pinch-zoom-modifier-3d/) - Pinch to zoom a 3D chart on touchscreen devices by updating the camera position/target
11. (3D Charts) [MouseWheelZoomModifier3D](https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/mouse-wheel-zoom-modifier-3d/) - Mousewheel to zoom a 3D chart by updating the camera position / target.
12. (3D Charts) [ResetCamera3DModifier](https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/reset-camera-modifier-3d/) - Resets the Camera to a default position on double-click, either calculated by fitting the 3D scene in the viewport, or a custom position / target

### Touchscreen Zoom Behaviours Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#touchscreen-zoom-behaviours-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Touchscreen Zoom Behaviours Supported by SciChart.js" translate="no" title="Direct link to Touchscreen Zoom Behaviours Supported by SciChart.js">â€‹</a>

All zoom behaviours respond to touch, but these specific ChartModifiers have pinch-zooming or touch-gesture behaviour built in.

- [ZoomPanModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/zoom-pan-modifier/) - Also supports touch to pan and pinch to zoom
- [PinchZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/pinch-zoom-modifier/) - Pinch zoom on touchscreen devices
- [MouseWheelZoomModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/zooming-and-panning/mouse-wheel-zoom-modifier/) - Responds to two-finger touch to replicate mousewheel zooming of 2D charts on touch devices.
- (3D Charts) [MouseWheelZoomModifier3D](https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/mouse-wheel-zoom-modifier-3d/) - Responds to two-finger touch to replicate mousewheel zooming of 3D charts on touch devices

### Selection & Hover Behaviours Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#selection--hover-behaviours-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Selection &amp; Hover Behaviours Supported by SciChart.js" translate="no" title="Direct link to Selection &amp; Hover Behaviours Supported by SciChart.js">â€‹</a>

SciChart.js features selection of series, hover of series and selection of data-points with callbacks and hover of annotations with callbacks. Specific ChartModifiers (behaviours) which ship out of the box include:

1.  [SeriesSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/series-selection/) - select or hover series with callbacks on which series was selected
2.  [DataPointSelectionModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/data-point-selection/) - select data points via click or drag rectangle with notification of which data-points were selected
3.  [AnnotationHoverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/selection/annotation-hover/) - callbacks when an annotation has been hovered or clicked, allowing tooltips and custom behaviour on user interaction with annotations.

#### 3D Chart Selection & Hover Behaviours<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#3d-chart-selection--hover-behaviours" class="hash-link" aria-label="Direct link to 3D Chart Selection &amp; Hover Behaviours" translate="no" title="Direct link to 3D Chart Selection &amp; Hover Behaviours">â€‹</a>

At the time of writing, there is no selection behaviours built for 3D Charts, but if this is a requirement, contact <a href="https://www.scichart.com/contact-us" rel="noopener noreferrer" target="_blank">tech support</a> to request a feature.

## What Tooltips, Inspections and Hit-Testing does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-tooltips-inspections-and-hit-testing-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Tooltips, Inspections and Hit-Testing does SciChart.js Feature?" translate="no" title="Direct link to What Tooltips, Inspections and Hit-Testing does SciChart.js Feature?">â€‹</a>

SciChart.js features several behaviours to add tooltips to charts. Tooltips are provided by ChartModifiers (behaviours) such as the [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/), [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview/) and [VerticalSliceModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/). Tooltip containers (tooltip shape, style) and contents (tooltip text, content) can be templated and customised to show extra data or to match your application style.

1.  [RolloverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/rollover-modifier/) - provides a vertical line which tracks the mouse and places a configurable tooltip container and template at the intersection with chart series.
2.  [CursorModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/cursor-modifier-overview/) - provides a crosshairs (horizontal/vertical line) which tracks the mouse and configurable tooltip container and template. The CursorModifier can also be used to display data in an external legend.
3.  [VerticalSliceModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/vertical-slice-modifier/vertical-slice-modifier-overview/) - provides multiple, placeable, draggable vertical lines which intersect time-series and display configurable tooltip contains and templates at each intersection with chart series.
4.  [Hit-Test API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/hit-test-api-overview/) - provides a programmatic API to detect clicks and taps on chart data-points with various functions to detect the nearest data-point in Xy, nearest data-point in X and series series body hit. Used in conjuction with the ChartModifier API for listening to mouse events via `modifierMouseDown`, `modifierMouseMove` etc, the Hit-Test API can be used to create custom interaction behaviours on the chart.
5.  (3D Charts) [TooltipModifier3D](https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/tooltip-modifier-3d/) - provides a configurable tooltip for 3D charts able to display seriesName, X, Y, Z value and optional metadata, with optional axis lines drawn to the far 3D axis walls

## Custom Zoom, Pan, Tooltip, Selection or Interaction Behaviours Supported by SciChart.js<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#custom-zoom-pan-tooltip-selection-or-interaction-behaviours-supported-by-scichartjs" class="hash-link" aria-label="Direct link to Custom Zoom, Pan, Tooltip, Selection or Interaction Behaviours Supported by SciChart.js" translate="no" title="Direct link to Custom Zoom, Pan, Tooltip, Selection or Interaction Behaviours Supported by SciChart.js">â€‹</a>

SciChart.js supports the creation of custom behaviours (custom ChartModifiers) to allow you to create any specific zooming, panning, tooltip, hover or inspection behaviour that you want. By inheriting a type `ChartModifierBase2D` you can override `modifierMouseDown` `modifierMouseMove` `modifierMouseUp` or even `modifierMouseLeave`, `modifierMouseEnter`, `modifierMouseWheel` or `modifierDoubleClick` allowing you to detect mouse-clicks on charts without subscribing to event handlers.

Custom ChartModifiers can be attached to the parent chart and get notified `onAttach` `onAttachSeries` `onDetach` `onDetachSeries` and have full access to the chart via `parentSurface`, it's axis, series and can manipulate properties or data, plus draw custom overlays such as Tooltips or Annotations.

For 3D Charts, the type `ChartModifierBase3D` can be inherited which also supports `modifierMouseDown` `modifierMouseMove` `modifierMouseUp` etc, and can access the 3D Chart via `parentSurface`, it's axis, 3D series and data, and can manipulate properties, data or draw custom overlays.

An overview of the [Custom Chart Modifier API is found here](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/custom-modifiers/custom-modifiers-overview/) is found here and an example [Detecting Clicks on Chart Parts can be found here](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/custom-modifiers/detecting-clicks-on-chart-parts/) which demonstrates how to detect and highlight areas of the chart that were clicked/hovered. A [Keyboard Zoom Modifier](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/keyboard-accessibility/) example is shown here using the custom ChartModifier API to create accessible charts with zoom on keypress (+/-).

## What Chart Legend Options does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-chart-legend-options-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Chart Legend Options does SciChart.js Feature?" translate="no" title="Direct link to What Chart Legend Options does SciChart.js Feature?">â€‹</a>

SciChart.js features a [LegendModifier](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/miscellaneous-modifiers/legend-modifier/) type which provides a rich, customisable legend. Legends support:

- Placement of the Legend inside the chart top-left, top-right, bottom-left or bottom-right aligned
- Horizontal or Vertical orientation of the Legend
- Showing data series name, line color/marker, visibility checkboxes
- Allowing full customization of the Legend using auto-generated CSS classes scichart\_\_legend, scichart\_\_legend-item
- Placing the Legend anywhere outside the chart

Certain modifiers such as the `CursorModifier` type also support the output of data to create an [active legend](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/cursor-modifier/active-legends-cursor-modifier/) which updates and tracks the position of the crosshair. This is demonstrated in a <a href="https://www.scichart.com/demo/react/candlestick-chart" rel="noopener noreferrer" target="_blank">Financial chart example</a> which outputs details of the candles, bars and series under the cross-hair in the top-left of the chart window in an active legend.

## What Annotation and Overlay Types does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-annotation-and-overlay-types-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Annotation and Overlay Types does SciChart.js Feature?" translate="no" title="Direct link to What Annotation and Overlay Types does SciChart.js Feature?">â€‹</a>

SciChart.js features a number of annotation overlays which can be placed at X,Y data-points on the chart, which scroll, zoom or pan with the chart. Annotations such as labels, lines, boxes and custom markers may be placed at specific X,Y data-points, or relative coordinates via `ECoordinateMode` (0..1 relative to the viewport height or width), or at specific pixel coordinates.

Annotations may also be clicked, selected, edited (dragged or resized by the user). The selection grips can be styled to fit your application style. Annotations may be serialized and state persisted in your app.

It's possible to get hover/click feedback on annotations via the [AnnotationHoverModifier](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotation-hover/). A [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation/) allows drawing of any SVG shape on the chart, and [HTMLAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/) allows placement of any HTML content over the chart.

Annotations available SciChart.js include:

1.  [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation/) - provides WebGL lines at X1 X2 Y1 Y2 coordinates
2.  [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation/) - provides a WebGL box with outline at X1 X2 Y1 Y2 coordinates
3.  [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation/) - displays SVG text labels at any X1 Y1 coordinate
4.  [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation/) - provides a stretched, draggable WebGL vertical line with optional X-Axis label on the chart
5.  [HorizontalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/horizontal-line-annotation/) - provides a stretched, draggable WebGL horizontal line or threshold with optional Y-Axis label on the chart
6.  [LineArrowAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-arrow-annotation/) -provides a WebGL line with arrowhead at X1 X2 Y1 Y2 coordinates
7.  [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview/) - provides a draggable axis marker on Y-Axis at specific Y-value, which can display the current value, or custom text
8.  [NativeTextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation/) - provides WebGL accelerated alterative to TextAnnotation for displaying large quantities of text labels on a chart
9.  [HTML Annotations](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/) - provides any HTML overlay on the chart
10. [ArcAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/arc-annotation/) - draws a filled arc sector or curved line on 2D cartesian charts.
11. [PolarArcAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-arc-annotation/) - draws a filled sector or arc line on a 2D Polar chart.
12. [PolarPointerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/polar-pointer-annotation/) - creates a customisable SVG pointer for polar charts, ideal for gauges or radial indicators.

### 3D Chart Annotations<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#3d-chart-annotations" class="hash-link" aria-label="Direct link to 3D Chart Annotations" translate="no" title="Direct link to 3D Chart Annotations">â€‹</a>

At the time of writing there are no 3D chart annotations, however HTML/SVG overlays in combination with the `ChartModifierBase3D` type could allow the development of custom behaviours. Get in touch with <a href="https://www.scichart.com/contact-us" rel="noopener noreferrer" target="_blank">tech support</a> to request a feature if required.

## What Data Structures does SciChart.js support?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-data-structures-does-scichartjs-support" class="hash-link" aria-label="Direct link to What Data Structures does SciChart.js support?" translate="no" title="Direct link to What Data Structures does SciChart.js support?">â€‹</a>

Both 2D and 3D charts in SciChart.js supports "structure of arrays" data in full 64-bit precision for optimal performance in big-data charts, with arrays of `xValues`, `yValues` or higher order charts requiring `y1Values` `y2Values` etc. Data is stored in WebAssembly (wasm) and is mem-copied from JavaScript to the wasm heap and read/scanned via pointer operations for the fastest possible performance.

Data can be passed to SciChart.js as individual data-points, via JavaScript arrays e.g. `const array = [1,2,3]` or via `Float64Array` for better performance and memory usage.

Optional point metadata (see [Metadata API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/point-metadata-api-overview/)) allows you to tag any X,y point with Js objects, allowing for more complex data structures to be stored and retrieved on inspection - with metadata available to Tooltips, Data Labels, per-point colouring (see [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/)) and more.

Some chart types such as heatmaps, surface mesh plot 3D support uniform grid data format: a 2-dimensional jagged array of values (e.g. `const zValues = [[1,2,3],[4,5,6]]`) used to render cell colours in the case of 2D Chart heatmaps, or cell heights and colours in the cause of 3D Chart surface plots.

## What Real-time Data Update Capabilities does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-real-time-data-update-capabilities-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Real-time Data Update Capabilities does SciChart.js Feature?" translate="no" title="Direct link to What Real-time Data Update Capabilities does SciChart.js Feature?">â€‹</a>

SciChart.js supports bulk create, read, update, delete ([DataSeries CRUD operations](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/append-insert-update-remove/)) via its [DataSeries API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview/). This applies to both 2D & 3D Charts. [Real-time updates](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/) are possible by rapidly updating the Data Series.

Any data-update automatically triggers a redraw. Chart rendering is scheduled and will render at 60 FPS maximum (or your display refresh rate) regardless of the frequency of data-updates.

Real-time data updates have been tested up to millions of data-points per second and SciChart.js handles these efficiently while scheduling re-draws, balancing CPU and GPU load.

SciChart.js also features a FIFO mode (first-in-first-out) via `fifoCapacity` flag for automatic scrolling and discarding of old data in time-series monitoring applications, and a `fifoSweeping` flag to allow wrap-around ECG-style charts. For more info see the demos on [Scrolling Data](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#scrolling-using-fifocapacity) and [Sweeping Data](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates/#sweeping-data)

SciChart is already optimised for the maximum possible performance out of the box. For learning about how to get the best performance from SciChart.js in your application, refer to our [Performance Tips & Tricks](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks/) article.

## What Performance Limits or Datapoint Limits does SciChart.js Have?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-performance-limits-or-datapoint-limits-does-scichartjs-have" class="hash-link" aria-label="Direct link to What Performance Limits or Datapoint Limits does SciChart.js Have?" translate="no" title="Direct link to What Performance Limits or Datapoint Limits does SciChart.js Have?">â€‹</a>

We often say, **"You will hit the limits of the browser before you hit the limits of SciChart.js"**. SciChart.js can draw up to 100,000,000 (100 million) data-points in Google Chrome browser which is limited to ~1.5GBytes RAM per tab, 4GB RAM per process. Higher data-point limits are possible with Mozilla Firefox (billions of data-points) which has a higher memory limit. Practically speaking, a limit of total data-points can be calculated assuming X,Y values with double-precision floating point (16-bytes per X,Y data-point) and the browser limit.

Dashboards of hundreds of charts all updating in real-time are possible with SciChart.js, such as this <a href="https://www.scichart.com/blog/creating-a-react-drag-drop-chart-dashboard/" rel="noopener noreferrer" target="_blank">Realtime 100 Chart Drag &amp; Drop React Dashboard</a> demo, and this <a href="https://www.scichart.com/demo/javascript/javascript-multiple-chart-dashboard-performance-demo" rel="noopener noreferrer" target="_blank">64-chart dashboard demo</a>.

There are no WebGL context limits with SciChart.js when using the standard `SciChartSurface.create()` function - which creates a single shared WebGL context for all charts on the page. This efficiently by-passes the browser WebGL Context limits which would otherwise limit the number of WebGL charts on a screen to as little as 2 for mobile devices, and 16 for desktop with Google Chrome.

SciChart.js is always pushing the limits of performance, ensuring low memory consumption, efficient, balanced CPU/GPU usage. If you discover a performance problem, please report it via <a href="https://www.scichart.com/contact-us" rel="noopener noreferrer" target="_blank">tech support</a> and our team will do their best to help.

## What Multi-Chart Linking and Dashboard Layout Capabilities does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-multi-chart-linking-and-dashboard-layout-capabilities-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Multi-Chart Linking and Dashboard Layout Capabilities does SciChart.js Feature?" translate="no" title="Direct link to What Multi-Chart Linking and Dashboard Layout Capabilities does SciChart.js Feature?">â€‹</a>

SciChart.js supports multi-chart linking for complex, synchronised dashboards as well as a [SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview/) for grouping several charts into a single master surface, charts within charts, charts on the axis and more.

Several APIs are available to link multiple charts.

### Linking Multiple Charts via Chart Groups<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#linking-multiple-charts-via-chart-groups" class="hash-link" aria-label="Direct link to Linking Multiple Charts via Chart Groups" translate="no" title="Direct link to Linking Multiple Charts via Chart Groups">â€‹</a>

At a basic level, one `SciChartSurface` is one chart. Two or more charts can be linked by [creating a vertical or horizontal chart group](https://www.scichart.com/documentation/js/v4/2d-charts/chart-synchronization-api/synchronizing-multiple-charts/) where zooming, panning and tooltips / mouse interactions are shared across multiple charts in the group. This allows the creation of multi-chart components such as <a href="https://www.scichart.com/demo/react/multi-pane-stock-charts-sync-technique" rel="noopener noreferrer" target="_blank">Financial charts</a> where indicator panels appear below the main Candlestick chart, but all zooming, panning and tooltips happen in unison.

The same technique can be used to create timeline controls in industrial process monitoring where multiple time-series charts are linked and zoom, pan and have tooltips act on all chart surfaces in unison.

### Appearance of multiple linked charts via Vertically Stacked Axis / Horizontally Stacked Axis feature<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#appearance-of-multiple-linked-charts-via-vertically-stacked-axis--horizontally-stacked-axis-feature" class="hash-link" aria-label="Direct link to Appearance of multiple linked charts via Vertically Stacked Axis / Horizontally Stacked Axis feature" translate="no" title="Direct link to Appearance of multiple linked charts via Vertically Stacked Axis / Horizontally Stacked Axis feature">â€‹</a>

A similar effect of creating timeline charts can be achieved via [Vertically Stacking Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/vertically-stacked-axis-layout/), or [Horizontally Stacking Axis](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/horizontally-stacked-axis-layout/). By taking advantage of SciChart.js' complex axis layout features via the [LayoutManager API](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/advanced-options-custom-layout-managers/) a single chart surface can be configured to appear as multiple charts arranged horizontally, or vertically with zoom, pan, tooltips and mouse-interactions shared across all charts.

### Creation of Dashboard Layout controls with multi-charts using the SubCharts API<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#creation-of-dashboard-layout-controls-with-multi-charts-using-the-subcharts-api" class="hash-link" aria-label="Direct link to Creation of Dashboard Layout controls with multi-charts using the SubCharts API" translate="no" title="Direct link to Creation of Dashboard Layout controls with multi-charts using the SubCharts API">â€‹</a>

The [SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview/) allows placement of multiple charts into a single parent chart surface, allowing charts within charts, groups of charts and creation of pre-defined dashboard chart groups with layout based on relative coordinates. This API is highly performant as per this <a href="https://www.scichart.com/blog/pushing-the-boundaries-of-javascript-chart-dashboard-performance/" rel="noopener noreferrer" target="_blank">blog post</a> on 100 charts with total of 200 axis, 1,000 text labels and 6 million datapoints, as grouping of charts into a single WebGL surface and single render call allows for batching strategies that aren't available with groups of individual chart surfaces.

Some worked examples can be found below:

- [Creating re-usable chart groups using SubCharts](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-reusable-chart-groups-with-sub-charts/)
- [Dynamically Add/Remove Charts to a group using SubCharts API](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/exampe-dynamic-multi-panel-charts-with-sub-charts/)
- [Resizable multi-chart panes using SubCharts](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-resizable-multi-pane-charts-with-sub-charts/)
- [Using SubCharts to create a large dashboard of 100 charts](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/example-using-sub-charts-to-create-large-dashboard/)
- [Creating a Chart within a Chart using SubCharts](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/sub-charts-positioning/)
- [Creating a 2-dimensional overlay by placing a chart within a chart using SubCharts](https://www.scichart.com/documentation/js/v4/2d-charts/subcharts-api/subcharts-api-overview/)
- <a href="https://www.scichart.com/blog/pushing-the-boundaries-of-javascript-chart-dashboard-performance/" rel="noopener noreferrer" target="_blank">Performance Benefits of SubCharts vs. separate charts for 100 chart dashboards</a>

## What Styling, Theming and Animations Capabilities does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-styling-theming-and-animations-capabilities-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Styling, Theming and Animations Capabilities does SciChart.js Feature?" translate="no" title="Direct link to What Styling, Theming and Animations Capabilities does SciChart.js Feature?">â€‹</a>

SciChart.js supports a rich styling, theming and animations API to create beautiful 2D & 3D charts on the web which match your application style, theme and branding.

Out of the box SciChart.js ships with three [themes](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/): Light, Dark and Navy. Additional [custom themes](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/) can be created by implementing `IThemeProvider`. Individual chart parts can be [styled in code](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code/), such as chart titles, labels, axis, gridlines, viewport boundaries, viewport backgrounds, legend containers, tooltip containers and text, chart margins and paddings. The [chart background](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/) can be transparent showing the underlying HTML DOM, blurred creating a glass-like effect or the chart background can be set to an image.

A rich [Animations API](https://www.scichart.com/documentation/js/v4/2d-charts/animations-api/animations-api-overview/) allows you to animate series on first-load, data-points, provide animated style or data transitions, or any element via the generic animations API.

## What Accessibility Options does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-accessibility-options-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Accessibility Options does SciChart.js Feature?" translate="no" title="Direct link to What Accessibility Options does SciChart.js Feature?">â€‹</a>

SciChart.js supports Accessibility with the following features:

- [Voice Over](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/) using the `SpeechSynthesisUterrance` API to announce data-point values on tap/touch, to announce axis labels on tap/touch or to announce changes to chart viewport zoom state.
- [Color and Contrast](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/color-and-contrast/).
- [Keyboard accessibility](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/keyboard-accessibility/) such as zooming on keypress.

## What Custom Drawing / Custom Rendering Options does SciChart.js Feature?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-custom-drawing--custom-rendering-options-does-scichartjs-feature" class="hash-link" aria-label="Direct link to What Custom Drawing / Custom Rendering Options does SciChart.js Feature?" translate="no" title="Direct link to What Custom Drawing / Custom Rendering Options does SciChart.js Feature?">â€‹</a>

SciChart.js features ways to customize the rendering of the charts beyond the standard chart types, for example if you wish to draw arbitrary shapes, polygons, manipulate rendering beyond the standard built-in annotations or series. Some ways to customize rendering are listed below.

### Triangle Series (Arbitrary Polygon Drawing) on 2D Charts<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#triangle-series-arbitrary-polygon-drawing-on-2d-charts" class="hash-link" aria-label="Direct link to Triangle Series (Arbitrary Polygon Drawing) on 2D Charts" translate="no" title="Direct link to Triangle Series (Arbitrary Polygon Drawing) on 2D Charts">â€‹</a>

The `FastTriangleRenderableSeries` [Triangle Series Type documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-triangle-renderable-series/) plots data as triangles, can be used for custom scatter plots, mesh visualisations, creating complex geometry in 2D such as <a href="https://www.scichart.com/demo/react/multi-map-example" rel="noopener noreferrer" target="_blank">Geo Maps</a> and more. This series type visualizes data as a series of triangles (polygons) on a 2D Chart. Triangles can be drawn as `List`, `Polygon` or `Strip` - consistent with 2D/3D graphics APIs such as OpenGL. Polygons can have an individual `fill`, are associated with `polygonVertices`.

Triangle Series fills support solid color as well as gradient fill via `fillLinearGradient` property. Custom textures may be applied to triangles or polygons drawn on a 2D chart using the Triangle Series. Conversion functions exist in SciChart.js examples such as `constrainedDelaunayTriangulation.ts` in the <a href="https://www.scichart.com/demo/javascript/polar-map-example" rel="noopener noreferrer" target="_blank">Polar Map Example</a> to convert Geo JSON data to `polygonVertices` for Geo Maps. It is recommended to convert and cache these on the server for optional performance.

### Custom Annotations - custom SVG markers, images and shapes on 2D Charts<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#custom-annotations---custom-svg-markers-images-and-shapes-on-2d-charts" class="hash-link" aria-label="Direct link to Custom Annotations - custom SVG markers, images and shapes on 2D Charts" translate="no" title="Direct link to Custom Annotations - custom SVG markers, images and shapes on 2D Charts">â€‹</a>

The [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation/) type allows you to create custom SVG markers, images, shapes and place these on a 2D chart at specific X,Y locations. This is part of the [Annotations API](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/annotations-api-overview/) and can e used to display custom shapes such as Buy/Sell markers on a Financial chart, custom warnings or points of interest or even images. A custom [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/image-axis-marker-annotation/) allows a similar technique (image, SVG, custom marker) to be placed on the axis.

Further customisation is possible via the [CustomHTMLAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/) type which allows placement of any HTML (such as buttons, dropdowns, forms) onto X,Y locations on the chart.

### Custom DataPoint Markers (Custom PointMarkers) on 2D Charts<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#custom-datapoint-markers-custom-pointmarkers-on-2d-charts" class="hash-link" aria-label="Direct link to Custom DataPoint Markers (Custom PointMarkers) on 2D Charts" translate="no" title="Direct link to Custom DataPoint Markers (Custom PointMarkers) on 2D Charts">â€‹</a>

The [PointMarkers API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-point-markers/) in SciChart.js allows you to place data-point markers on chart such as Ellipse, Square, Triangle, Cross, X. Also a custom `SpritePointMarker` type can be used to create a custom point-marker from image, which is cached to texture and rendered using WebGL. This allows custom markers to be placed on data-points such as scatter charts, line charts and more.

### Per-point Colouring of Data-points via the PaletteProvider API on 2D and 3D Charts<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#per-point-colouring-of-data-points-via-the-paletteprovider-api-on-2d-and-3d-charts" class="hash-link" aria-label="Direct link to Per-point Colouring of Data-points via the PaletteProvider API on 2D and 3D Charts" translate="no" title="Direct link to Per-point Colouring of Data-points via the PaletteProvider API on 2D and 3D Charts">â€‹</a>

The [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview/) applies to both 2D and 3D charts, and allows individual data-point colours, fills and strokes to be overridden via a programmatic rule. PaletteProviders are attached to the `BaseRenderableSeries.paletteProvider` property. They are called during the render process to return an overridden colour (or undefined for default colour) allowing the stroke, fill, or pointmarker stroke/fill to be updated dynamically, allowing creation of thresholds, warnings, and highlighting areas of interest in your data. For example, you can change the fill of a time-based histogram depending on day of the week, or change the colour of a column (bar) chart displaying Volume in financial charts depending on whether the candle was red or green.

In combination with the PaletteProvider API, we also have a [PaletteFactory](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-factory/) helper class which can create predefined palettes such as gradient fills in X or Y, which may be applied to line, scatter, column, mountain, area series and more.

### RenderDataTransforms - transform data and switch rendering immediately before draw on 2D Charts<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#renderdatatransforms---transform-data-and-switch-rendering-immediately-before-draw-on-2d-charts" class="hash-link" aria-label="Direct link to RenderDataTransforms - transform data and switch rendering immediately before draw on 2D Charts" translate="no" title="Direct link to RenderDataTransforms - transform data and switch rendering immediately before draw on 2D Charts">â€‹</a>

`RenderDataTransforms` (see [RenderDataTransforms API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/render-data-transforms-api/) allow you to transform data immediately before rendering, allowing for skipping or insertion of points for drawing (without affecting hit-test or drawing or actual data),

Some use-cases of `RenderDataTransforms` include:

- Interpolating Data: The `RenderDataTransforms` API is used internally to draw spline (bezier) series types
- Switching styles on series: for example changing the point-marker on a series dynamically during rendering
- Splitting line segments: for example, inserting extra points to define thresholds with discrete colour changes at a specific level
- Adding gaps to series: for example, by manipulating NaN values

### SVG Overlays on 2D and 3D Charts<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#svg-overlays-on-2d-and-3d-charts" class="hash-link" aria-label="Direct link to SVG Overlays on 2D and 3D Charts" translate="no" title="Direct link to SVG Overlays on 2D and 3D Charts">â€‹</a>

The `SciChartSurface` type has several SVG layers which can be drawn to for any custom overlay (think: custom tooltips, legend, annotations or content) should the built-in methods of customising rendering be insufficient. These are accessible via properties on the `SciChartSurface` such as <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#domsvgcontainer" rel="noopener noreferrer" target="_blank">domSvgContainerðŸ“˜</a>.

### CustomRenderableSeries (Custom Series in 2D Charts)<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#customrenderableseries-custom-series-in-2d-charts" class="hash-link" aria-label="Direct link to CustomRenderableSeries (Custom Series in 2D Charts)" translate="no" title="Direct link to CustomRenderableSeries (Custom Series in 2D Charts)">â€‹</a>

At the time of writing, `CustomRenderableSeries` - a type which allows you to create a custom 2D chart type by accessing rendering functions within SciChart.js - is not available. Custom Series are supported in other platforms of SciChart (SciChart WPF, iOS, Android) and will be ported to SciChart.js in future versions. If this is needed in your project or you would like to enquire more, contact <a href="https://www.scichart.com/contact-us" rel="noopener noreferrer" target="_blank">tech support</a>

## What other API Features does SciChart.js have?<a href="https://www.scichart.com/documentation/js/v4/user-manual/what-chart-types/#what-other-api-features-does-scichartjs-have" class="hash-link" aria-label="Direct link to What other API Features does SciChart.js have?" translate="no" title="Direct link to What other API Features does SciChart.js have?">â€‹</a>

SciChart.js features several other miscellaneous APIs to allow creating rich, interactive and flexible charts for the most demanding applications. This is not an exhaustive list of what's possible with SciChart.js, but provides an overview of the APIs which allow extensibility and flexibility of the chart for almost any requirement in scientific, medical, financial, industrial and business data visualisation.

These additional API features include:

- [Pixel to Data-point Conversion APIs](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates/)
- [Retina and Browser Zoom Support](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/retina-support-and-browser-zoom/)
- [Ordered Rendering](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/ordered-rendering/), allowing to control the order of rendering of chart series, chart annotations and chart gridlines
- [Render Events and callbacks](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/render-events/), such as `redrawRequested`, `preRenderAll`, `preRender`, `layoutMeasured`, `rendered`, `renderedToWebGL`, `renderedToDestination` and `painted` allowing precise control over insertion of commands and fine-grained monitoring of performance
- [Axis VisibleRange Changed Callbacks](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/listen-to-visible-range-changes/) allow you to be notified of changes to an axis range, such as zoom changes, pan changes, programmatic or user-zoom changes. Including the [ZoomState](https://www.scichart.com/documentation/js/v4/get-started/faqs/faq-zoomState/) property, it is possible to detect if the user is zooming or panning and apply custom logic to your chart.
- [Memory Debug tools](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/) are also included as well as detailed documentation on [Memory Best Practices](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/) and [Performance Tips](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks/) to get the best possible performance out of SciChart.js
- [Data Filters API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/), similar to `RenderDataTransforms`, the Data Filters API allows transforms on data before rendering, allowing you to create moving averages, low-pass or high-pass filters in Digital Signal Processing, calculate the ratio between two series, scale or offset a series, and apply linear regression line fitting to series. Custom filters can be created for any complex data-filtering scenario and filters may be chained (output of one filter as input to the next) for more complex operations.
- [Data Labels API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/) allows placement of per-point text labels on data-points. Text labels can display the Y-value, X-Y value, can be multi-line, can be custom text and support various culling options for showing/hiding labels which overlap.
- [Builder API (JSON Builder)](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview/), a complete way to define charts via JSON or JavaScript objects, vs. programmatic creation of charts in JavaScript. The Builder/JSON API allows creation of pre-defined chart configurations or partially configured charts which may be serialized, deserialized or created programmatically in your application. JSON served from a server can be used to populate a chart on the client.
- [Serialization / Deserialization](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/charts-serialization-deserialization/)
- [Drawing Gaps in series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/common-series-apis/drawing-gaps/) - drawing gaps or handling null data-points in polyline, mountain (area) and other series types by using Y=NaN
- Batching or temporarily suspending drawing via the [Update Suspender API](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/#the-suspend-updates-api)
- [Effects (glow, dropshadow)](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/glow-and-dro-shadow-shader-effects/) - experimental shaders which can be applied to series to provide glow effect (oscilloscope / VDU style effect). These may not work on all hardware due to the complexity of the shaders.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/user-manual/what-chart-types/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/user-manual/what-chart-types/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
