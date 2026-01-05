<img src="out_scichartv4/typedoc/classes/genericanimation_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [GenericAnimation](https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html)

# Class GenericAnimation\<T\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
Defines Animations that can be applied directly to a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) in SciChart's High Performance Real-time <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript Charts</a>

remarks  
When creating the animation, use the options to specify what to update

### Type parameters

- #### T

### Hierarchy

- GenericAnimation

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimation.html" class="tsd-signature-type">IGenericAnimation</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#delay" class="tsd-kind-icon">delay</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#duration" class="tsd-kind-icon">duration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#ease" class="tsd-kind-icon">ease</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#from" class="tsd-kind-icon">from</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#onanimate" class="tsd-kind-icon">onAnimate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#oncompleted" class="tsd-kind-icon">onCompleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#to" class="tsd-kind-icon">to</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#iscomplete" class="tsd-kind-icon">isComplete</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#cancel" class="tsd-kind-icon">cancel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#reset" class="tsd-kind-icon">reset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html#update" class="tsd-kind-icon">update</a>

## Constructors

### constructor

- new GenericAnimation(options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html" class="tsd-signature-type">IGenericAnimationOptions</a>\<T\>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html" class="tsd-signature-type">GenericAnimation</a>

<!-- -->

- #### Parameters

  - ##### options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/igenericanimationoptions.html" class="tsd-signature-type">IGenericAnimationOptions</a>\<T\>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/genericanimation.html" class="tsd-signature-type">GenericAnimation</a>

## Properties

### delay

delay: number = 0

The animation delay

### duration

duration: number = 1000

The animation duration

### ease

ease: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#teasingfn" class="tsd-signature-type">TEasingFn</a> = easing.linear

Sets the animation easing function

### from

from: T

The initial state of the animation parameter

### Readonly id

id: string

### onAnimate

onAnimate: (from: T, to: T, progress: number) =\> void

The function that is called each frame. Update the target of the animation here. progress is between 0 and 1.

#### Type declaration

- - (from: T, to: T, progress: number): void

  <!-- -->

  - #### Parameters

    - ##### from: T

    - ##### to: T

    - ##### progress: number

    #### Returns void

### onCompleted

onCompleted: () =\> void

The function that is called after the animation has finished.

#### Type declaration

- - (): void

  <!-- -->

  - #### Returns void

### to

to: T

The final state of the animation parameter

## Accessors

### isComplete

- get isComplete(): boolean

<!-- -->

- #### Returns boolean

## Methods

### cancel

- cancel(): void

<!-- -->

- Cancel the animation. onCompleted will not be called

  #### Returns void

### reset

- reset(): void

<!-- -->

- Reset the animation to its initial state. If reset while running or onCompleted, the animation will remain in the list and run again.

  #### Returns void

### update

- update(timeElapsed: number): void

<!-- -->

- Advance the animation according to the time elapsed since the last frame

  #### Parameters

  - ##### timeElapsed: number

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
