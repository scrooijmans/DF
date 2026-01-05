On this page

# Deleting DataSeries Memory

SciChart.js stores memory in WebAssembly. This allows us to achieve our incredible performance, and also provide a unfied experience across SciChart platforms (Windows, iOS, Android and JavaScript).

Unlike JavaScript which has built-in garbage collection, WebAssembly requires that you delete memory that you have allocated. Think of it as similar to closing a WebSocket connection if you want to close the connection and free memory.

![](out_scichartv4/2d-charts/chart-types/data-series-api/deleting-memory/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

See related articles:Â [Memory Usage Best Practices](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices) which has some further info on optimising memory usage with SciChart.js, andÂ [Memory Leak Debugging](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging)Â which introduces our new tools to identify and fix memory leaks.

## Examples of how to use Delete()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#examples-of-how-to-use-delete" class="hash-link" aria-label="Direct link to Examples of how to use Delete()" translate="no" title="Direct link to Examples of how to use Delete()">â€‹</a>

Once you are finished with the DataSeries, don't forget to callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">IDeletable.delete()ðŸ“˜</a>. This frees WebAssembly native memory and releases it back to the host.

Below are a few examples of best-practices when deleting dataSeries, renderableSeries and sciChartSurfaces.

### Example 1 - dataseries delete()<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#example-1---dataseries-delete" class="hash-link" aria-label="Direct link to Example 1 - dataseries delete()" translate="no" title="Direct link to Example 1 - dataseries delete()">â€‹</a>

``` prism-code
// Create a DataSeries
console.log(`Example A: Creating, clearing and deleting a dataseries`);
const xyDataSeries = new XyDataSeries(wasmContext, {
  xValues,
  yValues,
});
// Clear it - does not delete memory, just removes all data-points
xyDataSeries.clear();
console.log(`xyDataSeries is cleared but retains memory: ${xyDataSeries.getNativeXValues().capacity()} Datapoints`);
// Frees memory - the data-series cannot be re-used after this
xyDataSeries.delete();
console.log(`xyDataSeries is deleted: ${xyDataSeries.getIsDeleted()}`);
```

### Example 2 - reassign dataseries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#example-2---reassign-dataseries" class="hash-link" aria-label="Direct link to Example 2 - reassign dataseries" translate="no" title="Direct link to Example 2 - reassign dataseries">â€‹</a>

``` prism-code
// When re-assigning a dataseries, make sure to delete the old series
const oldSeries = lineSeries.dataSeries;
oldSeries.delete();
lineSeries.dataSeries = new XyDataSeries(wasmContext, {
  xValues: [1, 2, 3, 4, 5, 6, 7, 8, 9],
  yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0]
});

console.log(`oldSeries is deleted: ${oldSeries.getIsDeleted()}`);
console.log(`lineSeries.dataSeries is deleted: ${lineSeries.dataSeries.getIsDeleted()}`);
```

### Example 3 - deleting renderableseries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#example-3---deleting-renderableseries" class="hash-link" aria-label="Direct link to Example 3 - deleting renderableseries" translate="no" title="Direct link to Example 3 - deleting renderableseries">â€‹</a>

``` prism-code
// Calling delete on a RenderableSeries will delete both the RenderableSeries and its DataSeries
// The series is no longer usable
lineSeries.delete();
console.log(`lineSeries.dataSeries is now deleted`);
```

### Example 4 - deleting scichartsurface<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#example-4---deleting-scichartsurface" class="hash-link" aria-label="Direct link to Example 4 - deleting scichartsurface" translate="no" title="Direct link to Example 4 - deleting scichartsurface">â€‹</a>

``` prism-code
// Calling Delete on a SciChartSurface will delete and free memory on all elements in this chart
// This chart is no longer usable
sciChartSurface.delete()
console.log(`sciChartSurface is deleted: ${sciChartSurface.isDeleted}`);
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/deleting-memory/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Failing to callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">IDeletable.delete()ðŸ“˜</a> on a DataSeries or it's parent SciChartSurface when it is no longer needed can result in a memory leak.

To simplify your code, if you do not change DataSeries instances, you can call delete on the parent SciChartSurface once. This will delete all child objects that hold native memory.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

##### Miscellaneous APIs<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/deleting-memory/#miscellaneous-apis" class="hash-link" aria-label="Direct link to Miscellaneous APIs" translate="no" title="Direct link to Miscellaneous APIs">â€‹</a>

- [Memory Usage Best Practices](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices)
- [Memory Leak Debugging](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-series-api/deleting-memory/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-series-api/deleting-memory/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
