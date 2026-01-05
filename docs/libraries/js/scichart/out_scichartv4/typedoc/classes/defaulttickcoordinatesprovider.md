<img src="out_scichartv4/typedoc/classes/defaulttickcoordinatesprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DefaultTickCoordinatesProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html)

# Class DefaultTickCoordinatesProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/tickcoordinatesprovider.html" class="tsd-signature-type">TickCoordinatesProvider</a>
  - DefaultTickCoordinatesProvider
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/statictickcoordinatesprovider.html" class="tsd-signature-type">StaticTickCoordinatesProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html#parentaxis" class="tsd-kind-icon">parentAxis</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html#attachedtoaxis" class="tsd-kind-icon">attachedToAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html#detachedfromaxis" class="tsd-kind-icon">detachedFromAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html#gettickcoordinates" class="tsd-kind-icon">getTickCoordinates</a>

## Properties

### parentAxis

parentAxis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

The parent [AxisCore](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html). This will be set once [attachedToAxis](https://www.scichart.com/documentation/js/v4/typedoc/classes/defaulttickcoordinatesprovider.html#attachedtoaxis) is called

## Methods

### attachedToAxis

- attachedToAxis(axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>): void

<!-- -->

- Called when the [TickCoordinatesProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickcoordinatesprovider.html) is attached to an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Parameters

  - ##### axis: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html" class="tsd-signature-type">AxisCore</a>

    The Axis we are attached to.

  #### Returns void

### detachedFromAxis

- detachedFromAxis(): void

<!-- -->

- Called when the [TickCoordinatesProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/tickcoordinatesprovider.html) is detached from an [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html)

  #### Returns void

### getTickCoordinates

- getTickCoordinates(majorTicks: number\[\], minorTicks: number\[\]): { majorTickCoords: number\[\]; majorTickOverrides: number\[\]; minorTickCoords: number\[\]; minorTickOverRides: number\[\] }

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### majorTicks: number\[\]

  - ##### minorTicks: number\[\]

  #### Returns { majorTickCoords: number\[\]; majorTickOverrides: number\[\]; minorTickCoords: number\[\]; minorTickOverRides: number\[\] }

  - ##### majorTickCoords: number\[\]

  - ##### majorTickOverrides: number\[\]

  - ##### minorTickCoords: number\[\]

  - ##### minorTickOverRides: number\[\]

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
