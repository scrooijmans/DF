<img src="out_scichartv4/typedoc/interfaces/ihovercallbackargs_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IHoverCallbackArgs](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html)

# Interface IHoverCallbackArgs\<TEntityType\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

The arguments passed to hover event raised by a hover detection modifier, e.g. [PointerEventsMediatorModifier](https://www.scichart.com/documentation/js/v4/typedoc/classes/pointereventsmediatormodifier.html)

### Type parameters

- #### TEntityType: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihoverable.html" class="tsd-signature-type">IHoverable</a>

### Hierarchy

- IHoverCallbackArgs

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#hoveredentities" class="tsd-kind-icon">hoveredEntities</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#includedentities" class="tsd-kind-icon">includedEntities</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#mouseargs" class="tsd-kind-icon">mouseArgs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#previoushoveredentities" class="tsd-kind-icon">previousHoveredEntities</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#sender" class="tsd-kind-icon">sender</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihovercallbackargs.html#unhoveredentities" class="tsd-kind-icon">unhoveredEntities</a>

## Properties

### hoveredEntities

hoveredEntities: TEntityType\[\]

Items that are currently hovered accordingly ro hover detection rules

### includedEntities

includedEntities: TEntityType\[\]

Items that are are hit tested

### mouseArgs

mouseArgs: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/modifiermouseargs.html" class="tsd-signature-type">ModifierMouseArgs</a>

The mouse event properties

### previousHoveredEntities

previousHoveredEntities: TEntityType\[\]

Items that were hovered on previous event

### sender

sender: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/pointereventsmediatormodifier.html" class="tsd-signature-type">PointerEventsMediatorModifier</a>\<TEntityType\>

The modifier that raised the event

### unhoveredEntities

unhoveredEntities: TEntityType\[\]

Items that were hovered on previous event but are not anymore

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
