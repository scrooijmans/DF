<img src="out_scichartv4/typedoc/classes/scatteranimation_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ScatterAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html)

# Class ScatterAnimation

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>
  - ScatterAnimation

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#delay" class="tsd-kind-icon">delay</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#duration" class="tsd-kind-icon">duration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#ease" class="tsd-kind-icon">ease</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#iscomplete" class="tsd-kind-icon">isComplete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#isfadeeffectanimation" class="tsd-kind-icon">isFadeEffectAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#isonstartanimation" class="tsd-kind-icon">isOnStartAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#oncompleted" class="tsd-kind-icon">onCompleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#reverse" class="tsd-kind-icon">reverse</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#styles" class="tsd-kind-icon">styles</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#typemap" class="tsd-kind-icon">typeMap</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#isdataseriesanimation" class="tsd-kind-icon">isDataSeriesAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#isstyleanimation" class="tsd-kind-icon">isStyleAnimation</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#calculateanimationvalues" class="tsd-kind-icon">calculateAnimationValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#calculatedataseriesanimationvalues" class="tsd-kind-icon">calculateDataSeriesAnimationValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#getseriesstyle" class="tsd-kind-icon">getSeriesStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html#updateseriesproperties" class="tsd-kind-icon">updateSeriesProperties</a>

## Constructors

### constructor

- new ScatterAnimation(options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscatteranimationoptions.html" class="tsd-signature-type">IScatterAnimationOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html" class="tsd-signature-type">ScatterAnimation</a>

<!-- -->

- #### Parameters

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscatteranimationoptions.html" class="tsd-signature-type">IScatterAnimationOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatteranimation.html" class="tsd-signature-type">ScatterAnimation</a>

## Properties

### dataSeries

dataSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" class="tsd-signature-type">XyDataSeries</a>

inheritdoc  

### delay

delay: number = 0

The animation delay in ms

### duration

duration: number = 3000

The animation duration in ms

### ease

ease: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> = easing.linear

Sets the animation easing function

### isComplete

isComplete: true

### isFadeEffectAnimation

isFadeEffectAnimation: boolean = false

Enables fade effect animation

### isOnStartAnimation

isOnStartAnimation: boolean = false

Enables the animation for the effects like Wave, Sweep etc.

### Optional onCompleted

onCompleted: () =\> void

A function that is called after the animation has finished.

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### reverse

reverse: boolean

Set true to reverse the animation

### styles

styles: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseanimationstyle.html" class="tsd-signature-type">BaseAnimationStyle</a>

Styles of the [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) to be animated

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html#style" class="tsd-signature-type">Style</a> = EAnimationType.Style

inheritdoc  

### Protected typeMap

typeMap: Map\<string, string\> = new Map\<string, string\>()

## Accessors

### isDataSeriesAnimation

- get isDataSeriesAnimation(): boolean

<!-- -->

- Return flag if it is dataSeries animation

  #### Returns boolean

### isStyleAnimation

- get isStyleAnimation(): boolean

<!-- -->

- Return flag if it is styles animation

  #### Returns boolean

## Methods

### calculateAnimationValues

- calculateAnimationValues(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, originalValues: SCRTDoubleVector, animationValues: SCRTDoubleVector, progress: number, noZeroLine?: boolean): void

<!-- -->

- Runs on start up animation to update animation vectors

  #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### originalValues: SCRTDoubleVector

    original values

  - ##### animationValues: SCRTDoubleVector

    calculated values used for the animation

  - ##### progress: number

    Current animation progress

  - ##### Default value noZeroLine: boolean = false

    Sets zeroLine = 0, is used for XyzDataSeries

  #### Returns void

### calculateDataSeriesAnimationValues

- calculateDataSeriesAnimationValues(wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, initialValues: SCRTDoubleVector, finalValues: SCRTDoubleVector, interpolatedValues: SCRTDoubleVector, progress: number): void

<!-- -->

- Runs for data animation to update animation vectors

  #### Parameters

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### initialValues: SCRTDoubleVector

    The initial vector

  - ##### finalValues: SCRTDoubleVector

    The final vector

  - ##### interpolatedValues: SCRTDoubleVector

    The vector which will be updated with interpolated values

  - ##### progress: number

    Current animation progress

  #### Returns void

### getSeriesStyle

- getSeriesStyle(renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseanimationstyle.html" class="tsd-signature-type">BaseAnimationStyle</a>

<!-- -->

- Returns [BaseAnimationStyle](https://www.scichart.com/documentation/js/v4/typedoc/classes/baseanimationstyle.html) object which is used to create initial styles

  #### Parameters

  - ##### renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

    The renderable series

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseanimationstyle.html" class="tsd-signature-type">BaseAnimationStyle</a>

### toJSON

- toJSON(): { options: {}; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html" class="tsd-signature-type">EAnimationType</a> }

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns { options: {}; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html" class="tsd-signature-type">EAnimationType</a> }

  - ##### options: {}

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html" class="tsd-signature-type">EAnimationType</a>

### updateSeriesProperties

- updateSeriesProperties(renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>, initialStyles: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseanimationstyle.html" class="tsd-signature-type">BaseAnimationStyle</a>, animationProgress: number): void

<!-- -->

- Updates properties of [IRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

  #### Parameters

  - ##### renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

    The renderable series to be animated

  - ##### initialStyles: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseanimationstyle.html" class="tsd-signature-type">BaseAnimationStyle</a>

    The initial styles

  - ##### animationProgress: number

    The animation progress, should be between 0 and 1

  #### Returns void

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
