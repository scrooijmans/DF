<img src="out_scichartv4/typedoc/classes/columnanimation_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ColumnAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html)

# Class ColumnAnimation

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimation.html" class="tsd-signature-type">SeriesAnimation</a>
  - ColumnAnimation

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#dataseries" class="tsd-kind-icon">dataSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#delay" class="tsd-kind-icon">delay</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#duration" class="tsd-kind-icon">duration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#ease" class="tsd-kind-icon">ease</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#iscomplete" class="tsd-kind-icon">isComplete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#isfadeeffectanimation" class="tsd-kind-icon">isFadeEffectAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#isonstartanimation" class="tsd-kind-icon">isOnStartAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#oncompleted" class="tsd-kind-icon">onCompleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#reverse" class="tsd-kind-icon">reverse</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#styles" class="tsd-kind-icon">styles</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#typemap" class="tsd-kind-icon">typeMap</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#isdataseriesanimation" class="tsd-kind-icon">isDataSeriesAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#isstyleanimation" class="tsd-kind-icon">isStyleAnimation</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#calculateanimationvalues" class="tsd-kind-icon">calculateAnimationValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#calculatedataseriesanimationvalues" class="tsd-kind-icon">calculateDataSeriesAnimationValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#getseriesstyle" class="tsd-kind-icon">getSeriesStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html#updateseriesproperties" class="tsd-kind-icon">updateSeriesProperties</a>

## Constructors

### constructor

- new ColumnAnimation(options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnanimationoptions.html" class="tsd-signature-type">IColumnAnimationOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html" class="tsd-signature-type">ColumnAnimation</a>

<!-- -->

- #### Parameters

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnanimationoptions.html" class="tsd-signature-type">IColumnAnimationOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimation.html" class="tsd-signature-type">ColumnAnimation</a>

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

styles: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimationstyle.html" class="tsd-signature-type">ColumnAnimationStyle</a>

inheritdoc  

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

- getSeriesStyle(rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" class="tsd-signature-type">FastColumnRenderableSeries</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimationstyle.html" class="tsd-signature-type">ColumnAnimationStyle</a>

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" class="tsd-signature-type">FastColumnRenderableSeries</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimationstyle.html" class="tsd-signature-type">ColumnAnimationStyle</a>

### toJSON

- toJSON(): { options: {}; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html" class="tsd-signature-type">EAnimationType</a> }

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns { options: {}; type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html" class="tsd-signature-type">EAnimationType</a> }

  - ##### options: {}

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationtype.html" class="tsd-signature-type">EAnimationType</a>

### updateSeriesProperties

- updateSeriesProperties(rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" class="tsd-signature-type">FastColumnRenderableSeries</a>, initialStyles: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimationstyle.html" class="tsd-signature-type">ColumnAnimationStyle</a>, animationProgress: number): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### rs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html" class="tsd-signature-type">FastColumnRenderableSeries</a>

  - ##### initialStyles: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnanimationstyle.html" class="tsd-signature-type">ColumnAnimationStyle</a>

  - ##### animationProgress: number

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
