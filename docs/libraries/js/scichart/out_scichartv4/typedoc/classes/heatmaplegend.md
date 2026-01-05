<img src="out_scichartv4/typedoc/classes/heatmaplegend_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [HeatmapLegend](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html)

# Class HeatmapLegend

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The HeatmapLegend displays a control which hosts a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) in a specific Div on the chart. The legend contains a gradient fill and can be used in conjunction with [UniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html), [NonUniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/nonuniformheatmaprenderableseries.html) or [SurfaceMeshRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#surfacemeshrenderableseries3d) to allow the user to see what colors map to what values on the heatmap or surface.

remarks  
This control will expand to fit its parent Div. Suggest placing the div to the right and floating 100px wide to create a good effect.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/deletableentity.html" class="tsd-signature-type">DeletableEntity</a>
  - HeatmapLegend

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" class="tsd-signature-type">IDeletable</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#constructor" class="tsd-kind-icon">constructor</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#innerscichartsurface" class="tsd-kind-icon">innerSciChartSurface</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#delete" class="tsd-kind-icon">delete</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#getdefaultgradientstops" class="tsd-kind-icon">getDefaultGradientStops</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#getdefaultxaxisoptions" class="tsd-kind-icon">getDefaultXAxisOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#getdefaultyaxisoptions" class="tsd-kind-icon">getDefaultYAxisOptions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#getzvalues" class="tsd-kind-icon">getZValues</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#create" class="tsd-kind-icon">create</a>

## Constructors

### constructor

- new HeatmapLegend(sciChartSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html" class="tsd-signature-type">IHeatmapLegendOptions</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" class="tsd-signature-type">HeatmapLegend</a>

<!-- -->

- Creates a new HeatmapLegend wrapping a SciChartSurface. Use the {@link HeatmapLegend.create()} function to create this asynchronously

  #### Parameters

  - ##### sciChartSurface: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html" class="tsd-signature-type">SciChartSurface</a>

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html" class="tsd-signature-type">IHeatmapLegendOptions</a>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" class="tsd-signature-type">HeatmapLegend</a>

## Accessors

### innerSciChartSurface

- get innerSciChartSurface(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#twebassemblychart" class="tsd-signature-type">TWebAssemblyChart</a>

<!-- -->

- Returns the inner [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) which renders the heatmap legend

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#twebassemblychart" class="tsd-signature-type">TWebAssemblyChart</a>

## Methods

### delete

- delete(): void

<!-- -->

- Deletes the [HeatmapLegend](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html) control, its [innerSciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#innerscichartsurface) and all associated webassembly memory

  #### Returns void

### Protected getDefaultGradientStops

- getDefaultGradientStops(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

<!-- -->

- Gets the default [GradientStops](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop) to apply to the inner [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

### Protected getDefaultXAxisOptions

- getDefaultXAxisOptions(): { drawLabels: boolean; drawMajorGridLines: boolean; drawMajorTickLines: boolean; drawMinorGridLines: boolean; drawMinorTickLines: boolean }

<!-- -->

- Gets the default options to apply to the inner [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) xAxis. Must conform to [IAxisBase2dOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisbase2doptions.html) interface

  #### Returns { drawLabels: boolean; drawMajorGridLines: boolean; drawMajorTickLines: boolean; drawMinorGridLines: boolean; drawMinorTickLines: boolean }

  - ##### drawLabels: boolean

  - ##### drawMajorGridLines: boolean

  - ##### drawMajorTickLines: boolean

  - ##### drawMinorGridLines: boolean

  - ##### drawMinorTickLines: boolean

### Protected getDefaultYAxisOptions

- getDefaultYAxisOptions(): { drawLabels: boolean; drawMajorGridLines: boolean; drawMajorTickLines: boolean; drawMinorGridLines: boolean; drawMinorTickLines: boolean; maxAutoTicks: number }

<!-- -->

- Gets the default options to apply to the inner [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) yAxis. Must conform to [IAxisBase2dOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisbase2doptions.html) interface

  #### Returns { drawLabels: boolean; drawMajorGridLines: boolean; drawMajorTickLines: boolean; drawMinorGridLines: boolean; drawMinorTickLines: boolean; maxAutoTicks: number }

  - ##### drawLabels: boolean

  - ##### drawMajorGridLines: boolean

  - ##### drawMajorTickLines: boolean

  - ##### drawMinorGridLines: boolean

  - ##### drawMinorTickLines: boolean

  - ##### maxAutoTicks: number

### getZValues

- getZValues(minimum: number, maximum: number): number\[\]\[\]

<!-- -->

- #### Parameters

  - ##### minimum: number

  - ##### maximum: number

  #### Returns number\[\]\[\]

### Static create

- create(divElement: string \| HTMLDivElement, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html" class="tsd-signature-type">IHeatmapLegendOptions</a>): Promise\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#theatmaplegend" class="tsd-signature-type">THeatmapLegend</a>\>

<!-- -->

- Asynchronously creates a [HeatmapLegend](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html) and @link TSciChart \| WebAssembly Context} to occupy the div by element ID in your DOM.

  remarks  
  This method is async and must be awaited

  #### Parameters

  - ##### divElement: string \| HTMLDivElement

    The Div Element ID or reference where the [HeatmapLegend](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html) will reside

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html" class="tsd-signature-type">IHeatmapLegendOptions</a>

    Optional - Optional parameters for chart creation. See [IHeatmapLegendOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html) for more details

  #### Returns Promise\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#theatmaplegend" class="tsd-signature-type">THeatmapLegend</a>\>

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
