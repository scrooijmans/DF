On this page

# Data Filters API

The Filters and Transforms API was added to SciChart.js v2.xÂ as a way to create derived data, filters, indicators or perform functions on your data easily and simply.

SciChart comes with a number of filters built-in. The Filters API can be used easily where you want to:

- Add a Linear Trendline to a chart
- Perform a Moving Average on an underlying DataSeries
- Offset and Scale a data-series
- Calculate the ratio ofÂ two DataSeries
- Apply a custom function to each datapoint, allowing you to do high-pass, low-pass or any other kind of filtering or per-point transformation

The Filters API is also extremely configurable and allow you to create custom filters - the possibilities of this API really are endless!

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/trend-ma-ratio" target="_blank">Trendline, Moving Average and Ratio Filters example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Filters Built-in to SciChart.js<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#filters-built-in-to-scichartjs" class="hash-link" aria-label="Direct link to Filters Built-in to SciChart.js" translate="no" title="Direct link to Filters Built-in to SciChart.js">â€‹</a>

For each kind of filter, there may be multiple types specific to the type of DataSeries that it will be attached to. These are listed below.

For example, a **ScaleOffsetFilter** which applies a scale (mutiplier) or offset (addition) to data series values has sub-types **XyScaleOffsetFilter**, **XyyScaleOffsetFilter**, **XyzScaleOffsetFilter** and **OhlcScaleOffsetFilter** - one for each type of dataseries.

All types apart from Ohlc accept any type of DataSeries as input. See **\[Specifying the Input field\]** below.

- **ScaleOffsetFilter**Â - Applies a scale (multiplier) and an offset (addition) to each field value.
  - **XyScaleOffsetFilter**
  - **XyyScaleOffsetFilter**
  - **XyzScaleOffsetFilter**
  - **OhlcScaleOffsetFilter**
- **XyLinearTrendFilter** -Â Calculates a straight line best-fit for your dataseries.
- **XyMovingAverageFilter** -Â Calculates the moving average of your dataseries over the specified length.
- **XyRatioFilter** -Â Returns a series where each point is the ratio of the original series and the given divisorSeries.
- **CustomFilter** -Â A class that allows you to create a simple custom filter by just specifying a filter function that will be applied to each data point.
  - **XyCustomFilter**
  - **XyyCustomFilter**
  - **XyzCustomFilter**
  - **OhlcCustomFilter**

## Updating Data with Filters<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#updating-data-with-filters" class="hash-link" aria-label="Direct link to Updating Data with Filters" translate="no" title="Direct link to Updating Data with Filters">â€‹</a>

The beauty of the Filters API is that when the underlying data updates, the filter automatically updates. There is no need to recalculate your filter - SciChart does this for you! Where possible, only the changed points are recalculated.

![](out_scichartv4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

See the **XyMovingAverageFilter** page for an example of how to update charts with filters. All filters have an **originalSeries** property which you can use to access the original data, though in Typescript you will need to cast it to the correct type before you can use the data manipulation methods.

## Daisy-Chaining Filters<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#daisy-chaining-filters" class="hash-link" aria-label="Direct link to Daisy-Chaining Filters" translate="no" title="Direct link to Daisy-Chaining Filters">â€‹</a>

Fitlers may be daisy chained or cascaded in order to combine their effects. Changing the underlying DataSeries will cause all filters in the chain to trigger an update.

## Specifying the Input Field<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#specifying-the-input-field" class="hash-link" aria-label="Direct link to Specifying the Input Field" translate="no" title="Direct link to Specifying the Input Field">â€‹</a>

An Xy filter will produce an XyDataSeries, but it can accept any series type as input. The options includes a \[field\] property of type **EDataSeriesField**, which determines which field on the original series will be the input. For Xyy filters there are yField and y1Field, and for Xyz filters there are yField and zField options. See the **XyLinearTrendFilter** page for an example.

## Creating Advanced Custom Filters<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#creating-advanced-custom-filters" class="hash-link" aria-label="Direct link to Creating Advanced Custom Filters" translate="no" title="Direct link to Creating Advanced Custom Filters">â€‹</a>

One of the most powerful features of the Filters API is the ability to create custom filters.

To create an Advanced Custom Filter we simply override the **FilterBase** of the type of series we want ie (**XyFilterBase**, **XyyFilterBase**, **XyzFilterBase**, **OhlcFilterBase**). We must then override a few methods to perform the filtering.

The base class handles the field mapping in the **getOriginalYValues** method (and **getOriginalY1Values** for Xyy, and similar for the other types). See the **CustomFilter** page for more information or the online demoÂ <a href="https://www.scichart.com/demo/javascript-custom-filters" rel="noopener noreferrer" target="_blank">Realtime Aggregation using Advanced Custom Filter</a>.

## Filters API Demos<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#filters-api-demos" class="hash-link" aria-label="Direct link to Filters API Demos" translate="no" title="Direct link to Filters API Demos">â€‹</a>

Several demos are included in our JavaScript Chart Examples Suite. Please find them below:

- <a href="https://www.scichart.com/demo/javascript-percentage-change" rel="noopener noreferrer" target="_blank">Percentage Change using XyScaleOffsetFilter Example</a>
- <a href="https://www.scichart.com/demo/javascript-trend-ma-ratio" rel="noopener noreferrer" target="_blank">Ratio Filter with Chained Trend and Moving Average Example</a>
- <a href="https://www.scichart.com/demo/javascript-custom-filters" rel="noopener noreferrer" target="_blank">Realtime Aggregation using Advanced Custom Filter Example</a>

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Scale Offset Filters](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters)
- [Linear Trendline Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter)
- [Creating a Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter)
- [Ratio Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/ratio-filter)
- [Moving Average Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-filters-api/data-filters-api-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-filters-api/data-filters-api-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
