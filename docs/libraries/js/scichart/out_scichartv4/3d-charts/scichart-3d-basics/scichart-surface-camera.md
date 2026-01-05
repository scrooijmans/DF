On this page

# The SciChartSurface Camera

The propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" rel="noopener noreferrer" target="_blank">SciChart3DSurface.CameraðŸ“˜</a> defines anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html" rel="noopener noreferrer" target="_blank">ICameraControllerðŸ“˜</a> instance. By default this interface is implementedÂ inÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html" rel="noopener noreferrer" target="_blank">CameraControllerðŸ“˜</a>Â class.

By default, theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" rel="noopener noreferrer" target="_blank">SciChart3DSurface.CameraðŸ“˜</a> is set to a newÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html" rel="noopener noreferrer" target="_blank">CameraControllerðŸ“˜</a>Â instance which defines theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">positionðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target" rel="noopener noreferrer" target="_blank">targetðŸ“˜</a> (in world coordinates) of the camera, and whether the camera is in Perspective or Orthogonal projection modes.

<img src="out_scichartv4/3d-charts/scichart-3d-basics/scichart-surface-camera/index_media/f71201e889b46218718d584c5d0fb4f499a77be4.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

Above: the representation of a Camera in 3D Space. The camera is attached to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" rel="noopener noreferrer" target="_blank">SciChart3DSurface.CameraðŸ“˜</a>Â property and is defined by aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">positionðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">targetðŸ“˜</a>, plus other properties which define the properties of the viewport.

## Camera Position, Target and other Properties<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/#camera-position-target-and-other-properties" class="hash-link" aria-label="Direct link to Camera Position, Target and other Properties" translate="no" title="Direct link to Camera Position, Target and other Properties">â€‹</a>

The camera is defined by aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">positionðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">targetðŸ“˜</a> which are XYZ vectors in World Coordinates. These can be set in the create() function of SciChart3DSurface or after creation of the chart.

Other properties which define the viewport as seen by the camera include:

| Property | Description | Default |
|----|----|----|
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">positionðŸ“˜</a> | The position of the camera as a 3-component vector in World coordinates | `-` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target" rel="noopener noreferrer" target="_blank">targetðŸ“˜</a> | The target (look-at) of the camera as a 3-component vector in World coordinates | `-` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orbitalpitch" rel="noopener noreferrer" target="_blank">orbitalPitchðŸ“˜</a> | The pitch (vertical rotation angle) above or below the target in degrees | `-` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orbitalyaw" rel="noopener noreferrer" target="_blank">orbitalYawðŸ“˜</a> | The yaw (horizontal rotation angle) around the target in degrees | `-` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#projectionmode" rel="noopener noreferrer" target="_blank">projectionModeðŸ“˜</a> | Defines if the camera is perspective or orthogonal | `Perspective` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#fieldofview" rel="noopener noreferrer" target="_blank">fieldOfViewðŸ“˜</a> | When `projectionMode=Perspective`, defines the field of view in degrees | `60` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#radius" rel="noopener noreferrer" target="_blank">radiusðŸ“˜</a> | When `projectionMode=Perspective`, get or set the distance of the camera position to target in World units | `-` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orthowidth" rel="noopener noreferrer" target="_blank">orthoWidthðŸ“˜</a> | Defines the width of the projected viewport when in `projectionMode=Orthogonal` | `600` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orthoheight" rel="noopener noreferrer" target="_blank">orthoHeightðŸ“˜</a> | Defines the height of the projected viewport when in `projectionMode=Orthogonal` *(Note: Fixed typo from "Perspective" to "Orthogonal")* | `400` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#nearclip" rel="noopener noreferrer" target="_blank">nearClipðŸ“˜</a> | Objects closer than `nearClip` will be hidden | `1` |
| <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#farclip" rel="noopener noreferrer" target="_blank">farClipðŸ“˜</a> | Objects further than `farClip` will be hidden | `4000` |

A full list of properties, methods can be seen at theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html" rel="noopener noreferrer" target="_blank">CameraControllerðŸ“˜</a> Typedoc page.

### Debugging Camera Properties<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/#debugging-camera-properties" class="hash-link" aria-label="Direct link to Debugging Camera Properties" translate="no" title="Direct link to Debugging Camera Properties">â€‹</a>

You can be notified if any property on the camera is updated by subscribing toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#propertychanged" rel="noopener noreferrer" target="_blank">camera.propertyChangedðŸ“˜</a>. Note that this event will fire for properties set in code, or user input such as rotating, zooming or panning the chart.

It's also possible to debug the camera position & properties by callingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#debugoutput" rel="noopener noreferrer" target="_blank">camera.debugOutput()ðŸ“˜</a>. This logs properties to the console window. The same function also returns an array of strings, which you can use to update the UI.

Putting these together you can create the following example:

- TS
- HTML
- CSS

``` prism-code
import {
    SciChart3DSurface,
    NumericAxis3D,
    Vector3,
    SciChartJsNavyTheme,
    CameraController,
    MouseWheelZoomModifier3D,
    OrbitModifier3D,
    ResetCamera3DModifier
} from "scichart";

async function cameraProperties(divElementId) {
    const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme(),
        // Optional camera options passed into create Method
        cameraOptions: {
            position: new Vector3(300, 300, 300),
            target: new Vector3(0, 50, 0)
        }
    });

    // A camera may be attached to a chart after creation
    sciChart3DSurface.camera = new CameraController(wasmContext, {
        id: "Primary Camera",
        position: new Vector3(300, 300, 300),
        target: new Vector3(0, 50, 0)
    });

    const camera = sciChart3DSurface.camera;

    // propertyChanged is raised each time any property changes on the camera
    camera.propertyChanged.subscribe(args => {
        // Log current properties to console. debugOutput returns array of strings
        const cameraDebug = camera.debugOutput();

        // Output the same information to a div on the page
        // @ts-ignore
        document.getElementById("debug-camera").innerHTML = cameraDebug.map(line => `<p>${line}</p>`).join("");
    });

    // trigger a property change for the example
    camera.target = new Vector3(0, 60, 0);

    // SciChart.js 3D supports only a single x,y,z axis. Declare your axis like this
    sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, { axisTitle: "X Axis" });
    sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, { axisTitle: "Y Axis" });
    sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, { axisTitle: "Z Axis" });

    sciChart3DSurface.chartModifiers.add(
        new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
        new OrbitModifier3D(), // provides 3d rotation on left mouse drag
        new ResetCamera3DModifier()
    ); // resets camera position on double-click
}

cameraProperties("scichart-root");
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root" ></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Drag the mouse to view camera properties change</p>
    </div>
    <div id="debug-camera">
        <!-- Debug output from camera will be put here -->
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
#debug-camera {
    pointer-events: none;
    position: absolute;
    left: 10px;
    top: 10px;
    color: #ffffff;
    background: #00000077;
    padding: 10px;
    font-size: 13px;
}
```

## Camera Perspective vs. Orthogonal Mode<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/#camera-perspective-vs-orthogonal-mode" class="hash-link" aria-label="Direct link to Camera Perspective vs. Orthogonal Mode" translate="no" title="Direct link to Camera Perspective vs. Orthogonal Mode">â€‹</a>

SciChart.js 3D also has an orthogonal mode. In this mode, there is no perspective effect of lines converging or diverging and all lines in the same X,Y or Z plane are parallel.

### In Perspective Mode<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/#in-perspective-mode" class="hash-link" aria-label="Direct link to In Perspective Mode" translate="no" title="Direct link to In Perspective Mode">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position" rel="noopener noreferrer" target="_blank">Camera.positionðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target" rel="noopener noreferrer" target="_blank">camera.targetðŸ“˜</a> are obeyed.Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#radius" rel="noopener noreferrer" target="_blank">Camera.radiusðŸ“˜</a> can be get/set to adjust the distance between position & target.Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#fieldofview" rel="noopener noreferrer" target="_blank">Camera.fieldOfViewðŸ“˜</a> is obeyed.

### In Orthogonal Mode<a href="https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/#in-orthogonal-mode" class="hash-link" aria-label="Direct link to In Orthogonal Mode" translate="no" title="Direct link to In Orthogonal Mode">â€‹</a>

Apparent distance to target is defined byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orthowidth" rel="noopener noreferrer" target="_blank">camera.orthoWidthðŸ“˜</a>Â andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orthoheight" rel="noopener noreferrer" target="_blank">orthoHeight propertyðŸ“˜</a>. Camera.radius is ignored.

whenÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#projectionmode" rel="noopener noreferrer" target="_blank">projectionMode = ECameraProjectionMode.OrthogonalðŸ“˜</a>, instead of a pyramid shape for the camera view, imagine a box with parallel sides. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orthowidth" rel="noopener noreferrer" target="_blank">orthoWidthðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orthoheight" rel="noopener noreferrer" target="_blank">orthoHeightðŸ“˜</a> specify the width and height of the view in 3D world coordinates.Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#radius" rel="noopener noreferrer" target="_blank">Camera.radiusðŸ“˜</a> makes no difference to apparent closeness to target as there is no persective (closer objects are not larger).

<img src="out_scichartv4/3d-charts/scichart-3d-basics/scichart-surface-camera/index_media/e250d3a1cd9677765cd651499bfa2bf581b58ed5.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

In this mode, **radius** and **fieldOfView** are ignored. Update the **orthoWidth** and **orthoHeight** property to modify apparent zoom. Camera **position**, **target**Â as well as **orbitalPitch**, **orbitalYaw** can be set to define camera orientation.

Find an example demonstrating camera orthogonal projection below.

- TS
- HTML
- CSS

``` prism-code
import {
    SciChart3DSurface,
    NumericAxis3D,
    Vector3,
    SciChartJsNavyTheme,
    ECameraProjectionMode,
    MouseWheelZoomModifier3D,
    OrbitModifier3D,
    ResetCamera3DModifier,
} from "scichart";

async function orthogonalProjection(divElementId) {
  const { wasmContext, sciChart3DSurface } = await SciChart3DSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    cameraOptions: {
      id: "Primary Camera",
      position: new Vector3(400, 400, 400),
      target: new Vector3(0, 100, 0),
    }
  });

  // Start off with the default camera
  const camera = sciChart3DSurface.camera;

  // Debug log as before
  camera.propertyChanged.subscribe((args) => {
    const cameraDebug = camera.debugOutput();
    // @ts-ignore
    document.getElementById("debug-camera").innerHTML = cameraDebug.map(line => `<p>${line}</p>`).join("");
  });

  // Switch to orthogonal projection
  // In orthogonal projection mode, the camera is positioned by position,target
  // however the viewing 'cone' no longer obeys perspective but is parallel
  //
  // orthoWidth/orthoHeight must be set to the desired width/height of the camera's view in world dimensions
  // larger numbers means more 'zoomed out'
  // aspectRatio is the ratio of orthoWidth/orthoHeight
  camera.projectionMode = ECameraProjectionMode.Orthogonal;
  camera.orthoWidth = 800;
  camera.orthoHeight = 550;

  // Also note: SciChart.js has camera.toPersective() and camera.toOrthogonal() methods to quickly switch back/forth

  // SciChart.js 3D supports only a single x,y,z axis. Declare your axis like this
  sciChart3DSurface.xAxis = new NumericAxis3D(wasmContext, { axisTitle: "X Axis" });
  sciChart3DSurface.yAxis = new NumericAxis3D(wasmContext, { axisTitle: "Y Axis" })
  sciChart3DSurface.zAxis = new NumericAxis3D(wasmContext, { axisTitle: "Z Axis" });

  sciChart3DSurface.chartModifiers.add(
    new MouseWheelZoomModifier3D(), // provides camera zoom on mouse wheel
    new OrbitModifier3D(), // provides 3d rotation on left mouse drag
    new ResetCamera3DModifier()); // resets camera position on double-click
};

orthogonalProjection("scichart-root");
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root" ></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Drag the mouse to view camera properties change</p>
    </div>
    <div id="debug-camera">
        <!-- Debug output from camera will be put here -->
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
#debug-camera {
    pointer-events: none;
    position: absolute;
    left: 10px;
    top: 10px;
    color: #ffffff;
    background: #00000077;
    padding: 10px;
    font-size: 13px;
}
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/scichart-3d-basics/scichart-surface-camera/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/scichart-3d-basics/scichart-surface-camera/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
