<img src="out_scichartv4/typedoc/classes/animationfinitestatemachine_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [AnimationFiniteStateMachine](https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html)

# Class AnimationFiniteStateMachine

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- AnimationFiniteStateMachine
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesanimationfinitestatemachine.html" class="tsd-signature-type">SeriesAnimationFiniteStateMachine</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animationdelayelapsed" class="tsd-kind-icon">animationDelayElapsed</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animationdelaystarttimestamp" class="tsd-kind-icon">animationDelayStartTimestamp</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animationelapsed" class="tsd-kind-icon">animationElapsed</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animationproperty" class="tsd-kind-icon">animationProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animationstarttimestamp" class="tsd-kind-icon">animationStartTimestamp</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#stateproperty" class="tsd-kind-icon">stateProperty</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animation" class="tsd-kind-icon">animation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#animationprogress" class="tsd-kind-icon">animationProgress</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#state" class="tsd-kind-icon">state</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#is" class="tsd-kind-icon">is</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#tocompleted" class="tsd-kind-icon">toCompleted</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html#update" class="tsd-kind-icon">update</a>

## Constructors

### constructor

- new AnimationFiniteStateMachine(animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html" class="tsd-signature-type">AnimationFiniteStateMachine</a>

<!-- -->

- #### Parameters

  - ##### animation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/animationfinitestatemachine.html" class="tsd-signature-type">AnimationFiniteStateMachine</a>

## Properties

### Protected animationDelayElapsed

animationDelayElapsed: number

### Protected animationDelayStartTimestamp

animationDelayStartTimestamp: number

### Protected animationElapsed

animationElapsed: number

### Protected Readonly animationProperty

animationProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>

### Protected animationStartTimestamp

animationStartTimestamp: number

### Protected stateProperty

stateProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstate.html" class="tsd-signature-type">EAnimationState</a> = EAnimationState.InitialState

## Accessors

### animation

- get animation(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>

<!-- -->

- Gets the animation property

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ianimation.html" class="tsd-signature-type">IAnimation</a>

### animationProgress

- get animationProgress(): number

<!-- -->

- Gets the animation progress, the value from 0 to 1

  #### Returns number

### state

- get state(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstate.html" class="tsd-signature-type">EAnimationState</a>

<!-- -->

- Gets the current state

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstate.html" class="tsd-signature-type">EAnimationState</a>

## Methods

### is

- is(states: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstate.html" class="tsd-signature-type">EAnimationState</a>\[\]): boolean

<!-- -->

- Checks the current state

  #### Parameters

  - ##### states: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstate.html" class="tsd-signature-type">EAnimationState</a>\[\]

  #### Returns boolean

### toCompleted

- toCompleted(): void

<!-- -->

- Changes the state to Completed

  #### Returns void

### update

- update(timeElapsed: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstatetransition.html" class="tsd-signature-type">EAnimationStateTransition</a>

<!-- -->

- Updates the state

  #### Parameters

  - ##### timeElapsed: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eanimationstatetransition.html" class="tsd-signature-type">EAnimationStateTransition</a>

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
