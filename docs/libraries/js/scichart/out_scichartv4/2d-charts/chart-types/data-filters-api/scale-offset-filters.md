On this page

# Scale Offset Filters

The ScaleOffsetFilter Applies a scale (multiplier) and an offset (addition) to each field value in a Data Series. There is a specific filter for each type of dataseries:

- **XyScaleOffsetFilter**
- **XyyScaleOffsetFilter**
- **XyzScaleOffsetFilter**
- **OhlcScaleOffsetFilter**

## Applying Scale & Offset to Chart Data<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters/#applying-scale--offset-to-chart-data" class="hash-link" aria-label="Direct link to Applying Scale &amp; Offset to Chart Data" translate="no" title="Direct link to Applying Scale &amp; Offset to Chart Data">â€‹</a>

To create an **XyScaleOffsetFilter** and apply it to your chart, use the following code:

- TS

``` prism-code
import {
    SciChartSurface,
    NumericAxis,
    XyDataSeries,
    FastLineRenderableSeries,
    NumberRange,
    XyScaleOffsetFilter 
} from "scichart";
...
const { sciChartSurface, wasmContext } = await SciChartSurface.create('scichart-div-id');
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.1, 0.1) }));
// Original Data
const dataSeries = new XyDataSeries(wasmContext, {
    xValues: [1, 2, 3, 4],
    yValues: [1, 2, 3, 4],
});
const originalLine = new FastLineRenderableSeries(wasmContext, { dataSeries, stroke: "#5555ff" });
// Create the filter, passing in the original series
const scaleOffsetFilter = new XyScaleOffsetFilter(dataSeries, { scale: 2, offset: -3 });
const filteredLine = new FastLineRenderableSeries(wasmContext, { dataSeries: scaleOffsetFilter, stroke: "#cc6600" });
sciChartSurface.renderableSeries.add(originalLine, filteredLine);
```

This produces the following chart where the orange filtered data is twice as steep, and shifted down by 3.

<img src="out_scichartv4/2d-charts/chart-types/data-filters-api/scale-offset-filters/index_media/cc66cd14189cf5fc6f879d0dce2f54bab891a8c4.png" class="img_ev3q" decoding="async" loading="lazy" width="798" height="599" />

With the Filters API in SciChart.js, if you update the original data, or any of the parameters of the filter, the chart will automatically redraw.

Note that ScaleOffsetFilter only changes data in the Y direction. If you want to shift data in X, create aÂ [Complex Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter). The other ScaleOffsetFilters apply the same transformation to every non-x field. If you want to apply different filters to different fields, create aÂ [Complex Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter).

## Specifying the Input Field<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters/#specifying-the-input-field" class="hash-link" aria-label="Direct link to Specifying the Input Field" translate="no" title="Direct link to Specifying the Input Field">â€‹</a>

An Xy filter will produce an XyDataSeries, but it can accept any series type as input. The options includes a **field** property of type **EDataSeriesField**, which determines which field on the original series will be the input. For Xyy filters there are yField and y1Field, and for Xyz filters there are yField and zField options.

## Percentage Change<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters/#percentage-change" class="hash-link" aria-label="Direct link to Percentage Change" translate="no" title="Direct link to Percentage Change">â€‹</a>

You can use a ScaleOffset filter to show the percentage change in a series, which is useful for comparing data at different scales. For a running example of this with code see ourÂ <a href="https://www.scichart.com/demo/javascript-percentage-change" rel="noopener noreferrer" target="_blank">Percentage Change demo</a>

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/scale-offset-filters/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [What is the Filters API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview)
- [Linear Trendline Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/linear-trendline-filter)
- [Creating a Custom Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/custom-filter)
- [Ratio Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/ratio-filter)
- [Moving Average Filter](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/moving-average-filter)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/data-filters-api/scale-offset-filters/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/data-filters-api/scale-offset-filters/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
