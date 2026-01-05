<img src="out_scichartv4/typedoc/interfaces/imanuallegendoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IManualLegendOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html)

# Interface IManualLegendOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendoptionsbase.html" class="tsd-signature-type">ILegendOptionsBase</a>
  - IManualLegendOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#backgroundcolor" class="tsd-kind-icon">backgroundColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#ischeckedchangedcallback" class="tsd-kind-icon">isCheckedChangedCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#items" class="tsd-kind-icon">items</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#margin" class="tsd-kind-icon">margin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#orientation" class="tsd-kind-icon">orientation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#placement" class="tsd-kind-icon">placement</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#placementdivid" class="tsd-kind-icon">placementDivId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#showcheckboxes" class="tsd-kind-icon">showCheckboxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#showlegend" class="tsd-kind-icon">showLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#showseriesmarkers" class="tsd-kind-icon">showSeriesMarkers</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imanuallegendoptions.html#textcolor" class="tsd-kind-icon">textColor</a>

## Properties

### Optional backgroundColor

backgroundColor: string

Sets the legend background color. Defaults to theme.legendBackgroundBrush

### Optional isCheckedChangedCallback

isCheckedChangedCallback: (item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tlegenditem" class="tsd-signature-type">TLegendItem</a>, isChecked: boolean, legend?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/manuallegend.html" class="tsd-signature-type">ManualLegend</a>) =\> void

Callback when a legend item checkbox is checked or unchecked

param  

param  

#### Type declaration

- - (item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tlegenditem" class="tsd-signature-type">TLegendItem</a>, isChecked: boolean, legend?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/manuallegend.html" class="tsd-signature-type">ManualLegend</a>): void

  <!-- -->

  - #### Parameters

    - ##### item: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tlegenditem" class="tsd-signature-type">TLegendItem</a>

    - ##### isChecked: boolean

    - ##### Optional legend: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/manuallegend.html" class="tsd-signature-type">ManualLegend</a>

    #### Returns void

### Optional items

items: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tlegenditem" class="tsd-signature-type">TLegendItem</a>\[\]

### Optional margin

margin: number

Sets the margin for the legend control

### Optional orientation

orientation: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendorientation.html" class="tsd-signature-type">ELegendOrientation</a>

Sets the initial orientation of the legend. See [ELegendOrientation](https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendorientation.html) for a list of values

### Optional placement

placement: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendplacement.html" class="tsd-signature-type">ELegendPlacement</a>

Sets the initial legend placement in the parent chart surface. See [ELegendPlacement](https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendplacement.html) for a list of values

### Optional placementDivId

placementDivId: string \| HTMLDivElement

The parent div element Id or reference, the Legend will be appended to this element

### Optional showCheckboxes

showCheckboxes: boolean

Sets whether the legend has visibility checkboxes in it or not

### Optional showLegend

showLegend: boolean

Sets whether the legend is initially visible or not

### Optional showSeriesMarkers

showSeriesMarkers: boolean

Sets whether Series markers are visible or not

### Optional textColor

textColor: string

Sets the legend text color. Defaults to theme.labelForegroundBrush

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
