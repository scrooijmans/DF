<img src="out_scichartv4/typedoc/interfaces/ibandseriesdatalabelprovideroptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IBandSeriesDataLabelProviderOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html)

# Interface IBandSeriesDataLabelProviderOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilineseriesdatalabelprovideroptions.html" class="tsd-signature-type">ILineSeriesDataLabelProviderOptions</a>
  - IBandSeriesDataLabelProviderOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#abovebelow" class="tsd-kind-icon">aboveBelow</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#drawimmediate" class="tsd-kind-icon">drawImmediate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#engineeringprefix" class="tsd-kind-icon">engineeringPrefix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#horizontaltextposition" class="tsd-kind-icon">horizontalTextPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#isenabled" class="tsd-kind-icon">isEnabled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#metadataselector" class="tsd-kind-icon">metaDataSelector</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#numericformat" class="tsd-kind-icon">numericFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#pointcountthreshold" class="tsd-kind-icon">pointCountThreshold</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#pointgapthreshold" class="tsd-kind-icon">pointGapThreshold</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#precision" class="tsd-kind-icon">precision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#singlelabel" class="tsd-kind-icon">singleLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#skipmode" class="tsd-kind-icon">skipMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#skipnumber" class="tsd-kind-icon">skipNumber</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#style" class="tsd-kind-icon">style</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#updatetextinanimation" class="tsd-kind-icon">updateTextInAnimation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#verticaltextposition" class="tsd-kind-icon">verticalTextPosition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#yselector" class="tsd-kind-icon">ySelector</a>

## Properties

### Optional aboveBelow

aboveBelow: boolean

Sets whether text should be positioned above the line if it is going down, and below the line if it going up. Default true. When true this overrides horizontalAnchorPoint and verticalAnchorPoint to be left, bottom.

### Optional color

color: string

Sets the color for data labels. Defaults to axis label color

### Optional drawImmediate

drawImmediate: boolean

Draws immediately, set this True when using sub-surfaces for proper data label layering

### Optional engineeringPrefix

engineeringPrefix: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html" class="tsd-signature-type">IEngineeringPrefix</a>

Gets or sets the engineering prefixes to use when formatting values to text. Default - `['K','M','B,'T']` for "large" prefixes, `['m','u','n','p']` for small prefixes

remarks  
Only works when [ENumericFormat.Engineering](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#engineering) is selected

### Optional horizontalTextPosition

horizontalTextPosition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ehorizontaltextposition.html" class="tsd-signature-type">EHorizontalTextPosition</a>

Sets the horizontal text position for the label. Default Right

### Optional isEnabled

isEnabled: boolean

Flag to enable/disable dataLabel generation. Default true

### Optional metaDataSelector

metaDataSelector: (metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>) =\> string

If this is set it will be used to get text values from metaData, rather than formatting y values. The selector will be called even if the metaData for an index is undefined.

#### Type declaration

- - (metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>): string

  <!-- -->

  - #### Parameters

    - ##### metadata: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

    #### Returns string

### Optional numericFormat

numericFormat: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

Gets or sets numeric format to use for formatting values to text. For a list of values, see [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html)

### Optional pointCountThreshold

pointCountThreshold: number

Gets or sets the number of points below which drawing will start. Default Infinity This can be used instead of [pointGapThreshold](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#pointgapthreshold) when data is unevenly spaced or has large y variation

### Optional pointGapThreshold

pointGapThreshold: number

Gets or sets the zoom threshold above which label drawing will start. Default 0 This is expressed as the gap between the first points divided by the size of the first text. If data is unevenly spaced, consider [pointCountThreshold](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#pointcountthreshold) or override shouldDrawText

### Optional precision

precision: number

Gets or sets the precision to use when formatting values to text.

### Optional singleLabel

singleLabel: boolean

Sets whether to render a single label with both y and y1 values, in the center of the band, or separate labels for each line. Default false.

### Optional skipMode

skipMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/edatalabelskipmode.html" class="tsd-signature-type">EDataLabelSkipMode</a>

How do decide whether to keep or skip a label once generated. Override [shouldSkipLabel](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapdatalabelprovider.html#shouldskiplabel) for more control

### Optional skipNumber

skipNumber: number

The number of points to skip while generating labels. Default 0 = no skip. 1 = skip every other. When creating text with many data points, it will help performance to skip points rather than creating and checking overlap for every data point. This is applied before any logic related to [skipMode](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibandseriesdatalabelprovideroptions.html#skipmode)

### Optional style

style: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

Sets the text style used for data labels. The style must be set, with fontFamily and fontSize set, in order for text to be drawn.

### Optional updateTextInAnimation

updateTextInAnimation: boolean

Whether the label text should update when the label position is animating. Default false.

### Optional verticalTextPosition

verticalTextPosition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/everticaltextposition.html" class="tsd-signature-type">EVerticalTextPosition</a>

Sets the vertical text position for the label. Default Top

### Optional ySelector

ySelector: (ps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>) =\> SCRTDoubleVector

A function to pick which y values to use from the pointSeries passed in on the renderPassData

#### Type declaration

- - (ps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>): SCRTDoubleVector

  <!-- -->

  - #### Parameters

    - ##### ps: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointseries.html" class="tsd-signature-type">IPointSeries</a>

    #### Returns SCRTDoubleVector

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
