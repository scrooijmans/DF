<img src="out_scichartv4/typedoc/interfaces/iupdatesuspender_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IUpdateSuspender](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html)

# Interface IUpdateSuspender

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to an [UpdateSuspender](https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html), a class which allows nested suspend/resume operations on an [ISuspendable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isuspendable.html) target

### Hierarchy

- IUpdateSuspender

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html" class="tsd-signature-type">UpdateSuspender</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html#issuspended" class="tsd-kind-icon">isSuspended</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html#onresumed" class="tsd-kind-icon">onResumed</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html#lock" class="tsd-kind-icon">lock</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html#resume" class="tsd-kind-icon">resume</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html#suspend" class="tsd-kind-icon">suspend</a>

## Properties

### Readonly isSuspended

isSuspended: boolean

Gets a value indicating whether updates for the target are currently suspended

### onResumed

onResumed: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\>

An [EventHandler](https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html) that fires an event when the suspender is becomes unsuspended

## Methods

### lock

- lock(): () =\> boolean

<!-- -->

- experimental  
  Call this to force suspend drawing.

  remarks  
  All of the unlock tokens have to be called to resume drawing.

  #### Returns () =\> boolean

  an unlock token.

  - - (): boolean

    <!-- -->

    - #### Returns boolean

### resume

- resume(force: boolean): void

<!-- -->

- Call to resume updates on the target. Note this MUST be called or your target will stay suspended forever!

  remarks  
  to resume call this the same number of times as { @link IUpdateSuspender.suspend }, or call with `force` flag to ignore multiple `suspend` executions

  #### Parameters

  - ##### force: boolean

  #### Returns void

### suspend

- suspend(): void

<!-- -->

- Call to suspend updates on the target. Increments call counter.

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
