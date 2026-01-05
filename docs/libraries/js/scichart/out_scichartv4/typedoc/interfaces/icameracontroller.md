<img src="out_scichartv4/typedoc/interfaces/icameracontroller_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ICameraController](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html)

# Interface ICameraController

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to a [CameraController](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html) - a Class which manipulates and manages the camera in 3D space within SciChart's High Performance <a href="https://www.scichart.com/javascript-chart-features" class="external">JavaScript 3D Charts</a>

### Hierarchy

- ICameraController

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html" class="tsd-signature-type">CameraController</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#aspectratio" class="tsd-kind-icon">aspectRatio</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#farclip" class="tsd-kind-icon">farClip</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#fieldofview" class="tsd-kind-icon">fieldOfView</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#forward" class="tsd-kind-icon">forward</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#isattached" class="tsd-kind-icon">isAttached</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#nearclip" class="tsd-kind-icon">nearClip</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#orbitalpitch" class="tsd-kind-icon">orbitalPitch</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#orbitalyaw" class="tsd-kind-icon">orbitalYaw</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#orthoheight" class="tsd-kind-icon">orthoHeight</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#orthowidth" class="tsd-kind-icon">orthoWidth</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#position" class="tsd-kind-icon">position</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#projectionmode" class="tsd-kind-icon">projectionMode</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#propertychanged" class="tsd-kind-icon">propertyChanged</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#radius" class="tsd-kind-icon">radius</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#side" class="tsd-kind-icon">side</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#target" class="tsd-kind-icon">target</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#up" class="tsd-kind-icon">up</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#upglobal" class="tsd-kind-icon">upGlobal</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#debugoutput" class="tsd-kind-icon">debugOutput</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#resettodefaults" class="tsd-kind-icon">resetToDefaults</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#toorthogonal" class="tsd-kind-icon">toOrthogonal</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#toperspective" class="tsd-kind-icon">toPerspective</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#updateenginecamera" class="tsd-kind-icon">updateEngineCamera</a>

## Properties

### aspectRatio

aspectRatio: number

Gets or sets the camera aspect ratio. Defaults to 1.533333

### farClip

farClip: number

Gets or sets the far clipping in world coordinates, for example if set to 1,000 any object further than 1,000 world coordinates will not be rendered

### fieldOfView

fieldOfView: number

Gets the field of view in degrees. Default value is 60

### Readonly forward

forward: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets the Forward [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html); the vector which results from [target](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#target) - [position](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#position)

### id

id: string

Gets or sets the unique camera Id

### isAttached

isAttached: boolean

Gets whether the camera is attached to a [SciChart3DSurface](https://www.scichart.com/documentation/js/v4/typedoc/classes/scichart3dsurface.html) or not

### nearClip

nearClip: number

Gets or sets the near clipping in world coordinates, for example if set to 1.0, any object closer than 1 world coordinate will not be rendered

### orbitalPitch

orbitalPitch: number

Gets or sets the Orbital Pitch - a vertical rotation angle around the target - in degrees

remarks  
This property will be updated when you set [CameraController.position](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position) or [CameraController.target](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target). Similarly, setting [CameraController.orbitalPitch](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orbitalpitch) will result in position & target being updated.

### orbitalYaw

orbitalYaw: number

Gets or sets the Orbital Yaw - a horizontal rotation angle around the target - in degrees

remarks  
This property will be updated when you set [CameraController.position](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position) or [CameraController.target](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target). Similarly, setting [CameraController.orbitalYaw](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#orbitalyaw) will result in position & target being updated.

### orthoHeight

orthoHeight: number

Gets or sets the orthogonal viewable height in world coordinates

remarks  
Applicable when [CameraController.projectionMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#projectionmode) is Orthogonal

### orthoWidth

orthoWidth: number

Gets or sets the orthogonal viewable width in world coordinates

remarks  
Applicable when [CameraController.projectionMode](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#projectionmode) is Orthogonal

### position

position: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets or sets the camera position as a [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) in World Space

### projectionMode

projectionMode: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html" class="tsd-signature-type">ECameraProjectionMode</a>

Gets or sets the [ECameraProjectionMode](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html). Default is [ECameraProjectionMode.Perspective](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html#perspective) and using this property you can also set the camera mode to [Orthogonal](https://www.scichart.com/documentation/js/v4/typedoc/enums/ecameraprojectionmode.html#orthogonal)

### propertyChanged

propertyChanged: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/propertychangedeventargs.html" class="tsd-signature-type">PropertyChangedEventArgs</a>\>

An event handler which raises events when a property changes and the scene must be redrawn

### radius

radius: number

Gets or sets the radius - the distance from the [CameraController.target](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target)

remarks  
This property will be updated when you set [CameraController.position](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#position) or [CameraController.target](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#target). Similarly, setting [CameraController.radius](https://www.scichart.com/documentation/js/v4/typedoc/classes/cameracontroller.html#radius) will result in position & target being updated.

### Readonly side

side: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets the Side [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html); the vector which is orthogonal to [up](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#up) and [forward](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameracontroller.html#forward)

### target

target: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets or sets the camera target (Look-at) as a [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) in World Space

### Readonly up

up: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets the Up [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html); the vector pointing straight up from the camera's position

### Readonly upGlobal

upGlobal: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

Gets the global up [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html); a vector which defaults to Xyz = \[0,1,0\]

## Methods

### debugOutput

- debugOutput(): string\[\]

<!-- -->

- Used internally - call to dump camera properties to the console log

  #### Returns string\[\]

### resetToDefaults

- resetToDefaults(): void

<!-- -->

- Resets the camera properties to defaults

  #### Returns void

### toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html" class="tsd-signature-type">ICameraOptions</a>

<!-- -->

- Convert the object to a definition that can be serialized to JSON, or used directly with the builder api

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/icameraoptions.html" class="tsd-signature-type">ICameraOptions</a>

### toOrthogonal

- toOrthogonal(): void

<!-- -->

- Does the hard maths for you to convert the current Perspective camera to an Orthogonal camera with the same position and target

  #### Returns void

### toPerspective

- toPerspective(): void

<!-- -->

- Does the hard maths for you to convert the current Orthogonal camera to an Perspective camera with the same position and target

  #### Returns void

### updateEngineCamera

- updateEngineCamera(tsrCamera: TSRCamera): void

<!-- -->

- Used internally - Updates a {@link TSRCamera} instance which will be passed to SciChart's fast WebGL WebAssembly engine

  #### Parameters

  - ##### tsrCamera: TSRCamera

  #### Returns void

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
