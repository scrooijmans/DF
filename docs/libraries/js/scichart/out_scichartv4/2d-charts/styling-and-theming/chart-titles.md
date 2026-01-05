On this page

# Chart Styling - Chart Titles

New to SciChart.js v3.1, we've added a Chart Title property allowing for multi-line titles on the top, left, right, bottom of the chart and with various alignment options.

Adding a chart title is simple, you can do so with the following code:

- TS
- Builder API (JSON Config)

``` prism-code
// Demonstrates how to configure chart titles SciChart.js
const {
    SciChartSurface,
    NumericAxis,
    SciChartJSLightTheme,
    Thickness,
    EMultiLineAlignment,
    ETextAlignment,
    ETitlePosition
} = SciChart;

// or, for npm, import { SciChartSurface, ... } from "scichart"

const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJSLightTheme(),
    title: "SciChart.js Chart Title",
    titleStyle: {
        color: "#333333",
        fontSize: 32,
        padding: Thickness.fromString("14 8 4 8"), // Top, Right, Bottom, Left padding
        useNativeText: false, // Use WebGL accelerated text
        placeWithinChart: false, // When true, place inside chart, else outside
        multilineAlignment: EMultiLineAlignment.Left, // When \n present how does multiline text align (Left, Center, Right)
        alignment: ETextAlignment.Center, // Alignment of title (Left, Center, Right)
        position: ETitlePosition.Top // Vertical position of title (Top, Bottom, Left, Right)
    }
});

// Create an X and Y Axis with title
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
```

``` prism-code
// Demonstrates how to configure chart titles in SciChart.js using the Builder API
const {
    chartBuilder,
    EThemeProviderType,
    EAxisType,
    Thickness,
    EMultiLineAlignment,
    ETextAlignment,
    ETitlePosition
} = SciChart;

// or, for npm, import { chartBuilder, ... } from "scichart"

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: {
        theme: { type: EThemeProviderType.Dark },
        title: "SciChart.js Chart Title",
        titleStyle: {
            color: "#50C7E0",
            fontSize: 24,
            padding: Thickness.fromString("14 0 4 0"),
            useNativeText: true,
            placeWithinChart: false,
            multilineAlignment: EMultiLineAlignment.Center,
            alignment: ETextAlignment.Center,
            position: ETitlePosition.Top
        }
    },
    xAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "X Axis" }
    },
    yAxes: {
        type: EAxisType.NumericAxis,
        options: { axisTitle: "Y Axis" }
    }
});
```

This results in the following output:

![](out_scichartv4/2d-charts/styling-and-theming/chart-titles/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For an example of multiline chart titles see theÂ <a href="https://www.scichart.com/demo/javascript-chart-title" rel="noopener noreferrer" target="_blank">Chart Title demo</a> in our examples suite.

## Title Styling and Positioning<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/chart-titles/#title-styling-and-positioning" class="hash-link" aria-label="Direct link to Title Styling and Positioning" translate="no" title="Direct link to Title Styling and Positioning">â€‹</a>

For styling and positioning a title we can useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dsurfaceoptions.html#titlestyle" rel="noopener noreferrer" target="_blank">I2DSurfaceOptions.titleStyleðŸ“˜</a> property in constructor options or a surface propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#titlestyle" rel="noopener noreferrer" target="_blank">SciChartSurface.titleStyleðŸ“˜</a>. Available styling options are defined byÂ **<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcharttitlestyle" rel="noopener noreferrer" target="_blank">TChartTitleStyleðŸ“˜</a>** type.

![](out_scichartv4/2d-charts/styling-and-theming/chart-titles/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

When setting via a surface instance, titleStyle should be assigned to an object (or partial object) of type instead of directly setting individual properties. The object then will be merged with the current or default title style.

### Text Styling<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/chart-titles/#text-styling" class="hash-link" aria-label="Direct link to Text Styling" translate="no" title="Direct link to Text Styling">â€‹</a>

Base text styling options for a chart title are:

- **fontSize**
- **fontFamily**
- **color**

``` prism-code
// Text Styling

sciChartSurface.titleStyle = {
    fontSize: 30,
    fontFamily: "Arial",
    color: "#EC0F6C",
    fontWeight: "900",
    fontStyle: "italic",
}
```

Additionally we can set **fontWeight** and **fontStyle** for non-native text title. Find out more info about limitations in the **Native Text section** of the docs below.

### Title Positioning<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/chart-titles/#title-positioning" class="hash-link" aria-label="Direct link to Title Positioning" translate="no" title="Direct link to Title Positioning">â€‹</a>

A title could be placed on different sides relative to the surface. The options are defined byÂ **<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/etitleposition.html" rel="noopener noreferrer" target="_blank">ETitlePositionðŸ“˜</a>** enum and are set by **TChartTitleStyle.position**.

To specify an anchor for a title using **TChartTitleStyle.alignment**, where options are defined in ETextAlignment enum.

Also it is possible to place a title within the series view area using **TChartTitleStyle.placeWithinChart** flag.

``` prism-code
// Title Positioning

const {
    // ...
    ETitlePosition,
    ETextAlignment,
} = SciChart;
// or import { ETitlePosition, ETextAlignment } from "scichart";
sciChartSurface.titleStyle = {
    position: ETitlePosition.Left,
    alignment: ETextAlignment.Right,
    placeWithinChart: true,
};
```

## Multiline Chart Titles<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/chart-titles/#multiline-chart-titles" class="hash-link" aria-label="Direct link to Multiline Chart Titles" translate="no" title="Direct link to Multiline Chart Titles">â€‹</a>

To set multiline text as a title we can pass it as an array of lines or split lines with the new line character (\n).

Properties that could be applied to the multiline text are

- **TChartTitleStyle.multilineAlignment**
- **TChartTitleStyle.lineSpacing**

![](out_scichartv4/2d-charts/styling-and-theming/chart-titles/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The multilineAlignment options are described inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/emultilinealignment.html" rel="noopener noreferrer" target="_blank">EMultiLineAlignmentðŸ“˜</a> enum. The lineSpacing is a multiple of the line height.

``` prism-code
// Multiline Chart Titles

const { EMultiLineAlignment } = SciChart;
    // Add multiline chart title
    sciChartSurface.title = \["First line", "Second line", "Third line"\]; // "Or 'FirstLine \\n Second line'
    // Modify multiline text related options for the title
    sciChartSurface.titleStyle = {
        multilineAlignment: EMultiLineAlignment.Right,
        lineSpacing: 1.5
}
```

## WebGL Native Text Titles<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/chart-titles/#webgl-native-text-titles" class="hash-link" aria-label="Direct link to WebGL Native Text Titles" translate="no" title="Direct link to WebGL Native Text Titles">â€‹</a>

It is possible to enable the Native Text Rendering for the Chart Title. This can improve the rendering performance and allows using custom fonts, but have some differences and limitations compared to the non-native text rendering (e.g. native text doesn't support fontWeight and fontStyle). Find more info about theÂ [Native Text API here](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api).

The Native Text Rendering for a chart title is set by **TChartTitleStyle.useNativeText** flag or uses the default value defined inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartdefaults.html#usenativetext" rel="noopener noreferrer" target="_blank">SciChartDefaults.useNativeTextðŸ“˜</a>.

## Title Rendering Customisation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/chart-titles/#title-rendering-customisation" class="hash-link" aria-label="Direct link to Title Rendering Customisation" translate="no" title="Direct link to Title Rendering Customisation">â€‹</a>

For an advanced customization of the chart title rendering one may use a custom Chart Title Renderer.

The Chart Title renderer could be accessed or set withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#charttitlerenderer" rel="noopener noreferrer" target="_blank">SciChartSurface.chartTitleRendererðŸ“˜</a> property.

![](out_scichartv4/2d-charts/styling-and-theming/chart-titles/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The default Chart Title Renderer additionally provides a debug rendering and caching of non-native text which could be toggled withÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/charttitlerenderer.html#drawdebug" rel="noopener noreferrer" target="_blank">ChartTitleRenderer.drawDebugðŸ“˜</a> andÂ **<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/charttitlerenderer.html#usecache" rel="noopener noreferrer" target="_blank">ChartTitleRenderer.useCacheðŸ“˜</a>** flags appropriately.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/chart-titles/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/chart-titles/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
