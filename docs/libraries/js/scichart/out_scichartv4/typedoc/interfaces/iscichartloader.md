<img src="out_scichartv4/typedoc/interfaces/iscichartloader_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ISciChartLoader](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html)

# Interface ISciChartLoader

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to a loader - a class which adds HTML/DOM elements when the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) or [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) is loading webassembly

example  
``` ts
// Define a class which implements ISciChartLoader
class CustomChartLoader implements ISciChartLoader {
   public addChartLoader(domChartRoot: HTMLDivElement, theme: IThemeProvider): HTMLElement {
      const loaderContainerDiv = document.createElement("div");
      loaderContainerDiv.style.backgroundColor = theme.loadingAnimationBackground;
      loaderContainerDiv.style.height = "100%";
      loaderContainerDiv.style.width = "100%";
      const loaderText = document.createElement("p");
      loaderText.innerHTML = "Loading SciChart...";
      loaderText.style.color = theme.loadingAnimationForeground;
      loaderText.style.fontFamily = "Arial";
      loaderText.style.margin = "0";
      loaderText.style.padding = "50px";
      loaderContainerDiv.appendChild(loaderText);
      domChartRoot.appendChild(loaderContainerDiv);
      return loaderContainerDiv;
   }

   public removeChartLoader(domChartRoot: HTMLDivElement, loaderElement: HTMLElement): void {
      domChartRoot.removeChild(loaderElement);
   }
}

// Pass the class to the SciChartSurface.create() function
SciChartSurface.create("elementId", { loader: new CustomChartLoader(); });
```

### Hierarchy

- ISciChartLoader

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultscichartloader.html" class="tsd-signature-type">DefaultSciChartLoader</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html#type" class="tsd-kind-icon">type</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html#addchartloader" class="tsd-kind-icon">addChartLoader</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iscichartloader.html#removechartloader" class="tsd-kind-icon">removeChartLoader</a>

## Properties

### type

type: string

The type name of the loader. For Chart builder and serialization

## Methods

### addChartLoader

- addChartLoader(domChartRoot: HTMLDivElement, theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>): HTMLElement

<!-- -->

- Called when a chart loader is added to the DOM.

  example  
  ``` ts
  public addChartLoader(domChartRoot: HTMLDivElement, theme: IThemeProvider): HTMLElement {
     const loaderContainerDiv = document.createElement("div");
     loaderContainerDiv.style.backgroundColor = theme.loadingAnimationBackground;
     loaderContainerDiv.style.height = "100%";
     loaderContainerDiv.style.width = "100%";
     const loaderText = document.createElement("p");
     loaderText.innerHTML = "Loading SciChart...";
     loaderText.style.color = theme.loadingAnimationForeground;
     loaderText.style.fontFamily = "Arial";
     loaderText.style.margin = "0";
     loaderText.style.padding = "50px";
     loaderContainerDiv.appendChild(loaderText);
     domChartRoot.appendChild(loaderContainerDiv);
     return loaderContainerDiv;
  }
  ```

  #### Parameters

  - ##### domChartRoot: HTMLDivElement

    The root {@link HTMLDivElement} that makes up this [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) or [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  - ##### theme: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ithemeprovider.html" class="tsd-signature-type">IThemeProvider</a>

    The theme applied to the [SciChartSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichartsurface.html) or [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html)

  #### Returns HTMLElement

### removeChartLoader

- removeChartLoader(domChartRoot: HTMLDivElement, loaderElement: HTMLElement): void

<!-- -->

- Called to remove a chart loader from the DOM.

  example  
  ``` ts
  public removeChartLoader(domChartRoot: HTMLDivElement, loaderElement: HTMLElement): void {
     domChartRoot.removeChild(loaderElement);
  }
  ```

  #### Parameters

  - ##### domChartRoot: HTMLDivElement

  - ##### loaderElement: HTMLElement

  #### Returns void

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
