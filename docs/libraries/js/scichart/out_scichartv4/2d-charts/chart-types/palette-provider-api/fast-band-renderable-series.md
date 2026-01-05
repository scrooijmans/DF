On this page

# Per-Point Colouring of Band Segments

## Colour Band Series Segments with PaletteProvider<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series/#colour-band-series-segments-with-paletteprovider" class="hash-link" aria-label="Direct link to Colour Band Series Segments with PaletteProvider" translate="no" title="Direct link to Colour Band Series Segments with PaletteProvider">â€‹</a>

Let's create aÂ [FastBandRenderableSeries](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/fast-band-renderable-series).

- TS

``` prism-code
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
     theme: new SciChartJSLightTheme()
 });

 sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
 sciChartSurface.yAxes.add(new NumericAxis(wasmContext, { growBy: new NumberRange(0.4, 0.4) }));
 const dataSeries = new XyyDataSeries(wasmContext);
 const POINTS = 20;
 const STEP = (3 \* Math.PI) / POINTS;

 for (let i = 0; i <= POINTS; i++) {
     const k = 1 - i / 2000;
     const y = Math.sin(i \* STEP) \* k \* 0.7;
     const y1 = Math.cos(i \* STEP) \* k;
     dataSeries.append(i, y, y1);
 }

 const rendSeries = new FastBandRenderableSeries(wasmContext, {
     dataSeries,
     strokeThickness: 7,
     fill: 'rgba(39,155,39,0.7)',
     fillY1: 'rgba(255,25,25,0.7)',
     stroke: '#FF1919',
     strokeY1: '#279B27',
     opacity: 1,
     pointMarker: new EllipsePointMarker(wasmContext, {
         width: 14,
         height: 14,
         strokeThickness: 4,
         stroke: '#FFFF33',
         fill: '#33FF33',
         opacity: 1
     }),
 });

 sciChartSurface.renderableSeries.add(rendSeries);
 sciChartSurface.zoomExtents();
```

<img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series/index_media/1f5ca6220b086964d801cbeeeb681091d75ee7b4.png" class="img_ev3q" decoding="async" loading="lazy" width="1754" height="1326" />

Next we create a BandPaletteProvider by implementingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/istrokepaletteprovider.html" rel="noopener noreferrer" target="_blank">IStrokePaletteProviderðŸ“˜</a>,Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ifillpaletteprovider.html" rel="noopener noreferrer" target="_blank">IFillPaletteProviderðŸ“˜</a> andÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ipointmarkerpaletteprovider.html" rel="noopener noreferrer" target="_blank">IPointMarkerPaletteProviderðŸ“˜</a> interfaces in typescript, or extendingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/defaultpaletteprovider.html" rel="noopener noreferrer" target="_blank">DefaultPaletteProviderðŸ“˜</a> in JavaScript.

- JS
- TS

``` prism-code
// PaletteProvider implementation which colours stroke, fill and pointmarker
class BandPaletteProvider extends DefaultPaletteProvider {
    constructor() {
        super();
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.fillPaletteMode = EFillPaletteMode.SOLID;
        this.limeStroke = parseColorToUIntArgb("lime");
        this.yellowFill = parseColorToUIntArgb("rgba(255,255,0,0.7)");
        this.markerRedStroke = parseColorToUIntArgb("red");
        this.markerBlueFill = parseColorToUIntArgb("blue");
    }
    overrideFillArgb(xValue, yValue, index, opacity, metadata) {
        if (xValue >= 6 && xValue <= 12) {
            return opacity !== undefined ? uintArgbColorMultiplyOpacity(this.yellowFill, opacity) : this.yellowFill;
        }
        return undefined;
    }
    overrideStrokeArgb(xValue, yValue, index, opacity, metadata) {
        if (xValue >= 6 && xValue <= 12) {
            return opacity !== undefined ? uintArgbColorMultiplyOpacity(this.limeStroke, opacity) : this.limeStroke;
        }
        return undefined;
    }
    overridePointMarkerArgb(xValue, yValue, index, opacity, metadata) {
        if (xValue >= 6 && xValue <= 12) return { stroke: this.markerRedStroke, fill: this.markerBlueFill };
        return undefined;
    }
}
```

``` prism-code
// PaletteProvider implementation which colours stroke, fill and pointmarker
class BandPaletteProvider implements IStrokePaletteProvider, IFillPaletteProvider, IPointMarkerPaletteProvider {
    public readonly strokePaletteMode = EStrokePaletteMode.SOLID;
    public readonly fillPaletteMode = EFillPaletteMode.SOLID;
    private readonly limeStroke = parseColorToUIntArgb("lime");
    private readonly yellowFill = parseColorToUIntArgb("rgba(255,255,0,0.7)");
    private readonly markerRedStroke = parseColorToUIntArgb("red");
    private readonly markerBlueFill = parseColorToUIntArgb("blue");
    public onAttached(parentSeries: IRenderableSeries): void {}
    public onDetached(): void {}
    public overrideFillArgb(
        xValue: number,
        yValue: number,
        index: number,
        opacity?: number,
        metadata?: IPointMetadata
    ): number {
        if (xValue >= 6 && xValue <= 12) {
            return opacity !== undefined ? uintArgbColorMultiplyOpacity(this.yellowFill, opacity) : this.yellowFill;
        }
        return undefined;
    }
    public overrideStrokeArgb(
        xValue: number,
        yValue: number,
        index: number,
        opacity?: number,
        metadata?: IPointMetadata
    ): number {
        if (xValue >= 6 && xValue <= 12) {
            return opacity !== undefined ? uintArgbColorMultiplyOpacity(this.limeStroke, opacity) : this.limeStroke;
        }
        return undefined;
    }
    public overridePointMarkerArgb(
        xValue: number,
        yValue: number,
        index: number,
        opacity?: number,
        metadata?: IPointMetadata
    ): TPointMarkerArgb {
        if (xValue >= 6 && xValue <= 12) return { stroke: this.markerRedStroke, fill: this.markerBlueFill };
        return undefined;
    }
}
```

To use it, apply to the Paletteprovider property on a RenderableSeries:

``` prism-code
// Usage
const bandSeries = new FastBandRenderableSeries(wasmContext);
bandSeries.paletteProvider = new BandPaletteProvider();
```

Now we have a paletted band renderable series with fill, stroke and point markers overridden for X in range from 6 to 12.

<img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series/index_media/9c115c1396a423976a01453dfdfaeb028cf465e5.png" class="img_ev3q" decoding="async" loading="lazy" width="1758" height="1324" />

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-types/palette-provider-api/fast-band-renderable-series/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
