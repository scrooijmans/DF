On this page

# Tutorial 04 - Adding & Removing Charts To a Group in React

In the previous tutorials we showed you how to use `<SciChartReact/>` `onInit` and React State / Contexts to control chart data and chart state via buttons hosted in your app.

In this tutorial, we're going to show how to **dynamically add and remove chart panels** from a `<SciChartGroup/>` â€” a React component in **scichart-react** which allows you to host several charts together in a group and synchronize zooming, panning, and axis sizes to create dynamic linked dashboards using SciChart.js.

------------------------------------------------------------------------

## Before We Begin<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#before-we-begin" class="hash-link" aria-label="Direct link to Before We Begin" translate="no" title="Direct link to Before We Begin">â€‹</a>

As a basis for this tutorial, use [Tutorial 01 - Understanding the scichart-react boilerplate](https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-01-setting-up-project-with-scichart-react/) for the initial project setup.

Copy the boilerplate to a new folder or project. You can get the code from here: ðŸ‘‰ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/scichart-react" rel="noopener noreferrer" target="_blank">Boilerplates/scichart-react</a>

------------------------------------------------------------------------

## Step 1: Simplify the App Component<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#step-1-simplify-the-app-component" class="hash-link" aria-label="Direct link to Step 1: Simplify the App Component" translate="no" title="Direct link to Step 1: Simplify the App Component">â€‹</a>

Letâ€™s roll back some changes from the previous tutorial. We want our **App.jsx** to be simpler, removing `ToggleButton`, `<SciChartReact/>`, and `ChartContext`.

### App.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#appjsx" class="hash-link" aria-label="Direct link to App.jsx" translate="no" title="Direct link to App.jsx">â€‹</a>

``` prism-code
import React from "react";
import "./styles.css";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>&lt;SciChartReact/&gt; with custom chart controls</h1>
      </header>
      <div
        style={{
          display: "flex",
          justifyContent: "left",
          backgroundColor: "lightgrey",
          padding: "10px",
        }}
      >
        {/* TODO: Add chart controls here */}
      </div>
      {/* TODO: Add SciChartReact here */}
    </div>
  );
}

export default App;
```

------------------------------------------------------------------------

## Step 2: Creating a `<SciChartGroup/>`<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#step-2-creating-a-scichartgroup" class="hash-link" aria-label="Direct link to step-2-creating-a-scichartgroup" translate="no" title="Direct link to step-2-creating-a-scichartgroup">â€‹</a>

We will now create a `<SciChartGroup/>` which can host one or more `<SciChartReact/>` components to make a dashboard or group of charts.

Import the necessary components:

``` prism-code
import { SciChartGroup, SciChartReact } from "scichart-react";
```

Then replace `{/* TODO: Add SciChartReact here */}` with a `<SciChartGroup/>` element containing two `<SciChartReact/>` components.

Each chart will be initialized with a simple `initChart` function.

### App.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#appjsx-1" class="hash-link" aria-label="Direct link to App.jsx" translate="no" title="Direct link to App.jsx">â€‹</a>

``` prism-code
import React from "react";
import "./styles.css";
import { SciChartGroup, SciChartReact } from "scichart-react";
import { SciChartSurface, NumericAxis, SciChartJsNavyTheme } from "scichart";

const simpleChart = async (divElement, chartId) => {
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    divElement,
    {
      title: `Chart ${chartId}`,
      titleStyle: { fontSize: 28 },
      theme: new SciChartJsNavyTheme(),
    }
  );

  sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis" }));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { axisTitle: "Y Axis" }));

  return { sciChartSurface };
};

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>&lt;SciChartReact/&gt; chart groups</h1>
      </header>
      <div
        style={{
          display: "flex",
          justifyContent: "left",
          backgroundColor: "lightgrey",
          padding: "10px",
        }}
      >
        {/* TODO: Add chart controls here */}
      </div>
      <div style={{ height: "600px" }}>
        <SciChartGroup>
          <SciChartReact initChart={(div) => simpleChart(div, 0)} style={{ height: "50%" }} />
          <SciChartReact initChart={(div) => simpleChart(div, 1)} style={{ height: "50%" }} />
        </SciChartGroup>
      </div>
    </div>
  );
}

export default App;
```

------------------------------------------------------------------------

If you run the app now, you should see the following output:

<img src="out_scichartv4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/index_media/f869ace9f471f9c16a0f810b091c427d103f774a.png" class="img_ev3q" decoding="async" loading="lazy" width="1242" height="983" />

> ðŸ’¡ Note: `<SciChartGroup/>` is a non-visual React component. To control the overall size, wrap it in a `<div>` and set the height/width on that container. Individual `<SciChartReact/>` elements can use relative sizes like `"50%"`.

------------------------------------------------------------------------

## Step 3: Dynamically Adding / Removing Charts<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#step-3-dynamically-adding--removing-charts" class="hash-link" aria-label="Direct link to Step 3: Dynamically Adding / Removing Charts" translate="no" title="Direct link to Step 3: Dynamically Adding / Removing Charts">â€‹</a>

Next, letâ€™s make this example **dynamic** by adding toolbar buttons to **Add** and **Remove** charts. Weâ€™ll do this by modifying React state and re-rendering `<SciChartReact/>` components within the `<SciChartGroup/>`.

Start by importing `useState`:

``` prism-code
import React, { useState } from "react";
```

Then create an initial state and functions to add/remove charts.

### App.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#appjsx-2" class="hash-link" aria-label="Direct link to App.jsx" translate="no" title="Direct link to App.jsx">â€‹</a>

``` prism-code
function App() {
  const [charts, setCharts] = useState([0, 1]); // Initialize with 2 charts

  const addChart = () => {
    setCharts([...charts, charts.length]);
  };

  const removeChart = () => {
    if (charts.length > 0) {
      setCharts(charts.slice(0, -1));
    }
  };
```

------------------------------------------------------------------------

## Step 4: Mapping Charts from State<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#step-4-mapping-charts-from-state" class="hash-link" aria-label="Direct link to Step 4: Mapping Charts from State" translate="no" title="Direct link to Step 4: Mapping Charts from State">â€‹</a>

We now map over the `charts` array to create one `<SciChartReact/>` per item. The height of each chart will be dynamically adjusted based on how many charts exist, ensuring the total group height remains fixed at 600px.

### Final App.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#final-appjsx" class="hash-link" aria-label="Direct link to Final App.jsx" translate="no" title="Direct link to Final App.jsx">â€‹</a>

``` prism-code
import React, { useState } from "react";
import "./styles.css";
import { SciChartGroup, SciChartReact } from "scichart-react";
import { SciChartSurface, NumericAxis, SciChartJsNavyTheme } from "scichart";

const simpleChart = async (divElement, chartId) => {
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    divElement,
    {
      title: `Chart ${chartId}`,
      titleStyle: { fontSize: 16 },
      theme: new SciChartJsNavyTheme(),
    }
  );

  sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
      axisTitle: "X Axis",
      axisTitleStyle: { fontSize: 12 },
    })
  );
  sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
      axisTitle: "Y Axis",
      axisTitleStyle: { fontSize: 12 },
    })
  );

  return { sciChartSurface };
};

function App() {
  const [charts, setCharts] = useState([0, 1]); // Start with 2 charts

  const addChart = () => {
    setCharts([...charts, charts.length]);
  };

  const removeChart = () => {
    if (charts.length > 0) {
      setCharts(charts.slice(0, -1));
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>&lt;SciChartReact/&gt; chart groups</h1>
      </header>

      <div
        style={{
          display: "flex",
          justifyContent: "left",
          backgroundColor: "lightgrey",
          padding: "10px",
        }}
      >
        <button onClick={addChart} style={{ margin: "0 10px" }}>
          Add Chart
        </button>
        <button onClick={removeChart} style={{ margin: "0 10px" }}>
          Remove Chart
        </button>
      </div>

      <div style={{ height: "600px" }}>
        <SciChartGroup>
          {charts.map((chartId) => (
            <SciChartReact
              key={chartId}
              initChart={(div) => simpleChart(div, chartId)}
              style={{ height: `${100 / charts.length}%` }}
            />
          ))}
        </SciChartGroup>
      </div>
    </div>
  );
}

export default App;
```

------------------------------------------------------------------------

## Output<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#output" class="hash-link" aria-label="Direct link to Output" translate="no" title="Direct link to Output">â€‹</a>

Now go ahead and run the app. You should see:

<img src="out_scichartv4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/index_media/c63738d6a2b7d166348b965894b422f120bdf885.gif" class="img_ev3q" decoding="async" loading="lazy" width="1059" height="800" />

You can dynamically add or remove charts â€” each one synchronizes with the group layout.

------------------------------------------------------------------------

## Full Source Code<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/#full-source-code" class="hash-link" aria-label="Direct link to Full Source Code" translate="no" title="Direct link to Full Source Code">â€‹</a>

You can find the full example here: ðŸ‘‰ <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Tutorials/React" rel="noopener noreferrer" target="_blank">Tutorials/React/Tutorial_04_Adding_Removing_Syncing_Charts</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-react/tutorial-04-adding-and-removing-charts-to-a-group-in-react/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
