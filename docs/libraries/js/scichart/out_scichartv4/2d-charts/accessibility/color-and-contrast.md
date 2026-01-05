On this page

# Color and Contrast

Since colors and theming are most likely to be custom for each customer, we don't provide out of the box light and dark theme handling (nor special theme for High Contrast).

But that's easily achievable by using one of the provided themes, or creating a custom one.

## Default Themes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/color-and-contrast/#default-themes" class="hash-link" aria-label="Direct link to Default Themes" translate="no" title="Direct link to Default Themes">â€‹</a>

By default SciChart uses **SciChartJSDarkv2Theme**. Also SciChart exposes **SciChartJSDarkTheme** and **SciChartJSLightTheme**.

In this example we will show how to set a desired theme depending on user theme settings.

``` prism-code
// Setting a Theme

import { SciChartJSDarkTheme, SciChartJSLightTheme } from "scichart";
// ...
const isDarkThemeSelected = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
const newColorScheme = isDarkThemeSelected
    ? new SciChartJSDarkTheme() 
    : new SciChartJSLightTheme();
sciChartSurface.applyTheme(newColorScheme);
```

The snippet above should set the light or dark theme depending on user preferences.

It's easy to handle the theme change:

- JS
- TS

``` prism-code
const handleSystemThemeChange = (event) => {
    const newColorScheme = event.matches 
        ? new SciChartJSDarkTheme() 
        : new SciChartJSLightTheme();
    sciChartSurface.applyTheme(newColorScheme)
};
window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", handleSystemThemeChange);
```

``` prism-code
const handleSystemThemeChange = (event: MediaQueryListEvent) => {
    const newColorScheme = event.matches
        ? new SciChartJSDarkTheme()
        : new SciChartJSLightTheme();
    sciChartSurface.applyTheme(newColorScheme);
};
window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", handleSystemThemeChange);
```

Â Now the chart will detect user dark/light theme preference updates and will use an appropriate theme.

## Custom Themes<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/color-and-contrast/#custom-themes" class="hash-link" aria-label="Direct link to Custom Themes" translate="no" title="Direct link to Custom Themes">â€‹</a>

Refer toÂ [Chart Styling - Creating a Custom Theme](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/).

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/color-and-contrast/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Voice Over](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/)
- [Keyboard Accessibility](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/keyboard-accessibility/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/accessibility/color-and-contrast/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/accessibility/color-and-contrast/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
