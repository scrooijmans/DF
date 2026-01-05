<img src="out_scichartv4/typedoc/interfaces/iinneraxislayoutstrategyoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iinneraxislayoutstrategyoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IInnerAxisLayoutStrategyOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iinneraxislayoutstrategyoptions.html)

# Interface IInnerAxisLayoutStrategyOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options passed to the constructor of a [BaseCenteredAxisLayoutStrategy](https://www.scichart.com/documentation/js/v4/typedoc/classes/basecenteredaxislayoutstrategy.html), used to configure it at instantiation time

### Hierarchy

- IInnerAxisLayoutStrategyOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iinneraxislayoutstrategyoptions.html#axisposition" class="tsd-kind-icon">axisPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iinneraxislayoutstrategyoptions.html#coordinatemode" class="tsd-kind-icon">coordinateMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iinneraxislayoutstrategyoptions.html#orthogonalaxisid" class="tsd-kind-icon">orthogonalAxisId</a>

## Properties

### Optional axisPosition

axisPosition: number

description  
the coordinate of the Layout Area anchor point

remarks  
The axisPosition obeys [coordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iinneraxislayoutstrategyoptions.html#coordinatemode) which defines whether it is a pixel, data-value or relative coordinate

### Optional coordinateMode

coordinateMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/einneraxisplacementcoordinatemode.html" class="tsd-signature-type">EInnerAxisPlacementCoordinateMode</a>

The Coordinate mode. See [EInnerAxisPlacementCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/einneraxisplacementcoordinatemode.html) for a list of values

remarks  
Want to display an annotation stretching across the entire width (or height) or the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)? The [EInnerAxisPlacementCoordinateMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/einneraxisplacementcoordinatemode.html) enum has options which allow for relative, absolute or pixel coordinates which define annotation placement.

### Optional orthogonalAxisId

orthogonalAxisId: string

The id for the vertical or horizontal axis which is used for positioning the central axes

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
