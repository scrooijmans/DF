<img src="out_scichartv4/typedoc/classes/datetimedeltacalculator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DateTimeDeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html)

# Class DateTimeDeltaCalculator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The DateTimeDeltaCalculator is respinsible for calculating [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) and [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) on [NumericAxis](https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis) types.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericdeltacalculator.html" class="tsd-signature-type">NumericDeltaCalculator</a>
  - DateTimeDeltaCalculator

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#minticks" class="tsd-kind-icon">minTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#possibledeltas" class="tsd-kind-icon">possibleDeltas</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#getdeltafromrange" class="tsd-kind-icon">getDeltaFromRange</a>

## Constructors

### constructor

- new DateTimeDeltaCalculator(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideltacalculatoroptions.html" class="tsd-signature-type">IDeltaCalculatorOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html" class="tsd-signature-type">DateTimeDeltaCalculator</a>

<!-- -->

- #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideltacalculatoroptions.html" class="tsd-signature-type">IDeltaCalculatorOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html" class="tsd-signature-type">DateTimeDeltaCalculator</a>

## Properties

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

The parent [AxisCore](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html). This will be set once [attachedToAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/datetimedeltacalculator.html#attachedtoaxis) is called

## Accessors

### minTicks

- get minTicks(): number
- set minTicks(value: number): void

<!-- -->

- #### Returns number

- #### Parameters

  - ##### value: number

  #### Returns void

### possibleDeltas

- get possibleDeltas(): number\[\]
- set possibleDeltas(value: number\[\]): void

<!-- -->

- Gets or sets deltas array

  #### Returns number\[\]

- Gets or sets deltas array

  #### Parameters

  - ##### value: number\[\]

  #### Returns void

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
