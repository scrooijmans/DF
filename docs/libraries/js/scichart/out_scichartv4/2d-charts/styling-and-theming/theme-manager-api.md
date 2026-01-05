On this page

# Chart Styling - ThemeManager API

SciChart ships with a light and dark theme out of the box, which you can select and apply to the charts in your application. Most of the components of SciChart are alsoÂ stylable, and you canÂ [create your own themes](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme), so you can truly customize the chart to fit your application.

You can view our themes live at theÂ <a href="https://www.scichart.com/demo/javascript-chart-themes" rel="noopener noreferrer" target="_blank">ThemeManager example, over at the SciChart.js Examples Suite</a>.

## SciChart Dark Theme<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/#scichart-dark-theme" class="hash-link" aria-label="Direct link to SciChart Dark Theme" translate="no" title="Direct link to SciChart Dark Theme">â€‹</a>

SciChart had a dark theme before dark-mode was coolÂ :) Here's ourÂ default theme, SciChart Dark,Â in all it's glory below.

- Applying dark theme

``` prism-code
import { SciChartSurface, SciChartJSDarkv2Theme } from "scichart";

// For best results & applying to the loader animation, apply theme before chart creation
const { wasmContext, sciChartSurface } = await SciChartSurface.create("div-element-id", { 
    theme: new SciChartJSDarkv2Theme() 
});

// You can also change the theme after creation
sciChartSurface.applyTheme(new SciChartJSDarkv2Theme());
```

<img src="out_scichartv4/2d-charts/styling-and-theming/theme-manager-api/index_media/a8802c6bf5b5a97b9f1466b254dc4c8694144709.png" class="img_ev3q" decoding="async" loading="lazy" width="797" height="600" />

## SciChart Light Theme<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/#scichart-light-theme" class="hash-link" aria-label="Direct link to SciChart Light Theme" translate="no" title="Direct link to SciChart Light Theme">â€‹</a>

For applications with a white or lighter background color, we also ship a light theme. This is how it looks:

- Applying light Theme

``` prism-code
import { SciChartSurface, SciChartJSLightTheme } from "scichart";

// For best results & applying to the loader animation, apply theme before chart creation
const { wasmContext, sciChartSurface } = await SciChartSurface.create("div-element-id", { 
    theme: new SciChartJSLightTheme() 
});

// You can also change the theme after creation
sciChartSurface.applyTheme(new SciChartJSLightTheme());
```

<img src="out_scichartv4/2d-charts/styling-and-theming/theme-manager-api/index_media/804bb7e0ed8504bf8922058273c99c58caa619d7.png" class="img_ev3q" decoding="async" loading="lazy" width="798" height="600" />

## SciChart Navy Theme<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/#scichart-navy-theme" class="hash-link" aria-label="Direct link to SciChart Navy Theme" translate="no" title="Direct link to SciChart Navy Theme">â€‹</a>

In SciChart.js v3, we've added a new Navy theme. This looks great on both a light & dark background.Â This can be enabled as follows:

- Applying Navy Theme

``` prism-code
import { SciChartSurface, SciChartJsNavyTheme } from "scichart";

// For best results & applying to the loader animation, apply theme before chart creation
const { wasmContext, sciChartSurface } = await SciChartSurface.create("div-element-id", { 
    theme: new SciChartJsNavyTheme() 
});

// Changing theme after creation
sciChartSurface.applyTheme(new SciChartJsNavyTheme());
```

<img src="out_scichartv4/2d-charts/styling-and-theming/theme-manager-api/index_media/4035119b4dc739466c9083bf9c28334f1e602b26.png" class="img_ev3q" decoding="async" loading="lazy" width="910" height="567" />

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theme-manager-api/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Chart Styling - Creating a Custom Theme](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme)
- [Chart Styling - Style Chart Parts in Code](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/theme-manager-api/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/theme-manager-api/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
