On this page

# The Lines 3D Chart Type

![](out_scichartv4/3d-charts/chart-types/lines-3d-chart/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Examples for the Lines 3D Chart can be found in the SciChart.js Demo app which can be viewed on our website, or downloaded from SciChart.Js.Examples Github Repository

- <a href="https://www.scichart.com/demo/javascript-3d-point-line-chart" rel="noopener noreferrer" target="_blank">SciChart.js Demo app</a>
- <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">All examples on Github</a>

3D Line Charts are provided by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointlinerenderableseries3d.html" rel="noopener noreferrer" target="_blank">PointLineRenderableSeries3DðŸ“˜</a> type. This draws line segments from X,Y,Z data in the 3D world, with an optional point-marker. Charts can be static or dynamic, and updated in real-time if required.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-3d-point-line-chart" target="_blank">3D Point Lines Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing how to draw a Waterfall chart in 3D using SciChart.js

  

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointlinerenderableseries3d.html" rel="noopener noreferrer" target="_blank">PointLineRenderableSeries3DðŸ“˜</a> requires X,Y,Z data to render, stored in anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" rel="noopener noreferrer" target="_blank">XyzDataSeries3DðŸ“˜</a>. This series supports an optional pointmarker of multiple types, including:

### 3D Marker Types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/lines-3d-chart/#3d-marker-types" class="hash-link" aria-label="Direct link to 3D Marker Types" translate="no" title="Direct link to 3D Marker Types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spherepointmarker3d.html" rel="noopener noreferrer" target="_blank">SpherePointMarker3DðŸ“˜</a> - a 3D Sphere at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cubepointmarker3d.html" rel="noopener noreferrer" target="_blank">CubePointMarker3DðŸ“˜</a> - a 3D Cube at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pyramidpointmarker3d.html" rel="noopener noreferrer" target="_blank">PyramidPointMarker3DðŸ“˜</a> - a 3D Pyramid at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cylinderpointmarker3d.html" rel="noopener noreferrer" target="_blank">CylinderPointMarker3DðŸ“˜</a> - a 3D Cylinder at each point

### Fast 2D Marker types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/lines-3d-chart/#fast-2d-marker-types" class="hash-link" aria-label="Direct link to Fast 2D Marker types" translate="no" title="Direct link to Fast 2D Marker types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pixelpointmarker3d.html" rel="noopener noreferrer" target="_blank">PixelPointMarker3DðŸ“˜</a> - a single pixel at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/quadpointmarker.html" rel="noopener noreferrer" target="_blank">QuadPointMarker3DðŸ“˜</a> - a Quad (flat square) facing the camera at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker3d.html" rel="noopener noreferrer" target="_blank">EllipsePointMarker3DðŸ“˜</a> - a flat ellipse facing the camera at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html" rel="noopener noreferrer" target="_blank">TrianglePointMarker3DðŸ“˜</a> - a flat triangle facing the camera at each point

## Declaring a 3D Point-Line Series<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/lines-3d-chart/#declaring-a-3d-point-line-series" class="hash-link" aria-label="Direct link to Declaring a 3D Point-Line Series" translate="no" title="Direct link to Declaring a 3D Point-Line Series">â€‹</a>

To declare a 3D Point-Line Series with use the following code:

- TS
- HTML
- CSS

``` prism-code
// Demonstrates how to create a 3D Lines chart in SciChart.js

// Create a SciChart3DSurface in the host <div id=".." />
const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    worldDimensions: new Vector3(300, 200, 300),
    cameraOptions: {
        position: new Vector3(-300, 300, -300),
        target: new Vector3(0, 50, 0)
    }
});

// Declare your axis like this
sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "X Axis",
    autoRange: EAutoRange.Once
});
sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "Y Axis",
    autoRange: EAutoRange.Once
});
sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "Z Axis",
    autoRange: EAutoRange.Once,
    growBy: new NumberRange(0.2, 0.2)
});

// returns data in arrays of numbers e.g. xValues = [0,1,2,3,4], yValues = [0,1,2,3,4], zValues = [0,1,2,3,4]
const { xValues, yValues, zValues } = generateData(1);

// Add a PointLineRenderableSeries3D
sciChart3DSurface.renderableSeries.add(
    new PointLineRenderableSeries3D(wasmContext, {
        dataSeries: new XyzDataSeries3D(wasmContext, { xValues, yValues, zValues }),
        opacity: 0.9,
        stroke: "#EC0F6C",
        strokeThickness: 3
    })
);

// Repeat 2x
const dataset1 = generateData(2);
sciChart3DSurface.renderableSeries.add(
    new PointLineRenderableSeries3D(wasmContext, {
        dataSeries: new XyzDataSeries3D(wasmContext, {
            xValues: dataset1.xValues,
            yValues: dataset1.yValues,
            zValues: dataset1.zValues
        }),
        opacity: 0.9,
        stroke: "#50C7E0",
        strokeThickness: 3,
        // Pointmarkers are optional. Many different pointmarker types are supported
        pointMarker: new EllipsePointMarker3D(wasmContext, { size: 3 })
    })
);

const dataset2 = generateData(3);
sciChart3DSurface.renderableSeries.add(
    new PointLineRenderableSeries3D(wasmContext, {
        dataSeries: new XyzDataSeries3D(wasmContext, {
            xValues: dataset2.xValues,
            yValues: dataset2.yValues,
            zValues: dataset2.zValues
        }),
        opacity: 0.9,
        stroke: "#F48420",
        strokeThickness: 3
    })
);

// Optional: add zooming, panning for the example
sciChart3DSurface.chartModifiers.add(
    new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
    new OrbitModifier3D(), // provides 3d rotation on left mouse drag
    new ResetCamera3DModifier()
); // resets camera position on double-click
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subtitle">Demonstrates a 3D Lines Series in SciChart.js</p>
        <p class="subTitle">Zoom, pan the chart with the mouse.</p>
    </div>
</div>
```

``` prism-code
body {
    margin: 0;
    font-family: Arial;
}
.wrapper {
    width: 100%;
    height: 100vh;
    position: relative;
}
#scichart-root {
    width: 100%;
    height: 100%;
    position: relative;
}
.titleWrapper {
    position: absolute;
    width: 100%;
    top: 35%;
    text-align: center;
    pointer-events: none;
    color: #ffffffaa;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
```

This results in the following output:Â 

## Coloring Individual Line Segments<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/lines-3d-chart/#coloring-individual-line-segments" class="hash-link" aria-label="Direct link to Coloring Individual Line Segments" translate="no" title="Direct link to Coloring Individual Line Segments">â€‹</a>

Line segments in SciChart.js 3D points may be colored or scaled individually using the PointMetada3D API. To do this, set aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" rel="noopener noreferrer" target="_blank">PointMetadata3DðŸ“˜</a> instance with property **vertexColor** at each data-point in theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" rel="noopener noreferrer" target="_blank">XyzDataSeries3DðŸ“˜</a>.

Colors are in UInt Argb format. For this example below we use the helper functionsÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#parsecolortouintargb" rel="noopener noreferrer" target="_blank">parseColorToUIntArgb()ðŸ“˜</a> to convert from a JavaScript hex code to UInt32, andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#uintargbcolorlerp" rel="noopener noreferrer" target="_blank">uintArgbColorLerp()ðŸ“˜</a> to linearly interpolate two colours.

- TS
- HTML
- CSS

``` prism-code
// returns data in arrays of numbers e.g. xValues = [0,1,2,3,4], yValues = [0,1,2,3,4], zValues = [0,1,2,3,4]
const { xValues, yValues, zValues } = generateData(1);

const colorHigh = parseColorToUIntArgb("#EC0F6C");
const colorLow = parseColorToUIntArgb("#30BC9A");
const yMin = Math.min(...yValues);
const yMax = Math.max(...yValues);
const metadata = yValues.map((y, i) => {
  // interpolate y between colorLow and colorHigh using the helper function uintArgbColorLerp
  const t = (y - yMin) / (yMax - yMin);
  const color = uintArgbColorLerp(colorLow, colorHigh, t);
  return { vertexColor: color }
});

// Add a PointLineRenderableSeries3D
sciChart3DSurface.renderableSeries.add(new PointLineRenderableSeries3D(wasmContext, {
  dataSeries: new XyzDataSeries3D(wasmContext, { xValues, yValues, zValues, metadata }),
  opacity: 0.9,
  // When metadata colors are provided, stroke is ignored
  stroke: "#EC0F6C",
  strokeThickness: 3,
  // pointMarkers are optional
  pointMarker: new EllipsePointMarker3D(wasmContext, { size: 3 })
}));
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subtitle">Paletted 3D Lines Series in SciChart.js</p>
        <p class="subTitle">Zoom, pan the chart with the mouse.</p>
    </div>
</div>
```

``` prism-code
body {
    margin: 0;
    font-family: Arial;
}
.wrapper {
    width: 100%;
    height: 100vh;
    position: relative;
}
#scichart-root {
    width: 100%;
    height: 100%;
    position: relative;
}
.titleWrapper {
    position: absolute;
    width: 100%;
    top: 35%;
    text-align: center;
    pointer-events: none;
    color: #ffffffaa;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
```

This results in the following output:Â 

![](out_scichartv4/3d-charts/chart-types/lines-3d-chart/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" rel="noopener noreferrer" target="_blank">IPointMetadata3DðŸ“˜</a>Â can be any javascript object but the property **vertexColor** is used to determine scatter 3D datapoint colour. This is in hex format Alpha, Red, Green, Blue, so 0xFFFF0000 would correspond to red.Â The helper functionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#parseColorToUIntArgb" rel="noopener noreferrer" target="_blank">parseColorToUIntArgbðŸ“˜</a> can convert Javascript Hex codes to this format.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-types/lines-3d-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-types/lines-3d-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
