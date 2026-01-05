On this page

# Axis APIs â€“ Convert Polar Pixel to Data Coordinates

SciChart.js provides a dedicated API for transforming between polar pixel coordinates and data coordinates, enabling advanced annotations, hit-testing, and custom rendering on polar charts.

## Understanding Polar Coordinates in SciChart.js<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/#understanding-polar-coordinates-in-scichartjs" class="hash-link" aria-label="Direct link to Understanding Polar Coordinates in SciChart.js" translate="no" title="Direct link to Understanding Polar Coordinates in SciChart.js">â€‹</a>

In polar charts, coordinates are defined by an **angle** (Î¸) and a **radius** (r) from the chart's center, while Cartesian coordinates are defined by (x, y) positions.

### Where Polar Pixel Coordinates Are Measured From<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/#where-polar-pixel-coordinates-are-measured-from" class="hash-link" aria-label="Direct link to Where Polar Pixel Coordinates Are Measured From" translate="no" title="Direct link to Where Polar Pixel Coordinates Are Measured From">â€‹</a>

- **Origin**: The center of the polar chart (not the top-left corner as in Cartesian charts).
- **Angle**: Measured from the axis' start angle, increasing in the direction set by the chart configuration.
- **Radius**: Measured outward from the center.

All conversions are relative to the **series area** (viewRect) of the chart.

## Converting Polar Data to Cartesian (Pixel) Coordinates<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/#converting-polar-data-to-cartesian-pixel-coordinates" class="hash-link" aria-label="Direct link to Converting Polar Data to Cartesian (Pixel) Coordinates" translate="no" title="Direct link to Converting Polar Data to Cartesian (Pixel) Coordinates">â€‹</a>

To convert polar data (angle, radius) to Cartesian (pixel) coordinates for rendering or hit-testing, use the <a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html#annotationhelpers.convertpolartocartesian" rel="noopener noreferrer" target="_blank">annotationHelpers.convertPolarToCartesian()ðŸ“˜</a> function:

**Parameters:**

- `angularAxis`: The polar angular axis instance.
- `usePixelRatio`: Whether to apply device pixel ratio scaling.
- `wasmContext`: The WebAssembly context for SciChart.
- `coordinateMode`: Indicates if the angle/radius are in pixels, data values, or relative units.
- `angle`: The angle in radians
- `radius`: The radius in data units

## Usage Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/#usage-example" class="hash-link" aria-label="Direct link to Usage Example" translate="no" title="Direct link to Usage Example">â€‹</a>

Convert a data-value angle and radius to pixel coordinates for rendering an annotation:

``` prism-code
const angularAxis = sciChartSurface.xAxes.get(0) as PolarAxisBase;

const angle = 1.57; // Radians, for example
const radius = 100; // Data units

const { x, y } = convertPolarToCartesian(
    angularAxis,
    true, // usePixelRatio
    wasmContext,
    ECoordinateMode.DataValue,
    angle,
    radius
);
// (x, y) are now pixel coordinates relative to the series area (viewRect)
```

## Practical Applications<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/#practical-applications" class="hash-link" aria-label="Direct link to Practical Applications" translate="no" title="Direct link to Practical Applications">â€‹</a>

- **Hit-Testing**: To determine if a mouse event hits a data point or annotation, convert the event's pixel coordinates to polar data coordinates and compare.
- **Custom Rendering**: Any custom drawing on a polar chart surface should use this conversion to ensure correct placement.

## Hit-Testing in Polar Charts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/#hit-testing-in-polar-charts" class="hash-link" aria-label="Direct link to Hit-Testing in Polar Charts" translate="no" title="Direct link to Hit-Testing in Polar Charts">â€‹</a>

When hit-testing, convert the mouse (x, y) pixel coordinates to polar coordinates, then compare with data points.

For reference, here is our actual implementation of a hit-test provider for the <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polarlinerenderableseries.html" rel="noopener noreferrer" target="_blank">PolarLineRenderableSeriesðŸ“˜</a>:

``` prism-code
import { 
    BaseHitTestProvider,
    PolarLineRenderableSeries,
    PolarAxisBase,
    Point,
    HitTestInfo,
    XyDataSeries,
    hitTestHelpers,
    annotationHelpers,
    ECoordinateMode,
    calcDistance, calcDistanceFromLine,
    PolarDataPointHitTestProvider,
    DpiHelper 
} from "scichart";

const DEFAULT_RADIUS = 10;

/**
 * Hit-test provider for {@link PolarLineRenderableSeries}. See base class {@link BaseHitTestProvider} for further info
 */
export class PolarLineSeriesHitTestProvider extends PolarDataPointHitTestProvider {
    /** @inheritDoc */
    public hitTest(x: number, y: number, hitTestRadius: number = DEFAULT_RADIUS): HitTestInfo {
        // convert to polar and add necessary offset
        const hitTestPoint = this.getTranslatedHitTestPoint(x, y);
        if (!hitTestPoint) {
            return HitTestInfo.empty();
        }

        const { xCoordinateCalculator, yCoordinateCalculator, isVerticalChart } = this.currentRenderPassData;
        const xHitCoord = hitTestPoint.x;
        const yHitCoord = hitTestPoint.y;
        const dataSeries = this.parentSeries.dataSeries as XyDataSeries;
        if (!dataSeries) {
            return HitTestInfo.empty();
        }

        const xNativeValues = dataSeries.getNativeXValues();
        const yNativeValues = dataSeries.getNativeYValues();
        const xPolarAxis = this.parentSeries.xAxis as PolarAxisBase;
        const getCartesianCoordsFn = (index$: number): Point => {
            const x = xNativeValues.get(index$);
            const y = yNativeValues.get(index$);
            const xCoord = xCoordinateCalculator.getCoordinate(x);
            const yCoord = yCoordinateCalculator.getCoordinate(y);
            const angle = isVerticalChart ? yCoord : xCoord;
            const radius = isVerticalChart ? xCoord : yCoord;
            const res = annotationHelpers.convertPolarToCartesian(
                xPolarAxis,
                false,
                this.webAssemblyContext,
                ECoordinateMode.Pixel,
                angle * DpiHelper.PIXEL_RATIO,
                radius
            );
            return new Point(res.x, res.y);
        };
        let minDistance = Number.MAX_VALUE;
        let minDistanceIndex1 = -1;
        let minDistanceIndex2 = -1;
        const updateMinDistFn = (dist$: number, ind1$: number, ind2$: number = -1) => {
            if (dist$ < minDistance) {
                minDistance = dist$;
                minDistanceIndex1 = ind1$;
                minDistanceIndex2 = ind2$;
            }
        };
        if (dataSeries.count() > 0) {
            let point1Coords = getCartesianCoordsFn(0);
            let distanceToPoint1 = calcDistance(xHitCoord, yHitCoord, point1Coords.x, point1Coords.y);
            updateMinDistFn(distanceToPoint1, 0);
            for (let i = 1; i < dataSeries.count(); i++) {
                const point2Coords = getCartesianCoordsFn(i);
                const lineSegmentLength = calcDistance(point1Coords.x, point1Coords.y, point2Coords.x, point2Coords.y);
                const distanceToPoint2 = calcDistance(xHitCoord, yHitCoord, point2Coords.x, point2Coords.y);
                updateMinDistFn(distanceToPoint2, i);
                const isHitPointInLineSegmentVicinity =
                    distanceToPoint1 < lineSegmentLength && distanceToPoint2 < lineSegmentLength;
                if (isHitPointInLineSegmentVicinity) {
                    const distanceToLine = calcDistanceFromLine(
                        xHitCoord,
                        yHitCoord,
                        point1Coords.x,
                        point1Coords.y,
                        point2Coords.x,
                        point2Coords.y
                    );
                    if (distanceToLine < minDistance) {
                        if (distanceToPoint1 < distanceToPoint2) {
                            updateMinDistFn(distanceToLine, i - 1, i);
                        } else {
                            updateMinDistFn(distanceToLine, i, i - 1);
                        }
                    }
                }
                point1Coords = point2Coords;
                distanceToPoint1 = distanceToPoint2;
            }
        }

        const polarHitTestPoint = xPolarAxis.reverseTransform(hitTestPoint.x, hitTestPoint.y);

        const hitTestInfo = hitTestHelpers.createHitTestInfo(
            this.parentSeries,
            xCoordinateCalculator,
            yCoordinateCalculator,
            isVerticalChart,
            dataSeries,
            xNativeValues,
            yNativeValues,
            polarHitTestPoint.x,
            polarHitTestPoint.y,
            minDistanceIndex1,
            hitTestRadius
        );
        hitTestInfo.isHit = minDistance < hitTestRadius;
        return hitTestInfo;
    }
}
```

![](out_scichartv4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

The inverse function of `convertPolarToCartesian()` is <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/polaraxisbase.html#reversetransform" rel="noopener noreferrer" target="_blank">reverseTransform()ðŸ“˜</a>, which converts from cartesian to polar coordinates.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/misc/polar-pixel-and-data-coordinates/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
