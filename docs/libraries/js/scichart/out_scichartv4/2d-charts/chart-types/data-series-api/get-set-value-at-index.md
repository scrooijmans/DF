On this page

# Accessing DataSeries xValues, yValues and count (size)

## How to access DataSeries xValues, yValues<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#how-to-access-dataseries-xvalues-yvalues" class="hash-link" aria-label="Direct link to How to access DataSeries xValues, yValues" translate="no" title="Direct link to How to access DataSeries xValues, yValues">â€‹</a>

You can access `xValues`, `yValues` on a `DataSeries` by getting the internal WebAssembly native arrays via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativexvalues" rel="noopener noreferrer" target="_blank">dataSeries.getNativeXValues()ðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativeyvalues" rel="noopener noreferrer" target="_blank">dataSeries.getNativeYValues()ðŸ“˜</a> functions.

These functions return the x & y values as `SCRTDoubleVector`: a webassembly buffer type which stores underlying data as `Float64` array in the wasm heap.

### Accessing DataSeries data via getNativeXValues() / getNativeYValues().get(i)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#accessing-dataseries-data-via-getnativexvalues--getnativeyvaluesgeti" class="hash-link" aria-label="Direct link to Accessing DataSeries data via getNativeXValues() / getNativeYValues().get(i)" translate="no" title="Direct link to Accessing DataSeries data via getNativeXValues() / getNativeYValues().get(i)">â€‹</a>

Accessing dataSeries xValues, yValues can be done via the `dataSeries.getNativeXValues()`, `dataSeries.getNativeYValues()` functions. These return an `SCRTDoubleVector` type which allows you to get a value at index via `dataSeries.getNativeXValues().get(i)`.

Below find a simple example of accessing dataSeries x/y values point by point:

- Ts

``` prism-code
// Example: Accessing X, Y Values from DataSeries using getNativeXValues, getNativeYValues
const xyDataSeries = new XyDataSeries(wasmContext);
xyDataSeries.appendRange([1, 2, 3], [10, 20, 30]);

// Get xValues from the dataSeries
const xValues = xyDataSeries.getNativeXValues();
// Get yValues from the dataSeries
const yValues = xyDataSeries.getNativeYValues();
for (let i = 0; i < xyDataSeries.count(); i++) {
    // Note, this method of point by point access using .get(i) is slow
    // faster methods exist below using the helper functions `vectorToArrayViewF64` and `vectorToArray`
    console.log(`index=${i}, xy = ${xValues.get(i)}, ${yValues.get(i)}`);
}

// Will output to console
// index=0, xy=1, 10
// index=1, xy=2, 20
// index=2, xy=3, 30
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

point by point access to the DataSeries via this method is slow when you're dealing with millions of points. If you need to do bulk operations, it's better to read the entire vector out to JavaScript array first ([see how](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#reading-dataseries-xvalues-yvalues-as-a-float64array-view))

### What is the SCRTDoubleVector type returned by getNativeXValues()?<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#what-is-the-scrtdoublevector-type-returned-by-getnativexvalues" class="hash-link" aria-label="Direct link to What is the SCRTDoubleVector type returned by getNativeXValues()?" translate="no" title="Direct link to What is the SCRTDoubleVector type returned by getNativeXValues()?">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativexvalues" rel="noopener noreferrer" target="_blank">dataSeries.getNativeXValues()ðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativeyvalues" rel="noopener noreferrer" target="_blank">dataSeries.getNativeYValues()ðŸ“˜</a> allow you to access the dataSeries xValues, yValues.

These both return type `SCRTDoubleVector`. This is a type declared in webassembly which maps to a `Float64` array on the wasm heap.

The example above shows how you can access data point by point using the `SCRTDoubleVector.get(i)` function:

``` prism-code
export declare class SCRTDoubleVector {
    push_back(_dNewValue: number): void;
    resize(_iNewSize: number, _dInitialValue: number): void;
    resizeFast(_iNewSize: number): void;
    reserve(_iCount: number): void;
    clear(): void;
    size(): number;
    capacity(): number;
    get(_iIndex: number): number; // Access value at index (i)
    set(_iIndex: number, _dValue: number): void;
    insertAt(_iIndex: number, _dValue: number): void;
    removeAt(_iIndex: number): void;
    removeRange(_iIndex: number, _iCount: number): void;
    dataPtr(_iOffset: number): number; // returns a pointer to the wasm heap for this vector
    delete(): void;
}
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

The above type declaration for `SCRTDoubleVector` is included for information purposes only.

It's not recommended to use `SCRTDoubleVector.push_back`, `resize`, `clear`, `insertAt`, `removeAt` or `delete`. Instead, use the `append` `update` `insert` `remove` `clear` and `delete` functions directly on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> and related dataSeries types, as this will manage internal state as well as memory.

## Accessing DataSeries Count (length) and Capacity<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#accessing-dataseries-count-length-and-capacity" class="hash-link" aria-label="Direct link to Accessing DataSeries Count (length) and Capacity" translate="no" title="Direct link to Accessing DataSeries Count (length) and Capacity">â€‹</a>

The length, size or count or a `dataSeries` can be accessed via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#count" rel="noopener noreferrer" target="_blank">dataSeries.count()ðŸ“˜</a> function. Here is an example:

- TS

``` prism-code
// Example: get count (length, size) of dataSeries
const count = 1_000_000;
const xValues: number[] = Array.from(Array(count).keys());
const yValues: number[] = Array.from(Array(count).keys());

const series = new XyDataSeries(wasmContext, {
    xValues,
    yValues
});
console.log(`dataSeries count: ${series.count()}`);
// Outputs: "dataSeries count: 1,000,000"
```

The capacity of a `dataSeries` can be get/set via the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#capacity" rel="noopener noreferrer" target="_blank">dataSeries.capacityðŸ“˜</a> property. This sets the size of the underlying memory buffers allowing you to pre-allocate memory in demanding applications.

For example, if you plan to call `dataSeries.append()` or `.appendRange()` many times up to a capacity of 1,000,000, you can pre-allocate the memory now by setting the `capacity`:

- TS

``` prism-code
// Example: setting the capacity of a dataseries to preallocate memory
const series = new XyDataSeries(wasmContext);
series.capacity = 1000000; // pre-allocates 1,000,000 values for X,Y
console.log(`dataSeries count: ${series.count()}`);
console.log(`dataSeries capacity: ${series.capacity}`);

// Outputs: "dataSeries count: 0"
// "dataSeries capacity: 1,000,000"
```

## Reading DataSeries xValues, yValues as a Float64Array View<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#reading-dataseries-xvalues-yvalues-as-a-float64array-view" class="hash-link" aria-label="Direct link to Reading DataSeries xValues, yValues as a Float64Array View" translate="no" title="Direct link to Reading DataSeries xValues, yValues as a Float64Array View">â€‹</a>

As the `dataSeries.getNativeXValues()`, `dataSeries.getNativeYValues()` functions return the xValues and yValues as webassembly Float64 memory buffers (type `SCRTDoubleVector`), you can't operate on these like normal JavaScript arrays.

However, it is possible to create a view on the dataSeries xValues, yValues as a JS array (`Float64Array`) for further manipulation, read-back of dataSeries values or otherwise.

The following helper function `vectorToArrayViewF64()` (added in <a href="https://www.scichart.com/changelog/scichart-js/" rel="noopener noreferrer" target="_blank">version 4.0.873</a>) will convert a `SCRTDoubleVector` (Float64 webassembly memory buffer) into a `Float64Array` (JavaScript typed 64-bit array). This operation is super-fast and will allow you to read back values from a `dataSeries` into JavaScript arrays with little overhead.

Here's an example of how to use it to get a JavaScript array view of `dataSeries` x,y values:

- TS

``` prism-code
// vectorToArrayViewF64() (returns Float64Array) allows access to dataSeries xValues, yValues
// by creating a view onto webassembly memory
const count = 1_000_000;
const xValues: number[] = Array.from(Array(count).keys());
const yValues: number[] = Array.from(Array(count).keys());

// Create an XyDataSeries with 1,000,000 xValues, yValues
const dataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues
});

// Create a view into the data (maps dataSeries xValues, yValues onto a JavaScript Float64Array)
const startGet = performance.now();
const f64XValues: Float64Array = vectorToArrayViewF64(dataSeries.getNativeXValues(), wasmContext);
const f64YValues: Float64Array = vectorToArrayViewF64(dataSeries.getNativeYValues(), wasmContext);
console.log(`vectorToArrayViewF64 get: ${(performance.now() - startGet).toFixed(3)}ms`);

// Operate on these as normal JS arrays
const startIterate = performance.now();
// Check values
let test = 0;
for (let i = 0; i < count; i++) {
    test += f64XValues[i];
    test += f64YValues[i];
}
console.log(`vectorToArrayViewF64 iterate: ${(performance.now() - startIterate).toFixed(3)}ms`);

// Output to console:
// vectorToArrayViewF64 get: 0.072ms
// vectorToArrayViewF64 iterate: 2.747ms
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Note, the returned `Float64Array` is a **view** onto the wasm memory, **not a copy**. Updating this `Float64Array` view will update the dataSeries data and vice-versa.

This method of creating a `Float64Array` view onto the webassembly data is **much faster** than `getNativeXValues().get(i)` `getNativeYValues().get(i)` and can be used to read back dataSeries `xValues` `yValues` into JavaScript efficiently.

![](out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Since the `Float64Array` is a **view** onto the webassembly memory, note that you should re-map this view every time you use it. If the underlying dataSeries size is changed, the wasm memory may be moved (`.dataPtr(0)` will change), then you run the risk of getting strange errors like `TypeError: Cannot perform %TypedArray%.prototype.set on a detached ArrayBuffer`.

It's best to use this operation to **read/write** values from a dataSeries where you need fast access, but don't keep the `Float64Array` view instance for longer than needed (use once for an operation then discard). For passing JS array copies around your app, use `vectorToArray()` which provides a safer deep-copy.

For write operations, it's recommended to use the `append` `update` `insert` `remove` `clear` and `delete` functions directly on <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> and related dataSeries types unless you absolutely know what you're doing!

## Copying DataSeries xValues, yValues to JavaScript number\[\] array<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#copying-dataseries-xvalues-yvalues-to-javascript-number-array" class="hash-link" aria-label="Direct link to Copying DataSeries xValues, yValues to JavaScript number[] array" translate="no" title="Direct link to Copying DataSeries xValues, yValues to JavaScript number[] array">â€‹</a>

If you want to go a step further, you can convert a `Float64Array` to a JavaScript array (e.g. `number[]`) and perform a deep-copy of dataSeries data into JavaScript Arrays. This operation involves a copy and is slower, but you can also be assured that the underlying data won't change. It's also the most compatible with JavaScript frameworks and other parts of your code.

The `vectorToArray()` helper function (added in <a href="https://www.scichart.com/changelog/scichart-js/" rel="noopener noreferrer" target="_blank">version 4.0.873</a>) can be used to perform a deep-copy of dataSeries data (xValues, yValues).

- TS

``` prism-code
// vectorToArray() (returns number[]) performs a deep-copy of a scichart webassembly vector
// allowing for safer read-only access to dataseries data
const count = 1_000_000;
const xValues = Array.from(Array(count).keys());
const yValues = Array.from(Array(count).keys());

// Create an XyDataSeries with 1,000,000 xValues, yValues
const dataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues
});

const startGet = performance.now();

// vectorToArray creates first a Float64Array view onto dataSeries xValues, yValues
// then uses Array.from(typedArrayView) to copy to JS Array
const jsXValues: number[] = vectorToArray(dataSeries.getNativeXValues(), wasmContext);
const jsYValues: number[] = vectorToArray(dataSeries.getNativeYValues(), wasmContext);

console.log(`vectorToArray deepCopy: ${(performance.now() - startGet).toFixed(3)}ms`);

// Operate on these as normal JS arrays
const startIterate = performance.now();
// Check values
let test = 0;
for (let i = 0; i < count; i++) {
    test += jsXValues[i];
    test += jsYValues[i];
}
console.log(`vectorToArray iterate: ${(performance.now() - startIterate).toFixed(3)}ms`);

// Output to console:
// vectorToArray deepCopy: 62ms
// vectorToArray iterate: 2ms
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

This operation involves a deep copy of dataSeries data and is safer, but will introduce some extra latency depending on the size of the dataSeries data.

## Fast copy one XyDataSeries to another<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#fast-copy-one-xydataseries-to-another" class="hash-link" aria-label="Direct link to Fast copy one XyDataSeries to another" translate="no" title="Direct link to Fast copy one XyDataSeries to another">â€‹</a>

Using the utility function `vectorToArrayViewF64()` we discussed above, it's possible to fast copy an entire `XyDataSeries` to another. Use this in the case where you want to duplicate (copy) data from one DataSeries to another.

1.  Given a source `dataSeries` with `count()`
2.  Create a destination `dataSeries`
3.  set `destination.capacity = source.count()` ***important** to avoid detached ArrayBuffer errors*
4.  Use the `vectorToArrayViewF64()` helper function declared above to get `Float64Array` views into the source xValues, yValues
5.  call `destination.appendRange()` using these array views

- TS

``` prism-code
const count = 1_000_000;
const xValues: number[] = Array.from(Array(count).keys());
const yValues: number[] = Array.from(Array(count).keys());

const startCreate = performance.now();

// Create a src series and fill with values
const seriesSrc = new XyDataSeries(wasmContext, {
    xValues,
    yValues
});

console.log(`time to fill a dataSeries 1M points: ${(performance.now() - startCreate).toFixed(3)}ms`);

const startCopy = performance.now();

// Create a dest series and ensure the capacity (memory size) before calling
// vectorToArrayView. This will avoid potential "Cannot perform %TypedArray%.prototype.set on a detached ArrayBuffer"
// error as any resizes of memory might move other memory locations
const seriesDest = new XyDataSeries(wasmContext);
seriesDest.capacity = seriesSrc.count();

// Fast copy xValues, yValues from one dataSeries to another
console.time("Time to deep copy an entire dataSeries");
seriesDest.appendRange(
    vectorToArrayViewF64(seriesSrc.getNativeXValues(), wasmContext),
    vectorToArrayViewF64(seriesSrc.getNativeYValues(), wasmContext)
);

console.log(`time to deep copy a dataSeries 1M points: ${(performance.now() - startCopy).toFixed(3)}ms`);

// Console output
// Time to fill a dataSeries 1M points: 11.762ms
// Time to deep copy a dataSeries 1M points: 13.861ms
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The time to deep copy `dataSeries` data from one series to another using `vectorToArrayViewF64()` is comparable to the time to create the `dataSeries` in the first place.

This method can be used if you need to create copies (clones) of dataSeries in your js application.

## Performance Table of different dataSeries readback methods<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#performance-table-of-different-dataseries-readback-methods" class="hash-link" aria-label="Direct link to Performance Table of different dataSeries readback methods" translate="no" title="Direct link to Performance Table of different dataSeries readback methods">â€‹</a>

Here's a performance table of the various methods to get, set, read, copy dataSeries xValues yValues into JS Arrays. Performance will vary from system to system, but the following can be used as a guide to assess the impact of using different

| Method | Time (ms) | Note |
|----|----|----|
| Read 1M points with `getNativeXValues().get(i)` | 400ms | point-by-point iteration is slow and should be avoided |
| Read 1M points with `vectorToArray()` | 62ms | performs a deep-copy of xValues, yValues into number\[\] array |
| Read 1M points with `vectorToArrayViewF64()` | 4ms | returns an unsafe array view. Used for v. fast read/write access with caveats |
| create new 1M point dataSeries | 11ms | creation of a new dataSeries with pre-allocated arrays |
| deep copy 1M point dataSeries using `vectorToArrayViewF64()` | 13ms | fast deep-copy of dataSeries data to another dataSeries |

## Examples of Dynamic Updates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/get-set-value-at-index/#examples-of-dynamic-updates" class="hash-link" aria-label="Direct link to Examples of Dynamic Updates" translate="no" title="Direct link to Examples of Dynamic Updates">â€‹</a>

There are a number of worked examples of how to apply dynamic updates to the chart over at the pageÂ [DataSeries Realtime Updates](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-series-api/get-set-value-at-index/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-series-api/get-set-value-at-index/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
