<img src="out_scichartv4/typedoc/classes/scichartjsdarkv2theme_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SciChartJSDarkv2Theme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html)

# Class SciChartJSDarkv2Theme

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

An implementation of [ThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#themeprovider) which provides an improved dark theme

decription  
Applied to a 2D [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html), or a 3D [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html), the ThemeProvider may be applied using the [applyTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#applytheme) method, where it is passed down to child components so that all children of the chart get the same theme.

For example:

``` ts
// Applying a theme when creating a chart
const { sciChartSurface, wasmContext } = SciChartSurface.create("div-id", { theme: new SciChartJSDarkv2Theme() });
// Apply a theme after chart creation
sciChartSurface.applyTheme(new SciChartJSDarkv2Theme());

// Applying a custom theme
export class MyCustomTheme implements IThemeProvider {
   // todo: implement IThemeProvider interface and apply properties
}

sciChartSurface.applyTheme(new MyCustomTheme()); // Or apply in SciChartSurface.create()

// Overriding just some members of a theme
const myOverriddenTheme = {...new SciChartJSLightTheme(), sciChartBackground: "white" };
ciChartSurface.applyTheme(myOverriddenTheme); // Or apply in SciChartSurface.create()
```

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarktheme.html" class="tsd-signature-type">SciChartJSDarkTheme</a>
  - SciChartJSDarkv2Theme
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsnavytheme.html" class="tsd-signature-type">SciChartJsNavyTheme</a>

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#constructor" class="tsd-kind-icon">constructor</a>

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#annotationselectionstroke" class="tsd-kind-icon">annotationSelectionStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#annotationsgripsbackgroundbrush" class="tsd-kind-icon">annotationsGripsBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#annotationsgripsborderbrush" class="tsd-kind-icon">annotationsGripsBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#axis3dbandsfill" class="tsd-kind-icon">axis3DBandsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#axisbandsfill" class="tsd-kind-icon">axisBandsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#axisborder" class="tsd-kind-icon">axisBorder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#axisplanebackgroundfill" class="tsd-kind-icon">axisPlaneBackgroundFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#axistitlecolor" class="tsd-kind-icon">axisTitleColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#charttitlecolor" class="tsd-kind-icon">chartTitleColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#columnfillbrush" class="tsd-kind-icon">columnFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#columnlinecolor" class="tsd-kind-icon">columnLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#cursorlinebrush" class="tsd-kind-icon">cursorLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#defaultcolormapbrush" class="tsd-kind-icon">defaultColorMapBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#downbandseriesfillcolor" class="tsd-kind-icon">downBandSeriesFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#downbandserieslinecolor" class="tsd-kind-icon">downBandSeriesLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#downbodybrush" class="tsd-kind-icon">downBodyBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#downwickcolor" class="tsd-kind-icon">downWickColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#fillpalette" class="tsd-kind-icon">fillPalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#gridbackgroundbrush" class="tsd-kind-icon">gridBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#gridborderbrush" class="tsd-kind-icon">gridBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#impulsefillbrush" class="tsd-kind-icon">impulseFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#islightbackground" class="tsd-kind-icon">isLightBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#labelbackgroundbrush" class="tsd-kind-icon">labelBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#labelborderbrush" class="tsd-kind-icon">labelBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#labelforegroundbrush" class="tsd-kind-icon">labelForegroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#legendbackgroundbrush" class="tsd-kind-icon">legendBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#lineseriescolor" class="tsd-kind-icon">lineSeriesColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#loadinganimationbackground" class="tsd-kind-icon">loadingAnimationBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#loadinganimationforeground" class="tsd-kind-icon">loadingAnimationForeground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#majorgridlinebrush" class="tsd-kind-icon">majorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#minorgridlinebrush" class="tsd-kind-icon">minorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#mountainareabrush" class="tsd-kind-icon">mountainAreaBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#mountainlinecolor" class="tsd-kind-icon">mountainLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#overrides" class="tsd-kind-icon">overrides</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#overviewfillbrush" class="tsd-kind-icon">overviewFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#planebordercolor" class="tsd-kind-icon">planeBorderColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#polarmajorgridlinebrush" class="tsd-kind-icon">polarMajorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#polarminorgridlinebrush" class="tsd-kind-icon">polarMinorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#rolloverlinebrush" class="tsd-kind-icon">rolloverLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#rubberbandfillbrush" class="tsd-kind-icon">rubberBandFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#rubberbandstrokebrush" class="tsd-kind-icon">rubberBandStrokeBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#scichartbackground" class="tsd-kind-icon">sciChartBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#scrollbarbackgroundbrush" class="tsd-kind-icon">scrollbarBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#scrollbarborderbrush" class="tsd-kind-icon">scrollbarBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#scrollbargripsbackgroundbrush" class="tsd-kind-icon">scrollbarGripsBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#scrollbarviewportbackgroundbrush" class="tsd-kind-icon">scrollbarViewportBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#scrollbarviewportborderbrush" class="tsd-kind-icon">scrollbarViewportBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#shadoweffectcolor" class="tsd-kind-icon">shadowEffectColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#strokepalette" class="tsd-kind-icon">strokePalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#textannotationbackground" class="tsd-kind-icon">textAnnotationBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#textannotationforeground" class="tsd-kind-icon">textAnnotationForeground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#ticktextbrush" class="tsd-kind-icon">tickTextBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#upbandseriesfillcolor" class="tsd-kind-icon">upBandSeriesFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#upbandserieslinecolor" class="tsd-kind-icon">upBandSeriesLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#upbodybrush" class="tsd-kind-icon">upBodyBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#upwickcolor" class="tsd-kind-icon">upWickColor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#applyoverrides" class="tsd-kind-icon">applyOverrides</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#getfillcolor" class="tsd-kind-icon">getFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#getstrokecolor" class="tsd-kind-icon">getStrokeColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html#tojson" class="tsd-kind-icon">toJSON</a>

## Constructors

### constructor

- new SciChartJSDarkv2Theme(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html" class="tsd-signature-type">SciChartJSDarkv2Theme</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarkv2theme.html" class="tsd-signature-type">SciChartJSDarkv2Theme</a>

## Properties

### annotationSelectionStroke

annotationSelectionStroke: string = "#f00e0e66"

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) selection box stroke color as an HTML color code

### annotationsGripsBackgroundBrush

annotationsGripsBackgroundBrush: string = "#CDCDCD22"

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) grips background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### annotationsGripsBorderBrush

annotationsGripsBorderBrush: string = "#CDCDCD99"

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) grips border color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axis3DBandsFill

axis3DBandsFill: string = "#202123E1"

The default 3D Chart Axis Bands fill color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisBandsFill

axisBandsFill: string = "#202123E1"

The Axis Bands fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisBorder

axisBorder: string = "#00000000"

The default 2D Chart Axis Border color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisPlaneBackgroundFill

axisPlaneBackgroundFill: string = "TRANSPARENT"

The default 3D Chart Axis Plane background color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisTitleColor

axisTitleColor: string = "#C8C7C3FF"

The default Axis Title color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### chartTitleColor

chartTitleColor: string = "#C8C7C3FF"

The default Chart Title color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### columnFillBrush

columnFillBrush: string = "#FFFFFFFF"

The default [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html) fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### columnLineColor

columnLineColor: string = "#FFFFFFFF"

The default [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html) stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### cursorLineBrush

cursorLineBrush: string = "#228B22FF"

The Cursor Line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### defaultColorMapBrush

defaultColorMapBrush: { color: string; offset: number }\[\] = \[{ offset: 0, color: "DARKBLUE" },{ offset: 0.5, color: "CORNFLOWERBLUE" },{ offset: 1, color: "#FF22AAFF" }\]

The default [UniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html) color-map gradient stops

### downBandSeriesFillColor

downBandSeriesFillColor: string = "#FF191933"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) down band fill color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downBandSeriesLineColor

downBandSeriesLineColor: string = "#FF1919FF"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) down line color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downBodyBrush

downBodyBrush: string = "#ff5050B2"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) down body color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downWickColor

downWickColor: string = "#ff5050FF"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) down-wick color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### fillPalette

fillPalette: string\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\] = \[\]

### gridBackgroundBrush

gridBackgroundBrush: string = "TRANSPARENT"

The Grid area background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### gridBorderBrush

gridBorderBrush: string = "#5A5B5BFF"

The Grid border brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### impulseFillBrush

impulseFillBrush: string = "#FFFFFFFF"

The default [FastImpulseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html) fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### isLightBackground

isLightBackground: boolean = false

Tells SciChart if the theme has a light or dark background (used for calculating contrasting elements)

### labelBackgroundBrush

labelBackgroundBrush: string = "#42b649AA"

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### labelBorderBrush

labelBorderBrush: string = "#42b649FF"

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label border brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### labelForegroundBrush

labelForegroundBrush: string = "#EEEEEEFF"

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label foreground font color brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### legendBackgroundBrush

legendBackgroundBrush: string = "#1D2C35FF"

The [SciChartLegend](https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendtype.html#scichartlegend) background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### lineSeriesColor

lineSeriesColor: string = "#C6E6FFFF"

The default [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html) line stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### loadingAnimationBackground

loadingAnimationBackground: string = "#1C1C1EFF"

The background color of the loading animation dots, which can also be customized by overriding the [loader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesurfaceoptions.html#loader)

### loadingAnimationForeground

loadingAnimationForeground: string = "#AAA"

The foreground color of the loading animation dots, which can also be customized by overriding the [loader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesurfaceoptions.html#loader)

### majorGridLineBrush

majorGridLineBrush: string = "#323539FF"

The Major Gridlines brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### minorGridLineBrush

minorGridLineBrush: string = "#232426FF"

The Minor Gridlines brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### mountainAreaBrush

mountainAreaBrush: string = "#4083B777"

The default [FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html) mountain fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### mountainLineColor

mountainLineColor: string = "#C6E6FFFF"

The default [FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html) mountain line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Protected overrides

overrides: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemepartial.html" class="tsd-signature-type">IThemePartial</a>

### overviewFillBrush

overviewFillBrush: string = "#262728BB"

inheritdoc  

### planeBorderColor

planeBorderColor: string = "#333333FF"

The default 3D Chart Axis Plane border color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### polarMajorGridLineBrush

polarMajorGridLineBrush: string = "#323539FF"

The Major Gridlines brush for Polar surfaces as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### polarMinorGridLineBrush

polarMinorGridLineBrush: string = "#232426FF"

The Minor Gridlines brush for Polar surfaces as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rolloverLineBrush

rolloverLineBrush: string = "#42b64933"

The [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html) line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rubberBandFillBrush

rubberBandFillBrush: string = "#42b64933"

The [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) recticule fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rubberBandStrokeBrush

rubberBandStrokeBrush: string = "#42b64977"

The [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) recticule stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### sciChartBackground

sciChartBackground: string = "#1C1C1EFF"

The Background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### scrollbarBackgroundBrush

scrollbarBackgroundBrush: string = "#262728FF"

inheritdoc  

### scrollbarBorderBrush

scrollbarBorderBrush: string = "#121212FF"

inheritdoc  

### scrollbarGripsBackgroundBrush

scrollbarGripsBackgroundBrush: string = "#535353FF"

inheritdoc  

### scrollbarViewportBackgroundBrush

scrollbarViewportBackgroundBrush: string = "#222222FF"

inheritdoc  

### scrollbarViewportBorderBrush

scrollbarViewportBorderBrush: string = "#232323FF"

inheritdoc  

### shadowEffectColor

shadowEffectColor: string = "#000000FF"

The default shadow effect color applied to drop-shadows

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### strokePalette

strokePalette: string\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tgradientstop" class="tsd-signature-type">TGradientStop</a>\[\] = \[\]

### textAnnotationBackground

textAnnotationBackground: string = "#42b649AA"

The TextAnnotation background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### textAnnotationForeground

textAnnotationForeground: string = "#EEEEEEFF"

The TextAnnotation font color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### tickTextBrush

tickTextBrush: string = "#A6A7ACFF"

The Tick Text brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ethemeprovidertype.html" class="tsd-signature-type">EThemeProviderType</a> = EThemeProviderType.DarkV2

inheritdoc  

### upBandSeriesFillColor

upBandSeriesFillColor: string = "#279B2733"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) up band fill color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBandSeriesLineColor

upBandSeriesLineColor: string = "#279B27FF"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) up line color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBodyBrush

upBodyBrush: string = "#50ff50B2"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) up body color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upWickColor

upWickColor: string = "#50ff50FF"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) up-wick color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

## Methods

### applyOverrides

- applyOverrides(overrides: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemepartial.html" class="tsd-signature-type">IThemePartial</a>): void

<!-- -->

- #### Parameters

  - ##### overrides: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemepartial.html" class="tsd-signature-type">IThemePartial</a>

  #### Returns void

### getFillColor

- getFillColor(index: number, max: number, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): string

<!-- -->

- #### Parameters

  - ##### index: number

  - ##### max: number

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns string

### getStrokeColor

- getStrokeColor(index: number, max: number, wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>): string

<!-- -->

- #### Parameters

  - ##### index: number

  - ##### max: number

  - ##### wasmContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart" class="tsd-signature-type">TSciChart</a>

  #### Returns string

### toJSON

- toJSON(): { type: string }

<!-- -->

- #### Returns { type: string }

  - ##### type: string

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
