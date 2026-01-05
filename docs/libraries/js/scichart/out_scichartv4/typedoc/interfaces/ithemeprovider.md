<img src="out_scichartv4/typedoc/interfaces/ithemeprovider_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html)

# Interface IThemeProvider

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

summary  
The ThemeProvider has properties to define a theme within SciChart's 2D & 3D Charts

description  
Applied to a 2D [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html), or a 3D [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html), the ThemeProvider may be applied using the [applyTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#applytheme) method, where it is passed down to child components so that all children of the chart get the same theme.

For example:

``` ts
// Applying a built-in dark theme
const sciChartSurface: SciChartSurface;
sciChartSurface.applyTheme(new SciChartJSDarkTheme());
// Or light theme
sciChartSurface.applyTheme(new SciChartJSLightTheme());

// Applying a custom theme
export class MyCustomTheme implements IThemeProvider {
   // todo: implement IThemeProvider interface and apply properties
}

sciChartSurface.applyTheme(new MyCustomTheme());
```

### Hierarchy

- IThemeProvider

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarktheme.html" class="tsd-signature-type">SciChartJSDarkTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html" class="tsd-signature-type">SciChartJSDarkv2Theme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html" class="tsd-signature-type">SciChartJSLightTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsnavytheme.html" class="tsd-signature-type">SciChartJsNavyTheme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/themeprovider.html" class="tsd-signature-type">ThemeProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#annotationselectionstroke" class="tsd-kind-icon">annotationSelectionStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#annotationsgripsbackgroundbrush" class="tsd-kind-icon">annotationsGripsBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#annotationsgripsborderbrush" class="tsd-kind-icon">annotationsGripsBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#axis3dbandsfill" class="tsd-kind-icon">axis3DBandsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#axisbandsfill" class="tsd-kind-icon">axisBandsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#axisborder" class="tsd-kind-icon">axisBorder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#axisplanebackgroundfill" class="tsd-kind-icon">axisPlaneBackgroundFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#axistitlecolor" class="tsd-kind-icon">axisTitleColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#charttitlecolor" class="tsd-kind-icon">chartTitleColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#columnfillbrush" class="tsd-kind-icon">columnFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#columnlinecolor" class="tsd-kind-icon">columnLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#cursorlinebrush" class="tsd-kind-icon">cursorLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#defaultcolormapbrush" class="tsd-kind-icon">defaultColorMapBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#downbandseriesfillcolor" class="tsd-kind-icon">downBandSeriesFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#downbandserieslinecolor" class="tsd-kind-icon">downBandSeriesLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#downbodybrush" class="tsd-kind-icon">downBodyBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#downwickcolor" class="tsd-kind-icon">downWickColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#fillpalette" class="tsd-kind-icon">fillPalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#gridbackgroundbrush" class="tsd-kind-icon">gridBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#gridborderbrush" class="tsd-kind-icon">gridBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#impulsefillbrush" class="tsd-kind-icon">impulseFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#islightbackground" class="tsd-kind-icon">isLightBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#labelbackgroundbrush" class="tsd-kind-icon">labelBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#labelborderbrush" class="tsd-kind-icon">labelBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#labelforegroundbrush" class="tsd-kind-icon">labelForegroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#legendbackgroundbrush" class="tsd-kind-icon">legendBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#lineseriescolor" class="tsd-kind-icon">lineSeriesColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#loadinganimationbackground" class="tsd-kind-icon">loadingAnimationBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#loadinganimationforeground" class="tsd-kind-icon">loadingAnimationForeground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#majorgridlinebrush" class="tsd-kind-icon">majorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#minorgridlinebrush" class="tsd-kind-icon">minorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#mountainareabrush" class="tsd-kind-icon">mountainAreaBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#mountainlinecolor" class="tsd-kind-icon">mountainLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#overviewfillbrush" class="tsd-kind-icon">overviewFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#planebordercolor" class="tsd-kind-icon">planeBorderColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#polarmajorgridlinebrush" class="tsd-kind-icon">polarMajorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#polarminorgridlinebrush" class="tsd-kind-icon">polarMinorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#rolloverlinebrush" class="tsd-kind-icon">rolloverLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#rubberbandfillbrush" class="tsd-kind-icon">rubberBandFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#rubberbandstrokebrush" class="tsd-kind-icon">rubberBandStrokeBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#scichartbackground" class="tsd-kind-icon">sciChartBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#scrollbarbackgroundbrush" class="tsd-kind-icon">scrollbarBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#scrollbarborderbrush" class="tsd-kind-icon">scrollbarBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#scrollbargripsbackgroundbrush" class="tsd-kind-icon">scrollbarGripsBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#scrollbarviewportbackgroundbrush" class="tsd-kind-icon">scrollbarViewportBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#scrollbarviewportborderbrush" class="tsd-kind-icon">scrollbarViewportBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#shadoweffectcolor" class="tsd-kind-icon">shadowEffectColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#strokepalette" class="tsd-kind-icon">strokePalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#textannotationbackground" class="tsd-kind-icon">textAnnotationBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#textannotationforeground" class="tsd-kind-icon">textAnnotationForeground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#ticktextbrush" class="tsd-kind-icon">tickTextBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#upbandseriesfillcolor" class="tsd-kind-icon">upBandSeriesFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#upbandserieslinecolor" class="tsd-kind-icon">upBandSeriesLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#upbodybrush" class="tsd-kind-icon">upBodyBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#upwickcolor" class="tsd-kind-icon">upWickColor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#getfillcolor" class="tsd-kind-icon">getFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html#getstrokecolor" class="tsd-kind-icon">getStrokeColor</a>

## Properties

### Optional annotationSelectionStroke

annotationSelectionStroke: string

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) selection box stroke color as an HTML color code

### annotationsGripsBackgroundBrush

annotationsGripsBackgroundBrush: string

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) grips background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### annotationsGripsBorderBrush

annotationsGripsBorderBrush: string

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) grips border color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axis3DBandsFill

axis3DBandsFill: string

The default 3D Chart Axis Bands fill color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisBandsFill

axisBandsFill: string

The Axis Bands fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisBorder

axisBorder: string

The default 2D Chart Axis Border color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisPlaneBackgroundFill

axisPlaneBackgroundFill: string

The default 3D Chart Axis Plane background color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisTitleColor

axisTitleColor: string

The default Axis Title color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### chartTitleColor

chartTitleColor: string

The default Chart Title color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### columnFillBrush

columnFillBrush: string

The default [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html) fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### columnLineColor

columnLineColor: string

The default [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html) stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### cursorLineBrush

cursorLineBrush: string

The Cursor Line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### defaultColorMapBrush

defaultColorMapBrush: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

The default [UniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html) color-map gradient stops

### downBandSeriesFillColor

downBandSeriesFillColor: string

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) down band fill color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downBandSeriesLineColor

downBandSeriesLineColor: string

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) down line color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downBodyBrush

downBodyBrush: string

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) down body color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downWickColor

downWickColor: string

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) down-wick color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Optional fillPalette

fillPalette: string\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

An array of color codes (or GradientStops if you want to control the gaps between colors) which will be used to provide fill colors.

### gridBackgroundBrush

gridBackgroundBrush: string

The Grid area background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### gridBorderBrush

gridBorderBrush: string

The Grid border brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Optional impulseFillBrush

impulseFillBrush: string

The default [FastImpulseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html) fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Optional isLightBackground

isLightBackground: boolean

Tells SciChart if the theme has a light or dark background (used for calculating contrasting elements)

### labelBackgroundBrush

labelBackgroundBrush: string

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### labelBorderBrush

labelBorderBrush: string

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label border brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### labelForegroundBrush

labelForegroundBrush: string

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label foreground font color brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### legendBackgroundBrush

legendBackgroundBrush: string

The [SciChartLegend](https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendtype.html#scichartlegend) background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### lineSeriesColor

lineSeriesColor: string

The default [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html) line stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### loadingAnimationBackground

loadingAnimationBackground: string

The background color of the loading animation dots, which can also be customized by overriding the [loader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesurfaceoptions.html#loader)

### loadingAnimationForeground

loadingAnimationForeground: string

The foreground color of the loading animation dots, which can also be customized by overriding the [loader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesurfaceoptions.html#loader)

### majorGridLineBrush

majorGridLineBrush: string

The Major Gridlines brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### minorGridLineBrush

minorGridLineBrush: string

The Minor Gridlines brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### mountainAreaBrush

mountainAreaBrush: string

The default [FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html) mountain fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### mountainLineColor

mountainLineColor: string

The default [FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html) mountain line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### overviewFillBrush

overviewFillBrush: string

### planeBorderColor

planeBorderColor: string

The default 3D Chart Axis Plane border color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Optional polarMajorGridLineBrush

polarMajorGridLineBrush: string

The Major Gridlines brush for Polar surfaces as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Optional polarMinorGridLineBrush

polarMinorGridLineBrush: string

The Minor Gridlines brush for Polar surfaces as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rolloverLineBrush

rolloverLineBrush: string

The [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html) line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rubberBandFillBrush

rubberBandFillBrush: string

The [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) recticule fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rubberBandStrokeBrush

rubberBandStrokeBrush: string

The [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) recticule stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### sciChartBackground

sciChartBackground: string

The Background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### scrollbarBackgroundBrush

scrollbarBackgroundBrush: string

### scrollbarBorderBrush

scrollbarBorderBrush: string

### scrollbarGripsBackgroundBrush

scrollbarGripsBackgroundBrush: string

### scrollbarViewportBackgroundBrush

scrollbarViewportBackgroundBrush: string

### scrollbarViewportBorderBrush

scrollbarViewportBorderBrush: string

### shadowEffectColor

shadowEffectColor: string

The default shadow effect color applied to drop-shadows

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Optional strokePalette

strokePalette: string\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\]

An array of color codes (or GradientStops if you want to control the gaps between colors) which will be used to provide stroke colors.

### textAnnotationBackground

textAnnotationBackground: string

The TextAnnotation background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### textAnnotationForeground

textAnnotationForeground: string

The TextAnnotation font color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### tickTextBrush

tickTextBrush: string

The Tick Text brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBandSeriesFillColor

upBandSeriesFillColor: string

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) up band fill color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBandSeriesLineColor

upBandSeriesLineColor: string

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) up line color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBodyBrush

upBodyBrush: string

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) up body color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upWickColor

upWickColor: string

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) up-wick color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

## Methods

### Optional getFillColor

- getFillColor(index: number, max: number, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): string

<!-- -->

- get a color from the fillPalette, either directly (if max is less than fillPalette length) or by interpolation

  #### Parameters

  - ##### index: number

  - ##### max: number

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns string

### Optional getStrokeColor

- getStrokeColor(index: number, max: number, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): string

<!-- -->

- get a color from the strokePalette, either directly (if max is less than strokePalette length) or by interpolation

  #### Parameters

  - ##### index: number

  - ##### max: number

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns string

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
