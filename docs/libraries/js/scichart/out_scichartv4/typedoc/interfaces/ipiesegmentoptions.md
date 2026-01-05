<img src="out_scichartv4/typedoc/interfaces/ipiesegmentoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPieSegmentOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html)

# Interface IPieSegmentOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- IPieSegmentOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#colorlineargradient" class="tsd-kind-icon">colorLinearGradient</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#delta" class="tsd-kind-icon">delta</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#isselected" class="tsd-kind-icon">isSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#labeloffset" class="tsd-kind-icon">labelOffset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#labelprovider" class="tsd-kind-icon">labelProvider</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#labelstyle" class="tsd-kind-icon">labelStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#radiusadjustment" class="tsd-kind-icon">radiusAdjustment</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#showlabel" class="tsd-kind-icon">showLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#text" class="tsd-kind-icon">text</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html#value" class="tsd-kind-icon">value</a>

## Properties

### Optional color

color: string

The color of the segment as an HTML color code

### Optional colorLinearGradient

colorLinearGradient: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" class="tsd-signature-type">GradientParams</a>

An optional color gradient

### Optional delta

delta: number

The amount to shift the segment when it is selected. Default 15 px

### Optional id

id: string

### Optional isSelected

isSelected: boolean

Whether the segment is selected. Selected segments are shifted outwards by the delta

### Optional labelOffset

labelOffset: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

An x, y offset for the label position

### Optional labelProvider

labelProvider: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pielabelprovider.html" class="tsd-signature-type">PieLabelProvider</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tlabelproviderdefinition" class="tsd-signature-type">TLabelProviderDefinition</a>

Optional class that can override the default label formatting for this segment only. Must be or inherit from [PieLabelProvider](https://www.scichart.com/documentation/js/v4/typedoc/classes/pielabelprovider.html)

### Optional labelStyle

labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

Optional text style that will override the default style from the surface for this segment only

### Optional radiusAdjustment

radiusAdjustment: number

A relative adjustment of the radius for this segment. eg 1.5 will be 50% larger than normal

### Optional showLabel

showLabel: boolean

When true, labels are shown, else hidden. Default value true

### Optional text

text: string

A text value for the segment which will be displayed in the legend

### Optional value

value: number

The numerical value of the segment

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
