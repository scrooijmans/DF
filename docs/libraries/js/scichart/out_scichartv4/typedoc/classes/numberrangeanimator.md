<img src="out_scichartv4/typedoc/classes/numberrangeanimator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrangeanimator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [NumberRangeAnimator](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrangeanimator.html)

# Class NumberRangeAnimator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A class for animating a value of type [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html). Used throughout SciChart to animate [AxisCore.visibleRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html#visiblerange) when zooming or panning.

### Hierarchy

- NumberRangeAnimator

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrangeanimator.html#animate" class="tsd-kind-icon">animate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrangeanimator.html#interpolate" class="tsd-kind-icon">interpolate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrangeanimator.html#interpolatelog" class="tsd-kind-icon">interpolateLog</a>

## Methods

### Static animate

- animate(from: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, to: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, durationMs: number, onAnimate: (value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>) =\> void, onCompleted: () =\> void, easingFunction?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a>, isLog?: boolean): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html" class="tsd-signature-type">GenericAnimation</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>\>

<!-- -->

- Animates a [NumberRange](https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html) with a start, to value over a specified duration and with an optional completed and easing function

  #### Parameters

  - ##### from: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The start value to animate

  - ##### to: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

    The end value to animate

  - ##### durationMs: number

    The duration of the animation in milliseconds

  - ##### onAnimate: (value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>) =\> void

    A callback function which is called with intermediate values

    - - (value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>): void

      <!-- -->

      - #### Parameters

        - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

        #### Returns void

  - ##### onCompleted: () =\> void

    A callback function which is called when the animation completes

    - - (): void

      <!-- -->

      - #### Returns void

  - ##### Default value easingFunction: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> = easing.outExpo

    An optional easing function. See [IEasingMap](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieasingmap.html) for a list of values

  - ##### Default value isLog: boolean = false

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html" class="tsd-signature-type">GenericAnimation</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>\>

### Static interpolate

- interpolate(from: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, to: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, interpolationFactor: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- #### Parameters

  - ##### from: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### to: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### interpolationFactor: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

### Static interpolateLog

- interpolateLog(from: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, to: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>, interpolationFactor: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

<!-- -->

- #### Parameters

  - ##### from: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### to: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numberrange.html" class="tsd-signature-type">NumberRange</a>

  - ##### interpolationFactor: number

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
