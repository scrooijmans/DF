<img src="out_scichartv4/typedoc/classes/numerictickprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [NumericTickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html)

# Class NumericTickProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The NumericTickProvider is a [TickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html) implementation for Numeric 2D or 3D Axis.

description  
TickProviders are responsible for calculating the interval between major and minor gridlines, ticks and labels.

- The method [getMajorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#getmajorticks) returns an array of major ticks (data-values values where SciChart will place labels and major gridlines.
- The method [getMinorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#getminorticks) returns an array of minor ticks (data-values values where SciChart will place minor gridlines.
- The method [isParamsValid](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#isparamsvalid) performs some sanity checks.
- The method [calculateTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#calculateticks) performs the actual calculation

Override these methods to create custom implementations of Tick intervals in SciChart

remarks  
See also [TickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html) for the base implementation.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html" class="tsd-signature-type">TickProvider</a>
  - NumericTickProvider

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#calculateticks" class="tsd-kind-icon">calculateTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#getmajorticks" class="tsd-kind-icon">getMajorTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#getminorticks" class="tsd-kind-icon">getMinorTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#isparamsvalid" class="tsd-kind-icon">isParamsValid</a>

## Constructors

### constructor

- new NumericTickProvider(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html" class="tsd-signature-type">NumericTickProvider</a>

<!-- -->

- Creates an instance of a NumericTickProvider

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 WebAssembly Rendering Engine

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html" class="tsd-signature-type">NumericTickProvider</a>

## Properties

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

Gets the parent [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) this TickProvider is attached to

## Methods

### attachedToAxis

- attachedToAxis(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>): void

<!-- -->

- Called when the TickProvider is attached to an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

  #### Returns void

### Protected calculateTicks

- calculateTicks(visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, delta: number, majorDelta: number): number\[\]

<!-- -->

- summary  
  Performs the Numeric tick calculation

  #### Parameters

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) which is the minimum / maximum range visible on the Axis.

  - ##### delta: number

    The delta we are calculating for (could be major or minor delta)

  - ##### majorDelta: number

    The current [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) which is the difference between major gridlines requested by the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns number\[\]

### detachedFromAxis

- detachedFromAxis(): void

<!-- -->

- Called when the TickProvider is attached from an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns void

### getMajorTicks

- getMajorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### minorDelta: number

  - ##### majorDelta: number

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  #### Returns number\[\]

### getMinorTicks

- getMinorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- inheritdoc  

  #### Parameters

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
  If this method returns false, then we should not process or compute major/minor gridlines, but instead should return empty array `[]` in [getMajorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#getmajorticks) / [getMinorTicks](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html#getminorticks)

  #### Parameters

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) which is the minimum / maximum range visible on the Axis.

  - ##### deltaRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) and [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) which is the difference between minor and major gridlines requested by the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns boolean

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
