On this page

# Axis APIs - Convert Pixel to Data Coordinates

SciChart.js provides a clean and simple API to transform pixels to data-values and vice versa via the CoordinateCalculator API.

## Where Pixel Coordinates are measured from<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates/#where-pixel-coordinates-are-measured-from" class="hash-link" aria-label="Direct link to Where Pixel Coordinates are measured from" translate="no" title="Direct link to Where Pixel Coordinates are measured from">â€‹</a>

It is important to note when converting Pixels to Data Coordinates and vice versa that pixels are measured from the top-left inside corner of the series area of the chart known as theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#viewrect" rel="noopener noreferrer" target="_blank">viewRectðŸ“˜</a>. So, the pixel coordinate (0,0) corresponds to the data-value at \[xAxis.visibleRange.min, yAxis.visibleRange.max\] and the pixel coordinate (Width, Height) corresponds to the data-value at \[xAxis.visibleRange.max, yAxis.visibleRange.min\].

![](out_scichartv4/2d-charts/axis-api/misc/pixel-and-data-coordinates/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Learn about Axis.VisibleRange and how to get/set this property at the page:Â [Axis Ranging - Setting and Getting VisibleRange](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/ranging-scaling/set-range-zoom-to-fit)

<img src="out_scichartv4/2d-charts/axis-api/misc/pixel-and-data-coordinates/index_media/fb93f3ca8ba6009a760f88fb8dd280f8052549de.png" class="img_ev3q" decoding="async" loading="lazy" width="899" height="598" alt="Converting pixel coordinates to data coordinates using SciChart.js Fast Realtime JavaScript Charts" />

## Converting between Pixels and Data Coordinates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates/#converting-between-pixels-and-data-coordinates" class="hash-link" aria-label="Direct link to Converting between Pixels and Data Coordinates" translate="no" title="Direct link to Converting between Pixels and Data Coordinates">â€‹</a>

To convert between pixel and data coordinates, you must first get aÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html" rel="noopener noreferrer" target="_blank">CoordinateCalculatorðŸ“˜</a> instance. This is retrieved with the following code.

- Getting a CoordinateCalculator

``` prism-code
const xAxis = sciChartSurface.xAxes.get(0); // Type AxisBase2D
const coordCalc = xAxis.getCurrentCoordinateCalculator(); // Type CoordinateCalculatorBase
```

Data-values are converted to pixel coordinates via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getcoordinate" rel="noopener noreferrer" target="_blank">coordinateCalculator.getCoordinate()ðŸ“˜</a> method. Also, Coordinates in pixels are converted back to chart data-values via theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/coordinatecalculatorbase.html#getdatavalue" rel="noopener noreferrer" target="_blank">coordinateCalculator.getDataValue()ðŸ“˜</a>Â method. It expects a coordinate in pixels and returns the closest data value to that coordinate.

All coordinates are relative to theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/axisbase2d.html#viewrect" rel="noopener noreferrer" target="_blank">viewRectðŸ“˜</a> - the area where series are drawn inside the axis on theÂ [SciChartSurface](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview).

You can find some examples how to do the conversions below.

### Converting NumericAxis Data to Pixels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates/#converting-numericaxis-data-to-pixels" class="hash-link" aria-label="Direct link to Converting NumericAxis Data to Pixels" translate="no" title="Direct link to Converting NumericAxis Data to Pixels">â€‹</a>

TheÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#numericaxis" rel="noopener noreferrer" target="_blank">NumericAxisðŸ“˜</a> is aÂ Value-Axis which uses data-values for measurement. It can be used to display numbers, or dates (stored as unix time stamps) formatted as date/time using theÂ [LabelProvider](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/label-provider-api-overview) feature.

To convert between pixel-coordinates relative to viewRect and data-values on a NumericAxis, use the following code.

- TS

``` prism-code
const xAxis = sciChartSurface.xAxes.get(0); // Type AxisBase2D
const coordCalc = xAxis.getCurrentCoordinateCalculator(); // Type CoordinateCalculatorBase
// Gets the pixel coordinate relative to viewRect for data-value 1.23 on this axis only
const coord = coordCalc.getCoordinate(1.23);

// Converts a pixel coordinate back to dataValue
const dataValue= coordCalc.getDataValue(coord);

// dataValue should === 1.23
```

### Converting CategoryAxis Data to/from Pixels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates/#converting-categoryaxis-data-tofrom-pixels" class="hash-link" aria-label="Direct link to Converting CategoryAxis Data to/from Pixels" translate="no" title="Direct link to Converting CategoryAxis Data to/from Pixels">â€‹</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/eaxistype.html#categoryaxis" rel="noopener noreferrer" target="_blank">CategoryAxisðŸ“˜</a> are treated slightly differently. This axis type can also beÂ used to display numbers, or dates (stored as unix time stamps) but we must perform an extra step to convert between data-value, index and pixel coordinate.

AÂ Category Axis uses the index to data not the data-value itselfÂ for measurement. Learn more about Category Axis at the pageÂ [Category Axis in SciChart.js](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-types/category-axis)

- Category Coordinate conversion

``` prism-code
import { CategoryCoordinateCalculator, CategoryAxis } from "scichart";

const xAxis = new CategoryAxis(wasmContext);
// Get the CoordinateCalculator
const coordCalc = xAxis.getCurrentCoordinateCalculator();
// OR TypeScript only, cast as CategoryCoordinateCalculator
const coordCalc = xAxis.getCurrentCoordinateCalculator() as CategoryCoordinateCalculator;
// Get the pixel coordinate at index=10
const coord = coordCalc.getCoordinate(10);
// Convert a coordinate back to index
const index = coordCalc.getDataValue(coord);
// Convert an index to data-value
const dataValue = coordCalc.transformIndexToData(index);
```

## Transforming Pixels to the ViewRect<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/pixel-and-data-coordinates/#transforming-pixels-to-the-viewrect" class="hash-link" aria-label="Direct link to Transforming Pixels to the ViewRect" translate="no" title="Direct link to Transforming Pixels to the ViewRect">â€‹</a>

Functions exist to translate a point from the parent canvas of the chart to the viewRect. This is useful if you want to transform a mouse-coordinate on the parent canvas into a data-value on the chart. For more info about his, se theÂ [Hit-Test API section](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/hit-test-api/hit-test-api-overview).

<img src="out_scichartv4/2d-charts/axis-api/misc/pixel-and-data-coordinates/index_media/0be6df27c98d390114a3867ebef479a489473b66.png" class="img_ev3q" decoding="async" loading="lazy" width="899" height="598" />

***Above**: The SciChartSurface sits on a canvas in the DOM. The viewRect is the inner area which draws the series.*

To transform a point on the parent SciChartSurface to the viewRect, use the following code:

- Transforming Points from viewRect

``` prism-code
import { Point } from "../../Core/Point";
import { Rect } from "../../types/Rect";
import { translateFromCanvasToSeriesViewRect } from "../../utils/translate";

const sciChartSurface; // Assuming a SciChartSurface instance
const point = new Point(100, 200);
const viewRectPoint = translateFromCanvasToSeriesViewRect(point, sciChartSurface.seriesViewRect);
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/misc/pixel-and-data-coordinates/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/misc/pixel-and-data-coordinates/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
