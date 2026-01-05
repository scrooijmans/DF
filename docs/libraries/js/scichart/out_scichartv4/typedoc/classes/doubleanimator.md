<img src="out_scichartv4/typedoc/classes/doubleanimator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/doubleanimator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DoubleAnimator](https://www.scichart.com/documentation/js/v4/typedoc/classes/doubleanimator.html)

# Class DoubleAnimator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A class for animating a double-precision (number) value

### Hierarchy

- DoubleAnimator

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/doubleanimator.html#animate" class="tsd-kind-icon">animate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/doubleanimator.html#interpolate" class="tsd-kind-icon">interpolate</a>

## Methods

### Static animate

- animate(from: number, to: number, durationMs: number, onAnimate: (value: number) =\> void, onCompleted: () =\> void, easingFunction?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationtoken.html" class="tsd-signature-type">AnimationToken</a>

<!-- -->

- deprecated  
  Instead create an [GenericAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html) and pass it to sciChartSurface.addAnimation

  #### Parameters

  - ##### from: number

  - ##### to: number

  - ##### durationMs: number

  - ##### onAnimate: (value: number) =\> void

    - - (value: number): void

      <!-- -->

      - #### Parameters

        - ##### value: number

        #### Returns void

  - ##### onCompleted: () =\> void

    - - (): void

      <!-- -->

      - #### Returns void

  - ##### Default value easingFunction: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> = easing.outExpo

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationtoken.html" class="tsd-signature-type">AnimationToken</a>

### Static interpolate

- interpolate(from: number, to: number, interpolationFactor: number): number

<!-- -->

- #### Parameters

  - ##### from: number

  - ##### to: number

  - ##### interpolationFactor: number

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
