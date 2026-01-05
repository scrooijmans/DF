On this page

# Append, Insert, Update, Remove

SciChart.js is designed to be a highly dynamic chart library for frequently updating data. Once you have created a chart with a RenderableSeries / DataSeries pair, you can manipulate the data in any way and the chart will redrawÂ / update.

Data Updates are handled in a reactive way and are 'debounced' so that the chart only draws 1/60th of a second regardless of how many changes to data you make.

## The DataSeries Append, Update, Insert, Remove functions<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/append-insert-update-remove/#the-dataseries-append-update-insert-remove-functions" class="hash-link" aria-label="Direct link to The DataSeries Append, Update, Insert, Remove functions" translate="no" title="Direct link to The DataSeries Append, Update, Insert, Remove functions">â€‹</a>

Here's an table from theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">TypeDoc for XyDataSeriesðŸ“˜</a> showing functions available for updating the data.

|  |  |  |
|----|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#append" rel="noopener noreferrer" target="_blank">appendðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativeyvalues" rel="noopener noreferrer" target="_blank">getNativeYValuesðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removerange" rel="noopener noreferrer" target="_blank">removeRangeðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendmetadata" rel="noopener noreferrer" target="_blank">appendMetadataðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getwindowedyrange" rel="noopener noreferrer" target="_blank">getWindowedYRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#setfinalanimationvectors" rel="noopener noreferrer" target="_blank">setFinalAnimationVectorsðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendmetadatarange" rel="noopener noreferrer" target="_blank">appendMetadataRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getxrange" rel="noopener noreferrer" target="_blank">getXRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#setinitialanimationvectors" rel="noopener noreferrer" target="_blank">setInitialAnimationVectorsðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendrange" rel="noopener noreferrer" target="_blank">appendRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getxvalues" rel="noopener noreferrer" target="_blank">getXValuesðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#setmetadata" rel="noopener noreferrer" target="_blank">setMetadataðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#clear" rel="noopener noreferrer" target="_blank">clearðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#hasmetadatagenerator" rel="noopener noreferrer" target="_blank">hasMetadataGeneratorðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#setmetadataat" rel="noopener noreferrer" target="_blank">setMetadataAtðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#count" rel="noopener noreferrer" target="_blank">countðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insert" rel="noopener noreferrer" target="_blank">insertðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#setmetadatagenerator" rel="noopener noreferrer" target="_blank">setMetadataGeneratorðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#delete" rel="noopener noreferrer" target="_blank">deleteðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insertmetadata" rel="noopener noreferrer" target="_blank">insertMetadataðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#tojson" rel="noopener noreferrer" target="_blank">toJSONðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getindicesrange" rel="noopener noreferrer" target="_blank">getIndicesRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insertmetadatarange" rel="noopener noreferrer" target="_blank">insertMetadataRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#update" rel="noopener noreferrer" target="_blank">updateðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getisdeleted" rel="noopener noreferrer" target="_blank">getIsDeletedðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insertrange" rel="noopener noreferrer" target="_blank">insertRangeðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#updateanimationproperties" rel="noopener noreferrer" target="_blank">updateAnimationPropertiesðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getmetadataat" rel="noopener noreferrer" target="_blank">getMetadataAtðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#notifydatachanged" rel="noopener noreferrer" target="_blank">notifyDataChangedðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#updatexy" rel="noopener noreferrer" target="_blank">updateXyðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getmetadatalength" rel="noopener noreferrer" target="_blank">getMetadataLengthðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeat" rel="noopener noreferrer" target="_blank">removeAtðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#validateanimationvectors" rel="noopener noreferrer" target="_blank">validateAnimationVectorsðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativeindexes" rel="noopener noreferrer" target="_blank">getNativeIndexesðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removemetadataat" rel="noopener noreferrer" target="_blank">removeMetadataAtðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#validateindex" rel="noopener noreferrer" target="_blank">validateIndexðŸ“˜</a> |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#getnativexvalues" rel="noopener noreferrer" target="_blank">getNativeXValuesðŸ“˜</a> | <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removemetadatarange" rel="noopener noreferrer" target="_blank">removeMetadataRangeðŸ“˜</a> |  |

![](out_scichartv4/2d-charts/chart-types/data-series-api/append-insert-update-remove/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

All chart series updates are done via the DataSeries API using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#append" rel="noopener noreferrer" target="_blank">append()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insert" rel="noopener noreferrer" target="_blank">insert()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#update" rel="noopener noreferrer" target="_blank">update()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removeat" rel="noopener noreferrer" target="_blank">remove()ðŸ“˜</a> functions. There are also variations such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendrange" rel="noopener noreferrer" target="_blank">appendRange()ðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insertrange" rel="noopener noreferrer" target="_blank">insertRange()ðŸ“˜</a> etc... which accept an array of data.

Note that different dataSeries such asÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries.html" rel="noopener noreferrer" target="_blank">XyzDataSeriesðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyydataseries.html" rel="noopener noreferrer" target="_blank">XyyDataSeriesðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcdataseries.html" rel="noopener noreferrer" target="_blank">OhlcDataSeriesðŸ“˜</a> have slightly different function signatures for append/update functions. Click the links above to the Typedoc for more info.

Here are some common operations:

``` prism-code
// Append, Update, Insert, Remove

const xyDataSeries = new XyDataSeries(wasmContext);
xyDataSeries.append(1, 10); // Appends X=1, Y=10
xyDataSeries.append(2, 20); // Appends X=2, Y=20
xyDataSeries.appendRange([3, 4, 5], [30, 40, 50]); // Appends X=3,4,5 and Y=30,40,50
xyDataSeries.removeAt(0); // removes the 0th xy point
xyDataSeries.removeRange(0, 2); // Removes 2 points from index 0 onwards
xyDataSeries.insert(0, 100, 200); // Inserts X=100, Y=200 at index 0
//xyDataSeries.insertRange(...)
xyDataSeries.update(0, 22); // Updates the Y-value at index 0
xyDataSeries.clear(); // Clears the dataseries. NOTE: Does not free memory
xyDataSeries.delete(); // Deletes WebAssembly memory. The series is no longer usable.
```

![](out_scichartv4/2d-charts/chart-types/data-series-api/append-insert-update-remove/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

**For the best possible performance, when modifying large datasets**, use theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#appendrange" rel="noopener noreferrer" target="_blank">appendRangeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#insertrange" rel="noopener noreferrer" target="_blank">insertRangeðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html#removerange" rel="noopener noreferrer" target="_blank">removeRangeðŸ“˜</a> functions. These accept an array of values and are considerably faster than appending point-by-point.

![](out_scichartv4/2d-charts/chart-types/data-series-api/append-insert-update-remove/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Failing to callÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html#delete" rel="noopener noreferrer" target="_blank">IDeletable.delete()ðŸ“˜</a> on a DataSeries when it is no longer needed can result in a memory leak.

To simplify your code, if you do not change DataSeries instances, you can call delete on the parent SciChartSurface once. This will delete all child objects that hold native memory.

## Examples of Dynamic Updates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/append-insert-update-remove/#examples-of-dynamic-updates" class="hash-link" aria-label="Direct link to Examples of Dynamic Updates" translate="no" title="Direct link to Examples of Dynamic Updates">â€‹</a>

There are a number of worked examples of how to apply dynamic updates to the chart over at the pageÂ [DataSeries Realtime Updates](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/realtime-updates).

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-series-api/append-insert-update-remove/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-series-api/append-insert-update-remove/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
