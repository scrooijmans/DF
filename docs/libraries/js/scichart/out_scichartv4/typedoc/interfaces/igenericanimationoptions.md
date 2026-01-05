<img src="out_scichartv4/typedoc/interfaces/igenericanimationoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IGenericAnimationOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html)

# Interface IGenericAnimationOptions\<T\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options passed to a [GenericAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html) at construction time

### Type parameters

- #### T

### Hierarchy

- IGenericAnimationOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#delay" class="tsd-kind-icon">delay</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#duration" class="tsd-kind-icon">duration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#ease" class="tsd-kind-icon">ease</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#from" class="tsd-kind-icon">from</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#onanimate" class="tsd-kind-icon">onAnimate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#oncompleted" class="tsd-kind-icon">onCompleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#setinitialvalueimmediately" class="tsd-kind-icon">setInitialValueImmediately</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html#to" class="tsd-kind-icon">to</a>

## Properties

### Optional delay

delay: number

Time in ms before the animation is started. Animations are advanced on each frame, so this time is a minimum, not exact.

### Optional duration

duration: number

Time in ms that the animation will run for. Animations are advanced on each frame, so this time is a minimum, not exact

### Optional ease

ease: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> \| string

An easing function used to calculate progress

### from

from: T

The initial state of the animation parameter

### Optional id

id: string

An identifier for the animation. Will be set to a Guid if not specified

### onAnimate

onAnimate: (from: T, to: T, progress: number) =\> void

A function that is called each frame. Update the target of the animation here. progress is between 0 and 1.

#### Type declaration

- - (from: T, to: T, progress: number): void

  <!-- -->

  - #### Parameters

    - ##### from: T

    - ##### to: T

    - ##### progress: number

    #### Returns void

### Optional onCompleted

onCompleted: () =\> void

A function that is called after the animation has finished.

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### Optional setInitialValueImmediately

setInitialValueImmediately: boolean

When true, onAnimate is called immediately with progress=0 to set the initial value

### to

to: T

The final state of the animation parameter

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
