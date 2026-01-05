<img src="out_scichartv4/typedoc/enums/eclipmode_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [EClipMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html)

# Enumeration EClipMode

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the clipping mode for scrolling operations found on [AxisBase2D.scroll](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#scroll)

## Index

### Enumeration members

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html#clipatextents" class="tsd-kind-icon">ClipAtExtents</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html#clipatmax" class="tsd-kind-icon">ClipAtMax</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html#clipatmin" class="tsd-kind-icon">ClipAtMin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html#none" class="tsd-kind-icon">None</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eclipmode.html#stretchatextents" class="tsd-kind-icon">StretchAtExtents</a>

## Enumeration members

### ClipAtExtents

ClipAtExtents: = "ClipAtExtents"

Clips the [AxisBase2D.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange) to not allow scrolling past the minimum or maximum of the Axis range

### ClipAtMax

ClipAtMax: = "ClipAtMax"

Clips the [AxisBase2D.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange) to not allow scrolling past the maximum of the Axis range

### ClipAtMin

ClipAtMin: = "ClipAtMin"

Clips the [AxisBase2D.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange) to not allow scrolling past the minimum of the Axis range

### None

None: = "None"

Do not clip when scrolling the Axis

remarks  
Use this to resolve issues such as scaling or stretching when the user pans or scrolls outside of the range of the data.

### StretchAtExtents

StretchAtExtents: = "StretchAtExtents"

Stretch the [AxisBase2D.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#visiblerange) when scrolling past the extents of the data.

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
