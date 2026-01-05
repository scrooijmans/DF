<img src="out_scichartv4/typedoc/interfaces/ientityidprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IEntityIdProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html)

# Interface IEntityIdProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

An EntityIdProvider generates integer Ids (must fit into UInt32) for meshs in the SciChart3D engine There is a maximum of 4 billion vertices or meshes in the scene in any combination. Users can override and implement their own IdProvider to recycle Ids if they wish

### Hierarchy

- IEntityIdProvider

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultentityidprovider.html" class="tsd-signature-type">DefaultEntityIdProvider</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html#getnextentityid" class="tsd-kind-icon">getNextEntityId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ientityidprovider.html#releaseentityid" class="tsd-kind-icon">releaseEntityId</a>

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
