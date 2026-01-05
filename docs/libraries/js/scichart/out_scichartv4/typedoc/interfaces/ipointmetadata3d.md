<img src="out_scichartv4/typedoc/interfaces/ipointmetadata3d_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IPointMetadata3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html)

# Interface IPointMetadata3D

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Metadata may be added to some 3D Chart [DataSeries](https://www.scichart.com/documentation/js/v4/typedoc/classes/basedataseries3d.html) to define per-point colouring and scaling. You can optionally create types that implement [IPointMetadata3D](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html) to pass further info through per-point

remarks  
See an example of this in our demos for [ScatterRenderableSeries3D](https://www.scichart.com/documentation/js/v4/typedoc/enums/eseriestype3d.html#scatterrenderableseries3d)

### Hierarchy

- IPointMetadata3D

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html#pointscale" class="tsd-kind-icon">pointScale</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html#vertexcolor" class="tsd-kind-icon">vertexColor</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata3d.html#vertexcolorabgr" class="tsd-kind-icon">vertexColorAbgr</a>

## Properties

### Optional pointScale

pointScale: number

Override the scale for this particular point. Set=1.0 for default

### Optional vertexColor

vertexColor: number

Override the color for this particular point. Set=undefined for default. Note: This is a 32-bit integer in ARGB format, e.g. 0xFFFF0000 is Red

### Optional vertexColorAbgr

vertexColorAbgr: number

deprecated  
Property naming was wrong, we've updated it to vertexColor \[SCJS-1566\]

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
