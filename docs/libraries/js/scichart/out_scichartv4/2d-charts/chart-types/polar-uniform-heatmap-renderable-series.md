On this page

# The Polar Uniform Heatmap Chart Type

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaruniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">PolarUniformHeatmapRenderableSeriesðŸ“˜</a> displays "heat" values as colored cells in a polar (circular) coordinate systemâ€”perfect for visualizing intensity, density, or other matrix-like data in a circular form. Each cell is mapped from a two-dimensional array of z-values, with color gradients representing the data's magnitude.

![](out_scichartv4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/javascript/polar-uniform-heatmap-chart" rel="noopener noreferrer" target="_blank">JavaScript Polar Uniform Heatmap Chart</a> can be found in theÂ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/blob/release_v4.0/Examples/src/components/Examples/Charts2D/PolarCharts/PolarUniformHeatmapChart" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite &gt; Polar Uniform Heatmap Chart</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/react/polar-uniform-heatmap-chart" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-uniform-heatmap-chart" target="_blank">Polar Uniform Heatmap Series Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

## Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#properties" class="hash-link" aria-label="Direct link to Properties" translate="no" title="Direct link to Properties">â€‹</a>

Key options for <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaruniformheatmaprenderableseriesoptions.html" rel="noopener noreferrer" target="_blank">IPolarUniformHeatmapRenderableSeriesOptionsðŸ“˜</a> include:

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#dataseries" rel="noopener noreferrer" target="_blank">dataSeriesðŸ“˜</a> â€” A <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a> containing a 2D array of `zValues`, and X / Y Step and Start.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">colorMapðŸ“˜</a> â€” Maps zValues to colors via <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmapcolormapoptions.html#gradientstops" rel="noopener noreferrer" target="_blank">HeatmapColorMapðŸ“˜</a>.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#uselineartexturefiltering" rel="noopener noreferrer" target="_blank">useLinearTextureFilteringðŸ“˜</a> â€” Enables smooth texture filtering (default: `false`).
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#fillvaluesoutofrange" rel="noopener noreferrer" target="_blank">fillValuesOutOfRangeðŸ“˜</a> â€” Fills cells outside the color mapâ€™s min/max with a defined edge color.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#stroke" rel="noopener noreferrer" target="_blank">strokeðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#strokethickness" rel="noopener noreferrer" target="_blank">strokeThicknessðŸ“˜</a> â€” Cell border styling.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#datalabels" rel="noopener noreferrer" target="_blank">dataLabelsðŸ“˜</a> â€” Enable and style per-cell text labels.

## Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

### Basic Polar Heatmap<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#basic-polar-heatmap" class="hash-link" aria-label="Direct link to Basic Polar Heatmap" translate="no" title="Direct link to Basic Polar Heatmap">â€‹</a>

``` prism-code
const {
    PolarMouseWheelZoomModifier,
    SciChartJsNavyTheme,
    PolarZoomExtentsModifier,
    PolarPanModifier,
    EAxisAlignment,
    PolarNumericAxis,
    EPolarLabelMode,
    SciChartPolarSurface,
    EPolarAxisMode, 
    NumberRange, 
    HeatmapColorMap,
    UniformHeatmapDataSeries,
    PolarUniformHeatmapRenderableSeries,
} = SciChart;
// or, for npm, import { SciChartSurface, ... } from "scichart"

const { sciChartSurface, wasmContext } = await SciChartPolarSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
});

const HEATMAP_WIDTH = 48;
const HEATMAP_HEIGHT = 10;

const angularXAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Angular, 
    axisAlignment: EAxisAlignment.Left,
    visibleRange: new NumberRange(0, HEATMAP_WIDTH),

    autoTicks: false,
    majorDelta: 1,

    polarLabelMode: EPolarLabelMode.Perpendicular,
    flippedCoordinates: true, // go clockwise
    totalAngle: Math.PI * 2,
});
sciChartSurface.xAxes.add(angularXAxis);

const radialYAxis = new PolarNumericAxis(wasmContext, {
    polarAxisMode: EPolarAxisMode.Radial,
    axisAlignment: EAxisAlignment.Bottom,
    visibleRange: new NumberRange(0, HEATMAP_HEIGHT),

    drawLabels: false, // hide radial labels
    innerRadius: 0.1,
});
sciChartSurface.yAxes.add(radialYAxis);

// Add a heatmap series to the chart
const heatmapSeries = new PolarUniformHeatmapRenderableSeries(wasmContext, {
    dataSeries: new UniformHeatmapDataSeries(wasmContext, {
        zValues: Array.from({ length: HEATMAP_HEIGHT }, () => {
            return Array.from({ length: HEATMAP_WIDTH }, () => {
                return Math.random() * 100;
            });
        }),
        xStart: 0,
        yStart: 0,
        xStep: 1,
        yStep: 1,
    }),
    colorMap: new HeatmapColorMap({
        minimum: 0,
        maximum: 100,
        gradientStops: [
            { offset: 0, color: "lightblue" },
            { offset: 1, color: "indigo" },
        ]
    }),
});
sciChartSurface.renderableSeries.add(heatmapSeries);
```

In the code above:

- A 2D array of `zValues` is generated via `Array.from`, representing value "intensity" at each polar sector.
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaruniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">PolarUniformHeatmapRenderableSeriesðŸ“˜</a> is constructed, passing a <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/uniformheatmapdataseries.html" rel="noopener noreferrer" target="_blank">UniformHeatmapDataSeriesðŸ“˜</a> and a custom <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">HeatmapColorMapðŸ“˜</a> with gradient stops, from 0 to 100.
- Polar axes are configured for angular and radial sweep.
- Chart modifiers such as <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpanmodifier.html" rel="noopener noreferrer" target="_blank">PolarPanModifierðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">PolarZoomExtentsModifierðŸ“˜</a>, and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmousewheelzoommodifier.html" rel="noopener noreferrer" target="_blank">PolarMouseWheelZoomModifierðŸ“˜</a> provide interaction.

### Polar Heatmap with Legend<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#polar-heatmap-with-legend" class="hash-link" aria-label="Direct link to Polar Heatmap with Legend" translate="no" title="Direct link to Polar Heatmap with Legend">â€‹</a>

``` prism-code
const COLOR_MAP = new HeatmapColorMap({
    minimum: 0,
    maximum: 100,
    gradientStops: [
        { offset: 0, color: "#000000" },
        { offset: 1, color: "#3333AAAA" },
    ]
});

// Add a heatmap series to the chart
const heatmapSeries = new PolarUniformHeatmapRenderableSeries(wasmContext, {
    dataSeries: new UniformHeatmapDataSeries(wasmContext, {
        zValues: Array.from({ length: HEATMAP_HEIGHT }, () => {
            return Array.from({ length: HEATMAP_WIDTH }, () => {
                return Math.random() * 100;
            });
        }),
        xStart: 0,
        yStart: 0,
        xStep: 1,
        yStep: 1,
    }),
    colorMap: COLOR_MAP,
});
sciChartSurface.renderableSeries.add(heatmapSeries);

// also pass "scichart-legend-root" as `legendElementId` in drawExample()
const { heatmapLegend } = await HeatmapLegend.create(legendElementId, { 
    theme: {
        ...new SciChartJsNavyTheme(),
        sciChartBackground: "transparent",
        loadingAnimationBackground: "indigo"
    },
    yAxisOptions: {
        isInnerAxis: true,
        labelStyle: {
            fontSize: 14,
            color: "white",
        },
        axisBorder: {
            borderRight: 2,
            color: "white",
        },
        majorTickLineStyle: {
            color: "white",
            tickSize: 8,
            strokeThickness: 2,
        },
        minorTickLineStyle: {
            color: "white",
            tickSize: 4,
            strokeThickness: 1,
        },
    },
    colorMap: COLOR_MAP
});
```

You also need an additional HTML element for the legend:

``` prism-code
<section>
    <div id="scichart-root"></div>
    <div id="scichart-legend-root"></div>
</section>
```

With these CSS properties:

``` prism-code
body { margin: 0; }
section { width: 100%; height: 100vh; display: flex; }

#scichart-root { width: 100%; height: 100%; }

#scichart-legend-root { height: 100%; width: 65px; }
```

In the code above:

- The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" rel="noopener noreferrer" target="_blank">HeatmapLegendðŸ“˜</a> component creates an interactive vertical legend, visually linking the z-value color range.
- A custom color map with **multiple gradient stops** is defined for meaningful value zones.
- Data is generated using a `generateHeatmapData` utility function (see demo source for details).
- Both the heatmap and the legend share the same <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmapcolormap.html" rel="noopener noreferrer" target="_blank">HeatmapColorMapðŸ“˜</a> instance to keep color mapping consistent.
- Axes are positioned and styled for optimal polar data presentation.

### Medical Imaging (Ultrasound Heatmap)<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#medical-imaging-ultrasound-heatmap" class="hash-link" aria-label="Direct link to Medical Imaging (Ultrasound Heatmap)" translate="no" title="Direct link to Medical Imaging (Ultrasound Heatmap)">â€‹</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/polar-uniform-heatmap-ultrasound" target="_blank">Polar Ultrasound Heatmap Example</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a>

  

![](out_scichartv4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For the full code walkthrough and live demo, see the **<a href="https://www.scichart.com/demo/react/polar-ultrasound-heatmap" rel="noopener noreferrer" target="_blank">Polar Ultrasound Heatmap Example</a>**

## Tips & Best Practices<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#tips--best-practices" class="hash-link" aria-label="Direct link to Tips &amp; Best Practices" translate="no" title="Direct link to Tips &amp; Best Practices">â€‹</a>

- **Performance**: For large polar heatmaps, enable <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iheatmaprenderableseriesoptions.html#uselineartexturefiltering" rel="noopener noreferrer" target="_blank">useLinearTextureFilteringðŸ“˜</a> for smoother visual transitions, or disable it for sharp, pixelated looks.
- **Legend**: Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/heatmaplegend.html" rel="noopener noreferrer" target="_blank">HeatmapLegendðŸ“˜</a> for accessible, interpretable color mapping.
- **Donut charts**: Set the <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxisoptions.html#innerradius" rel="noopener noreferrer" target="_blank">innerRadiusðŸ“˜</a> property of the radial axis for donut-shaped polar heatmaps.
- **Manipulate Angles**: Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxisoptions.html#totalangle" rel="noopener noreferrer" target="_blank">totalAngleðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipolaraxisoptions.html#totalangledegrees" rel="noopener noreferrer" target="_blank">totalAngleDegreesðŸ“˜</a> properties of the angular axis to control the heatmap's angular range. From `0` to `2Ï€ radians` or `360 degrees`.
- **Interactivity**:
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarpanmodifier.html" rel="noopener noreferrer" target="_blank">PolarPanModifierðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarzoomextentsmodifier.html" rel="noopener noreferrer" target="_blank">PolarZoomExtentsModifierðŸ“˜</a>, and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarmousewheelzoommodifier.html" rel="noopener noreferrer" target="_blank">PolarMouseWheelZoomModifierðŸ“˜</a> can all be added for advanced user navigation.
  - Don't forget about annotations! Use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/linearrowannotation.html" rel="noopener noreferrer" target="_blank">LineArrowAnnotationðŸ“˜</a> or <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/textannotation.html" rel="noopener noreferrer" target="_blank">TextAnnotationðŸ“˜</a> to highlight specific areas or values in your polar heatmap. All other annotations from SciChart.js are also supported.

## Use Cases<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/#use-cases" class="hash-link" aria-label="Direct link to Use Cases" translate="no" title="Direct link to Use Cases">â€‹</a>

Polar heatmaps are widely used in:

- Physics and engineering for field visualization
- Medical imaging (e.g. **ultrasound**, see <a href="https://www.scichart.com/demo/react/polar-ultrasound-heatmap" rel="noopener noreferrer" target="_blank">Polar Ultrasound Demo</a>)
- Environmental mapping
- Radar and sonar data

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaruniformheatmaprenderableseries.html" rel="noopener noreferrer" target="_blank">PolarUniformHeatmapRenderableSeriesðŸ“˜</a> enables powerful, visually compelling circular heatmaps in JavaScript. With SciChart.js, you can combine rich color gradients, advanced axis control, interaction, and even medical imaging overlays in a performant, interactive chart component.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/polar-uniform-heatmap-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
