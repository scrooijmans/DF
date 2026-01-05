On this page

# Creating your first SciChart3DSurface

The root 3D chart view is called theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" rel="noopener noreferrer" target="_blank">SciChart3DSurfaceðŸ“˜</a>. This is the JavaScript chartÂ control you will be adding to your applications wherever you need a 3D chart. You can add more than oneÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" rel="noopener noreferrer" target="_blank">SciChart3DSurfaceðŸ“˜</a> to an HTML page, you can configure them independently.

Let's start by declaring one:

- TS

``` prism-code
// Demonstrates how to create a 3D chart with X,Y,Z axis in SciChart.js
const {
    SciChart3DSurface,
    NumericAxis3D,
    Vector3,
    SciChartJsNavyTheme,
    MouseWheelZoomModifier3D,
    OrbitModifier3D,
    ResetCamera3DModifier
} = SciChart;

// or, for npm, import { SciChart3DSurface, ... } from "scichart"

// Create a SciChart3DSurface in the host <div id=".." />
const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
    // Optional theme
    theme: new SciChartJsNavyTheme(),
    // Optional dimensions of the axis cube (X,Y,Z) in World coordinates
    worldDimensions: new Vector3(300, 200, 300),
    // Optional initial camera position and target
    cameraOptions: {
        position: new Vector3(300, 300, 300),
        target: new Vector3(0, 50, 0)
    }
});

// SciChart.js 3D supports only a single x,y,z axis. Declare your axis like this
sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, { axisTitle: "X Axis" });
sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, { axisTitle: "Y Axis" });
sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, { axisTitle: "Z Axis" });

// Optional: add zooming, panning for the example
sciChart3DSurface.chartModifiers.add(
    new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
    new OrbitModifier3D(), // provides 3d rotation on left mouse drag
    new ResetCamera3DModifier()
); // resets camera position on double-click
```

This results in the following output:

![](out_scichartv4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Click on **Edit in Codepen** above to create a copy of this example in CodePen that you can edit. Note in codepen settings the reference toÂ <a href="https://cdn.jsdelivr.net/npm/scichart/index.min.js" rel="noopener noreferrer" target="_blank">https://cdn.jsdelivr.net/npm/scichart/index.min.js</a>

## Breaking the example down<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/#breaking-the-example-down" class="hash-link" aria-label="Direct link to Breaking the example down" translate="no" title="Direct link to Breaking the example down">â€‹</a>

### Referencing SciChart library & Imports<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/#referencing-scichart-library--imports" class="hash-link" aria-label="Direct link to Referencing SciChart library &amp; Imports" translate="no" title="Direct link to Referencing SciChart library &amp; Imports">â€‹</a>

First need to have the correct scripts and imports. When loading SciChart.js in pure javascript (no npm), this is done by including index.min.js from <a href="https://www.jsdelivr.com/package/npm/scichart" rel="noopener noreferrer" target="_blank">jsdelivr.com/package/npm/scichart</a>Â and declaring constants as follows:

- Pure JS imports for SciChart3D.js

``` prism-code
<script src="https://cdn.jsdelivr.net/npm/scichart/index.min.js"></script>

const {
    SciChart3DSurface,
    NumericAxis3D,
    Vector3,
    SciChartJsNavyTheme,
    MouseWheelZoomModifier3D,
    OrbitModifier3D,
    ResetCamera3DModifier
} = SciChart;
```

If using npm, instead you can import types from "scichart":

- NPM imports for SciChart3D.js

``` prism-code
// npm install scichart

import {
    SciChart3DSurface,
    NumericAxis3D,
    Vector3,
    SciChartJsNavyTheme,
    MouseWheelZoomModifier3D,
    OrbitModifier3D,
    ResetCamera3DModifier
} from "scichart";
```

### Creating the 3D SciChartSurface<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/#creating-the-3d-scichartsurface" class="hash-link" aria-label="Direct link to Creating the 3D SciChartSurface" translate="no" title="Direct link to Creating the 3D SciChartSurface">â€‹</a>

Once you have referenced the library and have the correct imports or constants, you can now use SciChart's API to create a 3D chart surface.

- Creating SciChart3DSurface

``` prism-code
// Create a SciChart3DSurface in the host <div id=".." />
const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
    // Optional theme
    theme: new SciChartJsNavyTheme(),
    // Optional dimensions of the axis cube (X,Y,Z) in World coordinates
    worldDimensions: new Vector3(300, 200, 300),
    // Optional initial camera position and target
    cameraOptions: {
        position: new Vector3(300, 300, 300),
        target: new Vector3(0, 50, 0)
    }
});

// SciChart.js 3D supports only a single x,y,z axis. Declare your axis like this
sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, { axisTitle: "X Axis" });
sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, { axisTitle: "Y Axis" });
sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, { axisTitle: "Z Axis" });
```

This will now show a 3D chart on the screen with default sizing of the X,Y,Z axis and position of the 3D camera.

### Serving Wasm (WebAssembly) Files<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/#serving-wasm-webassembly-files" class="hash-link" aria-label="Direct link to Serving Wasm (WebAssembly) Files" translate="no" title="Direct link to Serving Wasm (WebAssembly) Files">â€‹</a>

At this point you may get an error in the console depending on your environment:

![](out_scichartv4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Could not load SciChart WebAssembly module. Check your build process and ensure that your scichart3d.wasm and scichart3d.js files are from the same version

If so, find out how to resolve this at the pageÂ [Deploying Wasm and Data files](https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm/).

### Adding Optional Zoom & Pan Behaviour<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/#adding-optional-zoom--pan-behaviour" class="hash-link" aria-label="Direct link to Adding Optional Zoom &amp; Pan Behaviour" translate="no" title="Direct link to Adding Optional Zoom &amp; Pan Behaviour">â€‹</a>

The last step, In SciChart.js interactivity is provided by ChartModifiers. These are classes which inherit **ChartModifierBase** which receive events such as mouseDown, mouseMove, mouseUp. All the zooming, panning, tooltip and interaction behaviour in SciChart.js comes from the modifier API which is shared between 2D and 3D SciChart.

You can declare and add some built-in modifiers to add zooming, panning behaviour to the chart. Find these below:

- Adding chart modifiers

``` prism-code
// Optional: add zooming, panning for the example
sciChart3DSurface.chartModifiers.add(
    new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
    new OrbitModifier3D(), // provides 3d rotation on left mouse drag
    new ResetCamera3DModifier()
); // resets camera position on double-click
```

Congratulations! You have just created your first SciChart3DSurface!

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/scichart-3d-basics/scichart-3d-basics-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
