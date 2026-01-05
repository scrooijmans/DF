<img src="out_scichartv4/typedoc/interfaces/itextlabeloptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ITextLabelOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html)

# Interface ITextLabelOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilabel2doptions.html" class="tsd-signature-type">ILabel2DOptions</a>
  - ITextLabelOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#asynclabels" class="tsd-kind-icon">asyncLabels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#cursorlabelformat" class="tsd-kind-icon">cursorLabelFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#cursorlabelprecision" class="tsd-kind-icon">cursorLabelPrecision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#labelformat" class="tsd-kind-icon">labelFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#labelpostfix" class="tsd-kind-icon">labelPostfix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#labelprecision" class="tsd-kind-icon">labelPrecision</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#labelprefix" class="tsd-kind-icon">labelPrefix</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#labels" class="tsd-kind-icon">labels</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#linespacing" class="tsd-kind-icon">lineSpacing</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#maxlength" class="tsd-kind-icon">maxLength</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#rotation" class="tsd-kind-icon">rotation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#usenativetext" class="tsd-kind-icon">useNativeText</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/itextlabeloptions.html#usesharedcache" class="tsd-kind-icon">useSharedCache</a>

## Properties

### Optional asyncLabels

asyncLabels: boolean

deprecated  
This functionality has been removed. useNativeText: true provides much greater performance benefit.

### Optional cursorLabelFormat

cursorLabelFormat: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

The formatting mode to apply to tooltips [CursorModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html), [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html)

### Optional cursorLabelPrecision

cursorLabelPrecision: number

Specifies the number of decimal places for the tooltip value [CursorModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/cursormodifier.html), [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html)

### Optional labelFormat

labelFormat: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html" class="tsd-signature-type">ENumericFormat</a>

The formatting mode to apply to the labels

### Optional labelPostfix

labelPostfix: string

A postfix for the label values

### Optional labelPrecision

labelPrecision: number

Specifies the number of decimal places for the label value

### Optional labelPrefix

labelPrefix: string

A prefix for the label values

### Optional labels

labels: Record\<number, string \| string\[\]\>

The label text to use. If you pass an object like {1:"one", 2:"two", 3:"three"} then labels will be chosen based on the tick value. eg ticks 3,1 will result in "three", "one". If you pass an array like \["one", "two", "three"\] then for a category axis it will use the labels in the order given, regardless of data value. If you know that your x data will be a fixed set in a fixed order, then passing an array of labels is simpler. To manually specify multiple lines, pass an array for the label eg \["One line",\["Two","Lines"\],\["Three","more","lines"\]\]

### Optional lineSpacing

lineSpacing: number

Line spacing to use if text is wrapped. For normal labels this is a multiple of the text height and defaults to 1.1 For native text it is a number of pixels and defaults to 2

### Optional maxLength

maxLength: number

Wrap text longer than this number of characters. Will only wrap whole words.

### Optional rotation

rotation: number

### Optional useNativeText

useNativeText: boolean

Whether to use WebGL for rendering axis labels. Default true (was false before v4). These are much faster than rendering using canvas text, but do not have quite the same font and style support.

note  
Set [SciChartDefaults.useNativeText](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#usenativetext) to change the global default.

note  
If you want native labels for best performance, but NON-native axis titles for a more flexible Title, set:

``` ts
const xAxis = new NumericAxis(wasmContext);

xAxis.useNativeText = true; // Native axis labels for performance - already set to `true` by default

xAxis.axisTitle = "X Axis Title";
xAxis.axisTitleStyle.useNativeText: false // Non-native axis title for more font and style options
```

### Optional useSharedCache

useSharedCache: boolean

Whether to use cached labels from other axes that have the same style. You may need to set this false if you are overriding getLabelTexture or getLabelTextureAsync without setting a unique style.

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
