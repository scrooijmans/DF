On this page

# Custom Subtypes

From time to time, you may need to create custom subtypes in order to fully access the powerful, customisable API that SciChart.js has to offer.

The [PaletteProvider API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/palette-provider-api/palette-provider-api-overview) isÂ one exampleÂ where you may have to provide your own implementation of an interface to a SciChart series. If you want your custom paletteProvider to be serialised and deserialised, you need to implement **toJSON** on it.

Here is an example below:

- JS
- TS

``` prism-code
import { 
    EFillPaletteMode, 
    EStrokePaletteMode, 
    IFillPaletteProvider, 
    IStrokePaletteProvider,
    parseColorToUIntArgb,
    EPaletteProviderType,
    EBaseType,
} from "scichart";

// ...

class ExampleMountainPaletteProvider {
    constructor(options) {
        this.options = options;
        this.palettedStroke = parseColorToUIntArgb(options.stroke);
        this.palettedFill = parseColorToUIntArgb(options.fill);
        this.strokePaletteMode = EStrokePaletteMode.SOLID;
        this.fillPaletteMode = EFillPaletteMode.SOLID;
    }

    onAttached(parentSeries) { }

    onDetached() { }

    overrideFillArgb(xValue, yValue, index) {
        if (yValue > 0.5 && yValue < 0.75) {
            return this.palettedFill;
        } else {
            return undefined;
        }
    }

    overrideStrokeArgb(xValue, yValue, index) {
        if (yValue > 0.5 && yValue < 0.75) {
            return this.palettedStroke;
        } else {
            return undefined;
        }
    }
    
    // Add a toJSON method so this can be serialized.
    toJSON() {
        return {
            type: EPaletteProviderType.Custom,
            customType: "ExampleMountainPaletteProvider",
            options: this.options
        };
    }
}
```

``` prism-code
import { 
    EFillPaletteMode, 
    EStrokePaletteMode, 
    IFillPaletteProvider, 
    IStrokePaletteProvider,
    parseColorToUIntArgb,
    EBaseType,
    EPaletteProviderType,
    TPaletteProviderDefinition 
} from "scichart";

// ...

class ExampleMountainPaletteProvider implements IStrokePaletteProvider, IFillPaletteProvider {
    public static Name: "ExampleMountain";
    public readonly strokePaletteMode = EStrokePaletteMode.SOLID;
    public readonly fillPaletteMode = EFillPaletteMode.SOLID;
    private readonly palettedStroke: number;
    private readonly palettedFill: number;
    private readonly options: { stroke: string; fill: string };

    constructor(options: { stroke: string; fill: string }) {
        this.options = options;
        this.palettedStroke = parseColorToUIntArgb(options.stroke);
        this.palettedFill = parseColorToUIntArgb(options.fill);
    }

    public onAttached(parentSeries: IRenderableSeries): void { }

    public onDetached(): void { }

    public overrideFillArgb(xValue: number, yValue: number, index: number): number {
        if (yValue > 0.5 && yValue < 0.75) {
            return this.palettedFill;
        } else {
            return undefined;
        }
    }

    public overrideStrokeArgb(xValue: number, yValue: number, index: number): number {
        if (yValue > 0.5 && yValue < 0.75) {
            return this.palettedStroke;
        } else {
            return undefined;
        }
    }

    // Add a toJSON method so this can be serialized.
    public toJSON(): TPaletteProviderDefinition {
        return {
            type: EPaletteProviderType.Custom,
            customType: ExampleMountainPaletteProvider.Name,
            options: this.options
        };
    }
}
```

Once you have created your custom type and implemented `toJSON()`, next you will need to register the type with the builder API to be able to use it.

``` prism-code
import { chartBuilder } from "scichart";

// Register it for use by the builder api
chartBuilder.registerType(
    EBaseType.PaletteProvider,
    "ExampleMountainPaletteProvider",
    (options) => new ExampleMountainPaletteProvider(options)
);
```

Now the usage of the custom type can be done as follows. It will appear to the Builder API as just another type like those already existing in SciChart.

``` prism-code
import { chartBuilder } from "scichart";

// Build the surface
const { sciChartSurface, wasmContext } = await chartBuilder.build2DChart(divElementId, {
    series: {
        type: ESeriesType.MountainSeries,
        options: {
            // Specify the custom palette provider which was previously registered
            paletteProvider: {
                type: EPaletteProviderType.Custom,
                customType: "ExampleMountainPaletteProvider",
                options: { stroke: "lime", fill: "yellow" }
            },
        }
    }
});
```

Remember that the definition of your custom class, and the registration of it, must also occur on the client that will be using it, before it is used in a SciChart chart.

## Custom Types Example<a href="https://www.scichart.com/documentation/js/v4/2d-charts/builder-api/custom-subtypes/#custom-types-example" class="hash-link" aria-label="Direct link to Custom Types Example" translate="no" title="Direct link to Custom Types Example">â€‹</a>

For a full example of how to use Custom Types with the Builder API, see the SciChart.js demo <a href="https://www.scichart.com/demo/javascript-custom-types" rel="noopener noreferrer" target="_blank">https://www.scichart.com/demo/javascript-custom-types</a>.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/builder-api/custom-subtypes/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/builder-api/custom-subtypes/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
