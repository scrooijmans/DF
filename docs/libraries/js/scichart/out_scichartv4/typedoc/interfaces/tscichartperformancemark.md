<img src="out_scichartv4/typedoc/interfaces/tscichartperformancemark_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [TSciChartPerformanceMark](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html)

# Interface TSciChartPerformanceMark\<TDetail\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

### Type parameters

- #### TDetail: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>

### Hierarchy

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a>
  - TSciChartPerformanceMark

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-kind-icon">PerformanceMark</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#detail" class="tsd-kind-icon">detail</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#duration" class="tsd-kind-icon">duration</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#entrytype" class="tsd-kind-icon">entryType</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#name" class="tsd-kind-icon">name</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#starttime" class="tsd-kind-icon">startTime</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#tojson" class="tsd-kind-icon">toJSON</a>

## Properties

### PerformanceMark

PerformanceMark: { constructor: any; prototype: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a> }

#### Type declaration

- ##### constructor: function

  - new \_\_type(markName: string, markOptions?: PerformanceMarkOptions): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a>

  <!-- -->

  - #### Parameters

    - ##### markName: string

    - ##### Optional markOptions: PerformanceMarkOptions

    #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a>

- ##### prototype: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a>

### Readonly detail

detail: TDetail & { relatedId: string }

### Readonly duration

duration: DOMHighResTimeStamp

### Readonly entryType

entryType: string

### Readonly name

name: string

### Readonly startTime

startTime: DOMHighResTimeStamp

## Methods

### toJSON

- toJSON(): any

<!-- -->

- #### Returns any

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
