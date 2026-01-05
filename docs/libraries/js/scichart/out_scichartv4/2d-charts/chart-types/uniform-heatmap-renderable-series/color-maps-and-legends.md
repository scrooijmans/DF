On this page

# Heatmap ColorMaps and Legends

## Converting Data-Values to Colors (Defining a Color Map)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/#converting-data-values-to-colors-defining-a-color-map" class="hash-link" aria-label="Direct link to Converting Data-Values to Colors (Defining a Color Map)" translate="no" title="Direct link to Converting Data-Values to Colors (Defining a Color Map)">â€‹</a>

Conversion of data value into color is defined by the propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmaprenderableseries.html#colormap" rel="noopener noreferrer" target="_blank">UniformHeatmapRenderableSeries.colorMapðŸ“˜</a>. The ColorMap isÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">type HeatmapColorPaletteðŸ“˜</a>.Â You can define a custom Color PaletteÂ in JavaScript as follows:

- TS

``` prism-code
// Create a Heatmap RenderableSeries with the color map. ColorMap.minimum/maximum defines the values in
// HeatmapDataSeries which correspond to gradient stops at 0..1
const heatmapSeries = new UniformHeatmapRenderableSeries(wasmContext, {
    dataSeries: heatmapDataSeries,
    useLinearTextureFiltering: false,
    fillValuesOutOfRange: true,
    colorMap: new HeatmapColorMap({
        minimum: 0,
        maximum: 200,
        gradientStops: [
            { offset: 1, color: "#EC0F6C" },
            { offset: 0.9, color: "#F48420" },
            { offset: 0.7, color: "#DC7969" },
            { offset: 0.5, color: "#67BDAF" },
            { offset: 0.3, color: "#50C7E0" },
            { offset: 0.2, color: "#264B9377" }, // Start to fade out the transparency here
            { offset: 0, color: "Transparent" } // Set the zero value as Transparent. Corresponds to zValue <= minimum
        ]
    })
});

sciChartSurface.renderableSeries.add(heatmapSeries);
```

What this means:

- The GradientStop at `Offset = 0` with `Color = "Transparent"` corresponds to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#minimum" rel="noopener noreferrer" target="_blank">HeatmapColorMap.minimumðŸ“˜</a> value of `0`
- The GradientStop at `Offset = 1` with `Color = "#EC0F6C"` corresponds toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#maximum" rel="noopener noreferrer" target="_blank">HeatmapColorMap.maximumðŸ“˜</a> value of `200`.
- Data within this range will be blended according to the gradient stops between `0` and `1`
- Data outside this range will be clamped to the minimum or maximum colors in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">HeatmapColorMapðŸ“˜</a>

### Defining how Data-values outside of ColorMap range are drawn<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/#defining-how-data-values-outside-of-colormap-range-are-drawn" class="hash-link" aria-label="Direct link to Defining how Data-values outside of ColorMap range are drawn" translate="no" title="Direct link to Defining how Data-values outside of ColorMap range are drawn">â€‹</a>

By default, when defining aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">HeatmapColorMapðŸ“˜</a>Â any values that fall outside the range are clipped to the edges of the colormap. e.g. in the above example data falling outside of the range `0-200` is clipped to color `"#000000"` and `"#EC0F6C"` respectively.

There is also aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/baseheatmaprenderableseries.html#fillvaluesoutofrange" rel="noopener noreferrer" target="_blank">fillValuesOutOfRangeðŸ“˜</a> property which defines how the values outside the range are treated. Either clamped to the min/max color or drawn as transparent.

## Heatmap Legends<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/#heatmap-legends" class="hash-link" aria-label="Direct link to Heatmap Legends" translate="no" title="Direct link to Heatmap Legends">â€‹</a>

A heatmap legend may be generated with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" rel="noopener noreferrer" target="_blank">HeatmapLegendðŸ“˜</a> class. It is placed in a element just like a SciChartSurface. It will expand to fit the parent div.

The constructor acceptsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html" rel="noopener noreferrer" target="_blank">IHeatmapLegendOptionsðŸ“˜</a> which lets you specify theme, colorMap and yAxisOptions. This allows configuration of the appearance of the heatmap legend.Â See these are theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaplegendoptions.html" rel="noopener noreferrer" target="_blank">TypeDoc documentationðŸ“˜</a> for this type.Â 

Here's a full code sample below.

- TS

``` prism-code
const { HeatmapLegend, SciChartJsNavyTheme } = SciChart;

const { heatmapLegend, wasmContext } = await HeatmapLegend.create(divElementId, {
    theme: {
        ...new SciChartJsNavyTheme(),
        sciChartBackground: "#14233CBB",
        loadingAnimationBackground: "#14233CBB"
    },
    yAxisOptions: {
        axisBorder: {
            borderLeft: 1,
            color: "#FFFFFF77"
        },
        majorTickLineStyle: {
            color: "White",
            tickSize: 6,
            strokeThickness: 1
        },
        minorTickLineStyle: {
            color: "White",
            tickSize: 3,
            strokeThickness: 1
        }
    },
    colorMap: {
        minimum: 0,
        maximum: 200,
        gradientStops: [
            { offset: 1, color: "#EC0F6C" },
            { offset: 0.9, color: "#F48420" },
            { offset: 0.7, color: "#DC7969" },
            { offset: 0.5, color: "#67BDAF" },
            { offset: 0.3, color: "#50C7E0" },
            { offset: 0.2, color: "#264B9377" }, // Start to fade out the transparency here
            { offset: 0, color: "Transparent" } // Set the zero value as Transparent. Corresponds to zValue <= minimum
        ]
    }
});
```

### Defining the ColorMap on the HeatmapLegend controlÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/#defining-the-colormap-on-the-heatmaplegend-control" class="hash-link" aria-label="Direct link to Defining the ColorMap on the HeatmapLegend controlÂ " translate="no" title="Direct link to Defining the ColorMap on the HeatmapLegend controlÂ ">â€‹</a>

ColorMaps obey similar rules toÂ [Heatmap series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/uniform-heatmap-chart-type)Â (see above).

### Styling the Axis on the HeatmapLegend control<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/#styling-the-axis-on-the-heatmaplegend-control" class="hash-link" aria-label="Direct link to Styling the Axis on the HeatmapLegend control" translate="no" title="Direct link to Styling the Axis on the HeatmapLegend control">â€‹</a>

yAxisOptions is typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iaxisbase2doptions.html" rel="noopener noreferrer" target="_blank">IAxisBase2dOptionsðŸ“˜</a>. This is the same type that is passed to an Axis in SciChart.

To Style the HeatmapLegend is very similar to styling an axis in SciChart. See more at the pageÂ [Axis Styling](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-styling/title-labels-gridlines-axis-band-style).

## Updating ColorMaps Dynamically<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/#updating-colormaps-dynamically" class="hash-link" aria-label="Direct link to Updating ColorMaps Dynamically" translate="no" title="Direct link to Updating ColorMaps Dynamically">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">HeatmapColorMapsðŸ“˜</a> can be updated dynamically by changing their properties. All the properties such as minimum, maximum, gradientStops are fully reactive and when set, the chart will redraw.

Below we've created a demo to show how to updateÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html#gradientstops" rel="noopener noreferrer" target="_blank">HeatmapColorMap.gradientStopsðŸ“˜</a> dynamically by adding interactivity to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" rel="noopener noreferrer" target="_blank">HeatmapLegendðŸ“˜</a>.

- TS

``` prism-code
// .. Assuming SciChartSurface, UniformHeatmapDataSeries already created

// Create a colormap
const colorMap = new HeatmapColorMap({
    minimum: 0,
    maximum: 200,
    gradientStops: [
        { offset: 1, color: "#EC0F6C" },
        { offset: 0.9, color: "#F48420" },
        { offset: 0.7, color: "#DC7969" },
        { offset: 0.5, color: "#67BDAF" },
        { offset: 0.3, color: "#50C7E0" },
        { offset: 0.2, color: "#264B9377" },
        { offset: 0, color: "Transparent" }
    ]
});

// Create a Heatmap RenderableSeries with the color map
sciChartSurface.renderableSeries.add(
    new UniformHeatmapRenderableSeries(wasmContext, {
        dataSeries: heatmapDataSeries,
        useLinearTextureFiltering: false,
        fillValuesOutOfRange: true,
        colorMap
    })
);

// Create the heatmapLegend with the same colorMap
const { heatmapLegend } = await HeatmapLegend.create(divElementIdLegend, {
    theme: {
        ...new SciChartJsNavyTheme(),
        sciChartBackground: "#14233CBB",
        loadingAnimationBackground: "#14233CBB"
    },
    colorMap
});

// The HeatmapLegend is implemented using a SciChartSurface, You can access the inner chart
const legendSciChartSurface = heatmapLegend.innerSciChartSurface.sciChartSurface;

// Create an AxisMarkerAnnotation and subscribe to onDrag
const axisAnnotation = new AxisMarkerAnnotation({
    y1: colorMap.maximum * 0.9,
    isEditable: true,
    onDrag: args => {
        // First step: prevent dragging outside the min/max
        if (axisAnnotation.y1 > 200) axisAnnotation.y1 = 200;
        if (axisAnnotation.y1 < 0) axisAnnotation.y1 = 0;

        // On-drag, update the gradient stops and re-assign. The Chart automatically redraws
        const gradientStops = [
            { offset: 1, color: "#EC0F6C" },
            { offset: axisAnnotation.y1 / 200, color: "#F48420" },
            { offset: 0.7, color: "#DC7969" },
            { offset: 0.5, color: "#67BDAF" },
            { offset: 0.3, color: "#50C7E0" },
            { offset: 0.2, color: "#264B9377" },
            { offset: 0, color: "Transparent" }
        ];
        colorMap.gradientStops = gradientStops;
    }
});

// Add it to the legend's SciChartSurface
legendSciChartSurface.annotations.add(axisAnnotation);
```

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" rel="noopener noreferrer" target="_blank">HeatmapLegendðŸ“˜</a>Â is implemented internally using a SciChartSurface. You can access the surface via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html#innerSciChartSurface" rel="noopener noreferrer" target="_blank">innerSciChartSurfaceðŸ“˜</a> property. After that, you can configure the axis, series, annotations just like you would any other SciChartSurface.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/uniform-heatmap-renderable-series/color-maps-and-legends/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
