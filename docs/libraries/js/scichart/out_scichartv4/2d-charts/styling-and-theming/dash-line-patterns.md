On this page

# Series Styling - Dash Line Patterns

In SciChart.js v1.3 and above, we have introduced a property to let you style dashed or dotted lines on certain series.

![](out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

We have an example in our SciChart.js Examples Suite which shows how to do this - theÂ <a href="https://www.scichart.com/demo/javascript-dashed-line-chart" rel="noopener noreferrer" target="_blank">Dashed Line Chart example</a>.

<img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/d52f5cba3d0abe546212addf5c744afadf526c8a.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

***Above:***Â <a href="https://www.scichart.com/demo/javascript-dashed-line-chart" rel="noopener noreferrer" target="_blank"><em>The Dashed Line Styling example</em></a> *in the SciChart.js demo*

## How to set a Stroke Dash<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns/#how-to-set-a-stroke-dash" class="hash-link" aria-label="Direct link to How to set a Stroke Dash" translate="no" title="Direct link to How to set a Stroke Dash">â€‹</a>

Certain series have a **StrokeDashArray** property which allows you to define the line pen as having a dotted or dashed pattern. A variety of dash patterns are possible in SciChart.js using this API.

A StrokeDashArray is an array which defines a dot-dash pattern. For example creating a series with a strokeDashArray as follows:

``` prism-code
// TheÂ StrokeDashArray property

const series = new FastLineRenderableSeries(wasmContext, {
            stroke: "SteelBlue",
            strokeThickness: 2,
            strokeDashArray: [10, 3]
        });
```

Results in the following output:

<img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/b44e614f930f3a3e693cc970cda97b243910faa0.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

The dash pattern can be configured to provide dots, dashes and more. For example:

#### strokeDashArray: \[2,2\]<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns/#strokedasharray-22" class="hash-link" aria-label="Direct link to strokeDashArray: [2,2]" translate="no" title="Direct link to strokeDashArray: [2,2]">â€‹</a>

<img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/6150a02576c23d411ffaa5632c97f4b4d149c90e.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### strokeDashArray: \[5,5\]<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns/#strokedasharray-55" class="hash-link" aria-label="Direct link to strokeDashArray: [5,5]" translate="no" title="Direct link to strokeDashArray: [5,5]">â€‹</a>

<img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/a16ad507112862cc658315ac7fb1c00144b9895a.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### strokeDashArray: \[10,25\]<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns/#strokedasharray-1025" class="hash-link" aria-label="Direct link to strokeDashArray: [10,25]" translate="no" title="Direct link to strokeDashArray: [10,25]">â€‹</a>

<img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/56fc4a1f4f10d90db7cc348ce85b0094f1714d7f.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Which Series support StrokeDashArray?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns/#which-series-support-strokedasharray" class="hash-link" aria-label="Direct link to Which Series support StrokeDashArray?" translate="no" title="Direct link to Which Series support StrokeDashArray?">â€‹</a>

All the line-based series in SciChart.js support StrokeDashArray for dashed or dotted lines. These are:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html#strokedasharray" rel="noopener noreferrer" target="_blank">FastLineRenderableSeries.strokeDashArrayðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html#strokedasharray" rel="noopener noreferrer" target="_blank">FastMountainRenderableSeries.strokeDashArrayðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html#strokedasharray" rel="noopener noreferrer" target="_blank">FastBandRenderableSeries.strokeDashArrayðŸ“˜</a>
- SplineLineRenderableSeries
- SplineMountainRenderableSeries
- SplineBandRenderableSeries

## Chart Parts which support StrokeDashArray<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/dash-line-patterns/#chart-parts-which-support-strokedasharray" class="hash-link" aria-label="Direct link to Chart Parts which support StrokeDashArray" translate="no" title="Direct link to Chart Parts which support StrokeDashArray">â€‹</a>

The strokeDashArray property also exists on AxisBase2D.majorGridLineStyle, AxisBase2D.minorGridLineStyle, allowing you to apply stroke dash / dot styling to axis gridlines.

For more information, see the related articleÂ [Axis Styling - Title, Labels, Gridlines and Axis Bands](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/title-labels-gridlines-axis-band-style/).

<img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/0adb56e7f032ae3ea3d81b68f9b1216097764d53.png" class="img_ev3q" decoding="async" loading="lazy" width="1582" height="1046" alt="Custom styling or themeing of JavaScript chart parts" />

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/dash-line-patterns/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/dash-line-patterns/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
