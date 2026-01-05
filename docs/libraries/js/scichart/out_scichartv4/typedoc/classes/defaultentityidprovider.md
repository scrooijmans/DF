<img src="out_scichartv4/typedoc/classes/defaultentityidprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultentityidprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DefaultEntityIdProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultentityidprovider.html)

# Class DefaultEntityIdProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Default implementation of [IEntityIdProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html). This implementation generates Ids from 0 to 4 billion and does not recycle Ids. If you encounter the error "The max Mesh ID has been exceeded", you can override releaseEntityId to create a more sophisticated IdProvider to recycle Ids.

### Hierarchy

- DefaultEntityIdProvider

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html" class="tsd-signature-type">IEntityIdProvider</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultentityidprovider.html#getnextentityid" class="tsd-kind-icon">getNextEntityId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultentityidprovider.html#releaseentityid" class="tsd-kind-icon">releaseEntityId</a>

## Methods

### getNextEntityId

- getNextEntityId(): number

<!-- -->

- Gets the next available EntityId (max value UInt32 is 4 billion)

  #### Returns number

### releaseEntityId

- releaseEntityId(id: number): void

<!-- -->

- Optional: releases an Id when an entity is removed fro the scene in the case where max mesh ID is hit

  #### Parameters

  - ##### id: number

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
