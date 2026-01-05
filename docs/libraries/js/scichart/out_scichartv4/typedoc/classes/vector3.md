<img src="out_scichartv4/typedoc/classes/vector3_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html)

# Class Vector3

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines a 3-component vector with X,Y,Z values

### Hierarchy

- Vector3

## Index

### Constructors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#constructor" class="tsd-kind-icon">constructor</a>

### Accessors

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#length" class="tsd-kind-icon">length</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#x" class="tsd-kind-icon">x</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#y" class="tsd-kind-icon">y</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#z" class="tsd-kind-icon">z</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#zero" class="tsd-kind-icon">zero</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#add" class="tsd-kind-icon">add</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#crossproduct" class="tsd-kind-icon">crossProduct</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#dotproduct" class="tsd-kind-icon">dotProduct</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#normalize" class="tsd-kind-icon">normalize</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#scalarmultiply" class="tsd-kind-icon">scalarMultiply</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#subtract" class="tsd-kind-icon">subtract</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#tostring" class="tsd-kind-icon">toString</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#totsrvector3" class="tsd-kind-icon">toTsrVector3</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#hydrate" class="tsd-kind-icon">hydrate</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html#isequal" class="tsd-kind-icon">isEqual</a>

## Constructors

### constructor

- new Vector3(x: number, y: number, z: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Creates a 3-component vector with X,Y,Z values

  #### Parameters

  - ##### x: number

  - ##### y: number

  - ##### z: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

## Accessors

### length

- get length(): number

<!-- -->

- Gets the euclidean length of the vector

  #### Returns number

### x

- get x(): number

<!-- -->

- Gets the X-value

  remarks  
  Warning! Treat [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) as immutable! Do not set this value but create new vectors if you need to change a value

  #### Returns number

### y

- get y(): number

<!-- -->

- Gets the Y-value

  remarks  
  Warning! Treat [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) as immutable! Do not set this value but create new vectors if you need to change a value

  #### Returns number

### z

- get z(): number

<!-- -->

- Gets the Z-value

  remarks  
  Warning! Treat [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) as immutable! Do not set this value but create new vectors if you need to change a value

  #### Returns number

### Static zero

- get zero(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Returns a static shared zero vector where X,Y,Z = 0

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

## Methods

### add

- add(other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Performs vector addition of this + other, returning the result in a new vector

  description  
  An example can be found below

  ``` ts
  const firstVector = new Vector3(1,2,3);
  const secondVector = new Vector3(1,1,1);
  const result = firstVector.add(secondVector);
  // Result is [2,3,4]
  ```

  #### Parameters

  - ##### other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

    the vector to add to this vector

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  A new vector with the addition result

### crossProduct

- crossProduct(rhs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Performs vector cross product of this vector and another vector, returning the result in a new vector

  description  
  An example can be found below

  ``` ts
  const firstVector = new Vector3(1,2,3);
  const secondVector = new Vector3(4,5,6);
  const result = firstVector.crossProduct(secondVector);
  // Result is firstVector ^ (cross) secondVector
  ```

  #### Parameters

  - ##### rhs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

    the other vector to apply to the right hand side of the cross product

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  A new vector with the cross product result

### dotProduct

- dotProduct(rhs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): number

<!-- -->

- Performs vector dot product of this vector and another vector, returning the result as a scalar

  #### Parameters

  - ##### rhs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

    the other vector to apply to the right hand side of the dot product

  #### Returns number

### normalize

- normalize(): void

<!-- -->

- Normalizes the current vector by computing its X,Y,Z components which make the length = 1 but direction the same

  remarks  
  This is the only operation which modifies the current vector (not immutable)

  #### Returns void

### scalarMultiply

- scalarMultiply(scalar: number): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Performs scalar multiplication of this vector x scalar constant, returning the result in a new vector

  description  
  An example can be found below

  ``` ts
  const firstVector = new Vector3(1,2,3);
  const result = firstVector.scalarMultiply(2);
  // Result is [2,4,6]
  ```

  #### Parameters

  - ##### scalar: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  A new vector with the multiply result

### subtract

- subtract(other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Performs vector subtraction of this - other, returning the result in a new vector

  description  
  An example can be found below

  ``` ts
  const firstVector = new Vector3(1,2,3);
  const secondVector = new Vector3(1,1,1);
  const result = firstVector.subtract(secondVector);
  // Result is [0,1,2]
  ```

  #### Parameters

  - ##### other: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

    the vector to substract from this vector

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  A new vector with the subtraction result

### toJSON

- toJSON(): { x: number; y: number; z: number }

<!-- -->

- #### Returns { x: number; y: number; z: number }

  - ##### x: number

  - ##### y: number

  - ##### z: number

### toString

- toString(): string

<!-- -->

- Returns a string representation of the vector for debugging purposes

  #### Returns string

### toTsrVector3

- toTsrVector3(webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>): TSRVector3

<!-- -->

- Used internally - converts the [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html) to a {@link TSRVector3} for compatibility with SciChart's webassembly engine

  #### Parameters

  - ##### webAssemblyContext: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d" class="tsd-signature-type">TSciChart3D</a>

    The [SciChart 3D WebAssembly Context](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichart3d) containing native methods and access to our WebGL2 Engine and WebAssembly numerical methods

  #### Returns TSRVector3

### Static hydrate

- hydrate(input: { x: number; y: number; z: number }): <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

<!-- -->

- Turns a { xProperty, yProperty, zProperty } object into a [Vector3](https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html), most helpful for JSON deserialization

  #### Parameters

  - ##### input: { x: number; y: number; z: number }

    - ##### x: number

    - ##### y: number

    - ##### z: number

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

### Static isEqual

- isEqual(vector1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>, vector2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>): boolean

<!-- -->

- #### Parameters

  - ##### vector1: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  - ##### vector2: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/vector3.html" class="tsd-signature-type">Vector3</a>

  #### Returns boolean

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
