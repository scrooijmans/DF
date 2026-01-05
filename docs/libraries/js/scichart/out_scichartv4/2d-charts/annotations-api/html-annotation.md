On this page

# HTML Annotations

Here we will describe how to use the HTML-based annotations feature of SciChart.JS and its advantages.

**Live Example**:

## Description<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#description" class="hash-link" aria-label="Direct link to Description" translate="no" title="Direct link to Description">â€‹</a>

### General Annotation Layer Types Overview<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#general-annotation-layer-types-overview" class="hash-link" aria-label="Direct link to General Annotation Layer Types Overview" translate="no" title="Direct link to General Annotation Layer Types Overview">â€‹</a>

The surface of the SciChart.JS chart consists of several layers in DOM tree. These are a combination of `canvas`, `div`, and `svg` nodes.

The annotations could be divided by the node type where they are rendered.

- Native "Render Context" Annotations - rendered using WebGl and displayed on the canvas layer. Some examples are:

  - [BoxAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/box-annotation)
  - [LineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/line-annotation)
  - [HorizontalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/horizontal-line-annotation)
  - [VerticalLineAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/vertical-line-annotation)
  - [NativeTextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation)
  - [AxisMarkerAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/axis-marker-annotation/axis-marker-annotation-overview)

- HTML Annotations - rendered as a `div` element placed within a DOM layer. For example:

  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmlcustomannotation.html" rel="noopener noreferrer" target="_blank">HtmlCustomAnnotationðŸ“˜</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmltextannotation.html" rel="noopener noreferrer" target="_blank">HtmlTextAnnotationðŸ“˜</a>

- SVG Annotations - rendered as an SVG element on one of the SVG layers. For example:

  - [CustomAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/custom-annotation)
  - [TextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/text-annotation)

We refer to HTML and SVG Annotations as "DOM Annotations" since they share some similar logic and the same rendering principles. Thus, the common base class is <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/domannotationbase.html" rel="noopener noreferrer" target="_blank">DomAnnotationðŸ“˜</a>.

![](out_scichartv4/2d-charts/annotations-api/html-annotation/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

There are <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#isdomannotation" rel="noopener noreferrer" target="_blank">isDomAnnotationðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/annotationbase.html#issvgannotation" rel="noopener noreferrer" target="_blank">isSvgAnnotationðŸ“˜</a> properties on an annotation.

So, the significant difference between DOM Annotations and Native Annotations is that each DOM Annotation instance is added as a separate node to the DOM tree

There might be multiple of layers of the same type to allow drawing DOM Annotations above or below the WebGl-drawn chart elements on the canvas.

## CustomHtmlAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#customhtmlannotation" class="hash-link" aria-label="Direct link to CustomHtmlAnnotation" translate="no" title="Direct link to CustomHtmlAnnotation">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmlcustomannotation.html" rel="noopener noreferrer" target="_blank">HtmlCustomAnnotationðŸ“˜</a> provides a basic functionality of an annotation and renders a `div` element on a chart at a specified position. And exposes a reference to this element via <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmlcustomannotation.html#htmlelement" rel="noopener noreferrer" target="_blank">HtmlCustomAnnotation.htmlElementðŸ“˜</a>.

The use case for this annotation is rendering arbitrary HTML content within a chart. This provides a great flexibility by allowing to apply standard JS APIs to work with the content and styling it with CSS.

So one can use the `htmlElement` reference and append content to it.

Example:

``` prism-code
// A CustomHtmlAnnotation which contains an HTML input element
const customHtmlAnnotation = new HtmlCustomAnnotation({
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,
    x1: 2,
    y1: 7,
    xCoordShift: 0,
    yCoordShift: 0
});
customHtmlAnnotation.htmlElement.classList.add("styledCustomAnnotation");
customHtmlAnnotation.htmlElement.innerHTML = `
    <label for="colorSelect">Choose a color:</label>
    <select id="colorSelect" name="colorSelect">
        <option value="red" style="background-color: red; color: white;">Red</option>
        <option value="green" style="background-color: green; color: white;">Green</option>
        <option value="blue" style="background-color: blue; color: white;">Blue</option>
        <option value="yellow" style="background-color: yellow; color: black;">Yellow</option>
        <option value="purple" style="background-color: purple; color: white;">Purple</option>
    </select>`;
sciChartSurface.annotations.add(customHtmlAnnotation);
```

Where the relevant CSS is:

``` prism-code
.styledCustomAnnotation {
    pointer-events: all;
    background-color: grey;
    padding: 4px;
}
```

This approach also allows the content to be rendered by UI frameworks. For more examples, check the our demo website. \#TODO reference HTML Annotations Demo

## HtmlTextAnnotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#htmltextannotation" class="hash-link" aria-label="Direct link to HtmlTextAnnotation" translate="no" title="Direct link to HtmlTextAnnotation">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/htmltextannotation.html" rel="noopener noreferrer" target="_blank">HtmlTextAnnotationðŸ“˜</a> extends the `CustomHtmlAnnotation` by providing a simple interface for adding textual annotations to a chart.

In the form of constructor options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ihtmltextannotationoptions.html" rel="noopener noreferrer" target="_blank">IHtmlTextAnnotationOptionsðŸ“˜</a> and instance properties:

- **HtmlTextAnnotation.text**
- **HtmlTextAnnotation.textContainerStyle**

Example:

``` prism-code
// A HtmlTextAnnotation which is resized on zoom and bound to data value coordinates
const textAnnotation = new HtmlTextAnnotation({
    xCoordinateMode: ECoordinateMode.DataValue,
    yCoordinateMode: ECoordinateMode.DataValue,
    x1: 4,
    y1: 5,
    x2: 7,
    xCoordShift: 0,
    yCoordShift: 0,
    text: "This annotation has X coordinates bound to data values. Try zooming or panning",
    // style object with CSSStyleDeclaration format. Supports camel-cased property names.
    // https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration
    textContainerStyle: {
        padding: "4px",
        fontSize: "1.5em",
        color: "white",
        background: "linear-gradient(135deg, #1f1c2c, #928dab)",
        border: "1px dotted black",
        borderRadius: "15px",
        textOverflow: "ellipsis",
        overflow: "hidden"
    }
});
sciChartSurface.annotations.add(textAnnotation);
```

## Positioning and Sizing<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#positioning-and-sizing" class="hash-link" aria-label="Direct link to Positioning and Sizing" translate="no" title="Direct link to Positioning and Sizing">â€‹</a>

Similar to other annotation types, `CustomHtmlAnnotation` and `HtmlTextAnnotation` could be positioned via `x1` and `y1` properties. Additionally, you can provide optional `x2` and `y2` values to bind the annotation size to specific coordinates. These annotations also support different coordinate modes defined in ECoordinateMode. xCoordinateMode, yCoordinateMode

So, for example, with the correct combination of the coordinates, coordinate modes, and CSS styles, you can achieve either a static size annotation or make it responsive to the visible range (zoom level), or the chart size. And apply other cool features that are available in CSS.

## Performance considerations<a href="https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/html-annotation/#performance-considerations" class="hash-link" aria-label="Direct link to Performance considerations" translate="no" title="Direct link to Performance considerations">â€‹</a>

This API is made to provide better annotation flexibility, a simple and familiar setup in a browser environment. However, depending on a use case you may find the Render Context annotations have a better performance compared to DOM Annotations. So consider trying them out first, and if you can't achieve the desired result, switch to Dom Annotations.

As an example: `NativeTextAnnotation`s have a great performance and support features as background, multiline text, rotation, etc... But, if you need more advanced features, consider whether the `HtmlTextAnnotation` or `TextAnnotation` (based on SVG) fits better.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/annotations-api/html-annotation/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/annotations-api/html-annotation/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
