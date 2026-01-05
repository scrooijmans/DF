<img src="out_scichartv4/typedoc/classes/includeditems_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IncludedItems](https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html)

# Class IncludedItems

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- IncludedItems

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#exclude" class="tsd-kind-icon">exclude</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#excludeall" class="tsd-kind-icon">excludeAll</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#excludelist" class="tsd-kind-icon">excludeList</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#getexcludeditemids" class="tsd-kind-icon">getExcludedItemIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#getincludeditemids" class="tsd-kind-icon">getIncludedItemIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#getincludeditems" class="tsd-kind-icon">getIncludedItems</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#getitemidssetsize" class="tsd-kind-icon">getItemIdsSetSize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#include" class="tsd-kind-icon">include</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#includeall" class="tsd-kind-icon">includeAll</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#includelist" class="tsd-kind-icon">includeList</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#reconcile" class="tsd-kind-icon">reconcile</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#removeid" class="tsd-kind-icon">removeId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/includeditems.html#testisincluded" class="tsd-kind-icon">testIsIncluded</a>

## Methods

### exclude

- exclude(itemId: string): boolean

<!-- -->

- Excludes one item. Returns True if the included items list has changed after the operation.

  #### Parameters

  - ##### itemId: string

  #### Returns boolean

### excludeAll

- excludeAll(): void

<!-- -->

- Clears and switches to inclusion set

  #### Returns void

### excludeList

- excludeList(itemIds: string\[\]): boolean

<!-- -->

- Excludes the list of items. Returns True if the included items list has changed after the operation.

  #### Parameters

  - ##### itemIds: string\[\]

  #### Returns boolean

### getExcludedItemIds

- getExcludedItemIds(): string\[\]

<!-- -->

- Returns excluded item ids. Useful for serialization.

  #### Returns string\[\]

### getIncludedItemIds

- getIncludedItemIds(): string\[\]

<!-- -->

- Returns included item ids. Useful for serialization.

  #### Returns string\[\]

### getIncludedItems

- getIncludedItems(allItems: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>\[\]): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>\[\]

<!-- -->

- Returns included items, but filtering out items from the all items array

  #### Parameters

  - ##### allItems: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>\[\]

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>\[\]

### getItemIdsSetSize

- getItemIdsSetSize(): number

<!-- -->

- Returns the size of the item ids set. Mostly for testing

  #### Returns number

### include

- include(itemId: string): boolean

<!-- -->

- Includes one item. Returns True if the included items list has changed after the operation.

  #### Parameters

  - ##### itemId: string

  #### Returns boolean

### includeAll

- includeAll(): void

<!-- -->

- Clears and switches to exclusion set

  #### Returns void

### includeList

- includeList(itemIds: string\[\]): boolean

<!-- -->

- Includes the list of items. Returns True if the included items list has changed after the operation.

  #### Parameters

  - ##### itemIds: string\[\]

  #### Returns boolean

### reconcile

- reconcile(allItems: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>\[\]): void

<!-- -->

- Reconciles the items set with the all items array by removing items that are no longer present

  #### Parameters

  - ##### allItems: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>\[\]

    the array of all items

  #### Returns void

### removeId

- removeId(id: string): void

<!-- -->

- Removes id. Is used when you need to remove item, for example remove an axis

  #### Parameters

  - ##### id: string

  #### Returns void

### testIsIncluded

- testIsIncluded(itemId: string): boolean

<!-- -->

- Tests if the item is included

  #### Parameters

  - ##### itemId: string

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
