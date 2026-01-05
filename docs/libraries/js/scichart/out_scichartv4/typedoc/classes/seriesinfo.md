<img src="out_scichartv4/typedoc/classes/seriesinfo_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SeriesInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html)

# Class SeriesInfo

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

SeriesInfo is a data-structure which provides enriched information about a hit-test operation. It's derived by calling {@link BaseRenderableSeries.hitTestProvider.hitTest} (returns [HitTestInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html)) and then enriched by calling [BaseRenderableSeries.getSeriesInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/baserenderableseries.html#getseriesinfo). There is a class hierachy for [SeriesInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html) which is a different class depending on series type, e.g. line, mountain, scatter series has [XySeriesInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html), heatmap series as [HeatmapSeriesInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapseriesinfo.html) etc.

### Hierarchy

- SeriesInfo
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedxyseriesinfo.html" class="tsd-signature-type">StackedXySeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyyseriesinfo.html" class="tsd-signature-type">XyySeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapseriesinfo.html" class="tsd-signature-type">HeatmapSeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hlcseriesinfo.html" class="tsd-signature-type">HlcSeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ohlcseriesinfo.html" class="tsd-signature-type">OhlcSeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyseriesinfo.html" class="tsd-signature-type">XySeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzseriesinfo.html" class="tsd-signature-type">XyzSeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/triangleseriesinfo.html" class="tsd-signature-type">TriangleSeriesInfo</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/boxplotseriesinfo.html" class="tsd-signature-type">BoxPlotSeriesInfo</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#dataseriesindex" class="tsd-kind-icon">dataSeriesIndex</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#dataseriestype" class="tsd-kind-icon">dataSeriesType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#distance" class="tsd-kind-icon">distance</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#fill" class="tsd-kind-icon">fill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#hittestinfo" class="tsd-kind-icon">hitTestInfo</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#hittestpointvalues" class="tsd-kind-icon">hitTestPointValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#ishit" class="tsd-kind-icon">isHit</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#iswithindatabounds" class="tsd-kind-icon">isWithinDataBounds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#point2metadata" class="tsd-kind-icon">point2metadata</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#point2xcoordinate" class="tsd-kind-icon">point2xCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#point2xvalue" class="tsd-kind-icon">point2xValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#point2ycoordinate" class="tsd-kind-icon">point2yCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#point2yvalue" class="tsd-kind-icon">point2yValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#pointmetadata" class="tsd-kind-icon">pointMetadata</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#renderableseries" class="tsd-kind-icon">renderableSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#seriesname" class="tsd-kind-icon">seriesName</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#stroke" class="tsd-kind-icon">stroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#xcoordinate" class="tsd-kind-icon">xCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#xvalue" class="tsd-kind-icon">xValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#ycoordinate" class="tsd-kind-icon">yCoordinate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#yvalue" class="tsd-kind-icon">yValue</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#formattedxvalue" class="tsd-kind-icon">formattedXValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#formattedyvalue" class="tsd-kind-icon">formattedYValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#isvisible" class="tsd-kind-icon">isVisible</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#concattwovalueswithlabels" class="tsd-kind-icon">concatTwoValuesWithLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#concatvaluewithlabel" class="tsd-kind-icon">concatValueWithLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#equals" class="tsd-kind-icon">equals</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#getvalueswithlabels" class="tsd-kind-icon">getValuesWithLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#getxcursorformattedvalue" class="tsd-kind-icon">getXCursorFormattedValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html#getycursorformattedvalue" class="tsd-kind-icon">getYCursorFormattedValue</a>

## Constructors

### constructor

- new SeriesInfo(renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>, hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" class="tsd-signature-type">SeriesInfo</a>

<!-- -->

- #### Parameters

  - ##### renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  - ##### hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" class="tsd-signature-type">SeriesInfo</a>

## Properties

### dataSeriesIndex

dataSeriesIndex: number

### dataSeriesType

dataSeriesType: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype.html" class="tsd-signature-type">EDataSeriesType</a>

Gets the [EDataSeriesType](https://www.scichart.com/documentation/js/v4/typedoc/enums/edataseriestype.html) of the [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html) which is the result of the hit-test operation

### distance

distance: number

### fill

fill: string

### hitTestInfo

hitTestInfo: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/hittestinfo.html" class="tsd-signature-type">HitTestInfo</a>

### hitTestPointValues

hitTestPointValues: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### isHit

isHit: boolean

### isWithinDataBounds

isWithinDataBounds: boolean

When true the hit-test operation was within the first and the last DataSeries X-values

### point2metadata

point2metadata: unknown

### point2xCoordinate

point2xCoordinate: number

### point2xValue

point2xValue: number

### point2yCoordinate

point2yCoordinate: number

### point2yValue

point2yValue: number

### pointMetadata

pointMetadata: unknown

### Readonly renderableSeries

renderableSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

Gets the associated [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html) that this [SeriesInfo](https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html) was generated by

### seriesName

seriesName: string

Gets the name of the [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries.html) which is the result of the hit-test operation

### stroke

stroke: string

### xCoordinate

xCoordinate: number

### xValue

xValue: number

### yCoordinate

yCoordinate: number

### yValue

yValue: number

## Accessors

### formattedXValue

- get formattedXValue(): string

<!-- -->

- #### Returns string

### formattedYValue

- get formattedYValue(): string

<!-- -->

- #### Returns string

### isVisible

- get isVisible(): boolean

<!-- -->

- #### Returns boolean

## Methods

### Protected concatTwoValuesWithLabels

- concatTwoValuesWithLabels(xValue: string, yValue: string, xLabel: string, yLabel: string): string

<!-- -->

- #### Parameters

  - ##### xValue: string

  - ##### yValue: string

  - ##### xLabel: string

  - ##### yLabel: string

  #### Returns string

### Protected concatValueWithLabel

- concatValueWithLabel(value: string, label: string): string

<!-- -->

- #### Parameters

  - ##### value: string

  - ##### label: string

  #### Returns string

### equals

- equals(other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" class="tsd-signature-type">SeriesInfo</a>): boolean

<!-- -->

- #### Parameters

  - ##### other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/seriesinfo.html" class="tsd-signature-type">SeriesInfo</a>

  #### Returns boolean

### getValuesWithLabels

- getValuesWithLabels(title: string \| undefined, xLabel: string, yLabel: string, isXYGrouped: boolean): string\[\]

<!-- -->

- #### Parameters

  - ##### title: string \| undefined

  - ##### xLabel: string

  - ##### yLabel: string

  - ##### isXYGrouped: boolean

  #### Returns string\[\]

### getXCursorFormattedValue

- getXCursorFormattedValue(value: number): string

<!-- -->

- #### Parameters

  - ##### value: number

  #### Returns string

### getYCursorFormattedValue

- getYCursorFormattedValue(value: number): string

<!-- -->

- #### Parameters

  - ##### value: number

  #### Returns string

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
