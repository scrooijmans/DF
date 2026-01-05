On this page

# The Line Segment Series Type

Line segment is a part of a straight line that is bounded by two distinct endpoints. <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinesegmentrenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineSegmentRenderableSeriesðŸ“˜</a> defines a line-series or line chart type in the SciChart's High Performance Real-time JavaScript Charts

Here is a simple Line Segment Series made using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>

``` prism-code
const lineSegmentPoints = [
    [
        [4, 3], // line start, on chart represented as red
        [2, 1]  // line end, on chart represented as blue
    ],
    [
        [4, 4],
        [2, 6]
    ],
    [
        [7, 6],
        [5, 4]
    ],
    [
        [7, 1],
        [5, 3]
    ]
];

const xValues = lineSegmentPoints.flat().map(d => d[0]); // [4, 2, 4, 2, 7, 5, 7, 5]
const yValues = lineSegmentPoints.flat().map(d => d[1]); // [3, 1, 4, 6, 6, 4, 1, 3]

const lineSegmentSeries = new FastLineSegmentRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues
    }),
    strokeThickness: 4,
    paletteProvider: new LineSegmentPaletteProvider()
});
```

## Properties<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-segment-renderable-series/#properties" class="hash-link" aria-label="Direct link to Properties" translate="no" title="Direct link to Properties">â€‹</a>

- dataSeries: we can use <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xyxydataseries.html" rel="noopener noreferrer" target="_blank">XyxyDataSeriesðŸ“˜</a> and <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>. The main difference is that when using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> we can use custom paletteProvider like `LineSegmentPaletteProvider` and that enables different colors on start and end of the line
- strokeThickness: defines thickness of the stroke
- paletteProvider: here is example of custom paletteProvider that is available when using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a>:

``` prism-code
class LineSegmentPaletteProvider implements IStrokePaletteProvider {
    public readonly strokePaletteMode = EStrokePaletteMode.GRADIENT;
    private readonly palettedStart = parseColorToUIntArgb("red");
    private readonly palettedEnd = parseColorToUIntArgb("blue");

    public onAttached(parentSeries: IRenderableSeries): void {}

    public onDetached(): void {}

    public overrideStrokeArgb(xValue: number, yValue: number, index: number): number {
        return index % 2 === 0 ? this.palettedStart : this.palettedEnd;
    }

    public toJSON(): TPaletteProviderDefinition {
        return { type: EPaletteProviderType.Custom, customType: "MyPaletteProvider" };
    }
}
```

![](out_scichartv4/2d-charts/chart-types/fast-line-segment-renderable-series/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

When using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> with <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/fastlinesegmentrenderableseries.html" rel="noopener noreferrer" target="_blank">FastLineSegmentRenderableSeriesðŸ“˜</a> we can use custom paletteProvider that enables different colors on start and end of the line.

## Examples<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-segment-renderable-series/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

### Line Segment Example with XyDataSeries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-segment-renderable-series/#line-segment-example-with-xydataseries" class="hash-link" aria-label="Direct link to Line Segment Example with XyDataSeries" translate="no" title="Direct link to Line Segment Example with XyDataSeries">â€‹</a>

This example is using <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/xydataseries.html" rel="noopener noreferrer" target="_blank">XyDataSeriesðŸ“˜</a> to create a simple line segment series.

``` prism-code
const lineSegmentPoints = [
    [
        [4, 3], // line start, on chart represented as red
        [2, 1]  // line end, on chart represented as blue
    ],
    [
        [4, 4],
        [2, 6]
    ],
    [
        [7, 6],
        [5, 4]
    ],
    [
        [7, 1],
        [5, 3]
    ]
];

const xValues = lineSegmentPoints.flat().map(d => d[0]); // [4, 2, 4, 2, 7, 5, 7, 5]
const yValues = lineSegmentPoints.flat().map(d => d[1]); // [3, 1, 4, 6, 6, 4, 1, 3]

const lineSegmentSeries = new FastLineSegmentRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues,
        yValues
    }),
    strokeThickness: 4,
    paletteProvider: new LineSegmentPaletteProvider()
});
```

### Line Segment Example with XyxyDataSeries<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-segment-renderable-series/#line-segment-example-with-xyxydataseries" class="hash-link" aria-label="Direct link to Line Segment Example with XyxyDataSeries" translate="no" title="Direct link to Line Segment Example with XyxyDataSeries">â€‹</a>

This example is using XyxyDataSeries

``` prism-code
const lineSegmentPoints = [
    [4, 3, 2, 1], // [x,y,x1,y1]
    [4, 4, 2, 6],
    [7, 6, 5, 4],
    [7, 1, 5, 3]
];

const xValues = lineSegmentPoints.map(d => d[0]); // [4,4,7,7,]
const yValues = lineSegmentPoints.map(d => d[1]); // [3,4,6,1]
const x1Values = lineSegmentPoints.map(d => d[2]); // [2,2,5,5]
const y1Values = lineSegmentPoints.map(d => d[3]); // [1,6,4,3]

const lineSegmentSeries = new FastLineSegmentRenderableSeries(wasmContext, {
    dataSeries: new XyxyDataSeries(wasmContext, {
        xValues,
        yValues,
        x1Values,
        y1Values
    }),
    strokeThickness: 4,
    stroke: "cornflowerblue"
});
```

### Gradient Field Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-line-segment-renderable-series/#gradient-field-example" class="hash-link" aria-label="Direct link to Gradient Field Example" translate="no" title="Direct link to Gradient Field Example">â€‹</a>

``` prism-code
class LineSegmentPaletteProvider implements IStrokePaletteProvider {
    public readonly strokePaletteMode = EStrokePaletteMode.GRADIENT;
    private readonly palettedStart = parseColorToUIntArgb("red");
    private readonly palettedEnd = parseColorToUIntArgb("blue");

    public onAttached(parentSeries: IRenderableSeries): void {}
    
    public onDetached(): void {}

    public overrideStrokeArgb(xValue: number, yValue: number, index: number): number {
        return index % 2 === 0 ? this.palettedStart : this.palettedEnd;
    }

    public toJSON(): TPaletteProviderDefinition {
        return { type: EPaletteProviderType.Custom, customType: "MyPaletteProvider" };
    }
}

async function gradientField(divElementId) {
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });

    // configure central axes
    const layoutManager = new CentralAxesLayoutManager();

    sciChartSurface.layoutManager = layoutManager;

    const xMin = -15;
    const xMax = 15;
    const yMin = -10;
    const yMax = 10;
    
    const xAxis = new NumericAxis(wasmContext, {
        axisBorder: { color: "white", borderBottom: 1 },
        visibleRange: new NumberRange(xMin, xMax),
        autoRange: EAutoRange.Never,
        drawMajorBands: false,
        drawMajorGridLines: false,
        drawMinorGridLines: false,
        zoomExtentsToInitialRange: true
    });
    sciChartSurface.xAxes.add(xAxis);
    const yAxis = new NumericAxis(wasmContext, {
        axisBorder: { color: "white", borderRight: 1 },
        visibleRange: new NumberRange(yMin, yMax),
        autoRange: EAutoRange.Never,
        drawMajorBands: false,
        drawMajorGridLines: false,
        drawMinorGridLines: false,
        zoomExtentsToInitialRange: true
    });
    sciChartSurface.yAxes.add(yAxis);

    // For FastLineSegmentRenderableSeries with palette provider having SOLID palette mode the first color is used
    // However for LineRendereableSeries with the same palette provider and SOLID palette the second color is used
    const lineSegmentSeries = new FastLineSegmentRenderableSeries(wasmContext, {
        strokeThickness: 4,
        paletteProvider: new LineSegmentPaletteProvider()
    });
    lineSegmentSeries.rolloverModifierProps.tooltipColor = "brown";

    sciChartSurface.renderableSeries.add(lineSegmentSeries);

    const multiplier = 0.01;
    const dataSeries = new XyDataSeries(wasmContext);
    for (let x = xMin; x <= xMax; x++) {
        for (let y = yMin; y <= yMax; y++) {
            // start point
            dataSeries.append(x, y);
            // end point
            const xEnd = x + (x * x - y * y - 4) * multiplier;
            const yEnd = y + 2 * x * y * multiplier;
            dataSeries.append(xEnd, yEnd);
        }
    }
    lineSegmentSeries.dataSeries = dataSeries;

    sciChartSurface.chartModifiers.add(new ZoomPanModifier());
    sciChartSurface.chartModifiers.add(new ZoomExtentsModifier());
    sciChartSurface.chartModifiers.add(new MouseWheelZoomModifier());
}
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/fast-line-segment-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/fast-line-segment-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
