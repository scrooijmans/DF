<img src="out_scichartv4/typedoc/interfaces/inonuniformheatmapseriesoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [INonUniformHeatmapSeriesOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html)

# Interface INonUniformHeatmapSeriesOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options to pass to the [NonUniformHeatmapSeries](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype.html#nonuniformheatmapseries) constructor

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibaseheatmapseriesoptions.html" class="tsd-signature-type">IBaseHeatmapSeriesOptions</a>
  - INonUniformHeatmapSeriesOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#containsnan" class="tsd-kind-icon">containsNaN</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#dataseriesname" class="tsd-kind-icon">dataSeriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#metadata" class="tsd-kind-icon">metadata</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#xcelloffsets" class="tsd-kind-icon">xCellOffsets</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#ycelloffsets" class="tsd-kind-icon">yCellOffsets</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/inonuniformheatmapseriesoptions.html#zvalues" class="tsd-kind-icon">zValues</a>

## Properties

### Optional containsNaN

containsNaN: boolean

The flag whether this Heatmap has NaN value, to display them as transparent tiles

### Optional dataSeriesName

dataSeriesName: string

The DataSeries name, used in legends, tooltips to identify the chart series

### Optional id

id: string

A unique Id for the [IDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idataseries.html)

### Optional metadata

metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\]\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a> \| { data?: any; type: string }

The Metadata values of type [IPointMetadata](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html) to pre-populate the [BaseHeatmapDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baseheatmapdataseries.html) If a single metadata value is supplied, this will be used as a template for all data values. If type AND data is specified, then the registered function should take the data and return a IPointData array. If only type is specified, the registered function will be set as the metadataGenerator and used to construct new metadata for each data value.

### xCellOffsets

xCellOffsets: number\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcellsizemapper" class="tsd-signature-type">TCellSizeMapper</a>

xCellOffsets defines cell X offsets on [NonUniformHeatmapDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html) for each cell in the heatmap. Can be an array of numbers or a mapping function [TCellSizeMapper](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcellsizemapper)

remarks  
mapping function would not be preserved by chart serialization

### yCellOffsets

yCellOffsets: number\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcellsizemapper" class="tsd-signature-type">TCellSizeMapper</a>

yCellOffsets defines cell Y offsets on [NonUniformHeatmapDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmapdataseries.html) for each cell in the heatmap. Can be an array of numbers or a mapping function [TCellSizeMapper](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcellsizemapper)

remarks  
mapping function would not be preserved by chart serialization

### Optional zValues

zValues: number\[\]\[\]

The 2-Dimensional array of cells which can be passed to populate the [BaseHeatmapDataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/baseheatmapdataseries.html) at construct time

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
