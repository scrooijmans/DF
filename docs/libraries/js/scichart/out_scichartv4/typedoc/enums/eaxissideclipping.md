<img src="out_scichartv4/typedoc/enums/eaxissideclipping_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxissideclipping.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [EAxisSideClipping](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxissideclipping.html)

# Enumeration EAxisSideClipping

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the Clipping rule for <a href="https://www.scichart.com/javascript-chart-features" class="external">Javascript 3D Chart</a> Axis

remarks  
Set Clipping rule on the [AxisBase3D.negativeSideClipping](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#negativesideclipping) and [AxisBase3D.positiveSideClipping](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase3d.html#positivesideclipping)

## Index

### Enumeration members

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxissideclipping.html#default" class="tsd-kind-icon">Default</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxissideclipping.html#none" class="tsd-kind-icon">None</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxissideclipping.html#visiblerange" class="tsd-kind-icon">VisibleRange</a>

## Enumeration members

### Default

Default: = "Default"

Default <a href="https://www.scichart.com/javascript-chart-features" class="external">Javascript 3D Chart</a> Axis clipping is [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) for X and Z Axis, and None for Y Axis

remarks  
Data from <a href="https://www.scichart.com/javascript-chart-features" class="external">3D Javascript Chart</a> types can spill out beyond the confines of the Axis for the YAxis (up/down) but be clipped to the [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) for the Z/X Axis

### None

None: = "None"

3D Axis Clipping is disabled

remarks  
Data from <a href="https://www.scichart.com/javascript-chart-features" class="external">3D Javascript Chart</a> types can spill out beyond the confines of the Axis.

### VisibleRange

VisibleRange: = "VisibleRange"

RenderableSeries are clipped by [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange)

remarks  
If a <a href="https://www.scichart.com/javascript-chart-features" class="external">Javascript Chart</a> type such as a 3D Scatter or 3D Surface mesh plot are plotted on this axis, the data will be clipped to be invisible outside the [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange)

## Legend

- Constructor
- Property
- Method
- Accessor

<!-- -->

- Inherited constructor
- Inherited property
- Inherited method
- Inherited accessor

<!-- -->

- Property
- Method

<!-- -->

- Protected property
- Protected method

<!-- -->

- Static property
- Static method

Generated using <a href="https://typedoc.org/" target="_blank">TypeDoc</a>
