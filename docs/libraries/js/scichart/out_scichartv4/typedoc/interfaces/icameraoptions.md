<img src="out_scichartv4/typedoc/interfaces/icameraoptions_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ICameraOptions](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html)

# Interface ICameraOptions

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Optional parameters passed to a [CameraController](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html) at construct time

### Hierarchy

- ICameraOptions

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#debugpositions" class="tsd-kind-icon">debugPositions</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#farclip" class="tsd-kind-icon">farClip</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#fieldofview" class="tsd-kind-icon">fieldOfView</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#nearclip" class="tsd-kind-icon">nearClip</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#orthoheight" class="tsd-kind-icon">orthoHeight</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#orthowidth" class="tsd-kind-icon">orthoWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#position" class="tsd-kind-icon">position</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#projectionmode" class="tsd-kind-icon">projectionMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html#target" class="tsd-kind-icon">target</a>

## Properties

### Optional debugPositions

debugPositions: boolean

When true, will output position, target to console

Default `false`

### Optional farClip

farClip: number

The far clipping in world coordinates, for example if set to 1,000 any object further than 1,000 world coordinates will not be rendered

Default `4000`

### Optional fieldOfView

fieldOfView: number

The field of view in degrees.

Default `60`

### Optional id

id: string

Gets or sets the unique camera Id

### Optional nearClip

nearClip: number

The near clipping in world coordinates, for example if set to 1.0, any object closer than 1 world coordinate will not be rendered

Default `1`

### Optional orthoHeight

orthoHeight: number

The orthogonal viewable height in world coordinates

remark  
Only works when `projectionMode` is set to `ECameraProjectionMode.Orthogonal`

Default `400`

### Optional orthoWidth

orthoWidth: number

The orthogonal viewable width in world coordinates

remark  
Only works when `projectionMode` is set to `ECameraProjectionMode.Orthogonal`

Default `600`

### Optional position

position: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets or sets the camera position as a [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) in World Space

Default `new Vector3(-1000, 1000, -1000)`

### Optional projectionMode

projectionMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html" class="tsd-signature-type">ECameraProjectionMode</a>

The [ECameraProjectionMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html).

Default [ECameraProjectionMode.Perspective](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html#perspective)

### Optional target

target: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets or sets the camera target (Look-at) as a [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) in World Space

Default `new Vector3(0, 0, 0)`

## Legend

- Constructor
- Property
- Method
- Accessor

<!-- -->

- Inherited constructor
- Inherited property
- Inherited method
- Inherited accessor

<!-- -->

- Property
- Method

<!-- -->

- Protected property
- Protected method

<!-- -->

- Static property
- Static method

Generated using <a href="https://typedoc.org/" target="_blank">TypeDoc</a>
