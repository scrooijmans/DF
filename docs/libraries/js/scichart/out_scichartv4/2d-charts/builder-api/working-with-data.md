On this page

# Working with Data

## Creating or Updating Data on Builder API Created charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#creating-or-updating-data-on-builder-api-created-charts" class="hash-link" aria-label="Direct link to Creating or Updating Data on Builder API Created charts" translate="no" title="Direct link to Creating or Updating Data on Builder API Created charts">â€‹</a>

Data can be supplied to charts created with the SciChart.js Builder API in one of three ways:

### 1. Supply Data via Values properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#1-supply-data-via-values-properties" class="hash-link" aria-label="Direct link to 1. Supply Data via Values properties" translate="no" title="Direct link to 1. Supply Data via Values properties">â€‹</a>

Values properties can be supplied within the series definition property.

This method is demonstrated below.Â This is also the format you will get by default when you serialise a chart containing data.

- TS

``` prism-code
const { sciChartSurface, wasmContext } = await chartBuilder.buildChart(divElementId, {
    series: {
        type: ESeriesType.LineSeries,
        xyData: {
            xValues: [1, 3, 4, 7, 9],
            yValues: [10, 6, 7, 2, 16]
        }
    }
});
```

### 2. Reference sharedData using dataId properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#2-reference-shareddata-using-dataid-properties" class="hash-link" aria-label="Direct link to 2. Reference sharedData using dataId properties" translate="no" title="Direct link to 2. Reference sharedData using dataId properties">â€‹</a>

Often you will want to define the structure of the chart, and reuse it with different data. Instead of setting `xValues` and `yValues`, you set `xDataId` and `yDataId` to the names you use in a `sharedData` section.

For example:

- JS

``` prism-code
const { chartBuilder, ESeriesType } = SciChart;

const DATA = { 
    x: [1, 2, 3, 4, 5], 
    col: [8, 2, 3, 7, 10], 
    line: [10, 6, 7, 2, 16] 
};

const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    series: [
        { type: ESeriesType.ColumnSeries, xyData: { xDataId: "x", yDataId: "col" } },
        { type: ESeriesType.LineSeries, xyData: { xDataId: "x", yDataId: "line" } },
    ],
    sharedData: DATA // pass the shared data object
});
```

This is good for multiple series which share x data, but is not as convenient if you want to be able to update the data later. For this you need to use our [DataSeries API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-series-api/data-series-api-overview).

### 3. Create a DataSeries and Manually Assign it<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#3-create-a-dataseries-and-manually-assign-it" class="hash-link" aria-label="Direct link to 3. Create a DataSeries and Manually Assign it" translate="no" title="Direct link to 3. Create a DataSeries and Manually Assign it">â€‹</a>

Once the chart is created, you can use the `wasmContext` that is returned to create a `dataSeries` in the normal way.

Here weâ€™re using <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder#build2dchart" rel="noopener noreferrer" target="_blank">build2DChartðŸ“˜</a> rather than <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder#buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a> so that we donâ€™t have to cast the result.

![](out_scichartv4/2d-charts/builder-api/working-with-data/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Note that <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder#build2dchart" rel="noopener noreferrer" target="_blank">build2DChartðŸ“˜</a> (and <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#chartbuilder#buildchart" rel="noopener noreferrer" target="_blank">buildChartðŸ“˜</a>) returns a `Promise` so we need to resolve it to use the result, e.g. use `async/await` syntax or `Promise chaining`.

- JS

``` prism-code
const { chartBuilder, XyDataSeries, ESeriesType } = SciChart;

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    series: [
        { 
            type: ESeriesType.ColumnSeries,
            options: {
                id: "columnSeries1",
                stroke: "blue",
                fill: "rgba(0, 0, 255, 0.3)"
            }
            // no "xyData" provided just yet, we will add it later
        }
    ]
});

const dataSeries = new XyDataSeries(wasmContext, { 
    xValues: [1, 2, 3, 4, 5], 
    yValues: [8, 2, 3, 7, 10] 
});
// assign the dataSeries to the renderable series
sciChartSurface.renderableSeries.get(0).dataSeries = dataSeries;

// Alternatively, you can set the dataSeries by ID:
// sciChartSurface.renderableSeries.getById("columnSeries1").dataSeries = dataSeries;
```

![](out_scichartv4/2d-charts/builder-api/working-with-data/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Note that Charts, Series and DataSeries created when using the Builder API can be accessed and modified using the JavaScript programmatic API in SciChart.js. If you want deep customization of the chart but a simple way to create templates, then this API is very powerful!

## Using the Filters API with the Builder API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#using-the-filters-api-with-the-builder-api" class="hash-link" aria-label="Direct link to Using the Filters API with the Builder API" translate="no" title="Direct link to Using the Filters API with the Builder API">â€‹</a>

SciChart.js v2.x features a newÂ [Filters API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview), which allows you to apply dynamic data transforms to data series which update as your underlying data updates.

Here is an example of adding a `Filter` or `DataTransform` to a SciChartSurface when using the Builder API:

- TS

``` prism-code
const { chartBuilder, ESeriesType, EDataFilterType } = SciChart;

const xyData = {
    xValues: [1, 2, 3, 4, 5, 6],
    yValues: [2, 5, 7, 4, 10, 15]
};
chartBuilder.buildChart(divElementId, {
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData, 
        },
        {
            type: ESeriesType.LineSeries,
            options: { stroke: "red" },
            xyData: {
                ...xyData,
                filter: { type: EDataFilterType.XyLinearTrend }
            }
        }
    ]
});
```

For more details regarding the Filters API, check theÂ [Filter API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-filters-api/data-filters-api-overview).

## Using PointMetadata with the Builder API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#using-pointmetadata-with-the-builder-api" class="hash-link" aria-label="Direct link to Using PointMetadata with the Builder API" translate="no" title="Direct link to Using PointMetadata with the Builder API">â€‹</a>

SciChart.js v2.x features a newÂ [PointMetadata API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/point-metadata-api-overview), which allows you to tag any X, Y datapoint with a custom object confirming to the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" rel="noopener noreferrer" target="_blank">IPointMetadataðŸ“˜</a> interface.

This lets you tag datapoints with objects, mark them as selected or deselected, or include further information to display in tooltips, on hit-test or selection etc...

When working with the Builder API, some extra consideration is needed if you are planning to serialize and deserialize metadata.

1.  You need a copy of the same metadata object applied to every point. This is needed to support datapoint selection. In this case, set the metadata poroperty on the dataSeries options to your desired object and it will be cloned to every point that is added. It will be serialized exactly as added.

``` prism-code
xyData: {
    metadata: {
        isSelected: false
        }
    }
```

2.You need to set an array of metadata with values specific to each data point. As long as your metadata object is pure data, just set the array on the metadata property.

3.Your metadata object contains functions. Now you need to supply a type name of a registered <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html" rel="noopener noreferrer" target="_blank">IMetadataGeneratorðŸ“˜</a>. This interface can return a single object which will be used to populate each data point, or as <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i1dmetadatagenerator.html" rel="noopener noreferrer" target="_blank">I1DMetadataGeneratorðŸ“˜</a> (or <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dmetadatagenerator.html" rel="noopener noreferrer" target="_blank">I2DMetadataGeneratorðŸ“˜</a> for heatmap data) it can return an array which should be the same size as your data. In this case you will probably want to set the data property, which will be passed into the function you register to create your metadataGenerator. In this case, the output of the toJSON method on the metadataGenerator should match the format of data passed in. As before, donâ€™t forget to define and register these things on the client. Hopefully now the type signature of the metadata option makes some sense.

``` prism-code
metadata?:
    | IPointMetadata[]
    | IPointMetadata
    | { type: string; data?: any };
```

For more information regarding the PointMetadata API, check theÂ [PointMetadata API Documentation](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/point-metadata-api/point-metadata-api-overview).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/working-with-data/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Intro to the Builder API](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/builder-api-overview)

- [Creating a Simple Chart](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/simple-chart)

- [Creating a Pie Chart](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/pie-chart)

- [Creating a Polar Chart](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/polar-chart)

- [Creating a 3D Chart](https://www.scichart.com/documentation/js/v4/3d-charts/builder-api/default-3d-chart)

- [Complex Options](https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/complex-options)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/working-with-data/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/working-with-data/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
