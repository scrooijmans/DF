<img src="out_scichartv4/typedoc/interfaces/iheatmaplegendoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IHeatmapLegendOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html)

# Interface IHeatmapLegendOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Hierarchy

- IHeatmapLegendOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html#colormap" class="tsd-kind-icon">colorMap</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html#theme" class="tsd-kind-icon">theme</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html#yaxisoptions" class="tsd-kind-icon">yAxisOptions</a>

## Properties

### Optional colorMap

colorMap: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" class="tsd-signature-type">HeatmapColorMap</a> \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmapcolormapoptions.html" class="tsd-signature-type">IHeatmapColorMapOptions</a>

The [HeatmapColorMap](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html) instance, which maps heatmap z-values to colors or an [IHeatmapColorMapOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmapcolormapoptions.html) object which will be used to build a HeatmapColorMap

### Optional theme

theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a> \| ({ type: string \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ethemeprovidertype.html" class="tsd-signature-type">EThemeProviderType</a> } & <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemepartial.html" class="tsd-signature-type">IThemePartial</a>)

Optional - The theme applied to the inner [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) on startup

remarks  
see [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html) for properties which can affect SciChart theme. Two default themes are included out of the box [SciChartJSLightTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjslighttheme.html) and [SciChartJSDarkTheme](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartjsdarktheme.html). Custom themes may be created by implementing [IThemeProvider](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html)

### Optional yAxisOptions

yAxisOptions: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisbase2doptions.html" class="tsd-signature-type">IAxisBase2dOptions</a>

Optional - options applied to the yaxis used in the inner [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) in the [HeatmapLegend](https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html) control

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
