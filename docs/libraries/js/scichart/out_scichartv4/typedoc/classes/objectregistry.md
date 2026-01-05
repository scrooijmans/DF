<img src="out_scichartv4/typedoc/classes/objectregistry_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ObjectRegistry](https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html)

# Class ObjectRegistry

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

ObjectRegistry represents a structure for storing object lifecycle info. Adding an object to the registry will place it into a category of undeleted until it is remove from the registry. Also the object will be placed into a category of uncollected until it is disposed by garbage collector.

### Hierarchy

- ObjectRegistry

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#finalizationregistry" class="tsd-kind-icon">finalizationRegistry</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#uncollectedobjectsmap" class="tsd-kind-icon">uncollectedObjectsMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#undeletedobjectsmap" class="tsd-kind-icon">undeletedObjectsMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#weakmapregistry" class="tsd-kind-icon">weakMapRegistry</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#add" class="tsd-kind-icon">add</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#deleteentry" class="tsd-kind-icon">deleteEntry</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#deleteideletableobjects" class="tsd-kind-icon">deleteIDeletableObjects</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#deletewasmobjects" class="tsd-kind-icon">deleteWasmObjects</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#getobjectid" class="tsd-kind-icon">getObjectId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#getstate" class="tsd-kind-icon">getState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#log" class="tsd-kind-icon">log</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#oncollect" class="tsd-kind-icon">onCollect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html#remove" class="tsd-kind-icon">remove</a>

## Properties

### Protected finalizationRegistry

finalizationRegistry: any = new FinalizationRegistry((id: string) =\> {this.uncollectedObjectsMap.delete(id);this.onCollect(id);})

### uncollectedObjectsMap

uncollectedObjectsMap: Map\<string, { isWasmObject: boolean; objectRef: TWeakRef; proxyRef?: TWeakRef; revocableTokenRef?: TWeakRef\<ReturnType\<ProxyConstructor\["revocable"\]\>\> }\> = new Map\<string, TObjectEntryInfo\>()

### undeletedObjectsMap

undeletedObjectsMap: Map\<string, { isWasmObject: boolean; objectRef: TWeakRef; proxyRef?: TWeakRef; revocableTokenRef?: TWeakRef\<ReturnType\<ProxyConstructor\["revocable"\]\>\> }\> = new Map\<string, TObjectEntryInfo\>()

### weakMapRegistry

weakMapRegistry: WeakMap\<object, any\> = new WeakMap()

## Methods

### add

- add(obj: any, id: string, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tobjectentryoptions" class="tsd-signature-type">TObjectEntryOptions</a>): void

<!-- -->

- Adds an object and its related info to the registry

  #### Parameters

  - ##### obj: any

  - ##### id: string

  - ##### Default value options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tobjectentryoptions" class="tsd-signature-type">TObjectEntryOptions</a> = { isWasmObject: false }

  #### Returns void

### Protected deleteEntry

- deleteEntry(entry: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tobjectentryinfo" class="tsd-signature-type">TObjectEntryInfo</a>, key: string): void

<!-- -->

- Calls `delete` on a specific object within the registry

  #### Parameters

  - ##### entry: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tobjectentryinfo" class="tsd-signature-type">TObjectEntryInfo</a>

  - ##### key: string

  #### Returns void

### deleteIDeletableObjects

- deleteIDeletableObjects(): void

<!-- -->

- Calls `delete` on instances of [IDeletable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html) objects within the registry

  #### Returns void

### deleteWasmObjects

- deleteWasmObjects(): void

<!-- -->

- Calls `delete` on instances of Web Assembly objects within the registry

  #### Returns void

### getObjectId

- getObjectId(obj: any): any

<!-- -->

- #### Parameters

  - ##### obj: any

  #### Returns any

### getState

- getState(): any

<!-- -->

- Returns the state of the registry

  #### Returns any

### log

- log(): void

<!-- -->

- Outputs the state of registry to the console

  #### Returns void

### Protected onCollect

- onCollect(id: string): void

<!-- -->

- The callback executed when an object is being garbage collected

  #### Parameters

  - ##### id: string

  #### Returns void

### remove

- remove(id: string): boolean

<!-- -->

- Removes the object from the undeleted objects collection

  #### Parameters

  - ##### id: string

  #### Returns boolean

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
