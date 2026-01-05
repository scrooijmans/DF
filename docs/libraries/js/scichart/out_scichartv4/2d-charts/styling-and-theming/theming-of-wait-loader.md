On this page

# Chart Styling - Theming of Wait Loader

When SciChart.js starts up, a short wait-loader is shown as your server downloads Wasm (webassembly) files and asynchronously initializes our high performance WebGL Graphics engine.

The wait time is a one-off cost when the page loads, and once WebAssembly files have been cached by your browser, subsequent loads are very fast.

We show a wait-loader screen in SciChart.js which can be fully customized to match the styling of your app.

## Default Wait Loader Styling<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#default-wait-loader-styling" class="hash-link" aria-label="Direct link to Default Wait Loader Styling" translate="no" title="Direct link to Default Wait Loader Styling">â€‹</a>

The Wait Loader picks up its styling from the theme. Since the default theme isÂ **SciChartJSDarkv2Theme** the wait loader will have a dark background with light foreground.

<img src="out_scichartv4/2d-charts/styling-and-theming/theming-of-wait-loader/index_media/d1d236a33d1deeadece9d12c8f48cb7efd343c95.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Setting Wait Loader Foreground/Background Color<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#setting-wait-loader-foregroundbackground-color" class="hash-link" aria-label="Direct link to Setting Wait Loader Foreground/Background Color" translate="no" title="Direct link to Setting Wait Loader Foreground/Background Color">â€‹</a>

You can customize the wait loader foreground and background. To do this, set the **IThemeProvider.loadingAnimationBackground** and **IThemeProvider.loadingAnimationForeground** properties. This theme must then be passed into the constructor options of SciChartSurface.create, as the loader is shown before asynchronous creation of the chart.

``` prism-code
// Wait loader styling

import {
    SciChartSurface,
    NumericAxis,
    SciChartJSDarkv2Theme
} from "scichart";

// Create a theme based on another theme
const theme = {... new SciChartJSDarkv2Theme()};

// Set loading Animation foreground / background colours
theme.loadingAnimationForeground = "#ff3333"; // Red
theme.loadingAnimationBackground = "#33ff33"; // Green
// Must pass theme to create options on SciChartSurface so it's shown before creation
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ theme });
```

<img src="out_scichartv4/2d-charts/styling-and-theming/theming-of-wait-loader/index_media/ffb217bc30df1f9e0688b44dcb00d5206d44136e.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

## Disabling the Wait Loader entirely<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#disabling-the-wait-loader-entirely" class="hash-link" aria-label="Direct link to Disabling the Wait Loader entirely" translate="no" title="Direct link to Disabling the Wait Loader entirely">â€‹</a>

New to SciChart.js v3.4, you can now disable the wait-loader entirely. To do this, pass *loader: false* toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html#create" rel="noopener noreferrer" target="_blank">SciChartSurface.create()ðŸ“˜</a>.

``` prism-code
// Disable wait-Loader

SciChartSurface.create({ loader: false });
```

## Customizing Wait Loader HTML<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#customizing-wait-loader-html" class="hash-link" aria-label="Direct link to Customizing Wait Loader HTML" translate="no" title="Direct link to Customizing Wait Loader HTML">â€‹</a>

Further customization of the wait loader is possible by implementing a chart loader. You need to create a class which confirms to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html" rel="noopener noreferrer" target="_blank">ISciChartLoaderðŸ“˜</a> interface and implementÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html#addchartloader" rel="noopener noreferrer" target="_blank">addChartLoader()ðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html#removechartloader" rel="noopener noreferrer" target="_blank">removeChartLoader()ðŸ“˜</a> functions. This will let you put anything in the chart loader, for example images, a company logo or animations.

Here's an example below:

- JS
- TS

``` prism-code
 import {
  SciChartSurface,
  NumericAxis,
  SciChartJSDarkv2Theme
} from "SciChart";

export async function waitLoaderThemeing(divId) {
    const loader = new CustomChartLoader();
    const theme = new SciChartJSDarkv2Theme();
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ loader, theme });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}

export class CustomChartLoader {
    addChartLoader(domChartRoot, theme) {
        const loaderContainerDiv = document.createElement("div");
        loaderContainerDiv.style.backgroundColor = "#0F151C";
        loaderContainerDiv.style.height = "100%";
        loaderContainerDiv.style.width = "100%";
        loaderContainerDiv.style.display = "flex";
        loaderContainerDiv.style.justifyContent = "center";
        loaderContainerDiv.style.alignItems = "center";
        const loaderImage = document.createElement("img");
        loaderImage.src = "https://i.giphy.com/media/2WjpfxAI5MvC9Nl8U7/giphy.webp";
        loaderContainerDiv.appendChild(loaderImage);
        const loaderText = document.createElement("div");
        loaderText.style.marginLeft = "auto";
        loaderText.style.marginRight = "auto";
        loaderText.style.float = "left";
        loaderText.style.bottom = "150px";
        loaderText.style.textAlign = "center";
        loaderText.style.position = "absolute";
        loaderText.innerHTML = "Initializing the Awesomeness...";
        loaderText.style.color = "#FF6600";
        loaderText.style.fontFamily = "Arial";
        loaderContainerDiv.appendChild(loaderText);
        domChartRoot.appendChild(loaderContainerDiv);
        return loaderContainerDiv;
    }
    removeChartLoader(domChartRoot, loaderElement) {
        // Remove loader after 2000ms timeout
        setTimeout(() => domChartRoot.removeChild(loaderElement), 100000);
        // For instant removal once scichart has loaded, just call domChartRoot.removeChild(loaderElement) without the setTimeout
        // e.g.
        // domChartRoot.removeChild(loaderElement);
    }
}
```

``` prism-code
 import {
  SciChartSurface,
  NumericAxis,
  ISciChartLoader,
  IThemeProvider,
  SciChartJSDarkv2Theme
} from "SciChart";

export async function waitLoaderThemeingTs(divId: string) {
    const loader = new CustomChartLoader();
    const theme = new SciChartJSDarkv2Theme();
    const { sciChartSurface, wasmContext } = await SciChartSurface.create(divId,{ loader, theme });
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
}

export class CustomChartLoader implements ISciChartLoader {
    public type: "Custom";
    public loadingText: string = "Initializing the Awesomeness...";
    constructor(options?: { loadingText?: string }) {
        this.loadingText = options?.loadingText ?? this.loadingText;
    }
    public addChartLoader(domChartRoot: HTMLDivElement, theme: IThemeProvider): HTMLElement {
        const loaderContainerDiv = document.createElement("div");
        loaderContainerDiv.style.backgroundColor = "#0F151C";
        loaderContainerDiv.style.height = "100%";
        loaderContainerDiv.style.width = "100%";
        loaderContainerDiv.style.display = "flex";
        loaderContainerDiv.style.justifyContent = "center";
        loaderContainerDiv.style.alignItems = "center";
        const loaderImage = document.createElement("img") as HTMLImageElement;
        loaderImage.src = "https://i.giphy.com/media/2WjpfxAI5MvC9Nl8U7/giphy.webp";
        loaderContainerDiv.appendChild(loaderImage);
        const loaderText = document.createElement("div");
        loaderText.style.marginLeft = "auto";
        loaderText.style.marginRight = "auto";
        loaderText.style.float = "left";
        loaderText.style.bottom = "150px";
        loaderText.style.textAlign = "center";
        loaderText.style.position = "absolute";
        loaderText.innerHTML = this.loadingText;
        loaderText.style.color = "#FF6600";
        loaderText.style.fontFamily = "Arial";
        loaderContainerDiv.appendChild(loaderText);
        domChartRoot.appendChild(loaderContainerDiv);
        return loaderContainerDiv;
    }
    public removeChartLoader(domChartRoot: HTMLDivElement, loaderElement: HTMLElement): void {
        // Remove loader after 2000ms timeout
        setTimeout(() => domChartRoot.removeChild(loaderElement), 100000);
        // For instant removal once scichart has loaded, just call domChartRoot.removeChild(loaderElement) without the setTimeout
        // e.g.
        // domChartRoot.removeChild(loaderElement);
    }
}
```

This results in the following output.

<img src="out_scichartv4/2d-charts/styling-and-theming/theming-of-wait-loader/index_media/3eb95cc2a4dceaf155e84fcd43eb803038d62c0b.gif" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

![](out_scichartv4/2d-charts/styling-and-theming/theming-of-wait-loader/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The wait loader accepts HTML into the DOM. You could make stunning wait screens to match your app using Videos, Gifs, Webp /images or animated SVG with a little creativity and input from a UX Designer

## Awaiting multiple charts for synchronized Chart Loaders<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#awaiting-multiple-charts-for-synchronized-chart-loaders" class="hash-link" aria-label="Direct link to Awaiting multiple charts for synchronized Chart Loaders" translate="no" title="Direct link to Awaiting multiple charts for synchronized Chart Loaders">â€‹</a>

![](out_scichartv4/2d-charts/styling-and-theming/theming-of-wait-loader/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Here's a tip if you have multiple charts in your application. Use **Promise.AwaitAll** for the all the **SciChartSurface.create()** calls you make. This will ensure all waitloaders start and end at the same time.

## Conclusion<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#conclusion" class="hash-link" aria-label="Direct link to Conclusion" translate="no" title="Direct link to Conclusion">â€‹</a>

So you can see now that SciChart.js supportsÂ several options for styling the chart wait-loader, to allow charts to fit in with your application theme.

See also our documentation below on styling & themeing, including colouring chart parts and creating custom themes.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/theming-of-wait-loader/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Chart Styling - Creating a Custom Theme](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/creating-custom-theme/)
- [Chart Styling - Image, Transparent or Blurred Backgrounds](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/image-transparent-blurred-backgrounds/)
- [Chart Styling - Style Chart Parts in Code](https://www.scichart.com/documentation/js/v4/2d-charts/styling-and-theming/style-chart-parts-in-code/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/styling-and-theming/theming-of-wait-loader/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/styling-and-theming/theming-of-wait-loader/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
