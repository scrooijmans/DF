On this page

# The Scatter 3D Chart Type

![](out_scichartv4/3d-charts/chart-types/scatter-3d-chart/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Examples for the Scatter 3D Chart can be found in the SciChart.js Demo app which can be viewed on our website, or downloaged from SciChart.Js.Examples Github Repository

- <a href="https://www.scichart.com/demo/javascript-3d-bubble-chart" rel="noopener noreferrer" target="_blank">SciChart.js Demo app</a>
- <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/dev_v4.0" rel="noopener noreferrer" target="_blank">All examples on Github</a>

3D Scatter Charts are provided by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatterrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ScatterRenderableSeries3DðŸ“˜</a> type. This draws a single PointMarker at an X,Y,Z location in the 3D world. Charts can be static or dynamic, and updated in real-time if required.

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/javascript-3d-bubble-chart" target="_blank">3D Bubble Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing how to create a Scatter/Bubble 3D chart with variable size and color of points.

  

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scatterrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ScatterRenderableSeries3DðŸ“˜</a> supports multiple pointmarkers, including:

### 3D Marker Types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/scatter-3d-chart/#3d-marker-types" class="hash-link" aria-label="Direct link to 3D Marker Types" translate="no" title="Direct link to 3D Marker Types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spherepointmarker3d.html" rel="noopener noreferrer" target="_blank">SpherePointMarker3DðŸ“˜</a> - a 3D Sphere at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cubepointmarker3d.html" rel="noopener noreferrer" target="_blank">CubePointMarker3DðŸ“˜</a> - 3D Cube at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pyramidpointmarker3d.html" rel="noopener noreferrer" target="_blank">PyramidPointMarker3DðŸ“˜</a> - a 3D Pyramid at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cylinderpointmarker3d.html" rel="noopener noreferrer" target="_blank">CylinderPointMarker3DðŸ“˜</a> - a 3D Cylinder at each point

### Fast 2D Marker types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/scatter-3d-chart/#fast-2d-marker-types" class="hash-link" aria-label="Direct link to Fast 2D Marker types" translate="no" title="Direct link to Fast 2D Marker types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pixelpointmarker3d.html" rel="noopener noreferrer" target="_blank">PixelPointMarker3DðŸ“˜</a> - a single pixel at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/quadpointmarker.html" rel="noopener noreferrer" target="_blank">QuadPointMarker3DðŸ“˜</a> - a Quad (flat square) facing the camera at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/ellipsepointmarker3d.html" rel="noopener noreferrer" target="_blank">EllipsePointMarker3DðŸ“˜</a> - a flat ellipse facing the camera at each point
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/trianglepointmarker3d.html" rel="noopener noreferrer" target="_blank">TrianglePointMarker3DðŸ“˜</a> - a flat triangle facing the camera at each point

## Declaring a 3D Scatter Series<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/scatter-3d-chart/#declaring-a-3d-scatter-series" class="hash-link" aria-label="Direct link to Declaring a 3D Scatter Series" translate="no" title="Direct link to Declaring a 3D Scatter Series">â€‹</a>

To declare a 3D Scatter Series with PointMarker use the following code:

- TS
- HTML
- CSS

``` prism-code
import {
    SciChart3DSurface,
    NumericAxis3D,
    Vector3,
    SciChartJsNavyTheme,
    ScatterRenderableSeries3D,
    XyzDataSeries3D,
    SpherePointMarker3D,
    MouseWheelZoomModifier3D,
    OrbitModifier3D,
    ResetCamera3DModifier,
    NumberRange
} from "scichart";

const generateData = () => {
    const gaussianRandom = (mean, stdev) => {
        const u = 1 - Math.random(); // Converting [0,1) to (0,1]
        const v = Math.random();
        const z = Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
        // Transform to the desired mean and standard deviation:
        return z * stdev + mean;
    };

    const xValues = [];
    const yValues = [];
    const zValues = [];

    for (let i = 0; i < 1000; i++) {
        xValues.push(gaussianRandom(0, 1));
        yValues.push(gaussianRandom(0, 1));
        zValues.push(gaussianRandom(0, 1));
    }
    return { xValues, yValues, zValues };
};

async function scatter3dChart(divElementId) {
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
        visibleRange: new NumberRange(-3, 3)
    });
    sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, {
        axisTitle: "Y Axis",
        visibleRange: new NumberRange(-3, 3)
    });
    sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, {
        axisTitle: "Z Axis",
        visibleRange: new NumberRange(-3, 3)
    });

    // returns data in arrays of numbers e.g. xValues = [0,1,2,3,4], yValues = [0,1,2,3,4], zValues = [0,1,2,3,4]
    const { xValues, yValues, zValues } = generateData();

    // Add a ScatterRenderableSeries3D
    sciChart3DSurface.renderableSeries.add(
        new ScatterRenderableSeries3D(wasmContext, {
            dataSeries: new XyzDataSeries3D(wasmContext, {
                xValues,
                yValues,
                zValues
            }),
            opacity: 0.5,
            pointMarker: new SpherePointMarker3D(wasmContext, {
                fill: "#EC0F6C",
                size: 10
            })
        })
    );

    // Optional: add zooming, panning for the example
    sciChart3DSurface.chartModifiers.add(
        new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
        new OrbitModifier3D(), // provides 3d rotation on left mouse drag
        new ResetCamera3DModifier()
    ); // resets camera position on double-click
}

scatter3dChart("scichart-root");
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subtitle">Demonstrates a Per-point Color Scatter Chart</p>
        <p class="subTitle">metadata.vertexColor defines color</p>
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

## Coloring Individual Scatter Points<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/scatter-3d-chart/#coloring-individual-scatter-points" class="hash-link" aria-label="Direct link to Coloring Individual Scatter Points" translate="no" title="Direct link to Coloring Individual Scatter Points">â€‹</a>

Scatter points may be colored or scaled individually using the PointMetadata3D API. To do this, set aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" rel="noopener noreferrer" target="_blank">PointMetadata3DðŸ“˜</a> instance with property **vertexColor** at each data-point in the XyzDataSeries3D.

- TS
- HTML
- CSS

``` prism-code
// returns data in arrays of numbers e.g. xValues = [0,1,2,3,4], yValues = [0,1,2,3,4], zValues = [0,1,2,3,4]
const { xValues, yValues, zValues } = generateData();

const colors = ["#EC0F6C", "#F48420", "#DC7969", "#67BDAF", "#50C7E0", "#264B93", "#14233C"];

// Metadata in scichart.js 3D overrides the color of a scatter point. It can also hold additional optional properties
// Below we format the xValues array into a metadata array, where each point is colored individually
const metadata = xValues.map((x, i) => {
    // Return a random colour from the array above
    const color = colors[Math.floor(Math.random() * colors.length)];
    return { vertexColor: parseColorToUIntArgb(color) };
});

// Add a ScatterRenderableSeries3D
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
            size: 7
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

This results in the following output:

![](out_scichartv4/3d-charts/chart-types/scatter-3d-chart/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html" rel="noopener noreferrer" target="_blank">IPointMetadata3DðŸ“˜</a> can be any javascript object but the property **vertexColor** is used to determine scatter 3D datapoint colour. This is in hex format Alpha, Red, Green, Blue, so 0xFFFF0000 would correspond to red. The helper functionÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#parseColorToUIntArgb" rel="noopener noreferrer" target="_blank">parseColorToUIntArgbðŸ“˜</a> can convert Javascript Hex codes to this format.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-types/scatter-3d-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-types/scatter-3d-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
