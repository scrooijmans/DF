<img src="out_scichartv4/typedoc/classes/deltacalculator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html)

# Class DeltaCalculator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
A base class for Delta Calculators within SciChart's 2D & 3D Charts.

description  
The [DeltaCalculator](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html) is responsible for calculating the min and max deltas on an axis.

A delta is the spacing between two gridlines, so the Major Delta is the spacing between major grid lines and the Minor Delta is the spacing between minor gridlines.

This calculator class computes these and they are later stored on [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) and [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) properties.

### Hierarchy

- DeltaCalculator
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmicdeltacalculator.html" class="tsd-signature-type">LogarithmicDeltaCalculator</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericdeltacalculator.html" class="tsd-signature-type">NumericDeltaCalculator</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html#getdeltafromrange" class="tsd-kind-icon">getDeltaFromRange</a>

## Properties

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

The parent [AxisCore](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html). This will be set once [attachedToAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/deltacalculator.html#attachedtoaxis) is called

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

### Abstract getDeltaFromRange

- getDeltaFromRange(min: number, max: number, minorsPerMajor: number, maxTicks: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- Gets a Delta from a visiblerange and with the provided properties

  #### Parameters

  - ##### min: number

    the [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) minimum

  - ##### max: number

    the [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) maximum

  - ##### minorsPerMajor: number

    A hint of how many minor gridlines you want per major gridline

  - ##### maxTicks: number

    A hint of the maximum number of major gridlines you want on the axis (result will vary depending on zoom level)

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
