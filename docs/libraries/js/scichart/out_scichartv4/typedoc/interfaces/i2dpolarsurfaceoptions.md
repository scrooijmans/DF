<img src="out_scichartv4/typedoc/interfaces/i2dpolarsurfaceoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [I2DPolarSurfaceOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html)

# Interface I2DPolarSurfaceOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Options passed to a [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html) in the [SciChartPolarSurface.create](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html#create) function

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dsurfaceoptions.html" class="tsd-signature-type">I2DSurfaceOptions</a>
  - I2DPolarSurfaceOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#autocolormode" class="tsd-kind-icon">autoColorMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#background" class="tsd-kind-icon">background</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#canvasborder" class="tsd-kind-icon">canvasBorder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#createsuspended" class="tsd-kind-icon">createSuspended</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#disableaspect" class="tsd-kind-icon">disableAspect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#drawseriesbehindaxis" class="tsd-kind-icon">drawSeriesBehindAxis</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#freezewhenoutofview" class="tsd-kind-icon">freezeWhenOutOfView</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#heightaspect" class="tsd-kind-icon">heightAspect</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#layoutmanager" class="tsd-kind-icon">layoutManager</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#loader" class="tsd-kind-icon">loader</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#padding" class="tsd-kind-icon">padding</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#theme" class="tsd-kind-icon">theme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#title" class="tsd-kind-icon">title</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#titlestyle" class="tsd-kind-icon">titleStyle</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#touchaction" class="tsd-kind-icon">touchAction</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#viewportborder" class="tsd-kind-icon">viewportBorder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dpolarsurfaceoptions.html#widthaspect" class="tsd-kind-icon">widthAspect</a>

## Properties

### Optional autoColorMode

autoColorMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eautocolormode.html" class="tsd-signature-type">EAutoColorMode</a>

Optional - An [EAutoColorMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/eautocolormode.html) which controls how often series colours set to AUTO_COLOR will be resolved. Default OnAddRemoveSeries

### Optional background

background: string

Surface Background as an HTML color code;

remarks  
A sub-chart only supports plain color without gradients

### Optional canvasBorder

canvasBorder: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tborder" class="tsd-signature-type">TBorder</a>

Optional - Properties of the canvas border

### Optional createSuspended

createSuspended: boolean

experimental  
Optional - enabling prevents chart rendering until {@link SciChartSurface.resume} is called on the surface instance

### Optional disableAspect

disableAspect: boolean

Optional - the option of disabling / enabling scaling of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html). If false - the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html) will take the height and width of parent div without scaling.

### Optional drawSeriesBehindAxis

drawSeriesBehindAxis: boolean

drawSeriesBehindAxis property is not supported for [SciChartPolarSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartpolarsurface.html)

### Optional freezeWhenOutOfView

freezeWhenOutOfView: boolean

Optional - when true, charts that are out of the viewport will be frozen (pausing rendering). Data updates can resume Once the chart is in view again, rendering will resume. This can be useful for performance optimization.

### Optional heightAspect

heightAspect: number

Optional - the height aspect ratio of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html). By default SciChart will scale to fit the parent Div. However if height of the div is not provided it will use width/height aspect ratio to calculate the height. The default ratio is 3/2.

### Optional id

id: string

Allows you to set custom Id for the surface;

remarks  
If skipped the Id will be auto-generated

### Optional layoutManager

layoutManager: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tlayoutmanagerdefinition" class="tsd-signature-type">TLayoutManagerDefinition</a>

Optional - Prove a layoutManager to customise the axis layout. Use CentralAxesLayoutManager for an easy way to configure central axes.

### Optional loader

loader: false \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html" class="tsd-signature-type">ISciChartLoader</a> \| { options?: any; type: string }

Allows you to customize the loading elements or animation as part of the HTML page / DOM when a [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) or [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) is loading WebAssembly.

Set to false for disabling.

### Optional padding

padding: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/thickness.html" class="tsd-signature-type">Thickness</a>

Optional - Padding between the SciChartSurface and its inner elements. Default 10

### Optional theme

theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a> \| ({ type: string \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ethemeprovidertype.html" class="tsd-signature-type">EThemeProviderType</a> } & <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemepartial.html" class="tsd-signature-type">IThemePartial</a>)

Optional - The theme applied to the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html) on startup

remarks  
see [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html) for properties which can affect SciChart theme. Two default themes are included out of the box [SciChartJSLightTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html) and [SciChartJSDarkTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarktheme.html). Custom themes may be created by implementing [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html)

### Optional title

title: string \| string\[\]

Optional a title for the SciChartSurface

### Optional titleStyle

titleStyle: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcharttitlestyle" class="tsd-signature-type">TChartTitleStyle</a>

The title text style and placement for the SciChartSurface as [TChartTitleStyle](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcharttitlestyle)

### Optional touchAction

touchAction: string

Sets a `touch-action` property to the canvas style. Useful if touch interactions in browser should be configured. <https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action>

remarks  
By default a chart will use `touch-action: none` to prevent the default browser behavior.

### Optional viewportBorder

viewportBorder: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tborder" class="tsd-signature-type">TBorder</a>

Optional - Properties of the viewport border (where series are drawn)

### Optional widthAspect

widthAspect: number

Optional - the width aspect ratio of the [SciChartSurfaceBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurfacebase.html). By default SciChart will scale to fit the parent Div. However if height of the div is not provided it will use width/height aspect ratio to calculate the height. The default ratio is 3/2.

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
