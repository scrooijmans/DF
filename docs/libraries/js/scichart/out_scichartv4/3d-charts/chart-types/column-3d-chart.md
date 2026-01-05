On this page

# The Column 3D Chart Type

JavaScript 3D Column Charts can be created using the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ColumnRenderableSeries3DðŸ“˜</a> type. This chart type draws columns or bars from X,Y,Z data in the 3D world, with a pointmarker denoting the shape of the column. Column 3D Charts can be static or dynamic, and updated in real-time if required.

![](out_scichartv4/3d-charts/chart-types/column-3d-chart/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The <a href="https://www.scichart.com/demo/react/3d-column-chart" rel="noopener noreferrer" target="_blank">JavaScript / React 3D Column Chart Example</a> can be found in theÂ <a href="https://github.com/abtsoftware/scichart.js.examples" rel="noopener noreferrer" target="_blank">SciChart.Js Examples Suite</a>Â on Github, or our live demo atÂ <a href="https://www.scichart.com/demo/" rel="noopener noreferrer" target="_blank">scichart.com/demo</a>

Above: The JavaScript <a href="https://www.scichart.com/demo/iframe/3d-column-chart" target="_blank">3D Column Chart</a> example from the <a href="https://www.scichart.com/demo/react" target="_blank">SciChart.js Demo</a> showing a variety of line options in SciChart.js.

  

## Create aÂ 3D Column Chart<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/column-3d-chart/#create-a3d-column-chart" class="hash-link" aria-label="Direct link to Create aÂ 3D Column Chart" translate="no" title="Direct link to Create aÂ 3D Column Chart">â€‹</a>

To declare a 3D Column Chart in JavaScript, use the following code:

- TS
- HTML
- CSS

``` prism-code
// Demonstrates how to create a 3D Column chart in SciChart.js

// Create a SciChart3DSurface in the host <div id=".." />
const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    worldDimensions: new Vector3(300, 200, 300),
    cameraOptions: {
        position: new Vector3(-270, 230, -160),
        target: new Vector3(0, 50, 0)
    }
});

// Declare your axis like this
sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "X Axis"
});
sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "Y Axis"
});
sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, {
    axisTitle: "Z Axis"
});

// Create a 2D array and fill this with data. returns 2D array [zIndex][xIndex]
const heightmapArray = generateData(25, 25);

// Unwrap into 3x 1D arrays for xValues, yValues, zValues
const xValues = [];
const zValues = [];
const yValues = heightmapArray.flatMap((row, xIndex) => {
    row.forEach((_, zIndex) => {
        xValues.push(xIndex); // X corresponds to the row index
        zValues.push(zIndex); // Z corresponds to the column index
    });
    return row; // Flattened Z-values for each row
});

// Declare an XyzDataSeries3D passing in the x,y,zValues which specify 3d positions
// of columns.
// The xValues, zValues provide the position on the floor plane.
// The yValues provide the heights of columns
const dataSeries = new XyzDataSeries3D(wasmContext, {
    xValues,
    yValues,
    zValues,
    dataSeriesName: "Column Series 3D"
});

// Add the 3D column series to the chart
sciChart3DSurface.renderableSeries.add(
    new ColumnRenderableSeries3D(wasmContext, {
        dataSeries,
        fill: "#F48420",
        dataPointWidthX: 0.5,
        dataPointWidthZ: 0.5,
        pointMarker: new CubePointMarker3D(wasmContext)
    })
);

// Optional: add zooming, panning for the example
sciChart3DSurface.chartModifiers.add(
    new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
    new OrbitModifier3D(), // provides 3d rotation on left mouse drag
    new ResetCamera3DModifier() // resets camera position on double-click
);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Column Chart Example</p>
        <p class="subtitle">Demonstrates 3D Column Series in SciChart.js</p>
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

Breaking this code down:

We initialize a 3D chart by callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create" rel="noopener noreferrer" target="_blank">SciChart3DSurface.create()ðŸ“˜</a>. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#worlddimensions" rel="noopener noreferrer" target="_blank">worldDimensionsðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html" rel="noopener noreferrer" target="_blank">cameraOptionsðŸ“˜</a> are passed toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#create" rel="noopener noreferrer" target="_blank">SciChart3DSurface.create()ðŸ“˜</a> to initialize the 3D scene.

An xAxis, yAxis and zAxis are declared of typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/numericaxis3d.html" rel="noopener noreferrer" target="_blank">NumericAxis3DðŸ“˜</a>.

Additional code is added to generate some data which is unwrapped into 1D arrays of X,Y and ZValues. These will specify the discrete 3D points on the column chart: X-Z providing the position and Y value providing the height. Data is passed into aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" rel="noopener noreferrer" target="_blank">XyzDataSeries3DðŸ“˜</a> which is the data source for the 3D column chart.

AÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ColumnRenderableSeries3DðŸ“˜</a> is created and added to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#renderableseries" rel="noopener noreferrer" target="_blank">SciChart3DSurface.renderableSeriesðŸ“˜</a> collection. We set theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#datapointwidthx" rel="noopener noreferrer" target="_blank">dataPointWidthðŸ“˜</a> properties to define the size of the 3D bar as well as theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#fill" rel="noopener noreferrer" target="_blank">fillðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#pointmarker" rel="noopener noreferrer" target="_blank">pointMarkerðŸ“˜</a> properties to define the colour and type (shape) of the column.

This results in the following output:

## Choosing a Column / Bar Type<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/column-3d-chart/#choosing-a-column--bar-type" class="hash-link" aria-label="Direct link to Choosing a Column / Bar Type" translate="no" title="Direct link to Choosing a Column / Bar Type">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html" rel="noopener noreferrer" target="_blank">ColumnRenderableSeries3DðŸ“˜</a> requires X,Y,Z data to render, stored in anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" rel="noopener noreferrer" target="_blank">XyzDataSeries3DðŸ“˜</a>. This series supports an optional pointmarker of multiple types, including:

### 3D Marker Types<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/column-3d-chart/#3d-marker-types" class="hash-link" aria-label="Direct link to 3D Marker Types" translate="no" title="Direct link to 3D Marker Types">â€‹</a>

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/spherepointmarker3d.html" rel="noopener noreferrer" target="_blank">SpherePointMarker3DðŸ“˜</a> - a 3D Sphere represents each bar/column
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cubepointmarker3d.html" rel="noopener noreferrer" target="_blank">CubePointMarker3DðŸ“˜</a> - a 3D Cube represents each bar/column
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pyramidpointmarker3d.html" rel="noopener noreferrer" target="_blank">PyramidPointMarker3DðŸ“˜</a> - a 3D Pyramid represents each bar/column
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cylinderpointmarker3d.html" rel="noopener noreferrer" target="_blank">CylinderPointMarker3DðŸ“˜</a> - a 3D Cylinder represents each bar/column

Changing theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icolumnrenderableseries3doptions.html#pointmarker" rel="noopener noreferrer" target="_blank">ColumnRenderableSeries3D.pointMarkerðŸ“˜</a> property will update the type / shape of object used to denote a column.

## Colouring Individual Columns<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-types/column-3d-chart/#colouring-individual-columns" class="hash-link" aria-label="Direct link to Colouring Individual Columns" translate="no" title="Direct link to Colouring Individual Columns">â€‹</a>

By default, the colour of the Column series is defined byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#fill" rel="noopener noreferrer" target="_blank">ColumnRenderableSeries.fillðŸ“˜</a>. This can be overridden on a per-column basis using theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html#metadata" rel="noopener noreferrer" target="_blank">metadataðŸ“˜</a> array passed toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" rel="noopener noreferrer" target="_blank">XyzDataSeries3DðŸ“˜</a>.

Update your code sample as follows:

- TS
- HTML
- CSS

``` prism-code
// Create a SciChart3DSurface in the host <div id=".." />
const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(
  divElementId,
  {
    theme: new SciChartJsNavyTheme(),
    worldDimensions: new Vector3(300, 200, 300),
    cameraOptions: {
      position: new Vector3(-270, 230, -160),
      target: new Vector3(0, 50, 0),
    },
  }
);

// Declare your axis like this
sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, {
  axisTitle: "X Axis",
});
sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, {
  axisTitle: "Y Axis",
});
sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, {
  axisTitle: "Z Axis",
});

// Metadata colours are in ARGB format, e.g. 0xFFFF0000 is red, 0xFF00FF00 is green
// The helper function parseColorToUIntArgb can be used to convert HTML colors to ARGB
const htmlColors = [
  "#EC0F6C",
  "orange",
  "#67BDAF",
  "SteelBlue",
  "#14233C",
  "#67BDAF",
  "orange",
  "#F48420",
  "SteelBlue",
  "#EC0F6C",
  "#EC0F6C",
  "#67BDAF",
  "#67BDAF",
  "SteelBlue",
  "DarkBlue",
  "#F48420",
  "orange",
  "#67BDAF",
  "SteelBlue",
  "#67BDAF",
];

// Declare a dataSeries with xValues, yValues (heights), zValues
// Metadata can be any javascript object
// metadata.vertexColor is the UINT ARGB color of the column
const dataSeries = new XyzDataSeries3D(wasmContext, {
  xValues: [0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4],
  yValues: [0, 1, 2, 3, 4, 3, 2, 4, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5],
  zValues: [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
  ],
  metadata: htmlColors.map((c) => ({ vertexColor: parseColorToUIntArgb(c) })),
});

sciChart3DSurface.renderableSeries.add(
  new ColumnRenderableSeries3D(wasmContext, {
    dataSeries,
    dataPointWidthX: 0.7,
    dataPointWidthZ: 0.7,
    // Per column coloring using metadata
    useMetadataColors: true,
    pointMarker: new CylinderPointMarker3D(wasmContext),
  })
);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Column Chart Example</p>
        <p class="subtitle">Demonstrates 3D Column Series in SciChart.js</p>
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

This example also shows a variationÂ in the column type to useÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cylinderpointmarker3d.html" rel="noopener noreferrer" target="_blank">CylinderPointMarker3DðŸ“˜</a> which must be imported.

Metadata are simply javascript objects attached to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyzdataseries3d.html" rel="noopener noreferrer" target="_blank">XyzDataSeries3DðŸ“˜</a> in SciChart. The propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html#vertexcolor" rel="noopener noreferrer" target="_blank">metadata.vertexColorðŸ“˜</a> is used to determine column 3D datapoint colour.

Finally, when specifying metadata colors, the propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/columnrenderableseries3d.html#usemetadatacolors" rel="noopener noreferrer" target="_blank">ColumnRenderableSeries3D.useMetadataColorsðŸ“˜</a> must be set to true.

Here's the updated output:

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-types/column-3d-chart/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-types/column-3d-chart/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
