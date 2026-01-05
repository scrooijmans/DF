<img src="out_scichartv4/typedoc/classes/scichartjslighttheme_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [SciChartJSLightTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html)

# Class SciChartJSLightTheme

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

An implementation of [ThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/enums/ebasetype.html#themeprovider) which provides a light theme

decription  
Applied to a 2D [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html), or a 3D [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html), the ThemeProvider may be applied using the [applyTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#applytheme) method, where it is passed down to child components so that all children of the chart get the same theme.

For example:

``` ts
// Applying a theme when creating a chart
const { sciChartSurface, wasmContext } = SciChartSurface.create("div-id", { theme: new SciChartJSDarkTheme() });
// Apply a theme after chart creation
sciChartSurface.applyTheme(new SciChartJSDarkTheme());

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

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/themeprovider.html" class="tsd-signature-type">ThemeProvider</a>
  - SciChartJSLightTheme

### Implements

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#annotationselectionstroke" class="tsd-kind-icon">annotationSelectionStroke</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#annotationsgripsbackgroundbrush" class="tsd-kind-icon">annotationsGripsBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#annotationsgripsborderbrush" class="tsd-kind-icon">annotationsGripsBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#axis3dbandsfill" class="tsd-kind-icon">axis3DBandsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#axisbandsfill" class="tsd-kind-icon">axisBandsFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#axisborder" class="tsd-kind-icon">axisBorder</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#axisplanebackgroundfill" class="tsd-kind-icon">axisPlaneBackgroundFill</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#axistitlecolor" class="tsd-kind-icon">axisTitleColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#charttitlecolor" class="tsd-kind-icon">chartTitleColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#columnfillbrush" class="tsd-kind-icon">columnFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#columnlinecolor" class="tsd-kind-icon">columnLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#cursorlinebrush" class="tsd-kind-icon">cursorLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#defaultcolormapbrush" class="tsd-kind-icon">defaultColorMapBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#downbandseriesfillcolor" class="tsd-kind-icon">downBandSeriesFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#downbandserieslinecolor" class="tsd-kind-icon">downBandSeriesLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#downbodybrush" class="tsd-kind-icon">downBodyBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#downwickcolor" class="tsd-kind-icon">downWickColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#fillpalette" class="tsd-kind-icon">fillPalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#gridbackgroundbrush" class="tsd-kind-icon">gridBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#gridborderbrush" class="tsd-kind-icon">gridBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#impulsefillbrush" class="tsd-kind-icon">impulseFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#islightbackground" class="tsd-kind-icon">isLightBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#labelbackgroundbrush" class="tsd-kind-icon">labelBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#labelborderbrush" class="tsd-kind-icon">labelBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#labelforegroundbrush" class="tsd-kind-icon">labelForegroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#legendbackgroundbrush" class="tsd-kind-icon">legendBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#lineseriescolor" class="tsd-kind-icon">lineSeriesColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#loadinganimationbackground" class="tsd-kind-icon">loadingAnimationBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#loadinganimationforeground" class="tsd-kind-icon">loadingAnimationForeground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#majorgridlinebrush" class="tsd-kind-icon">majorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#minorgridlinebrush" class="tsd-kind-icon">minorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#mountainareabrush" class="tsd-kind-icon">mountainAreaBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#mountainlinecolor" class="tsd-kind-icon">mountainLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#overrides" class="tsd-kind-icon">overrides</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#overviewfillbrush" class="tsd-kind-icon">overviewFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#planebordercolor" class="tsd-kind-icon">planeBorderColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#polarmajorgridlinebrush" class="tsd-kind-icon">polarMajorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#polarminorgridlinebrush" class="tsd-kind-icon">polarMinorGridLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#rolloverlinebrush" class="tsd-kind-icon">rolloverLineBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#rubberbandfillbrush" class="tsd-kind-icon">rubberBandFillBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#rubberbandstrokebrush" class="tsd-kind-icon">rubberBandStrokeBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#scichartbackground" class="tsd-kind-icon">sciChartBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#scrollbarbackgroundbrush" class="tsd-kind-icon">scrollbarBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#scrollbarborderbrush" class="tsd-kind-icon">scrollbarBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#scrollbargripsbackgroundbrush" class="tsd-kind-icon">scrollbarGripsBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#scrollbarviewportbackgroundbrush" class="tsd-kind-icon">scrollbarViewportBackgroundBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#scrollbarviewportborderbrush" class="tsd-kind-icon">scrollbarViewportBorderBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#shadoweffectcolor" class="tsd-kind-icon">shadowEffectColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#strokepalette" class="tsd-kind-icon">strokePalette</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#textannotationbackground" class="tsd-kind-icon">textAnnotationBackground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#textannotationforeground" class="tsd-kind-icon">textAnnotationForeground</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#ticktextbrush" class="tsd-kind-icon">tickTextBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#type" class="tsd-kind-icon">type</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#upbandseriesfillcolor" class="tsd-kind-icon">upBandSeriesFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#upbandserieslinecolor" class="tsd-kind-icon">upBandSeriesLineColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#upbodybrush" class="tsd-kind-icon">upBodyBrush</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#upwickcolor" class="tsd-kind-icon">upWickColor</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#applyoverrides" class="tsd-kind-icon">applyOverrides</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#getfillcolor" class="tsd-kind-icon">getFillColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#getstrokecolor" class="tsd-kind-icon">getStrokeColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html#tojson" class="tsd-kind-icon">toJSON</a>

## Properties

### annotationSelectionStroke

annotationSelectionStroke: string = "#f00e0e66"

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) selection box stroke color as an HTML color code

### annotationsGripsBackgroundBrush

annotationsGripsBackgroundBrush: string = "#FFFFFF33"

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) grips background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### annotationsGripsBorderBrush

annotationsGripsBorderBrush: string = "#232323FF"

The [AnnotationBase](https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html) grips border color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axis3DBandsFill

axis3DBandsFill: string = "#33333333"

The default 3D Chart Axis Bands fill color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### axisBandsFill

axisBandsFill: string = "#AAAAAA09"

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

axisTitleColor: string = "#777777FF"

The default Axis Title color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### chartTitleColor

chartTitleColor: string = "#777777FF"

The default Chart Title color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### columnFillBrush

columnFillBrush: string = "#777777FF"

The default [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html) fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### columnLineColor

columnLineColor: string = "#777777FF"

The default [FastColumnRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcolumnrenderableseries.html) stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### cursorLineBrush

cursorLineBrush: string = "#33333355"

The Cursor Line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### defaultColorMapBrush

defaultColorMapBrush: { color: string; offset: number }\[\] = \[{ offset: 0, color: "DARKBLUE" },{ offset: 0.5, color: "CORNFLOWERBLUE" },{ offset: 1, color: "#FF22AAFF" }\]

The default [UniformHeatmapRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html) color-map gradient stops

### downBandSeriesFillColor

downBandSeriesFillColor: string = "#E26565A0"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) down band fill color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downBandSeriesLineColor

downBandSeriesLineColor: string = "#E26565FF"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) down line color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downBodyBrush

downBodyBrush: string = "#E26565D0"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) down body color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### downWickColor

downWickColor: string = "#E26565FF"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) down-wick color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### fillPalette

fillPalette: string\[\] = \["#F4842077", "#AE408E77", "#30BC9A77", "#209FD977", "#264B9377"\]

An array of color codes (or GradientStops if you want to control the gaps between colors) which will be used to provide fill colors.

### gridBackgroundBrush

gridBackgroundBrush: string = "#05333377"

The Grid area background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### gridBorderBrush

gridBorderBrush: string = "#33333399"

The Grid border brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### impulseFillBrush

impulseFillBrush: string = "#777777FF"

The default [FastImpulseRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastimpulserenderableseries.html) fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### isLightBackground

isLightBackground: boolean = true

Tells SciChart if the theme has a light or dark background (used for calculating contrasting elements)

### labelBackgroundBrush

labelBackgroundBrush: string = "#D0D0D0BB"

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### labelBorderBrush

labelBorderBrush: string = "#33333377"

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label border brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### labelForegroundBrush

labelForegroundBrush: string = "#555555FF"

The [Axis](https://www.scichart.com/documentation/js/v4/typedoc/classes/axiscore.html) label foreground font color brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### legendBackgroundBrush

legendBackgroundBrush: string = "#33333333"

The [SciChartLegend](https://www.scichart.com/documentation/js/v4/typedoc/enums/elegendtype.html#scichartlegend) background brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### lineSeriesColor

lineSeriesColor: string = "#777777FF"

The default [FastLineRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinerenderableseries.html) line stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### loadingAnimationBackground

loadingAnimationBackground: string = "#F9F9F9FF"

The background color of the loading animation dots, which can also be customized by overriding the [loader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesurfaceoptions.html#loader)

### loadingAnimationForeground

loadingAnimationForeground: string = "#777777FF"

The foreground color of the loading animation dots, which can also be customized by overriding the [loader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipiesurfaceoptions.html#loader)

### majorGridLineBrush

majorGridLineBrush: string = "#CFCFCFFF"

The Major Gridlines brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### minorGridLineBrush

minorGridLineBrush: string = "#CFCFCF77"

The Minor Gridlines brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### mountainAreaBrush

mountainAreaBrush: string = "#76B7E2B4"

The default [FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html) mountain fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### mountainLineColor

mountainLineColor: string = "#777777FF"

The default [FastMountainRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastmountainrenderableseries.html) mountain line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### Protected overrides

overrides: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemepartial.html" class="tsd-signature-type">IThemePartial</a>

### overviewFillBrush

overviewFillBrush: string = "#33333322"

inheritdoc  

### planeBorderColor

planeBorderColor: string = "#EEEEEEFF"

The default 3D Chart Axis Plane border color applied as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### polarMajorGridLineBrush

polarMajorGridLineBrush: string = "#CFCFCFFF"

The Major Gridlines brush for Polar surfaces as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### polarMinorGridLineBrush

polarMinorGridLineBrush: string = "#E5E5E5FF"

The Minor Gridlines brush for Polar surfaces as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rolloverLineBrush

rolloverLineBrush: string = "#33333333"

The [RolloverModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html) line brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rubberBandFillBrush

rubberBandFillBrush: string = "#33333333"

The [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) recticule fill brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### rubberBandStrokeBrush

rubberBandStrokeBrush: string = "#33333377"

The [RubberBandXyZoomModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/rubberbandxyzoommodifier.html) recticule stroke brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### sciChartBackground

sciChartBackground: string = "#F9F9F9FF"

The Background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### scrollbarBackgroundBrush

scrollbarBackgroundBrush: string = "#33333322"

inheritdoc  

### scrollbarBorderBrush

scrollbarBorderBrush: string = "#12121255"

inheritdoc  

### scrollbarGripsBackgroundBrush

scrollbarGripsBackgroundBrush: string = "#FFFFFF66"

inheritdoc  

### scrollbarViewportBackgroundBrush

scrollbarViewportBackgroundBrush: string = "#FFFFFF44"

inheritdoc  

### scrollbarViewportBorderBrush

scrollbarViewportBorderBrush: string = "#12121255"

inheritdoc  

### shadowEffectColor

shadowEffectColor: string = "#A0AABAFA"

The default shadow effect color applied to drop-shadows

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### strokePalette

strokePalette: string\[\] = \["#F48420", "#AE408E", "#30BC9A", "#209FD9", "#264B93"\]

An array of color codes (or GradientStops if you want to control the gaps between colors) which will be used to provide stroke colors.

### textAnnotationBackground

textAnnotationBackground: string = "#FFFFFFFF"

The TextAnnotation background color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### textAnnotationForeground

textAnnotationForeground: string = "#000000FF"

The TextAnnotation font color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### tickTextBrush

tickTextBrush: string = "#333333FF"

The Tick Text brush as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### type

type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ethemeprovidertype.html" class="tsd-signature-type">EThemeProviderType</a> = EThemeProviderType.Light

inheritdoc  

### upBandSeriesFillColor

upBandSeriesFillColor: string = "#52CC5490"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) up band fill color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBandSeriesLineColor

upBandSeriesLineColor: string = "#52CC54FF"

The default [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastbandrenderableseries.html) up line color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upBodyBrush

upBodyBrush: string = "#52CC54A0"

The default [FastCandlestickRenderableSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/fastcandlestickrenderableseries.html) up body color as an HTML color code

remarks  
Acceptable values include RGB format e.g. `#FF0000`, RGBA format e.g. ``` #FF000077`` and RGBA format e.g. ```rgba(255,0,0,0.5)\`\`\`

### upWickColor

upWickColor: string = "#52CC54FF"

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
