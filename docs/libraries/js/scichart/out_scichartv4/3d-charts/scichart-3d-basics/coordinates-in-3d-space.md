On this page

# Coordinates in 3D Space

## Coordinate Systems<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#coordinate-systems" class="hash-link" aria-label="Direct link to Coordinate Systems" translate="no" title="Direct link to Coordinate Systems">â€‹</a>

### The Left Handed Coordinate System (LHS)<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#the-left-handed-coordinate-system-lhs" class="hash-link" aria-label="Direct link to The Left Handed Coordinate System (LHS)" translate="no" title="Direct link to The Left Handed Coordinate System (LHS)">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html" rel="noopener noreferrer" target="_blank">SciChartSurface3DðŸ“˜</a> by default renders a 3D world using the Left Handed Coordinate system or LHS (as is common to WebGL). In the LHS X and Z form the horizontal plane, and Y is always up YDirection=(0,1,0). It is helpful to think of the 3D world as a 2D Chart in X-Y and Z goes â€˜into the screenâ€™.

<img src="out_scichartv4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/index_media/d35328c35b4f49a95fdfa6128a2cb63e228f3270.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

<img src="out_scichartv4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/index_media/c0b6b4f478e117471ee9cd53f797c3637f69f356.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

**Left handed coordinate system. X-Z is a horizontal plane, Y is up.**

## World Coordinates<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#world-coordinates" class="hash-link" aria-label="Direct link to World Coordinates" translate="no" title="Direct link to World Coordinates">â€‹</a>

World Coordinates is the term used to describe coordinates in the 3D World. These are the raw X,Y,Z coordinates of a vertex. By default the origin (0,0,0) is in the centre, bottom of the chart.

### WorldDimensions and the Axis Cube<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#worlddimensions-and-the-axis-cube" class="hash-link" aria-label="Direct link to WorldDimensions and the Axis Cube" translate="no" title="Direct link to WorldDimensions and the Axis Cube">â€‹</a>

The box in the chart is called the Axis Cube. The AxisCube size is defined by theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#worlddimensions" rel="noopener noreferrer" target="_blank">SciChartSurface3D.WorldDimensionsðŸ“˜</a> property.

The WorldDimensions is a singleÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" rel="noopener noreferrer" target="_blank">Vector3ðŸ“˜</a> with X,Y,Z size, but defines the size of a cube as follows:

<img src="out_scichartv4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/index_media/9f36fef6459f922d85eaee755fe25b4d21fe0127.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

**How WorldDimensions Relates to the AxisCube in SciChart3D**

Thereforeâ€¦

- The size of the AxisCube in the X Direction extends from -WorldDimensions.X/2 to +WorldDimensions.X/2
- The size of the AxisCube in the Y Direction extends from 0 to +WorldDimensions.Y
- The size of the AxisCube in the Z Direction extends from -WorldDimensions.Z/2 to +WorldDimensions.Z/2

NOTE: By default,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#worlddimensions" rel="noopener noreferrer" target="_blank">SciChart3DSurface.WorldDimensionsðŸ“˜</a> property is set to X=300, Y=200, Z=300.

### Setting the WorldDimensions Property<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#setting-the-worlddimensions-property" class="hash-link" aria-label="Direct link to Setting the WorldDimensions Property" translate="no" title="Direct link to Setting the WorldDimensions Property">â€‹</a>

To set the WorldDimensions Property, simply use the following code to define a Vector3 (3-component vector):

- Setting WorldDimensions

``` prism-code
import { SciChart3DSurface, Vector3 } from "scichart";

// World dimensions can be set at creation of the chart
const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
  // Optional dimensions of the axis cube (X,Y,Z) in World coordinates
  worldDimensions: new Vector3(300, 200, 300),
});

// Or, it can be set later
sciChart3DSurface.worldDimensions = new Vector3(300, 200, 300);
```

## Data Coordinates<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#data-coordinates" class="hash-link" aria-label="Direct link to Data Coordinates" translate="no" title="Direct link to Data Coordinates">â€‹</a>

By contrast to WorldCoordinates, which are absolute coordinates in the 3D World, in SciChart 3D there is the concept of Data Coordinates.

All Data in SciChart3D is provided in Data Coordinates. They are converted to World Coordinates by SciChart 3D for display on the chart.

Data Coordinates are measured on an Axis, for example, the YAxis (which is UP) might have a size of 200 in the World Coordinates, but might display a VisibleRange of 0...10. Therefore, Data which falls in the range 0...10 will be spaced on this axis from 0...200 World Coordinates.

The difference between World Coordinates, Data Coordinates is shown in the following diagram:

<img src="out_scichartv4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/index_media/2e997acb7527773508ff3643876935115574868a.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

**How WorldDimensions Relates to the AxisCube in SciChart3D**

## Converting from World to Data Coordinates<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/#converting-from-world-to-data-coordinates" class="hash-link" aria-label="Direct link to Converting from World to Data Coordinates" translate="no" title="Direct link to Converting from World to Data Coordinates">â€‹</a>

The conversion between Data Coordinates and World Coordinates is done by the Axis. For example. the following code converts from Data to World Coordinates on theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#xaxis" rel="noopener noreferrer" target="_blank">SciChartSurface3D.xAxisðŸ“˜</a>.

- Coordinate calculator

``` prism-code
// Get the coordinate calculator
const coordCalc = sciChart3DSurface.xAxis.getCurrentCoordinateCalculator();

// Get a world coordinate from data values
const worldX0 = coordCalc.getCoordinate(0);
const worldX10 = coordCalc.getCoordinate(10);

// Convert back to dataValue
const dataValue0 = coordCalc.getDataValue(worldX0);
const dataValue10 = coordCalc.getDataValue(worldX10);

console.log(`Data value at X=${dataValue0} corresponds to world coordinate X=${worldX0}`);
console.log(`Data value at X=${dataValue10} corresponds to world coordinate X=${worldX10}`);
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/scichart-3d-basics/coordinates-in-3d-space/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/scichart-3d-basics/coordinates-in-3d-space/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
