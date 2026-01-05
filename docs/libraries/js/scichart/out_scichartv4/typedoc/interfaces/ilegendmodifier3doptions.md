<img src="out_scichartv4/typedoc/interfaces/ilegendmodifier3doptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ILegendModifier3DOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html)

# Interface ILegendModifier3DOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Optional parameters used to configure a [LegendModifier3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/legendmodifier3d.html) at construct time

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ichartmodifierbase3doptions.html" class="tsd-signature-type">IChartModifierBase3DOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendoptionsbase.html" class="tsd-signature-type">ILegendOptionsBase</a>
  - ILegendModifier3DOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#backgroundcolor" class="tsd-kind-icon">backgroundColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#excludedseriesids" class="tsd-kind-icon">excludedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#executecondition" class="tsd-kind-icon">executeCondition</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#includedseriesids" class="tsd-kind-icon">includedSeriesIds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#ischeckedchangedcallback" class="tsd-kind-icon">isCheckedChangedCallback</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#legend" class="tsd-kind-icon">legend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#margin" class="tsd-kind-icon">margin</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#orientation" class="tsd-kind-icon">orientation</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#placement" class="tsd-kind-icon">placement</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#placementdivid" class="tsd-kind-icon">placementDivId</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#showcheckboxes" class="tsd-kind-icon">showCheckboxes</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#showlegend" class="tsd-kind-icon">showLegend</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#showseriesmarkers" class="tsd-kind-icon">showSeriesMarkers</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ilegendmodifier3doptions.html#textcolor" class="tsd-kind-icon">textColor</a>

## Properties

### Optional backgroundColor

backgroundColor: string

Sets the legend background color. Defaults to theme.legendBackgroundBrush

### Optional excludedSeriesIds

excludedSeriesIds: string\[\]

A list of 3D renderable series to exclude from this modifier

remarks  
Also see [ChartModifierBase3D.includedSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html#includedseries) which has methods to include or exclude a series by instance

### Optional executeCondition

executeCondition: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tmodifierexecutecondition" class="tsd-signature-type">TModifierExecuteCondition</a>

The primary action execute condition that modifier should respond to

### Optional id

id: string

A unique Id for the [ChartModifierBase3D](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html)

### Optional includedSeriesIds

includedSeriesIds: string\[\]

A list of 3D renderable series to include to this modifier

remarks  
Also see [ChartModifierBase3D.includedSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase3d.html#includedseries) which has methods to include or exclude a series by instance

### Optional isCheckedChangedCallback

isCheckedChangedCallback: (series: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>, isChecked: boolean) =\> void

Callback when a legend item checkbox is checked or unchecked (by default, this corresponds to [IRenderableSeries3D.isVisible](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html#isvisible)

param  

param  

#### Type declaration

- - (series: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>, isChecked: boolean): void

  <!-- -->

  - #### Parameters

    - ##### series: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/irenderableseries3d.html" class="tsd-signature-type">IRenderableSeries3D</a>

    - ##### isChecked: boolean

    #### Returns void

### Optional legend

legend: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dlegend.html" class="tsd-signature-type">SciChart3DLegend</a>

Set this only if you need to pass in a custom legend instance. showCheckboxes, showSeriesMarkers and isCheckedChangedCallback will be set on the instance you pass if specified in the options.

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
