On this page

# Chart Styling - Image, Transparent or Blurred Backgrounds

New to SciChart.js v2.x and above, we now support transparent backgrounds, blurred backgrounds or image backgrounds behind charts.

You can create stunning designs and visual effects with SciChart.js as well as support advanced features such as charting over a background image or tiles of images.

Read on for how to enable this:

## Transparent Backgrounds in Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/#transparent-backgrounds-in-charts" class="hash-link" aria-label="Direct link to Transparent Backgrounds in Charts" translate="no" title="Direct link to Transparent Backgrounds in Charts">â€‹</a>

SciChart.js v2.x and above now properly supports Transparent Backgrounds on charts, allowing you to show through the underlying DOM in your page.

To use this, simply set sciChartSurface.background = Transparent, or haveÂ Transparent in the theme. You can also use semi-transparent backgrounds, e.g. "#FFFFFF33"

``` prism-code
// Transparent background

<!-- HTML -->
<div style="padding: 20px; background: repeating-linear-gradient(45deg, #606dbc, #606dbc 10px,#465298 10px,#465298 20px)">
    <p style="color: white;">This chart has a transparent background</p>
    <div id="scichart-div-id-1" style="width: 800px; height: 600px;"></div>
</div>

// Js
import {
    SciChartSurface,
    NumericAxis,
    SciChartJSLightTheme
} from "scichart";

// Where divId is the ID of the div you wish to place SciChart
export async function transparentBackground(divId) {
    const theme = {... new SciChartJSLightTheme()};
    theme.tickTextBrush = "White";
    // You can set a SciChartSurface background transparent in the theme
    theme.sciChartBackground = "Transparent"
    theme.loadingAnimationBackground = "Transparent";
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ theme });
    // Or you can set it in code
    // SciChart also supports semi-transparent backgrounds like this
    sciChartSurface.background = "#FFFFFF33";
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}
```

This results in the following output:

<img src="out_scichartv4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/index_media/b48f89fb08ea2be19b72c8740d3d0af102682858.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Image Backgrounds in Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/#image-backgrounds-in-charts" class="hash-link" aria-label="Direct link to Image Backgrounds in Charts" translate="no" title="Direct link to Image Backgrounds in Charts">â€‹</a>

As Transparent Backgrounds are now supported, so are image backgrounds. You can place an image background behind a chart using the following code:

``` prism-code
// Transparent background

<!-- HTML -->
<div style="padding: 20px; background-image: url('https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe'); background-size: 100%">
    <p style="color: white;">This chart has an image background</p>
    <div id="scichart-div-id-2" style="width: 800px; height: 600px;"></div>
</div>

// JS
import {
    SciChartSurface,
    NumericAxis,
    SciChartJSLightTheme
} from "scichart";
// Where divId is the ID of the div you wish to place SciChart
export async function imageBackground(divId) {
    // Ensure background transparent to show the image through
    const theme = {... new SciChartJSLightTheme()};
    theme.tickTextBrush = "White";
    theme.sciChartBackground = "Transparent"
    theme.loadingAnimationBackground = "Transparent";
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ theme });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}
```

<img src="out_scichartv4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/index_media/3c0cc1046231ce06d9131bf82264ebb59af1dd96.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Blurred / Glass Effect Backgrounds in Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/#blurred--glass-effect-backgrounds-in-charts" class="hash-link" aria-label="Direct link to Blurred / Glass Effect Backgrounds in Charts" translate="no" title="Direct link to Blurred / Glass Effect Backgrounds in Charts">â€‹</a>

Since Transparent Backgrounds are supported, a few tricks in CSS and you can now achieve blurred / glass effect backgrounds in SciChart.

Try modifying the code above as follows:

``` prism-code
// BLURRED BACKGROUNDS

<!-- HTML -->
<div style="padding: 20px; background-image: url('https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe'); background-size: 100%">
    <p style="color: white;">This chart has a blurred / glass effect background</p>
    <div id="scichart-div-id-3" style="backdrop-filter: blur(15px); width: 800px; height: 600px;"></div>
</div>

// JS
import {
    SciChartSurface
    NumericAxis
    SciChartJSLightTheme
} from "scichart";

// Where divId is the ID of the div you wish to place SciChart
export async function blurredBackground(divId) {
    // Ensure background almost transparent to show the image through
    const theme = {... new SciChartJSLightTheme()};
    theme.tickTextBrush = "White";
    theme.sciChartBackground = "#FFFFFF22"
    theme.loadingAnimationBackground = "#FFFFFF22";
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ theme });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}
```

This results in the following output:

<img src="out_scichartv4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/index_media/7963a7061ecd66ad0454ab4dff580607da999250.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Conclusion<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/#conclusion" class="hash-link" aria-label="Direct link to Conclusion" translate="no" title="Direct link to Conclusion">â€‹</a>

So you can see now that SciChart.js supports a myriad of options for styling the background of the chart to create stunning visual effects.

See also our documentation below on styling & themeing, including colouring chart parts and creating custom themes.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Chart Styling - ThemeManager API](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/)
- [Chart Styling - Creating a Custom Theme](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/)
- [Chart Styling - Style Chart Parts in Code](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
