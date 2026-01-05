On this page

# Chart Styling - Margin and Padding

In SciChart.js v1.2 and above, we have introduced some properties that let you define the margin/padding inside the SciChartSurface, as well as certain chart parts such as Axis Labels and Axis Titles.

Find out how to set these below.

# SciChartSurface Padding

The SciChartSurface.padding property can be used to apply a top, right, bottom, left padding to the chart. This defines the spacing around the viewport area and to the chart edge (which includes series, gridlines and axis).

<img src="out_scichartv4/2d-charts/styling-and-theming/margin-and-padding/index_media/f8c8e0e292b01d9aee9b063d95268f874b3c20f9.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

To set the padding, use this code:

``` prism-code
// Set SciChartSurface.padding

// Set padding via Thickness constructor
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId);
const top = 10;
const right = 20;
const bottom = 30;
const left = 40;
sciChartSurface.padding = new Thickness(top, right, bottom, left);

// Set padding via string. Order is Top, Right, Bottom, Left
sciChartSurface.padding = Thickness.fromString("10 20 30 40");

// Set padding via single number. Top=Bottom=Right=Left
sciChartSurface.padding = Thickness.fromNumber(10);
```

For example, to have a padding of zero andÂ make the chart render right to the edges of it's parent canvas, use this code:

``` prism-code
// Set chart padding to zero

sciChartSurface.padding = Thickness.fromNumber(0);
```

This results in the following:

<img src="out_scichartv4/2d-charts/styling-and-theming/margin-and-padding/index_media/53897265b97d68efa12caa9bb3dc7cef89f29ff5.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Axis Label Padding<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/margin-and-padding/#axis-label-padding" class="hash-link" aria-label="Direct link to Axis Label Padding" translate="no" title="Direct link to Axis Label Padding">â€‹</a>

The Axis Labels have a default padding of 4 on all sides. This puts some spacing between the axis and the inner and outer edge of the chart.

Note that horizontal axis ignore axis padding left/right, and vertical axis ignore padding top/bottom.

To set some padding on the axis labels, use the following code.

``` prism-code
// Set Axis Label

// Set padding via Thickness constructor
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId);
sciChartSurface.padding = Thickness.fromNumber(10);

const bottomXAxis = new NumericAxis(wasmContext);
bottomXAxis.labelStyle.padding = new Thickness(10, 0, 30, 0);
bottomXAxis.axisAlignment = EAxisAlignment.Bottom;
sciChartSurface.xAxes.add(bottomXAxis);

const rightYAxis = new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) });
rightYAxis.labelStyle.padding = new Thickness(0, 30, 0, 10);
rightYAxis.axisAlignment = EAxisAlignment.Right;
sciChartSurface.yAxes.add(rightYAxis);
```

This results in the following:

<img src="out_scichartv4/2d-charts/styling-and-theming/margin-and-padding/index_media/b3e47f0c509ed6884bfdeaf730a2ee940a51b4f6.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Axis Title Padding<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/margin-and-padding/#axis-title-padding" class="hash-link" aria-label="Direct link to Axis Title Padding" translate="no" title="Direct link to Axis Title Padding">â€‹</a>

The Axis Titles have a default padding ofÂ 6 on all sides. This puts some spacing between the axis title andÂ row of labels andÂ outer edge of the chart.

![](out_scichartv4/2d-charts/styling-and-theming/margin-and-padding/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Note that horizontal axis ignore axis padding left/right, and vertical axis ignore padding top/bottom.

To set some padding on the axis title, use the following code.

``` prism-code
// Set Axis Label

const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId);
sciChartSurface.debugRendering = false;
sciChartSurface.applyTheme(new SciChartJSLightTheme());
sciChartSurface.padding = Thickness.fromNumber(10);
const bottomXAxis = new NumericAxis(wasmContext);
bottomXAxis.axisTitle = "Bottom, Padding Top:10,Bottom:30";
bottomXAxis.axisTitleStyle.color = "Black";
bottomXAxis.axisTitleStyle.padding = new Thickness(10, 0, 30, 0);
bottomXAxis.axisAlignment = EAxisAlignment.Bottom;
sciChartSurface.xAxes.add(bottomXAxis);
const rightYAxis = new NumericAxis(wasmContext, { growBy: new NumberRange(0.05, 0.05) });
rightYAxis.axisTitle = "Right, Padding Left:10,Right:30";
rightYAxis.axisTitleStyle.color = "Black";
rightYAxis.axisTitleStyle.padding = new Thickness(0, 30, 0, 10);
rightYAxis.axisAlignment = EAxisAlignment.Right;
sciChartSurface.yAxes.add(rightYAxis);
```

This results in the following:

<img src="out_scichartv4/2d-charts/styling-and-theming/margin-and-padding/index_media/24bb93e3718dfc7cba889045919bed2872b6f19b.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/margin-and-padding/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/margin-and-padding/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
