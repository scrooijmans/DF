On this page

# The Bubble 3D Chart Type

![](out_scichartv4/3d-charts/chart-types/bubble-3d-chart/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Examples for the Scatter/Bubble 3D Chart can be found in the SciChart.js Demo app which can be viewed on our website, or downloaded from SciChart.Js.Examples Github Repository

- <a href="https://www.scichart.com/demo/javascript-3d-bubble-chart" rel="noopener noreferrer" target="_blank">SciChart.js Demo app</a>
- <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">All examples on Github</a>

3D Bubble Charts are provided by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatterrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ScatterRenderableSeries3DðŸ“˜</a> type. This draws a single PointMarker at an X,Y,Z location in the 3D world with a per-point scaling factor.Â Charts can be static or dynamic, and updated in real-time if required.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-3d-bubble-chart" target="_blank">3D Bubble Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing how to create 3D Bubble/Scatter charts with variable size and color of points.

  

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatterrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ScatterRenderableSeries3DðŸ“˜</a> allows creation of 3D Bubble charts and supports multiple pointmarkers, including:

### 3D Marker Types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/bubble-3d-chart/#3d-marker-types" class="hash-link" aria-label="Direct link to 3D Marker Types" translate="no" title="Direct link to 3D Marker Types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spherepointmarker3d.html" rel="noopener noreferrer" target="_blank">SpherePointMarker3DðŸ“˜</a> - a 3D Sphere at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cubepointmarker3d.html" rel="noopener noreferrer" target="_blank">CubePointMarker3DðŸ“˜</a> - 3D Cube at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pyramidpointmarker3d.html" rel="noopener noreferrer" target="_blank">PyramidPointMarker3DðŸ“˜</a> - a 3D Pyramid at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cylinderpointmarker3d.html" rel="noopener noreferrer" target="_blank">CylinderPointMarker3DðŸ“˜</a> - a 3D Cylinder at each point

### Fast 2D Marker types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/bubble-3d-chart/#fast-2d-marker-types" class="hash-link" aria-label="Direct link to Fast 2D Marker types" translate="no" title="Direct link to Fast 2D Marker types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pixelpointmarker3d.html" rel="noopener noreferrer" target="_blank">PixelPointMarker3DðŸ“˜</a> - a single pixel at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/quadpointmarker.html" rel="noopener noreferrer" target="_blank">QuadPointMarker3DðŸ“˜</a> - a Quad (flat square) facing the camera at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker3d.html" rel="noopener noreferrer" target="_blank">EllipsePointMarker3DðŸ“˜</a> - a flat ellipse facing the camera at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html" rel="noopener noreferrer" target="_blank">TrianglePointMarker3DðŸ“˜</a> - a flat triangle facing the camera at each point

## Declaring a 3D Bubble Series with custom Sizes & Colors<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/bubble-3d-chart/#declaring-a-3d-bubble-series-with-custom-sizes--colors" class="hash-link" aria-label="Direct link to Declaring a 3D Bubble Series with custom Sizes &amp; Colors" translate="no" title="Direct link to Declaring a 3D Bubble Series with custom Sizes &amp; Colors">â€‹</a>

To declare a 3D Bubble Series with individual sizes & colors, use the following code.

- TS
- HTML
- CSS

``` prism-code
// returns data in arrays of numbers e.g. xValues = [0,1,2,3,4], yValues = [0,1,2,3,4], zValues = [0,1,2,3,4]
const { xValues, yValues, zValues } = generateData();

const colors = ["#EC0F6C", "#F48420", "#DC7969", "#67BDAF", "#50C7E0", "#264B93", "#14233C"];

// Metadata in scichart.js 3D overrides the color of a scatter point. It can also hold additional optional properties
// Below we format the xValues array into a metadata array, where each point is colored individually
const metadata = [];
for (let i = 0; i < xValues.length; i++) {
    const { x, y, z } = { x: xValues[i], y: yValues[i], z: zValues[i] };
    // Compute a scale factor based on distance from origin
    const distanceFromOrigin = Math.sqrt(x * x + y * y + z * z);
    const scaleFactor = 1 - distanceFromOrigin / 3;

    // Return a random colour from the array above
    const color = colors[Math.floor(Math.random() * colors.length)];

    console.log(`Point ${i} has scale factor ${scaleFactor} and color ${color}`);

    // Return IPointMetadat3D with pointScale and vertexColor properties
    metadata.push({
        vertexColor: parseColorToUIntArgb(color),
        pointScale: scaleFactor
    });
}

// Add a ScatterRenderableSeries3D configured as bubble chart
sciChart3DSurface.renderableSeries.add(
    new ScatterRenderableSeries3D(wasmContext, {
        dataSeries: new XyzDataSeries3D(wasmContext, {
            xValues,
            yValues,
            zValues,
            metadata // Optional metadata here. Property vertexColor is read to color the point
        }),
        // When metadata colours are provided, the pointMarker.fill is ignored
        pointMarker: new SpherePointMarker3D(wasmContext, {
            size: 25
        })
    })
);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subtitle">Demonstrates a Gaussian Distribution Scatter Chart</p>
        <p class="subTitle">Drag the mouse to rotate, use MouseWheel to zoom</p>
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
    color: #ffffff77;
}
.title {
    font-size: 20px;
}
.subTitle {
    font-size: 16px;
}
```

This results in the following output:Â 

![](out_scichartv4/3d-charts/chart-types/bubble-3d-chart/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" rel="noopener noreferrer" target="_blank">IPointMetadata3DðŸ“˜</a> can be any javascript object but the property **vertexColor** is used to determine scatter 3D datapoint colour. This is in hex format Alpha, Red, Green, Blue, so 0xFFFF0000 would correspond to red. The helper functionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#parseColorToUIntArgb" rel="noopener noreferrer" target="_blank">parseColorToUIntArgbðŸ“˜</a> can convert Javascript Hex codes to this format.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-types/bubble-3d-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-types/bubble-3d-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
