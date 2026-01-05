<img src="out_scichartv4/typedoc/classes/performancedebughelper_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [PerformanceDebugHelper](https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html)

# Class PerformanceDebugHelper

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

experimental  
An util used for adding performance checkpoints which can be then used for analyzing the chart performance. The checkpoints are created via the [Performance API](https://developer.mozilla.org/en-US/docs/Web/API/Performance)

remarks  
By default it is disabled, to make use of the utils set [PerformanceDebugHelper.enableDebug](https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#enabledebug). Some checkpoints are set implicitly. Custom checkpoints could be set using [PerformanceDebugHelper.mark](https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#mark). To output the results use [PerformanceDebugHelper.outputLogs](https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#outputlogs).

### Hierarchy

- PerformanceDebugHelper

## Index

### Properties

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#separator" class="tsd-kind-icon">separator</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#debuglevel" class="tsd-kind-icon">debugLevel</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#enabledebug" class="tsd-kind-icon">enableDebug</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#instance" class="tsd-kind-icon">instance</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#measuredmarks" class="tsd-kind-icon">measuredMarks</a>

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#addmark" class="tsd-kind-icon">addMark</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#clearmarks" class="tsd-kind-icon">clearMarks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#createmark" class="tsd-kind-icon">createMark</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#createmeasure" class="tsd-kind-icon">createMeasure</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#createmeasurefromendmark" class="tsd-kind-icon">createMeasureFromEndMark</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#getmarks" class="tsd-kind-icon">getMarks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#getmeasures" class="tsd-kind-icon">getMeasures</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#clearmarks-1" class="tsd-kind-icon">clearMarks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#getmarks-1" class="tsd-kind-icon">getMarks</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#getmeasures-1" class="tsd-kind-icon">getMeasures</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#mark" class="tsd-kind-icon">mark</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#measure" class="tsd-kind-icon">measure</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#outputlogs" class="tsd-kind-icon">outputLogs</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html#tojson" class="tsd-kind-icon">toJSON</a>

## Properties

### separator

separator: string = "\_"

### Static debugLevel

debugLevel: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancedebuglevel.html" class="tsd-signature-type">EPerformanceDebugLevel</a> = EPerformanceDebugLevel.Info

### Static enableDebug

enableDebug: boolean = false

### Static instance

instance: <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/performancedebughelper.html" class="tsd-signature-type">PerformanceDebugHelper</a> = new PerformanceDebugHelper()

### Static measuredMarks

measuredMarks: string\[\] = \[\]

## Methods

### addMark

- addMark\<TDetail\>(type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancemarkoptions" class="tsd-signature-type">TPerformanceMarkOptions</a>\<TDetail\>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<TDetail\>

<!-- -->

- Processes and creates a [TSciChartPerformanceMark](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html) entry accordingly to provided [options](https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancemarkoptions).

  remarks  
  Make sure to provide a correct `level` option to define which marks are relevance. Default: EPerformanceDebugLevel.Info

  #### Type parameters

  - #### TDetail: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>

  #### Parameters

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancemarkoptions" class="tsd-signature-type">TPerformanceMarkOptions</a>\<TDetail\>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<TDetail\>

### clearMarks

- clearMarks(name?: string): void

<!-- -->

- Removes a specific mark by name or all marks if no name provided

  remarks  
  the default implementation removes marks the browser's performance timeline as well.

  #### Parameters

  - ##### Optional name: string

  #### Returns void

### Protected createMark

- createMark\<TDetail\>(type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string, groupId: string, detail: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tprocesseddetail" class="tsd-signature-type">TProcessedDetail</a>\<TDetail\>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<TDetail\>

<!-- -->

- Creates and returns [TSciChartPerformanceMark](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html)

  remarks  
  default implementation adds mark to the browser's performance timeline. Alternatively, override this to add a mark to a separate collection without polluting the global object. [MDN Reference](https://developer.mozilla.org/docs/Web/API/PerformanceMark)

  #### Type parameters

  - #### TDetail: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>

  #### Parameters

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string

  - ##### groupId: string

  - ##### detail: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tprocesseddetail" class="tsd-signature-type">TProcessedDetail</a>\<TDetail\>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<TDetail\>

### createMeasure

- createMeasure(type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string, name?: string, contextId?: string): PerformanceMeasure

<!-- -->

- Creates a performance measure based on the given mark type

  #### Parameters

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string

  - ##### Optional name: string

  - ##### Optional contextId: string

  #### Returns PerformanceMeasure

### createMeasureFromEndMark

- createMeasureFromEndMark(mark: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a>, name?: string): PerformanceMeasure

<!-- -->

- #### Parameters

  - ##### mark: <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html#performancemark" class="tsd-signature-type">PerformanceMark</a>

  - ##### Optional name: string

  #### Returns PerformanceMeasure

### getMarks

- getMarks(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>\>\[\]

<!-- -->

- Retrieves the marks.

  remarks  
  the default implementation will return all PerformanceMark instances the browser's performance timeline (e.g. created via performance.mark)

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>\>\[\]

### getMeasures

- getMeasures(): PerformanceEntryList

<!-- -->

- #### Returns PerformanceEntryList

### Static clearMarks

- clearMarks(name?: string): void

<!-- -->

- #### Parameters

  - ##### Optional name: string

  #### Returns void

### Static getMarks

- getMarks(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>\>\[\]

<!-- -->

- inheritdoc  
  {@link PerformanceDebugHelper.getMarks{}

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>\>\[\]

### Static getMeasures

- getMeasures(): PerformanceEntryList

<!-- -->

- #### Returns PerformanceEntryList

### Static mark

- mark\<TDetail\>(type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string, options?: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancemarkoptions" class="tsd-signature-type">TPerformanceMarkOptions</a>\<TDetail\>): <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<TDetail\>

<!-- -->

- #### Type parameters

  - #### TDetail: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancedetail" class="tsd-signature-type">TPerformanceDetail</a>

  #### Parameters

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string

  - ##### Optional options: <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tperformancemarkoptions" class="tsd-signature-type">TPerformanceMarkOptions</a>\<TDetail\>

  #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/tscichartperformancemark.html" class="tsd-signature-type">TSciChartPerformanceMark</a>\<TDetail\>

### Static measure

- measure(type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string, name?: string, contextId?: string): PerformanceMeasure

<!-- -->

- #### Parameters

  - ##### type: <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eperformancemarktype.html" class="tsd-signature-type">EPerformanceMarkType</a> \| string

  - ##### Optional name: string

  - ##### Optional contextId: string

  #### Returns PerformanceMeasure

### Static outputLogs

- outputLogs(): void

<!-- -->

- #### Returns void

### Static toJSON

- toJSON(): <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartperformancedata" class="tsd-signature-type">TSciChartPerformanceData</a>

<!-- -->

- #### Returns <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#tscichartperformancedata" class="tsd-signature-type">TSciChartPerformanceData</a>

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
