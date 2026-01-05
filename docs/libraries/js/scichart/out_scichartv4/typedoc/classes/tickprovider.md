<img src="out_scichartv4/typedoc/classes/tickprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [TickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html)

# Class TickProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The TickProvider is a base class for calculating ticks (interval between major and minor gridlines, ticks and labels).

description  
TickProviders are responsible for calculating the interval between major and minor gridlines, ticks and labels.

The method {@getMajorTicks} returns an array of major ticks (data-values values where SciChart will place labels and major gridlines). The method {@getMinorTicks} returns an array of minor ticks (data-values values where SciChart will place minor gridlines). The method {@attachedToAxis} is called when the TickProvider is attached to an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html).

Override these methods to create custom implementations of Tick intervals in SciChart or use our built-in [NumericTickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html)

remarks  
TickProviders are shared between 2D & 3D Charts. See also [NumericTickProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html) for a concrete implementation.

### Hierarchy

- TickProvider
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numerictickprovider.html" class="tsd-signature-type">NumericTickProvider</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/logarithmictickprovider.html" class="tsd-signature-type">LogarithmicTickProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html#getmajorticks" class="tsd-kind-icon">getMajorTicks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickprovider.html#getminorticks" class="tsd-kind-icon">getMinorTicks</a>

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

### detachedFromAxis

- detachedFromAxis(): void

<!-- -->

- Called when the TickProvider is attached from an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns void

### Abstract getMajorTicks

- getMajorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- summary  
  Gets an array of major ticks (data-values values where SciChart will place labels and major gridlines).

  description  
  Major ticks are data-values where we will place the major gridlines and labels. For example. if the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) has a [visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) of 100..200 and we want to place gridlines at 100,120,140,160,180,200, then the getMajorTicks() method should return an array with `[100,120,140,160,180,200]`.

  link  
  AxisCore \| Axis}

  #### Parameters

  - ##### minorDelta: number

    The current [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) which is the difference between minor gridlines requested by the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  - ##### majorDelta: number

    The current [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) which is the difference between major gridlines requested by the {

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) which is the minimum / maximum range visible on the Axis.

  #### Returns number\[\]

  The array of major ticks, e.g. if we want to place gridlines at 100,120,140,160,180,200, then the getMajorTicks() method should return an array with `[100,120,140,160,180,200]`.

### Abstract getMinorTicks

- getMinorTicks(minorDelta: number, majorDelta: number, visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): number\[\]

<!-- -->

- summary  
  Gets an array of minor ticks (data-values values where SciChart will place minor gridlines).

  description  
  Minor ticks are data-values where we will place the minor gridlines. For example. if the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) has a [visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) of 100..200 and we want to place minor gridlines every 5, then the getMinorTicks() method should return an array with `[105, 110, 115]` etc...

  remarks  
  getMinorTicks should not include locations where major ticks lie.

  link  
  AxisCore \| Axis}

  #### Parameters

  - ##### minorDelta: number

    The current [AxisCore.minorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#minordelta) which is the difference between minor gridlines requested by the [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  - ##### majorDelta: number

    The current [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) which is the difference between major gridlines requested by the {

  - ##### visibleRange: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The current [AxisCore.majorDelta](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#majordelta) which is the minimum / maximum range visible on the Axis.

  #### Returns number\[\]

  The array of minor ticks, e.g. if we want to place minor gridlines every 5, then the getMinorTicks() method should return an array with `[105, 110, 115]` etc...

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
