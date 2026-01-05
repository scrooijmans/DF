<img src="out_scichartv4/typedoc/classes/logarithmicdeltacalculator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [LogarithmicDeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html)

# Class LogarithmicDeltaCalculator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The LogarithmicDeltaCalculator is responsible for calculating [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) and [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) on [LogarithmicAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#logarithmicaxis) types.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html" class="tsd-signature-type">DeltaCalculator</a>
  - LogarithmicDeltaCalculator

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#ishighprecisionticks" class="tsd-kind-icon">isHighPrecisionTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#logarithmicbase" class="tsd-kind-icon">logarithmicBase</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#getdeltafromrange" class="tsd-kind-icon">getDeltaFromRange</a>

## Constructors

### constructor

- new LogarithmicDeltaCalculator(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html" class="tsd-signature-type">LogarithmicDeltaCalculator</a>

<!-- -->

- Creates an instance of the [LogarithmicDeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html)

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) or {@link TSciChart2D \| SciChart 2D WebAssembly Context} containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html" class="tsd-signature-type">LogarithmicDeltaCalculator</a>

## Properties

### isHighPrecisionTicks

isHighPrecisionTicks: boolean = true

### logarithmicBase

logarithmicBase: number = 10

Gets or sets the logarithmic base. Defaults to 10

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

The parent [AxisCore](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html). This will be set once [attachedToAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html#attachedtoaxis) is called

## Methods

### attachedToAxis

- attachedToAxis(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>): void

<!-- -->

- Called when the [DeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html) is attached to an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

    The Axis we are attached to.

  #### Returns void

### detachedFromAxis

- detachedFromAxis(): void

<!-- -->

- Called when the [DeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html) is detached from [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns void

### getDeltaFromRange

- getDeltaFromRange(min: number, max: number, minorsPerMajor: number, maxTicks: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### min: number

  - ##### max: number

  - ##### minorsPerMajor: number

  - ##### maxTicks: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

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
