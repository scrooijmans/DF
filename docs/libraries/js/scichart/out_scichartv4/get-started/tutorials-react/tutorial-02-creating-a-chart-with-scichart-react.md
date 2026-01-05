On this page

# Tutorial 02 - Creating a Chart with scichart-react

In this tutorial, we'll show how to create a JavaScript chart in React using **scichart.js** and **scichart-react**.

The previous tutorial [Tutorial 01 - Understanding the scichart-react boilerplate](https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-01-setting-up-project-with-scichart-react/) serves as a reference. We'll use that boilerplate (npm setup, webpack config, package.json) as a starting point.

Copy the boilerplate to a new folder or project. You can get the code from: <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/scichart-react" rel="noopener noreferrer" target="_blank">Boilerplates/scichart-react</a>.

------------------------------------------------------------------------

## The `<SciChartReact />` React Component<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/#the-scichartreact--react-component" class="hash-link" aria-label="Direct link to the-scichartreact--react-component" translate="no" title="Direct link to the-scichartreact--react-component">â€‹</a>

### Props and Configuration<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/#props-and-configuration" class="hash-link" aria-label="Direct link to Props and Configuration" translate="no" title="Direct link to Props and Configuration">â€‹</a>

`<SciChartReact />` has the following props:

| Name | Description |
|----|----|
| **fallback** | ReactNode rendered while the chart is initializing |
| **onInit** | Callback function after the chart is initialized |
| **onDelete** | Callback function when the chart is unmounted |
| **innerContainerProps** | Props passed to the inner container `<div>` hosting the chart |
| **initChart** | Function accepting an HTMLDivElement and returning a SciChartSurface instance |
| **config** | Chart definition as a JSON object or string compatible with the Builder API |
| **style** | Optional styles applied to the outer `<div>` |

At a minimum, pass either **initChart** (programmatic JS API) or **config** (Builder API) to `<SciChartReact />`.

You can also specify **fallback**, **onInit**, and **onDelete**, and style the chart via **style** and **innerContainerProps**.

------------------------------------------------------------------------

### DOM Outputted by `<SciChartReact />`<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/#dom-outputted-by-scichartreact-" class="hash-link" aria-label="Direct link to dom-outputted-by-scichartreact-" translate="no" title="Direct link to dom-outputted-by-scichartreact-">â€‹</a>

Each `<SciChartReact />` outputs three `<div>` elements:

``` prism-code
<div> <!-- outer div, style applied here -->
  <div> <!-- innerContainerProps applied here -->
    <div id="scichart-root"> <!-- Chart hosted here -->
    </div>
  </div>
</div>
```

Example usage:

``` prism-code
<SciChartReact
  initChart={initChartFunc} // (divElement) => { return { sciChartSurface }; }
  onInit={onInitFunc} // (initResult) => console.log(`surface: ${initResult.sciChartSurface.id}`);
  onDelete={onDeleteFunc} // (initResult) => console.log(`surface: ${initResult.sciChartSurface.id}`);
  innerContainerProps={{ style: { width: "100%" } }}
  style={{ maxWidth: 900, height: 600 }}
/>
```

------------------------------------------------------------------------

## Creating a scichart-react Chart Using `initChart`<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/#creating-a-scichart-react-chart-using-initchart" class="hash-link" aria-label="Direct link to creating-a-scichart-react-chart-using-initchart" translate="no" title="Direct link to creating-a-scichart-react-chart-using-initchart">â€‹</a>

Copy the <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/scichart-react" rel="noopener noreferrer" target="_blank">Boilerplates/scichart-react</a> folder into a new project. Change your `App.jsx` as follows:

- App.jsx

``` prism-code
import React from "react";
import { SciChartReact } from "scichart-react";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>&lt;SciChartReact/&gt; with initChart Tutorial</h1>
      </header>
      <SciChartReact
        initChart={initChart}
        onInit={onInit}
        onDelete={onDelete}
        innerContainerProps={{ style: { width: "100%" } }}
        style={{ maxWidth: 900, height: 600 }}
      />
    </div>
  );
}

export default App;
```

The `<SciChartReact />` component renders a single chart. **initChart** sets up the chart, **onInit**/**onDelete** handle mount/unmount events, and **innerContainerProps**/**style** control the layout.

------------------------------------------------------------------------

### Initialization Code<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/#initialization-code" class="hash-link" aria-label="Direct link to Initialization Code" translate="no" title="Direct link to Initialization Code">â€‹</a>

- TS

``` prism-code
import {
  SciChartSurface,
  SciChartJsNavyTheme,
  NumericAxis,
  FastLineRenderableSeries,
  XyDataSeries,
  EllipsePointMarker,
  ZoomPanModifier,
  MouseWheelZoomModifier,
  ZoomExtentsModifier,
  SweepAnimation,
} from "scichart";

const initChart = async (rootElement) => {
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(rootElement, {
    id: "New SciChart Chart",
    theme: new SciChartJsNavyTheme(),
    title: "SciChart-React with initChart",
    titleStyle: { fontSize: 16, color: "White" },
  });

  sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis" }));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { axisTitle: "Y Axis" }));

  sciChartSurface.renderableSeries.add(
    new FastLineRenderableSeries(wasmContext, {
      dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        yValues: [0, 0.0998, 0.1986, 0.2955, 0.3894, 0.4794, 0.5646, 0.6442, 0.7173, 0.7833],
      }),
      stroke: "SteelBlue",
      pointMarker: new EllipsePointMarker(wasmContext, {
        fill: "LightSteelBlue",
        stroke: "White",
        size: 9,
      }),
      animation: new SweepAnimation({ duration: 750 }),
    })
  );

  sciChartSurface.chartModifiers.add(
    new ZoomPanModifier({ enableZoom: true }),
    new MouseWheelZoomModifier(),
    new ZoomExtentsModifier()
  );

  return { sciChartSurface };
};

const onInit = (initResult) => {
  const sciChartSurface = initResult.sciChartSurface;
  console.log(`SciChartSurface initialized: id=${sciChartSurface.id}`);
};

const onDelete = (initResult) => {
  const sciChartSurface = initResult.sciChartSurface;
  console.log(`SciChartSurface deleted: id=${sciChartSurface.id} isDeleted=${sciChartSurface.isDeleted}`);
};
```

------------------------------------------------------------------------

### Running the Code<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/#running-the-code" class="hash-link" aria-label="Direct link to Running the Code" translate="no" title="Direct link to Running the Code">â€‹</a>

<img src="out_scichartv4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/index_media/5a1108319d099189418365dded21857e7aa4d3de.png" class="img_ev3q" decoding="async" loading="lazy" width="1152" height="863" />

The full source code is available at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Tutorials/React" rel="noopener noreferrer" target="_blank">SciChart.JS.Examples Tutorials/React/Tutorial_02</a>.

------------------------------------------------------------------------

This version:

- Uses Docusaurus-friendly code blocks and `<CodeSnippetBlock>` for JS/TS snippets.
- Replaces raw HTML `<div>` descriptions with code blocks.
- Adds `tip` and `warning` callouts.
- Keeps headings structured for readability.
- Cleans up redundant text and fixes grammar.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-react/tutorial-02-creating-a-chart-with-scichart-react/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
