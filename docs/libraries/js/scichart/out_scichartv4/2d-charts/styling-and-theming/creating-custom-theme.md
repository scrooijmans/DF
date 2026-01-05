On this page

# Chart Styling - Creating a Custom Theme

As well as the built-inÂ [Light and Dark theme](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/), with SciChart.js you can also create a custom theme. To do this, you will need to pass all the properties of theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" rel="noopener noreferrer" target="_blank">IThemeProviderðŸ“˜</a> interface to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#applytheme" rel="noopener noreferrer" target="_blank">SciChartSurface.applyThemeðŸ“˜</a> function.

For example, the following code:

- JS
- TS

``` prism-code
import {SciChartSurface, NumericAxis} from "scichart";

export async function createCustomTheme(divId) {
// Create a custom theme by implementing all the properties from IThemeProvider
const customTheme = {
    axisBorder: "Transparent",
    axisTitleColor: "#6495ED",
    annotationsGripsBackroundBrush: "white",
    annotationsGripsBorderBrush: "white",
    axis3DBandsFill: "#1F3D6833",
    axisBandsFill: "#1F3D6833",
    axisPlaneBackgroundFill: "Transparent",
    columnFillBrush: "white",
    columnLineColor: "white",
    cursorLineBrush: "#6495ED99",
    defaultColorMapBrush: [
        { offset: 0, color: "DarkBlue" },
        { offset: 0.5, color: "CornflowerBlue" },
        { offset: 1, color: "#FF22AA" }
    ],
    downBandSeriesFillColor: "#52CC5490",
    downBandSeriesLineColor: "#E26565FF",
    downBodyBrush: "white",
    downWickColor: "white",
    gridBackgroundBrush: "white",
    gridBorderBrush: "white",
    labelBackgroundBrush: "#6495EDAA",
    labelBorderBrush: "#6495ED",
    labelForegroundBrush: "#EEEEEE",
    legendBackgroundBrush: "#1D2C35",
    lineSeriesColor: "white",
    loadingAnimationBackground: "#0D213A",
    loadingAnimationForeground: "#6495ED",
    majorGridLineBrush: "#1F3D68",
    minorGridLineBrush: "#102A47",
    mountainAreaBrush: "white",
    mountainLineColor: "white",
    overviewFillBrush: "white",
    planeBorderColor: "white",
    rolloverLineBrush: "#FD9F2533",
    rubberBandFillBrush: "#99999933",
    rubberBandStrokeBrush: "#99999977",
    sciChartBackground: "#0D213A",
    scrollbarBackgroundBrush: "white",
    scrollbarBorderBrush: "white",
    scrollbarGripsBackgroundBrush: "white",
    scrollbarViewportBackgroundBrush: "white",
    scrollbarViewportBorderBrush: "white",
    shadowEffectColor: "white",
    textAnnotationBackground: "#6495EDAA",
    textAnnotationForeground: "#EEEEEE",
    tickTextBrush: "#6495ED",
    upBandSeriesFillColor: "white",
    upBandSeriesLineColor: "white",
    upBodyBrush: "#6495EDA0",
    upWickColor: "#6495ED"
}
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
sciChartSurface.applyTheme(customTheme);
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}
```

``` prism-code
import {SciChartSurface, NumericAxis, IThemeProvider} from "scichart";

export async function createCustomThemeTs(divId: string) {
// Create a custom theme by implementing all the properties from IThemeProvider
const customTheme: IThemeProvider = {
    axisBorder: "Transparent",
    axisTitleColor: "#6495ED",
    annotationsGripsBackroundBrush: "white",
    annotationsGripsBorderBrush: "white",
    axis3DBandsFill: "#1F3D6833",
    axisBandsFill: "#1F3D6833",
    axisPlaneBackgroundFill: "Transparent",
    columnFillBrush: "white",
    columnLineColor: "white",
    cursorLineBrush: "#6495ED99",
    defaultColorMapBrush: [
        { offset: 0, color: "DarkBlue" },
        { offset: 0.5, color: "CornflowerBlue" },
        { offset: 1, color: "#FF22AA" }
    ],
    downBandSeriesFillColor: "#52CC5490",
    downBandSeriesLineColor: "#E26565FF",
    downBodyBrush: "white",
    downWickColor: "white",
    gridBackgroundBrush: "white",
    gridBorderBrush: "white",
    labelBackgroundBrush: "#6495EDAA",
    labelBorderBrush: "#6495ED",
    labelForegroundBrush: "#EEEEEE",
    legendBackgroundBrush: "#1D2C35",
    lineSeriesColor: "white",
    loadingAnimationBackground: "#0D213A",
    loadingAnimationForeground: "#6495ED",
    majorGridLineBrush: "#1F3D68",
    minorGridLineBrush: "#102A47",
    mountainAreaBrush: "white",
    mountainLineColor: "white",
    overviewFillBrush: "white",
    planeBorderColor: "white",
    rolloverLineBrush: "#FD9F2533",
    rubberBandFillBrush: "#99999933",
    rubberBandStrokeBrush: "#99999977",
    sciChartBackground: "#0D213A",
    scrollbarBackgroundBrush: "white",
    scrollbarBorderBrush: "white",
    scrollbarGripsBackgroundBrush: "white",
    scrollbarViewportBackgroundBrush: "white",
    scrollbarViewportBorderBrush: "white",
    shadowEffectColor: "white",
    textAnnotationBackground: "#6495EDAA",
    textAnnotationForeground: "#EEEEEE",
    tickTextBrush: "#6495ED",
    upBandSeriesFillColor: "white",
    upBandSeriesLineColor: "white",
    upBodyBrush: "#6495EDA0",
    upWickColor: "#6495ED"
}
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId);
sciChartSurface.applyTheme(customTheme);
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}
```

Results in this output:

<img src="out_scichartv4/2d-charts/styling-and-theming/creating-custom-theme/index_media/56f7152b931bb69e9c72c8aa79b8a09c9c7d29f7.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

![](out_scichartv4/2d-charts/styling-and-theming/creating-custom-theme/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

All colors in SciChart.js are strings, which are HTML color codes. Supported values are 6-digit hex codes e.g. "#ADFF2F", 8-digit hex codes in RGBA format where the last two digits are opacity e.g. "#AAFF2F33" and rgba CSS color codes e.g. "rgba(173, 255, 47, 0.3)"

![](out_scichartv4/2d-charts/styling-and-theming/creating-custom-theme/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

You can see an example live of creating a custom theme over at theÂ <a href="https://www.scichart.com/demo/javascript-chart-custom-themes" rel="noopener noreferrer" target="_blank">SciChart.js Examples Suite</a>.

## Inheriting a Built-In ThemeÂ <a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/#inheriting-a-built-in-theme" class="hash-link" aria-label="Direct link to Inheriting a Built-In ThemeÂ " translate="no" title="Direct link to Inheriting a Built-In ThemeÂ ">â€‹</a>

If you want to create a custom theme built on one of the default themes supplied with SciChart.js, you can use the JavaScript spread operator. For example. Say you wanted to base a theme on SciChartJSLightTheme but change the background to a gradient background and override gridline colours,Â use code like this:

``` prism-code
// Inheriting Themes
import {
    SciChartSurface,
    NumericAxis,
    SciChartJSLightTheme
} from "scichart";

export async function inheritThemeGradientBackground(divId) {
    // Create a theme and inherit / override some properties
    const myTheme = { ...new SciChartJSLightTheme() };

    // Override axis text label
    myTheme.tickTextBrush = "#ff6600";

    // Override gridlines
    myTheme.majorGridLineBrush = "#777";
    myTheme.minorGridLineBrush = "#aaa";

    // Override background with a gradient
    myTheme.sciChartBackground =
        "radial-gradient(circle, #ffffff 0%, #eeeeee 50%, #AAAAAA 100%)";

    // Apply theme
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId, {
        theme: myTheme
    });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}
```

This results in the following custom theme, based off one of our themes:

<img src="out_scichartv4/2d-charts/styling-and-theming/creating-custom-theme/index_media/2c3fe8b9cbaf267415fe05fda70ccb702e776f24.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Chart Styling - Creating a Custom Theme](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/)
- [Chart Styling - Style Chart Parts in Code](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code/)
- [Chart Styling - ThemeManager API](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/)
- [Chart Styling - Image, Transparent or Blurred Backgrounds](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/)
- [Chart Styling - Theming of Wait Loader](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/creating-custom-theme/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/creating-custom-theme/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
