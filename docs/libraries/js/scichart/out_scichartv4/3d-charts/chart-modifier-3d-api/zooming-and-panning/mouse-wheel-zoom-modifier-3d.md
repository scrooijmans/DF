On this page

# Mouse Wheel Zoom Modifier 3D

Zooming and Panning a Chart in SciChart.js is achieved by moving theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html#camera" rel="noopener noreferrer" target="_blank">SciChart3DSurface.cameraðŸ“˜</a> to a new location.

The articleÂ ["The SciChart3DSurface Camera"](https://www.scichart.com/documentation/js/v4/3d-charts/scichart-3d-basics/scichart-surface-camera/) goes into detail how this camera class works and how to manipulate it programatically to achieve various views.

If you want to add simple zooming in/out of the camera to the chart then you can do so using our ChartModifiers API. TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier3d.html" rel="noopener noreferrer" target="_blank">MouseWheelZoomModifier3DðŸ“˜</a> performsÂ movement of the camera by adjusting the radius property,Â  giving the appearance of the chart zooming.

## Declaring an MouseWheelZoomModifier3D<a href="https://www.scichart.com/documentation/js/v4/3d-charts/chart-modifier-3d-api/zooming-and-panning/mouse-wheel-zoom-modifier-3d/#declaring-an-mousewheelzoommodifier3d" class="hash-link" aria-label="Direct link to Declaring an MouseWheelZoomModifier3D" translate="no" title="Direct link to Declaring an MouseWheelZoomModifier3D">â€‹</a>

Declaring aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/mousewheelzoommodifier3d.html" rel="noopener noreferrer" target="_blank">MouseWheelZoomModifier3DðŸ“˜</a> is as simple as adding one to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/orbitmodifier3d.html" rel="noopener noreferrer" target="_blank">SciChart3DSurface.chartModifiersðŸ“˜</a> property. This can be done as a single modifier, or as part of a group.

- TS
- HTML
- CSS

``` prism-code
// const { MouseWheelZoomModifier3D } = SciChart;
// or for npm: import { MouseWheelZoomModifier3D } from "scichart"

sciChart3DSurface.chartModifiers.add(new MouseWheelZoomModifier3D());
```

``` prism-code
<div class="wrapper">
    <div id="scichart-root"></div>
    <div class="titleWrapper">
        <p class="title">SciChart.js 3D Chart Example</p>
        <p class="subTitle">Use the Mouse Wheel to Zoom the 3D chart</p>
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

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/3d-charts/chart-modifier-3d-api/zooming-and-panning/mouse-wheel-zoom-modifier-3d/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/3d-charts/chart-modifier-3d-api/zooming-and-panning/mouse-wheel-zoom-modifier-3d/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
