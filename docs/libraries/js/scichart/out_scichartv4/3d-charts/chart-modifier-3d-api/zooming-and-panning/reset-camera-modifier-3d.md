On this page

# Reset Camera Modifier 3D

Zooming and Panning a Chart in SciChart.js is achieved by moving theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" rel="noopener noreferrer" target="_blank">SciChart3DSurface.cameraðŸ“˜</a> to a new location.

The articleÂ ["The SciChart3DSurface Camera"](https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/) goes into detail how this camera class works and how to manipulate it programatically to achieve various views.

If you add any zooming or panning to the chart you might want to reset the viewport to it's original state. You can do this with theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resetcamera3dmodifier.html" rel="noopener noreferrer" target="_blank">ResetCamera3DModifierðŸ“˜</a>.

How this modifier works:

- When the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resetcamera3dmodifier.html" rel="noopener noreferrer" target="_blank">ResetCamera3DModifierðŸ“˜</a>Â is attached to the chart, it saves the currentÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" rel="noopener noreferrer" target="_blank">cameraðŸ“˜</a> state.
- An optional destination object of typeÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tcamerastate" rel="noopener noreferrer" target="_blank">TCameraStateðŸ“˜</a>Â may be set to override this state.
- When you double click on the chart, the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resetcamera3dmodifier.html" rel="noopener noreferrer" target="_blank">ResetCamera3DModifierðŸ“˜</a>Â animates the camera position to the initial camera state.

## Declaring an ResetCameraModifier3D<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/reset-camera-modifier-3d/#declaring-an-resetcameramodifier3d" class="hash-link" aria-label="Direct link to Declaring an ResetCameraModifier3D" translate="no" title="Direct link to Declaring an ResetCameraModifier3D">â€‹</a>

Declaring aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/resetcamera3dmodifier.html" rel="noopener noreferrer" target="_blank">ResetCamera3DModifierðŸ“˜</a>Â is as simple as adding one to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/orbitmodifier3d.html" rel="noopener noreferrer" target="_blank">SciChart3DSurface.chartModifiersðŸ“˜</a> property. This can be done as a single modifier, or as part of a group.

- TS
- HTML
- CSS

``` prism-code
// const { ResetCamera3DModifier } = SciChart;
// or for npm: import { ResetCameraModifier3D } from "scichart"

sciChart3DSurface.chartModifiers.add(
  new ResetCamera3DModifier ({
    // Optional properties. If these are not set,
    // the ResetCameraModifier3D grabs initial state from the SciChart3DSurface.camera
    isAnimated: true,
    animationDuration: 2000,
    // Camera will animate to this position on double click (or initial position if not set)
    destination: {
      radius: 450,
      pitch: 30,
      yaw: 45,
    }
  }),
);
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Double Click the chart to reset Camera State</p>
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

This results in the following behaviour added to the chart.

If you double click by the left mouse button you will notice how the camera resets.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-modifier-3d-api/zooming-and-panning/reset-camera-modifier-3d/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-modifier-3d-api/zooming-and-panning/reset-camera-modifier-3d/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
