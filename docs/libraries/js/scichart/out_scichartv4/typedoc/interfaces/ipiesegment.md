<img src="out_scichartv4/typedoc/interfaces/ipiesegment_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPieSegment](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html)

# Interface IPieSegment

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- IPieSegment

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/piesegment.html" class="tsd-signature-type">PieSegment</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#color" class="tsd-kind-icon">color</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#colorlineargradient" class="tsd-kind-icon">colorLinearGradient</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#delta" class="tsd-kind-icon">delta</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#isselected" class="tsd-kind-icon">isSelected</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#labeloffset" class="tsd-kind-icon">labelOffset</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#labelstyle" class="tsd-kind-icon">labelStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#oldvalue" class="tsd-kind-icon">oldValue</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#radiusadjustment" class="tsd-kind-icon">radiusAdjustment</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#shift" class="tsd-kind-icon">shift</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#showlabel" class="tsd-kind-icon">showLabel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#text" class="tsd-kind-icon">text</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#value" class="tsd-kind-icon">value</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#getlabeltext" class="tsd-kind-icon">getLabelText</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#getpercentage" class="tsd-kind-icon">getPercentage</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#onattach" class="tsd-kind-icon">onAttach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#ondetach" class="tsd-kind-icon">onDetach</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegment.html#tojson" class="tsd-kind-icon">toJSON</a>

## Properties

### color

color: string

### colorLinearGradient

colorLinearGradient: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/gradientparams.html" class="tsd-signature-type">GradientParams</a>

### delta

delta: number

### Readonly id

id: string

### isSelected

isSelected: boolean

### labelOffset

labelOffset: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/point.html" class="tsd-signature-type">Point</a>

### labelStyle

labelStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#ttextstyle" class="tsd-signature-type">TTextStyle</a>

### oldValue

oldValue: number

### radiusAdjustment

radiusAdjustment: number

### shift

shift: number

### showLabel

showLabel: boolean

When true (default value = true) a label is shown, else false

### text

text: string

### value

value: number

## Methods

### getLabelText

- getLabelText(total: number): string

<!-- -->

- #### Parameters

  - ##### total: number

  #### Returns string

### getPercentage

- getPercentage(total: number): number

<!-- -->

- #### Parameters

  - ##### total: number

  #### Returns number

### onAttach

- onAttach(scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" class="tsd-signature-type">SciChartPieSurface</a>): void

<!-- -->

- #### Parameters

  - ##### scs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpiesurface.html" class="tsd-signature-type">SciChartPieSurface</a>

  #### Returns void

### onDetach

- onDetach(): void

<!-- -->

- #### Returns void

### toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html" class="tsd-signature-type">IPieSegmentOptions</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesegmentoptions.html" class="tsd-signature-type">IPieSegmentOptions</a>

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
