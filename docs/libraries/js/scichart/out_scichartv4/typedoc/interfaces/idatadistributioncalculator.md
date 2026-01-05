<img src="out_scichartv4/typedoc/interfaces/idatadistributioncalculator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IDataDistributionCalculator](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html)

# Interface IDataDistributionCalculator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- IDataDistributionCalculator

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datadistributioncalculator.html" class="tsd-signature-type">DataDistributionCalculator</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#containsnan" class="tsd-kind-icon">containsNaN</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#issortedascending" class="tsd-kind-icon">isSortedAscending</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#clear" class="tsd-kind-icon">clear</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#onappend" class="tsd-kind-icon">onAppend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#oninsert" class="tsd-kind-icon">onInsert</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#onupdate" class="tsd-kind-icon">onUpdate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#setcontainsnan" class="tsd-kind-icon">setContainsNaN</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatadistributioncalculator.html#setissortedascending" class="tsd-kind-icon">setIsSortedAscending</a>

## Properties

### Readonly containsNaN

containsNaN: boolean

containsNaN Data Distribution flag

### Readonly isSortedAscending

isSortedAscending: boolean

isSortedAscending Data Distribution flag

## Methods

### clear

- clear(isSorted: boolean, containsNaN: boolean): void

<!-- -->

- Clears the DataDistributionCalculator flags

  #### Parameters

  - ##### isSorted: boolean

  - ##### containsNaN: boolean

  #### Returns void

### onAppend

- onAppend(isSorted: boolean, containsNaN: boolean, currentXValues: SCRTDoubleVector, newXValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>, newYValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>): void

<!-- -->

- Called when X Values are appended. Should update the Data Distribution flags

  #### Parameters

  - ##### isSorted: boolean

  - ##### containsNaN: boolean

  - ##### currentXValues: SCRTDoubleVector

  - ##### newXValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

  - ##### newYValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

  #### Returns void

### onInsert

- onInsert(isSorted: boolean, containsNaN: boolean, currentXValues: SCRTDoubleVector, newXValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>, newYValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>, indexWhereInserted: number): void

<!-- -->

- Called when new values are inserted. Should update the Data Distribution flags

  #### Parameters

  - ##### isSorted: boolean

  - ##### containsNaN: boolean

  - ##### currentXValues: SCRTDoubleVector

  - ##### newXValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

  - ##### newYValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

  - ##### indexWhereInserted: number

  #### Returns void

### onUpdate

- onUpdate(isSorted: boolean, containsNaN: boolean, currentXValues: SCRTDoubleVector, newXValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>, newYValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>, indexWhereUpdated: number): void

<!-- -->

- Called when yValues are updated. Should update the Data Distribution flags

  #### Parameters

  - ##### isSorted: boolean

  - ##### containsNaN: boolean

  - ##### currentXValues: SCRTDoubleVector

  - ##### newXValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

  - ##### newYValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#numberarray" class="tsd-signature-type">NumberArray</a>

  - ##### indexWhereUpdated: number

  #### Returns void

### setContainsNaN

- setContainsNaN(value: boolean): void

<!-- -->

- Called to update containsNaN flag manually, for example when [IDataSeries.containsNaN](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#containsnan) property changes

  #### Parameters

  - ##### value: boolean

  #### Returns void

### setIsSortedAscending

- setIsSortedAscending(value: boolean): void

<!-- -->

- Called to update isSortedAscending flag manually, for example when [IDataSeries.isSorted](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html#issorted) property changes

  #### Parameters

  - ##### value: boolean

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
