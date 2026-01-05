On this page

# Tutorial 03 - Modifying Chart Data and Behavior in React

In this tutorial we will show how to modify the **`initChart`** callback in **scichart-react** to pass properties, functions, or data back into **onInit**. This allows you to connect chart behavior to other UI elements in your React application, such as buttons or controls, to manipulate chart data or modify chart state.

The previous tutorial [Tutorial 01 - Understanding the scichart-react boilerplate](https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-01-setting-up-project-with-scichart-react/) should be used as a reference for project setup. Copy the boilerplate to a new folder or project from: <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/BoilerPlates/scichart-react" rel="noopener noreferrer" target="_blank">Boilerplates/scichart-react</a>.

------------------------------------------------------------------------

## Accessing & Controlling the Chart via Nested Components<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#accessing--controlling-the-chart-via-nested-components" class="hash-link" aria-label="Direct link to Accessing &amp; Controlling the Chart via Nested Components" translate="no" title="Direct link to Accessing &amp; Controlling the Chart via Nested Components">â€‹</a>

We will modify **App.jsx** to replace `chartConfig` with an **initChart** function. This function initializes the chart and returns additional functions to control it. These are accessible through **initResult**, which can be used in **onInit** or via **SciChartSurfaceContext**.

### initChart Function<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#initchart-function" class="hash-link" aria-label="Direct link to initChart Function" translate="no" title="Direct link to initChart Function">â€‹</a>

``` prism-code
import {
  SciChartJsNavyTheme,
  SciChartSurface,
  NumericAxis,
  SplineMountainRenderableSeries,
  CursorModifier,
  XyDataSeries,
} from "scichart";
import React, { useContext } from "react";
import { SciChartReact, SciChartSurfaceContext } from "scichart-react";

export const initChart = async (rootElement) => {
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(
    rootElement,
    { theme: new SciChartJsNavyTheme() }
  );

  sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

  const xValues = [0,1,2,3,4,5,6,7,8,9];
  const yValues = [1,4,7,3,7,6,7,4,2,5];

  const mountainSeries = new SplineMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { xValues, yValues }),
    fill: "SteelBlue",
    stroke: "White",
    strokeThickness: 4,
    opacity: 0.4,
  });

  sciChartSurface.renderableSeries.add(mountainSeries);

  const cursor = new CursorModifier({
    showTooltip: true,
    showYLine: true,
    showXLine: true,
    showAxisLabels: true,
    crosshairStroke: "White",
    crosshairStrokeDashArray: [5, 5],
  });
  cursor.isEnabled = false;
  sciChartSurface.chartModifiers.add(cursor);

  const addData = () => {
    const x = xValues.length;
    const y = Math.random() * 10;
    xValues.push(x);
    yValues.push(y);
    mountainSeries.dataSeries.append(x, y);
    sciChartSurface.zoomExtents(500);
  };

  const enableTooltip = (enable) => { cursor.isEnabled = enable; };
  const getTooltipEnabled = () => cursor.isEnabled;

  return { sciChartSurface, addData, enableTooltip, getTooltipEnabled };
};
```

### Nested Components in App.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#nested-components-in-appjsx" class="hash-link" aria-label="Direct link to Nested Components in App.jsx" translate="no" title="Direct link to Nested Components in App.jsx">â€‹</a>

``` prism-code
import React, { useContext } from "react";
import { SciChartReact, SciChartSurfaceContext } from "scichart-react";
import { initChart } from "./initChart";

const AddDataButton = () => {
  const initResult = useContext(SciChartSurfaceContext);
  return <input type="button" value="Add Data" onClick={() => initResult.addData()} />;
};

const EnableTooltipButton = () => {
  const initResult = useContext(SciChartSurfaceContext);
  const handleClick = () => {
    const tooltipEnabled = initResult.getTooltipEnabled();
    initResult.enableTooltip(!tooltipEnabled);
  };
  return <input type="button" value="Toggle Tooltip" onClick={handleClick} />;
};

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>&lt;SciChartReact/&gt; with custom chart controls</h1>
      </header>
      <SciChartReact initChart={initChart} style={{ maxWidth: 900, height: 600 }}>
        <div style={{ display: "flex", justifyContent: "center" }}>
          <AddDataButton />
          <EnableTooltipButton />
        </div>
      </SciChartReact>
    </div>
  );
}

export default App;
```

<img src="out_scichartv4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/index_media/7af17340e65e4770c1beb3f42be24f027612568e.gif" class="img_ev3q" decoding="async" loading="lazy" width="904" height="703" />

------------------------------------------------------------------------

## Accessing the Chart via Non-Nested Components<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#accessing-the-chart-via-non-nested-components" class="hash-link" aria-label="Direct link to Accessing the Chart via Non-Nested Components" translate="no" title="Direct link to Accessing the Chart via Non-Nested Components">â€‹</a>

To access the chart from buttons or UI components outside the `<SciChartReact />` element, you can store **initResult** in React state and share it via Context.

### initChart.js (with modifiers)<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#initchartjs-with-modifiers" class="hash-link" aria-label="Direct link to initChart.js (with modifiers)" translate="no" title="Direct link to initChart.js (with modifiers)">â€‹</a>

``` prism-code
import {
  SciChartJsNavyTheme,
  SciChartSurface,
  NumericAxis,
  SplineMountainRenderableSeries,
  RubberBandXyZoomModifier,
  ZoomPanModifier,
  RolloverModifier,
  XyDataSeries,
  EllipsePointMarker,
  ZoomExtentsModifier,
  MouseWheelZoomModifier,
} from "scichart";

export const initChart = async (rootElement) => {
  const { sciChartSurface, wasmContext } = await SciChartSurface.create(rootElement, { theme: new SciChartJsNavyTheme() });

  sciChartSurface.xAxes.add(new NumericAxis(wasmContext, { axisTitle: "X Axis" }));
  sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { axisTitle: "Y Axis" }));

  const xValues = [0,1,2,3,4,5,6,7,8,9];
  const yValues = [1,4,7,3,7,6,7,4,2,5];

  const mountainSeries = new SplineMountainRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, { dataSeriesName: "Mountain Series", xValues, yValues }),
    fill: "SteelBlue",
    stroke: "White",
    strokeThickness: 4,
    opacity: 0.4,
    pointMarker: new EllipsePointMarker(wasmContext, { width: 7, height: 7, fill: "White", strokeThickness: 0 }),
  });

  mountainSeries.rolloverModifierProps.tooltipTextColor = "#fff";
  mountainSeries.rolloverModifierProps.tooltipColor = "SteelBlue";
  mountainSeries.tooltipLabelX = "X";
  mountainSeries.tooltipLabelY = "Y";
  sciChartSurface.renderableSeries.add(mountainSeries);

  const rolloverModifier = new RolloverModifier({ rolloverLineStroke: "LightSteelBlue", snapToDataPoint: true });
  const rubberBandZoomModifier = new RubberBandXyZoomModifier({ stroke: "#FFFFFF77", fill: "#FFFFFF33", strokeThickness: 1 });
  const zoomPanModifier = new ZoomPanModifier();
  const zoomExtentsModifier = new ZoomExtentsModifier();
  const mouseWheelZoomModifier = new MouseWheelZoomModifier();

  rolloverModifier.isEnabled = false;
  zoomPanModifier.isEnabled = false;
  rubberBandZoomModifier.isEnabled = true;

  sciChartSurface.chartModifiers.add(rolloverModifier, rubberBandZoomModifier, zoomPanModifier, zoomExtentsModifier, mouseWheelZoomModifier);

  return { sciChartSurface, rolloverModifier, zoomPanModifier, rubberBandZoomModifier };
};
```

------------------------------------------------------------------------

### App.jsx with Toolbar & Context<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#appjsx-with-toolbar--context" class="hash-link" aria-label="Direct link to App.jsx with Toolbar &amp; Context" translate="no" title="Direct link to App.jsx with Toolbar &amp; Context">â€‹</a>

``` prism-code
import React, { useState } from "react";
import { SciChartReact } from "scichart-react";
import { ToggleButton } from "./ToggleButton";
import { ChartContext } from "./ChartContext";
import { initChart } from "./initChart";
import "./styles.css";

function App() {
  const [chartState, setChartState] = useState(null);

  return (
    <ChartContext.Provider value={{ chartState, setChartState }}>
      <div className="App">
        <header className="App-header">
          <h1>&lt;SciChartReact/&gt; with custom chart controls</h1>
        </header>
        <div style={{ display: "flex", justifyContent: "left", backgroundColor: "lightgrey", padding: "10px" }}>
          <ToggleButton label="Zoom" modifierKey="rubberBandZoomModifier" />
          <ToggleButton label="Pan" modifierKey="zoomPanModifier" />
          <ToggleButton label="Tooltip" modifierKey="rolloverModifier" />
          <button onClick={() => chartState?.sciChartSurface?.zoomExtents(500)} className="normal-button">Zoom to Fit</button>
        </div>
        <SciChartReact initChart={initChart} onInit={setChartState} style={{ maxWidth: 900, height: 600 }} />
      </div>
    </ChartContext.Provider>
  );
}

export default App;
```

------------------------------------------------------------------------

### ToggleButton.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#togglebuttonjsx" class="hash-link" aria-label="Direct link to ToggleButton.jsx" translate="no" title="Direct link to ToggleButton.jsx">â€‹</a>

``` prism-code
import React, { useContext } from "react";
import { ChartContext } from "./ChartContext";
import "./styles.css";

export const ToggleButton = ({ label, modifierKey }) => {
  const { chartState, setChartState } = useContext(ChartContext);

  const handleClick = () => {
    if (!chartState) return;

    const { sciChartSurface, ...modifiers } = chartState;
    Object.values(modifiers).forEach((modifier) => modifier.isEnabled = false);
    modifiers[modifierKey].isEnabled = true;

    setChartState({ ...chartState });
  };

  return (
    <button onClick={handleClick} className={`toggle-button ${chartState?.[modifierKey]?.isEnabled ? "active" : ""}`}>
      {label}
    </button>
  );
};
```

------------------------------------------------------------------------

### ChartContext.jsx<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#chartcontextjsx" class="hash-link" aria-label="Direct link to ChartContext.jsx" translate="no" title="Direct link to ChartContext.jsx">â€‹</a>

``` prism-code
import { createContext } from "react";

export const ChartContext = createContext(null);
```

------------------------------------------------------------------------

### Styles (styles.css)<a href="https://www.scichart.com/documentation/js/v4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/#styles-stylescss" class="hash-link" aria-label="Direct link to Styles (styles.css)" translate="no" title="Direct link to Styles (styles.css)">â€‹</a>

``` prism-code
.toggle-button {
  background-color: #e0e0e0;
  color: #333;
  border: none;
  padding: 8px 16px;
  border-radius: 2px;
  cursor: pointer;
  font-size: 14px;
  font-weight: bold;
  box-shadow: 0 1px 3px rgba(0,0,0,0.2);
  transition: background 0.2s, color 0.2s, box-shadow 0.2s;
}

.toggle-button.active {
  background-color: #007aff;
  color: white;
  box-shadow: 0 2px 5px rgba(0,122,255,0.5);
}

.normal-button {
  background-color: #e0e0e0;
  color: #333;
  border: none;
  padding: 8px 16px;
  border-radius: 2px;
  cursor: pointer;
  font-size: 14px;
  font-weight: bold;
  box-shadow: 0 2px 5px rgba(0,0,0,0.2);
}

.normal-button:hover {
  background-color: #55aaff;
}
```

To load this CSS, install `style-loader` and `css-loader` and update **webpack.config.js**:

``` prism-code
npm install --save-dev style-loader css-loader
```

Add the rule:

``` prism-code
{
  test: /\.css$/,
  use: ["style-loader", "css-loader"],
}
```

<img src="out_scichartv4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/index_media/7406cdeb79f11363a7ad0d4af68bcb4bba2a9c0e.gif" class="img_ev3q" decoding="async" loading="lazy" width="902" height="706" />

Full source code is available on GitHub: <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Tutorials/React" rel="noopener noreferrer" target="_blank">Tutorials/React/Tutorial_03b_Controlling_Chart_Behaviour_With_Toolbar</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/tutorials-react/tutorial-03-modifying-chart-data-and-behavior-in-react/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
