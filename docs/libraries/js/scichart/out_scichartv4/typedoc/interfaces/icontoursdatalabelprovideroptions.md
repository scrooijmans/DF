<img src="out_scichartv4/typedoc/interfaces/icontoursdatalabelprovideroptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IContoursDataLabelProviderOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html)

# Interface IContoursDataLabelProviderOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ibasedatalabelprovideroptions.html" class="tsd-signature-type">IBaseDataLabelProviderOptions</a>
  - IContoursDataLabelProviderOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#drawimmediate" class="tsd-kind-icon">drawImmediate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#isenabled" class="tsd-kind-icon">isEnabled</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#labelrowcount" class="tsd-kind-icon">labelRowCount</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#metadataselector" class="tsd-kind-icon">metaDataSelector</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#numericformat" class="tsd-kind-icon">numericFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#precision" class="tsd-kind-icon">precision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icontoursdatalabelprovideroptions.html#style" class="tsd-kind-icon">style</a>

## Properties

### Optional color

color: string

Sets the color for data labels. Defaults to axis label color

### Optional drawImmediate

drawImmediate: boolean

Draws immediately, set this True when using sub-surfaces for proper data label layering

### Optional isEnabled

isEnabled: boolean

Flag to enable/disable dataLabel generation. Default true

### Optional labelRowCount

labelRowCount: number

sets the number of rows of data labels. Default 10

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

### Optional precision

precision: number

Gets or sets the precision to use when formatting values to text.

### Optional style

style: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tdatalabelstyle" class="tsd-signature-type">TDataLabelStyle</a>

Sets the text style used for data labels. The style must be set, with fontFamily and fontSize set, in order for text to be drawn.

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
