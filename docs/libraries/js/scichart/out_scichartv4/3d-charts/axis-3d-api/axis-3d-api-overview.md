On this page

# Axis3D APIs Overview

SciChart.js 3D shares the sameÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" rel="noopener noreferrer" target="_blank">AxisCoreðŸ“˜</a> as SciChart.js 2D. Many features such as **Text Formatting**, **Autorange** (zooming to fit) and **Styling** **are** **shared**.Â For your convenience, some of the documentation has been duplicated here, with some referring to other sections of the user manual.

The Axis Types in SciChart all inherit fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" rel="noopener noreferrer" target="_blank">AxisCoreðŸ“˜</a>Â and 3D axis inheritÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" rel="noopener noreferrer" target="_blank">AxisBase3DðŸ“˜</a>. Below you can find an inheritance diagram. In the next section we're going to go over the main properties, types and what you can do with SciChart.js 3D Axis.

### The AxisCore Type<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#the-axiscore-type" class="hash-link" aria-label="Direct link to The AxisCore Type" translate="no" title="Direct link to The AxisCore Type">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" rel="noopener noreferrer" target="_blank">AxisCoreðŸ“˜</a> properties and methods can be seen at theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" rel="noopener noreferrer" target="_blank">TypeDoc documentationðŸ“˜</a>.

Some important properties to note: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#autorange" rel="noopener noreferrer" target="_blank">autoRangeðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#textformatting" rel="noopener noreferrer" target="_blank">textFormattingðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majorgridlinestyle" rel="noopener noreferrer" target="_blank">majorGridLineStyleðŸ“˜</a> and properties like <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#drawlabels" rel="noopener noreferrer" target="_blank">drawLabelsðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#drawmajorgridlines" rel="noopener noreferrer" target="_blank">drawMajorGridLinesðŸ“˜</a> etc...Â Â We're going to explain more on how to use these later.

### The AxisBase3D Type<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#the-axisbase3d-type" class="hash-link" aria-label="Direct link to The AxisBase3D Type" translate="no" title="Direct link to The AxisBase3D Type">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" rel="noopener noreferrer" target="_blank">AxisBase3DðŸ“˜</a> inherits AxisCore and has some additional properties specific to 3D charts. These include:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#axisplanebackgroundfill" rel="noopener noreferrer" target="_blank">axisPlaneBackgroundFillðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#backgroundcolor" rel="noopener noreferrer" target="_blank">backgroundColorðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#labeldepthtestenabled" rel="noopener noreferrer" target="_blank">labelDepthTestEnabledðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#negativesideclipping" rel="noopener noreferrer" target="_blank">negativeSideClippingðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#planebordercolor" rel="noopener noreferrer" target="_blank">planeBorderColorðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#planeborderthickness" rel="noopener noreferrer" target="_blank">planeBorderThicknessðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#positivesideclipping" rel="noopener noreferrer" target="_blank">positiveSideClippingðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#ticklabelalignment" rel="noopener noreferrer" target="_blank">tickLabelAlignmentðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#ticklabelsoffset" rel="noopener noreferrer" target="_blank">tickLabelsOffsetðŸ“˜</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#titleoffset" rel="noopener noreferrer" target="_blank">titleOffsetðŸ“˜</a>

## Basic Examples of how to declare an Axis<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#basic-examples-of-how-to-declare-an-axis" class="hash-link" aria-label="Direct link to Basic Examples of how to declare an Axis" translate="no" title="Direct link to Basic Examples of how to declare an Axis">â€‹</a>

For a super-simple primer with code sample on how to declare an axis in SciChart.js 3D, see the articleÂ [Numeric and Date Axis in SciChart3D](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/numeric-and-date-axis-in-scichart-3d/).

## Axis 3D APIs<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#axis-3d-apis" class="hash-link" aria-label="Direct link to Axis 3D APIs" translate="no" title="Direct link to Axis 3D APIs">â€‹</a>

Below are the key things you can do with the axis in SciChart.js 3D and where to find more information.

### Axis 3D Text (Label) Formatting<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#axis-3d-text-label-formatting" class="hash-link" aria-label="Direct link to Axis 3D Text (Label) Formatting" translate="no" title="Direct link to Axis 3D Text (Label) Formatting">â€‹</a>

All Axis in SciChart use the labelProviders to format text for the axis labels and cursor (tooltip) labels.

Background information can be foundÂ found at theÂ [Axis LabelProvider API Overview](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview/).

Specific example code for formatting 3D Axis text labels can be found in the articleÂ [Axis3D Text (Label) Formatting)](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-text-label-formatting/).

### Axis 3D AutoRanging & Setting VisibleRange<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#axis-3d-autoranging--setting-visiblerange" class="hash-link" aria-label="Direct link to Axis 3D AutoRanging &amp; Setting VisibleRange" translate="no" title="Direct link to Axis 3D AutoRanging &amp; Setting VisibleRange">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html" rel="noopener noreferrer" target="_blank">AxisBase3DðŸ“˜</a> derived Types also have auto-ranging behaviour as per the 2D axis types. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#autorange" rel="noopener noreferrer" target="_blank">axis.autoRangeðŸ“˜</a> property defines how the axis will autorange when data is changed.

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#visiblerange" rel="noopener noreferrer" target="_blank">axis.visibleRangeðŸ“˜</a> property allows you to set or get the min, max on the axis.

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#growby" rel="noopener noreferrer" target="_blank">axis.growByðŸ“˜</a> allows you to set padding on the visibleRange.

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#visiblerangechanged" rel="noopener noreferrer" target="_blank">axis.visibleRangeChangedðŸ“˜</a> is an event or callback which fires when the range is updated.

For more info see:

- [Axis Ranging - AutoRanging](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/auto-range/)
- [Axis Ranging - Setting and Getting VisibleRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit/)
- [Axis Ranging - Listen to VisibleRange Changes](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/listen-to-visible-range-changes/)

![](out_scichartv4/3d-charts/axis-3d-api/axis-3d-api-overview/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

**NOTE**: In a 3D Axis, AutoRanging means â€˜given a fixed size of Axis in 3D world coordinates, change the VisibleRange Max/Min to fit the dataâ€™.

Dynamically positioning the camera to view all of the 3D Chart would require updating the camera position, target. See theÂ [article on Camera 3D](https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/) for more information.

### Axis 3D Tick / Label Frequency<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#axis-3d-tick--label-frequency" class="hash-link" aria-label="Direct link to Axis 3D Tick / Label Frequency" translate="no" title="Direct link to Axis 3D Tick / Label Frequency">â€‹</a>

In SciChart.js, the ticks are small marks around the chart on an axis. They Also define the spacing of Gridlines, Axis Labels and Axis Bands.

AxisBase3D tick intervals can be changed using the same APIs as SciChart 2D. For further information seeÂ [Axis 3D Gridline and Label Spacing](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-gridline-and-label-spacing-interval/).

### Axis 3D Element Styling<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#axis-3d-element-styling" class="hash-link" aria-label="Direct link to Axis 3D Element Styling" translate="no" title="Direct link to Axis 3D Element Styling">â€‹</a>

For styling gridlines, labels and titles, the rules in SciChart.js 3D are the same as 2D charts.

There are some addditional elements on the 3D chart which can be styled, such as the axis walls. For more info see the article onÂ [Styling Gridlines, Labels and Elements](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code/).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/axis-3d-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Numeric and Date Axis in SciChart3D](https://www.scichart.com/documentation/js/v4/3d-charts/axis-3d-api/numeric-and-date-axis-in-scichart-3d/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/axis-3d-api/axis-3d-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/axis-3d-api/axis-3d-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
