<img src="out_scichartv4/typedoc/classes/updatesuspender_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [UpdateSuspender](https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html)

# Class UpdateSuspender

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A class which allows nested suspend/resume operations on an [ISuspendable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/isuspendable.html) target

### Hierarchy

- UpdateSuspender

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iupdatesuspender.html" class="tsd-signature-type">IUpdateSuspender</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#locks" class="tsd-kind-icon">locks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#onresumed" class="tsd-kind-icon">onResumed</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#suspendcounter" class="tsd-kind-icon">suspendCounter</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#issuspended" class="tsd-kind-icon">isSuspended</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#lock" class="tsd-kind-icon">lock</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#resume" class="tsd-kind-icon">resume</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/updatesuspender.html#suspend" class="tsd-kind-icon">suspend</a>

## Properties

### Protected locks

locks: Set\<() =\> boolean\> = new Set\<() =\> boolean\>()

### onResumed

onResumed: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<void\> = new EventHandler\<void\>()

### Protected suspendCounter

suspendCounter: number = 0

## Accessors

### isSuspended

- get isSuspended(): boolean

<!-- -->

- Gets whether the current instance is suspended

  #### Returns boolean

## Methods

### lock

- lock(): token

<!-- -->

- Call this to force suspend drawing. Return unlock token. All of the unlock tokens are required to be called to resume drawing.

  #### Returns token

### resume

- resume(force?: boolean): void

<!-- -->

- Call this to resume drawing

  #### Parameters

  - ##### Default value force: boolean = false

  #### Returns void

### suspend

- suspend(): void

<!-- -->

- Call this to suspend drawing

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
