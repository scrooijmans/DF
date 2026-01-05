On this page

# The Partial Polar Chart Type

![](out_scichartv4/2d-charts/chart-types/polar-partial-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

TheÂ <a href="https://www.scichart.com/demo/iframe/polar-partial-arc" rel="noopener noreferrer" target="_blank">JavaScript Partial Polar Chart Example</a>Â can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarPartialArc" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Partial Arc</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-partial-arc" target="_blank">Partial Polar Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

This demo shows how a Polar plot can be used to look similar to a Cartesian plot, by setting <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#innerradius" rel="noopener noreferrer" target="_blank">innerRadiusðŸ“˜</a>: to `0.998` (from the default of `0`) and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a>: to `0.001 * Math.PI` radians (from the default of 2 \* Math.PI), so that it only displays a very small section of the total circle / polar system, which auto-ranges and zooms to fit in the parent div.

The gridlines are never actually parallel, but the radius of the imaginary circle it's drawn on is so large when setting these 2 properties this way, that it looks like a Cartesian plot.

## What is a Partial Polar Chart?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-partial-chart-type/#what-is-a-partial-polar-chart" class="hash-link" aria-label="Direct link to What is a Partial Polar Chart?" translate="no" title="Direct link to What is a Partial Polar Chart?">â€‹</a>

The Partial Polar Chart is just a regular polar chart, but with the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> property of the **Angular** axis set to a value less than `2 * Math.PI` (or `360` degrees). This allows you to create a chart that only displays a portion of the polar coordinate system, which can be useful for visualizing data that is only relevant within a specific angular range.

The example above is extreme, but partial polar charts refers to all plots that span across an angle that is less than `360` degrees (or `2 * Math.PI` radians).

## Other examples of Partial Polar Charts:<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-partial-chart-type/#other-examples-of-partial-polar-charts" class="hash-link" aria-label="Direct link to Other examples of Partial Polar Charts:" translate="no" title="Direct link to Other examples of Partial Polar Charts:">â€‹</a>

### 1. Gauge Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-partial-chart-type/#1-gauge-charts" class="hash-link" aria-label="Direct link to 1. Gauge Charts" translate="no" title="Direct link to 1. Gauge Charts">â€‹</a>

> Many Gauge charts have sweeping arcs in between `180` and `270` degrees. Check this out for more info: [Gauge Chart Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-gauge-chart)

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-gauge-chart" target="_blank">Partial Polar Gauge Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

### 2. Polar Column / Line / Mountain series<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-partial-chart-type/#2-polar-column--line--mountain-series" class="hash-link" aria-label="Direct link to 2. Polar Column / Line / Mountain series" translate="no" title="Direct link to 2. Polar Column / Line / Mountain series">â€‹</a>

> They might also not need the full `360` degrees, but rather a partial arc

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-radial-column-chart" target="_blank">Polar Radial Column Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-stacked-radial-column-chart" target="_blank">Polar Stacked Radial Column Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

### 3. Polar Uniform Heatmap Chart<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-partial-chart-type/#3-polar-uniform-heatmap-chart" class="hash-link" aria-label="Direct link to 3. Polar Uniform Heatmap Chart" translate="no" title="Direct link to 3. Polar Uniform Heatmap Chart">â€‹</a>

> A polar uniform heatmap chart can also be created by setting the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> property of the **Angular** axis to a value less than `2 * Math.PI` (or `360` degrees).

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-uniform-heatmap-chart" target="_blank">Polar Uniform Heatmap Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

![](out_scichartv4/2d-charts/chart-types/polar-partial-chart-type/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For more information about polar chart layout, styling and axes, check out these documentation pages:

- [Polar Chart Layout](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/multi-axis-and-layout/polar-chart-layout)
- [Polar Axis Styling](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/polar-axis-styling)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-partial-chart-type/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-partial-chart-type/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
