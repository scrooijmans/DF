<img src="out_scichartv4/typedoc/classes/logarithmictickprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [LogarithmicTickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html)

# Class LogarithmicTickProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The LogarithmicTickProvider is a [TickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html) implementation for Logarithmic 2D or 3D Axis.

description  
TickProviders are responsible for calculating the interval between major and minor gridlines, ticks and labels.

- The method [getMajorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getmajorticks) returns an array of major ticks (data-values values where SciChart will place labels and major gridlines.
- The method [getMinorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getminorticks) returns an array of minor ticks (data-values values where SciChart will place minor gridlines.
- The method [calculateTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#calculateticks) performs the actual calculation

Override these methods to create custom implementations of Tick intervals in SciChart

remarks  
See also [TickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html) for the base implementation.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html" class="tsd-signature-type">TickProvider</a>
  - LogarithmicTickProvider

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#ishighprecisionticks" class="tsd-kind-icon">isHighPrecisionTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#logarithmicbase" class="tsd-kind-icon">logarithmicBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#majortickmode" class="tsd-kind-icon">majorTickMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#minortickmode" class="tsd-kind-icon">minorTickMode</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getmajorticks" class="tsd-kind-icon">getMajorTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getminorticks" class="tsd-kind-icon">getMinorTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getroundnumbermajorticks" class="tsd-kind-icon">getRoundNumberMajorTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#isparamsvalid" class="tsd-kind-icon">isParamsValid</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#roundnum" class="tsd-kind-icon">roundNum</a>

## Constructors

### constructor

- new LogarithmicTickProvider(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html" class="tsd-signature-type">LogarithmicTickProvider</a>

<!-- -->

- #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html" class="tsd-signature-type">LogarithmicTickProvider</a>

## Properties

### isHighPrecisionTicks

isHighPrecisionTicks: boolean = true

### logarithmicBase

logarithmicBase: number = 10

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

Gets the parent [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) this TickProvider is attached to

## Accessors

### majorTickMode

- get majorTickMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html" class="tsd-signature-type">ELogarithmicMajorTickMode</a>
- set majorTickMode(mode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html" class="tsd-signature-type">ELogarithmicMajorTickMode</a>): void

<!-- -->

- Gets or sets the mode for Major ticks using [ELogarithmicMajorTickMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html) Equally spaced (best for large ranges) or Round numbers (better for small ranges)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html" class="tsd-signature-type">ELogarithmicMajorTickMode</a>

- Gets or sets the mode for Major ticks using [ELogarithmicMajorTickMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html) Equally spaced (best for large ranges) or Round numbers (better for small ranges)

  #### Parameters

  - ##### mode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicmajortickmode.html" class="tsd-signature-type">ELogarithmicMajorTickMode</a>

  #### Returns void

### minorTickMode

- get minorTickMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicminortickmode.html" class="tsd-signature-type">ELogarithmicMinorTickMode</a>
- set minorTickMode(mode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicminortickmode.html" class="tsd-signature-type">ELogarithmicMinorTickMode</a>): void

<!-- -->

- Gets or sets the mode for minor ticks using [ELogarithmicMinorTickMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicminortickmode.html), Linear (default, best for smaller ranges), Logarithmic (better for very large ranges) or Auto (switches from linear to Logarithmic when the visible range is such that the first linear minor tick would be more than 70% of the major tick)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicminortickmode.html" class="tsd-signature-type">ELogarithmicMinorTickMode</a>

- Gets or sets the mode for minor ticks using [ELogarithmicMinorTickMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicminortickmode.html), Linear (default, best for smaller ranges), Logarithmic (better for very large ranges) or Auto (switches from linear to Logarithmic when the visible range is such that the first linear minor tick would be more than 70% of the major tick)

  #### Parameters

  - ##### mode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elogarithmicminortickmode.html" class="tsd-signature-type">ELogarithmicMinorTickMode</a>

  #### Returns void

## Methods

### attachedToAxis

- attachedToAxis(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>): void

<!-- -->

- Called when the TickProvider is attached to an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

  #### Returns void

### detachedFromAxis

- detachedFromAxis(): void

<!-- -->

- Called when the TickProvider is attached from an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns void

### getMajorTicks

- getMajorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- #### Parameters

  - ##### minorDelta: number

  - ##### majorDelta: number

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns number\[\]

### getMinorTicks

- getMinorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- #### Parameters

  - ##### minorDelta: number

  - ##### majorDelta: number

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns number\[\]

### getRoundNumberMajorTicks

- getRoundNumberMajorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- #### Parameters

  - ##### minorDelta: number

  - ##### majorDelta: number

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns number\[\]

### Protected isParamsValid

- isParamsValid(visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, deltaRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): boolean

<!-- -->

- summary  
  Performs sanity checks to see if parameters are valid.

  description  
  If this method returns false, then we should not process or compute major/minor gridlines, but instead should return empty array `[]` in [getMajorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getmajorticks) / [getMinorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html#getminorticks)

  #### Parameters

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) which is the minimum / maximum range visible on the Axis.

  - ##### deltaRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) and [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) which is the difference between minor and major gridlines requested by the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns boolean

### roundNum

- roundNum(value: number, difference?: number): number

<!-- -->

- #### Parameters

  - ##### value: number

  - ##### Optional difference: number

  #### Returns number

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
