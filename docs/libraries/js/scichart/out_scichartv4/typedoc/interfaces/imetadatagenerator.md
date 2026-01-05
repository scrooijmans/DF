<img src="out_scichartv4/typedoc/interfaces/imetadatagenerator_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IMetadataGenerator](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html)

# Interface IMetadataGenerator

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

A MetadataGenerator is used with the builder api to provide metadata when it is unecessary or impossible to provide it all as pure data Case 1: You want to use the same metadata object for all data points (eg to enable data point selection). getSingleMetadata will be called to populate metadata for each data point added. Case 2: You want to use a class for metadata that contains logic that cannot be serialized. Use I1DMetadataGenerator (or I2DMetadataGenerator for heatmap data). getMetadata will be called to populate the metadata.

### Hierarchy

- IMetadataGenerator
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i1dmetadatagenerator.html" class="tsd-signature-type">I1DMetadataGenerator</a>
  - <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/i2dmetadatagenerator.html" class="tsd-signature-type">I2DMetadataGenerator</a>

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/templatemetadatagenerator.html" class="tsd-signature-type">TemplateMetadataGenerator</a>

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html#getsinglemetadata" class="tsd-kind-icon">getSingleMetadata</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html#tojson" class="tsd-kind-icon">toJSON</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/imetadatagenerator.html#type" class="tsd-kind-icon">type</a>

## Properties

### getSingleMetadata

getSingleMetadata: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

#### Type declaration

- - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

  <!-- -->

  - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>

### toJSON

toJSON: () =\> <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\]\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a> \| { data?: any; type: string }

#### Type declaration

- - (): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\]\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a> \| { data?: any; type: string }

  <!-- -->

  - #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\]\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a>\[\] \| <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmetadata.html" class="tsd-signature-type">IPointMetadata</a> \| { data?: any; type: string }

### type

type: string

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
