<img src="out_scichartv4/typedoc/interfaces/ihoverable_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IHoverable](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html)

# Interface IHoverable

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The interface describing a visual chart component that could be hovered.

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iincludable.html" class="tsd-signature-type">IIncludable</a>
  - IHoverable
    - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iannotation.html" class="tsd-signature-type">IAnnotation</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#checkiswithinbounds" class="tsd-kind-icon">checkIsWithinBounds</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#hover" class="tsd-kind-icon">hover</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#hovered" class="tsd-kind-icon">hovered</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#id" class="tsd-kind-icon">id</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#ishovered" class="tsd-kind-icon">isHovered</a>

## Properties

### checkIsWithinBounds

checkIsWithinBounds: (args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>) =\> boolean

Calculates if pointer is within entity bounds

#### Type declaration

- - (args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>): boolean

  <!-- -->

  - #### Parameters

    - ##### args: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

    #### Returns boolean

### hover

hover: (options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoveroptions.html" class="tsd-signature-type">IHoverOptions</a>) =\> void

Executes a hover action on the annotation if it is hit

#### Type declaration

- - (options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoveroptions.html" class="tsd-signature-type">IHoverOptions</a>): void

  <!-- -->

  - #### Parameters

    - ##### options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoveroptions.html" class="tsd-signature-type">IHoverOptions</a>

    #### Returns void

### hovered

hovered: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>\<any\>

Fires based on hover rules passed into [hover](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html#hover)

### id

id: string

### isHovered

isHovered: boolean

Defines if the entity is hovered

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
