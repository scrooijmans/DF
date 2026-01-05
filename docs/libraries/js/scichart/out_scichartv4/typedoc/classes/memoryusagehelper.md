<img src="out_scichartv4/typedoc/classes/memoryusagehelper_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [MemoryUsageHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html)

# Class MemoryUsageHelper

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

[MemoryUsageHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html) provides tools for tracking, debugging, and testing common issus related to lifecycle of SciChart entities.

### Hierarchy

- MemoryUsageHelper

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#ismemoryusagedebugenabledproperty" class="tsd-kind-icon">isMemoryUsageDebugEnabledProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#objectregistry" class="tsd-kind-icon">objectRegistry</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#ismemoryusagedebugenabled" class="tsd-kind-icon">isMemoryUsageDebugEnabled</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#destroyallcharts" class="tsd-kind-icon">destroyAllCharts</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#destroymultichart" class="tsd-kind-icon">destroyMultiChart</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#destroypiecharts" class="tsd-kind-icon">destroyPieCharts</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#destroysinglecharts" class="tsd-kind-icon">destroySingleCharts</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#register" class="tsd-kind-icon">register</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#unregister" class="tsd-kind-icon">unregister</a>

## Properties

### Static Protected isMemoryUsageDebugEnabledProperty

isMemoryUsageDebugEnabledProperty: boolean = false

### Static objectRegistry

objectRegistry: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html" class="tsd-signature-type">ObjectRegistry</a>

## Accessors

### Static isMemoryUsageDebugEnabled

- get isMemoryUsageDebugEnabled(): boolean
- set isMemoryUsageDebugEnabled(value: boolean): void

<!-- -->

- Gets or sets the `Memory Usage Debug Mode`. Enabling the mode, provides warnings about wrong usage or cleanup. Also it wraps SciChart entities and adds them to the [objectRegistry](https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#objectregistry) to track their lifecycle

  #### Returns boolean

- Gets or sets the `Memory Usage Debug Mode`. Enabling the mode, provides warnings about wrong usage or cleanup. Also it wraps SciChart entities and adds them to the [objectRegistry](https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#objectregistry) to track their lifecycle

  #### Parameters

  - ##### value: boolean

  #### Returns void

## Methods

### Static destroyAllCharts

- destroyAllCharts(): void

<!-- -->

- Calls `delete` on all charts

  #### Returns void

### Static destroyMultiChart

- destroyMultiChart(): void

<!-- -->

- Calls `delete` on all 2D and 3D charts instantiated with [SciChartSurface.create](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create) or [SciChart3DSurface.create](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create)

  #### Returns void

### Static destroyPieCharts

- destroyPieCharts(): void

<!-- -->

- Calls `delete` on all charts instantiated with [SciChartPieSurface.create](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html#create)

  #### Returns void

### Static destroySingleCharts

- destroySingleCharts(): void

<!-- -->

- Calls `delete` on all 2D and 3D charts instantiated with [SciChartSurface.createSingle](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#createsingle) or [SciChart3DSurface.createSingle](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#createsingle)

  #### Returns void

### Static register

- register(entity: any, id?: string): void

<!-- -->

- Adds entity to the object registry to keep track of it being collected

  #### Parameters

  - ##### entity: any

  - ##### Optional id: string

    optional custom ID of the entity

  #### Returns void

### Static unregister

- unregister(id: string): void

<!-- -->

- Removes entity from the object registry

  #### Parameters

  - ##### id: string

    ID of the entity

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
