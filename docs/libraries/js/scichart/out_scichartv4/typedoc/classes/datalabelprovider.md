<img src="out_scichartv4/typedoc/classes/datalabelprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [DataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html)

# Class DataLabelProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

This is the standard DataLabelProvider which provides a number of options for configuring data labels. It defines a much richer api than BaseDataLabelProvider and is intended to be used as a base for doing small changes to data label behaviour generateDataLabels calls the following functions which you can override parts of the behaviour [ySelector](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#yselector) to pick the desired yValues from the pointSeries. [shouldGenerate](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldgenerate) to determine if any labels should be generated so you can enable labels depending on zoom or data level. Then for each data point: [getText](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#gettext) Build in behaviour can get text from metadata using [metaDataSelector](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#metadataselector) or format the y values using [numericFormat](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#numericformat) and [precision](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#precision) [getPosition](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getposition) By default return the x and y coordinate of the data point. Series-specific DataLabelProviders eg [LineSeriesDataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdatalabelprovider.html) have logic to adjust text position based on the series [getColor](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getcolor) Returns the color from the text style. Use [parseColorToUIntArgb](https://www.scichart.com/documentation/js/v4/typedoc/index.html#parsecolortouintargb) to turn color string to the required numeric value if overriding. [shouldSkipLabel](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldskiplabel) Decides whether to keep or ship the generated label based on the {@link }

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/basedatalabelprovider.html" class="tsd-signature-type">BaseDataLabelProvider</a>
  - DataLabelProvider
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/lineseriesdatalabelprovider.html" class="tsd-signature-type">LineSeriesDataLabelProvider</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnseriesdatalabelprovider.html" class="tsd-signature-type">ColumnSeriesDataLabelProvider</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/stackedcolumnseriesdatalabelprovider.html" class="tsd-signature-type">StackedColumnSeriesDataLabelProvider</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/bubbleseriesdatalabelprovider.html" class="tsd-signature-type">BubbleSeriesDataLabelProvider</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rectangleseriesdatalabelprovider.html" class="tsd-signature-type">RectangleSeriesDataLabelProvider</a>
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polardatalabelprovider.html" class="tsd-signature-type">PolarDataLabelProvider</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#colorproperty" class="tsd-kind-icon">colorProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#datalabels" class="tsd-kind-icon">dataLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#drawimmediate" class="tsd-kind-icon">drawImmediate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#engineeringprefixproperty" class="tsd-kind-icon">engineeringPrefixProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#horizontaltextpositionproperty" class="tsd-kind-icon">horizontalTextPositionProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#isenabledproperty" class="tsd-kind-icon">isEnabledProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#metadataselector" class="tsd-kind-icon">metaDataSelector</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#numericformatproperty" class="tsd-kind-icon">numericFormatProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#parentseries" class="tsd-kind-icon">parentSeries</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthresholdproperty" class="tsd-kind-icon">pointCountThresholdProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthresholdproperty" class="tsd-kind-icon">pointGapThresholdProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#precisionproperty" class="tsd-kind-icon">precisionProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipmodeproperty" class="tsd-kind-icon">skipModeProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipnumberproperty" class="tsd-kind-icon">skipNumberProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#state" class="tsd-kind-icon">state</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#styleproperty" class="tsd-kind-icon">styleProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#updatetextinanimation" class="tsd-kind-icon">updateTextInAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#verticaltextpositionproperty" class="tsd-kind-icon">verticalTextPositionProperty</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#webassemblycontext" class="tsd-kind-icon">webAssemblyContext</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#yselector" class="tsd-kind-icon">ySelector</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#engineeringprefix" class="tsd-kind-icon">engineeringPrefix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#horizontaltextposition" class="tsd-kind-icon">horizontalTextPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#isenabled" class="tsd-kind-icon">isEnabled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#numericformat" class="tsd-kind-icon">numericFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthreshold" class="tsd-kind-icon">pointCountThreshold</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold" class="tsd-kind-icon">pointGapThreshold</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#precision" class="tsd-kind-icon">precision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipmode" class="tsd-kind-icon">skipMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#skipnumber" class="tsd-kind-icon">skipNumber</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#style" class="tsd-kind-icon">style</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#userotation" class="tsd-kind-icon">useRotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#verticaltextposition" class="tsd-kind-icon">verticalTextPosition</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#draw" class="tsd-kind-icon">draw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#generatedatalabels" class="tsd-kind-icon">generateDataLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getcolor" class="tsd-kind-icon">getColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getdatalabelstate" class="tsd-kind-icon">getDataLabelState</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#getposition" class="tsd-kind-icon">getPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#gettext" class="tsd-kind-icon">getText</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#invalidateparent" class="tsd-kind-icon">invalidateParent</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#resolveautocolors" class="tsd-kind-icon">resolveAutoColors</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldgenerate" class="tsd-kind-icon">shouldGenerate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldskiplabel" class="tsd-kind-icon">shouldSkipLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#updatestyle" class="tsd-kind-icon">updateStyle</a>

## Constructors

### constructor

- new DataLabelProvider(options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatalabelprovideroptions.html" class="tsd-signature-type">IDataLabelProviderOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html" class="tsd-signature-type">DataLabelProvider</a>

<!-- -->

- Creates an instance of the [DataLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#datalabelprovider)

  #### Parameters

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/idatalabelprovideroptions.html" class="tsd-signature-type">IDataLabelProviderOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html" class="tsd-signature-type">DataLabelProvider</a>

## Properties

### Protected colorProperty

colorProperty: string

### dataLabels

dataLabels: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabel" class="tsd-signature-type">TDataLabel</a>\[\] = \[\]

The text to draw, along with the sizes and positions. Usually generated, but can be set or updated before final drawing

### drawImmediate

drawImmediate: boolean = false

Draws immediately, set this True when using sub-surfaces for proper data label layering

### Protected engineeringPrefixProperty

engineeringPrefixProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>

### Protected horizontalTextPositionProperty

horizontalTextPositionProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehorizontaltextposition.html" class="tsd-signature-type">EHorizontalTextPosition</a> = EHorizontalTextPosition.Right

### Protected isEnabledProperty

isEnabledProperty: boolean = true

### metaDataSelector

metaDataSelector: (metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>) =\> string

If this is set it will be used to get text values from metaData, rather than formatting y values. The selector will be called even if the metaData for an index is undefined.

#### Type declaration

- - (metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>): string

  <!-- -->

  - #### Parameters

    - ##### metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

    #### Returns string

### Protected numericFormatProperty

numericFormatProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a> = ENumericFormat.Decimal

### Protected parentSeries

parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

The Parent [RenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html)

### Protected pointCountThresholdProperty

pointCountThresholdProperty: number = Infinity

### Protected pointGapThresholdProperty

pointGapThresholdProperty: number = 0

### Protected precisionProperty

precisionProperty: number = 1

### Protected skipModeProperty

skipModeProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" class="tsd-signature-type">EDataLabelSkipMode</a> = EDataLabelSkipMode.SkipIfOverlapPrevious

### Protected skipNumberProperty

skipNumberProperty: number = 0

### Protected state

state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

### Protected styleProperty

styleProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

### Readonly type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a> = EDataLabelProviderType.Default

### updateTextInAnimation

updateTextInAnimation: boolean = false

Whether the label text should update when the label position is animating. Default false.

### Protected verticalTextPositionProperty

verticalTextPositionProperty: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticaltextposition.html" class="tsd-signature-type">EVerticalTextPosition</a> = EVerticalTextPosition.Above

### Protected webAssemblyContext

webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

The [SciChart 2D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

### ySelector

ySelector: (ps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>) =\> SCRTDoubleVector

A function to pick which y values to use from the pointSeries passed in on the renderPassData

#### Type declaration

- - (ps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>): SCRTDoubleVector

  <!-- -->

  - #### Parameters

    - ##### ps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

    #### Returns SCRTDoubleVector

## Accessors

### color

- get color(): string
- set color(value: string): void

<!-- -->

- Gets or sets the color for data labels. Defaults to axis label color

  #### Returns string

- Gets or sets the color for data labels. Defaults to axis label color

  #### Parameters

  - ##### value: string

  #### Returns void

### engineeringPrefix

- get engineeringPrefix(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>
- set engineeringPrefix(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>): void

<!-- -->

- Gets the engineering prefixes to use when formatting values to text.

  remarks  
  Only works when [ENumericFormat.Engineering](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#engineering) is selected

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>

- Gets or sets the engineering prefixes to use when formatting values to text. Default - `['K','M','B','T']` for "large" prefixes, `['m','u','n','p']` for small prefixes

  remarks  
  Only works when [ENumericFormat.Engineering](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#engineering) is selected

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>

  #### Returns void

### horizontalTextPosition

- get horizontalTextPosition(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehorizontaltextposition.html" class="tsd-signature-type">EHorizontalTextPosition</a>
- set horizontalTextPosition(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehorizontaltextposition.html" class="tsd-signature-type">EHorizontalTextPosition</a>): void

<!-- -->

- Gets or sets the horizontal text position for the label For more control, override getPosition

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehorizontaltextposition.html" class="tsd-signature-type">EHorizontalTextPosition</a>

- Gets or sets the horizontal text position for the label For more control, override getPosition

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehorizontaltextposition.html" class="tsd-signature-type">EHorizontalTextPosition</a>

  #### Returns void

### isEnabled

- get isEnabled(): boolean
- set isEnabled(value: boolean): void

<!-- -->

- Flag to enable/disable dataLabel generation. Default true

  #### Returns boolean

- Flag to enable/disable dataLabel generation. Default true

  #### Parameters

  - ##### value: boolean

  #### Returns void

### numericFormat

- get numericFormat(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>
- set numericFormat(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>): void

<!-- -->

- Gets or sets numeric format to use when formatting values to text. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html) For more control, override getText

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

- Gets or sets numeric format to use when formatting values to text. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html) For more control, override getText

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

  #### Returns void

### pointCountThreshold

- get pointCountThreshold(): number
- set pointCountThreshold(value: number): void

<!-- -->

- Gets or sets the number of points below which drawing will start. Default Infinity This can be used instead of [pointGapThreshold](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold) when data is unevenly spaced or has large y variation

  #### Returns number

- Gets or sets the number of points below which drawing will start. Default Infinity This can be used instead of [pointGapThreshold](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold) when data is unevenly spaced or has large y variation

  #### Parameters

  - ##### value: number

  #### Returns void

### pointGapThreshold

- get pointGapThreshold(): number
- set pointGapThreshold(value: number): void

<!-- -->

- Gets or sets the zoom threshold above which label drawing will start. Default 0. This is expressed as the gap between the first points divided by the size of the first text If data is unevenly spaced, consider [pointCountThreshold](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthreshold) or override shouldDrawText

  #### Returns number

- Gets or sets the zoom threshold above which label drawing will start. Default 0. This is expressed as the gap between the first points divided by the size of the first text If data is unevenly spaced, consider [pointCountThreshold](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthreshold) or override shouldDrawText

  #### Parameters

  - ##### value: number

  #### Returns void

### precision

- get precision(): number
- set precision(value: number): void

<!-- -->

- Gets or sets the precision to use when formatting values to text For more control, override getText

  #### Returns number

- Gets or sets the precision to use when formatting values to text For more control, override getText

  #### Parameters

  - ##### value: number

  #### Returns void

### skipMode

- get skipMode(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" class="tsd-signature-type">EDataLabelSkipMode</a>
- set skipMode(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" class="tsd-signature-type">EDataLabelSkipMode</a>): void

<!-- -->

- How do decide whether to keep or skip a label once generated. Override [shouldSkipLabel](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldskiplabel) for more control

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" class="tsd-signature-type">EDataLabelSkipMode</a>

- How do decide whether to keep or skip a label once generated. Override [shouldSkipLabel](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#shouldskiplabel) for more control

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" class="tsd-signature-type">EDataLabelSkipMode</a>

  #### Returns void

### skipNumber

- get skipNumber(): number
- set skipNumber(value: number): void

<!-- -->

- The number of points to skip while generating labels. Default 0 = no skip. 1 = skip every other. When creating text with many data points, it will help performance to skip points rather than creating and checking overlap for every data point.

  #### Returns number

- The number of points to skip while generating labels. Default 0 = no skip. 1 = skip every other. When creating text with many data points, it will help performance to skip points rather than creating and checking overlap for every data point.

  #### Parameters

  - ##### value: number

  #### Returns void

### style

- get style(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>
- set style(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>): void

<!-- -->

- Gets or sets the text style used for data labels. The style must be set, with fontFamily and fontSize set, in order for text to be drawn.

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

- Gets or sets the text style used for data labels. The style must be set, with fontFamily and fontSize set, in order for text to be drawn.

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

  #### Returns void

### Protected useRotation

- get useRotation(): boolean

<!-- -->

- #### Returns boolean

### verticalTextPosition

- get verticalTextPosition(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticaltextposition.html" class="tsd-signature-type">EVerticalTextPosition</a>
- set verticalTextPosition(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticaltextposition.html" class="tsd-signature-type">EVerticalTextPosition</a>): void

<!-- -->

- Gets or sets the vertical text position for the label For more control, override getPosition

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticaltextposition.html" class="tsd-signature-type">EVerticalTextPosition</a>

- Gets or sets the vertical text position for the label For more control, override getPosition

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticaltextposition.html" class="tsd-signature-type">EVerticalTextPosition</a>

  #### Returns void

## Methods

### delete

- delete(): void

<!-- -->

- #### Returns void

### draw

- draw(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>): void

<!-- -->

- #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  #### Returns void

### generateDataLabels

- generateDataLabels(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>): void

<!-- -->

- Generates labels using getText, getPosition, getColor. Overrides manually set labels.

  #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  #### Returns void

### getColor

- getColor(state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>, text: string): number

<!-- -->

- #### Parameters

  - ##### state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

  - ##### text: string

  #### Returns number

### getDataLabelState

- getDataLabelState(renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>, renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>, style: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>, color: string, yValues: SCRTDoubleVector, parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

<!-- -->

- Called by generateDataLabels to get the DataLabelState helper object that supplies label values and coordinates

  #### Parameters

  - ##### renderContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/webglrendercontext2d.html" class="tsd-signature-type">WebGlRenderContext2D</a>

  - ##### renderPassData: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/renderpassdata.html" class="tsd-signature-type">RenderPassData</a>

  - ##### style: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

  - ##### color: string

  - ##### yValues: SCRTDoubleVector

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

### getPosition

- getPosition(state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>, textBounds: TSRTextBounds): { position: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>; rotationAngle: number; rotationCenter: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a> }

<!-- -->

- Returns x, y coordinates where the data-label should be drawn

  #### Parameters

  - ##### state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

  - ##### textBounds: TSRTextBounds

  #### Returns { position: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>; rotationAngle: number; rotationCenter: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a> }

  - ##### position: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

  - ##### rotationAngle: number

  - ##### rotationCenter: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### getText

- getText(state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>): string

<!-- -->

- Applies numeric format to current yValue

  #### Parameters

  - ##### state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

  #### Returns string

### Protected invalidateParent

- invalidateParent(): void

<!-- -->

- #### Returns void

### onAttach

- onAttach(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>, parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>): void

<!-- -->

- inheritdoc  

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  - ##### parentSeries: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries.html" class="tsd-signature-type">IRenderableSeries</a>

  #### Returns void

### onDetach

- onDetach(): void

<!-- -->

- #### Returns void

### resolveAutoColors

- resolveAutoColors(index: number, maxSeries: number, theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): void

<!-- -->

- #### Parameters

  - ##### index: number

  - ##### maxSeries: number

  - ##### theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

  #### Returns void

### shouldGenerate

- shouldGenerate(state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>): boolean

<!-- -->

- Called at the start of generateDataLabels. If False, no labels will be generated. Checks [pointCountThreshold](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointcountthreshold) then [pointGapThreshold](https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelprovider.html#pointgapthreshold)

  #### Parameters

  - ##### state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

  #### Returns boolean

### shouldSkipLabel

- shouldSkipLabel(state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>, label: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabel" class="tsd-signature-type">TDataLabel</a>): boolean

<!-- -->

- Called before adding a new label to state. If True the label will not be rendered.

  #### Parameters

  - ##### state: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/datalabelstate.html" class="tsd-signature-type">DataLabelState</a>

  - ##### label: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabel" class="tsd-signature-type">TDataLabel</a>

  #### Returns boolean

### toJSON

- toJSON(): { type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>; options: object }

<!-- -->

- #### Returns { type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>; options: object }

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelprovidertype.html" class="tsd-signature-type">EDataLabelProviderType</a>

  - ##### options: object

    - ##### color: string

    - ##### style: { fontFamily?: string; fontSize?: number; lineSpacing?: number; multiLineAlignment?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emultilinealignment.html" class="tsd-signature-type">EMultiLineAlignment</a>; padding?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a> }

      - ##### Optional fontFamily?: string

      - ##### Optional fontSize?: number

      - ##### Optional lineSpacing?: number

      - ##### Optional multiLineAlignment?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emultilinealignment.html" class="tsd-signature-type">EMultiLineAlignment</a>

      - ##### Optional padding?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

### updateStyle

- updateStyle(value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>): void

<!-- -->

- Update the style. Only the properties passed will be updated

  #### Parameters

  - ##### value: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

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
