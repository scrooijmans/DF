On this page

# Custom Chart Modifier API

The ChartModifierBase API is one of the most powerful APIs in the SciChart library.

Using this API you can create behaviours which you can attach to a chart to perform custom Zooming, Panning, Annotation & Markers, Legend output and much much more. Any time you want to do something in JavaScript or Typescript code to alter the behaviour of a SciChartSurface you should be thinking about creating a custom modifier to do it.

## The ChartModifierBase2D Type<a href="https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/custom-modifiers/custom-modifiers-overview/#the-chartmodifierbase2d-type" class="hash-link" aria-label="Direct link to The ChartModifierBase2D Type" translate="no" title="Direct link to The ChartModifierBase2D Type">â€‹</a>

The <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html" rel="noopener noreferrer" target="_blank">ChartModifierBase2DðŸ“˜</a> provides a base class for all of the ChartModifiers within SciChart. All of our built-in modifiers such as <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/zoompanmodifier.html" rel="noopener noreferrer" target="_blank">ZoomPanModifierðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/rollovermodifier.html" rel="noopener noreferrer" target="_blank">RolloverModifierðŸ“˜</a>, <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/yaxisdragmodifier.html" rel="noopener noreferrer" target="_blank">YAxisDragModifierðŸ“˜</a> inherit ChartModifierBase2D.

You can create your own ChartModifiers by inheriting this class. For example, this code sample shows how to declare a custom ChartModifier class in JavaScript ES6 or TypeScript.

- TypeScript
- JavaScript

``` prism-code
import {ChartModifierBase2D, EChart2DModifierType, ModifierMouseArgs} from "scichart";

export class SimpleChartModifierTs extends ChartModifierBase2D {
    readonly type: EChart2DModifierType = EChart2DModifierType.Custom;

    override modifierMouseDown(args: ModifierMouseArgs) {
        super.modifierMouseDown(args);
        console.log(`MouseDown at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }

    override modifierMouseMove(args: ModifierMouseArgs) {
        super.modifierMouseMove(args);
        console.log(`MouseMove at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }

    override modifierMouseUp(args: ModifierMouseArgs) {
        super.modifierMouseUp(args);
        console.log(`MouseUp at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }

    override modifierDoubleClick(args: ModifierMouseArgs) {
        super.modifierDoubleClick(args);
        console.log(`DoubleClick at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }

    override modifierMouseWheel(args: ModifierMouseArgs) {
        super.modifierMouseWheel(args);
        console.log(`MouseWheel delta=${args.mouseWheelDelta} at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }

    override modifierMouseEnter(args: ModifierMouseArgs) {
        super.modifierMouseEnter(args);
        console.log(`MouseEnter!`);
    }

    override modifierMouseLeave(args: ModifierMouseArgs) {
        super.modifierMouseLeave(args);
        console.log(`MouseLeave!`);
    }
}
```

``` prism-code
import { ChartModifierBase2D, EChart2DModifierType } from "scichart";
export class SimpleChartModifierTs extends ChartModifierBase2D {
    type = EChart2DModifierType.Custom;
    modifierMouseDown(args) {
        super.modifierMouseDown(args);
        console.log(`MouseDown at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }
    modifierMouseMove(args) {
        super.modifierMouseMove(args);
        console.log(`MouseMove at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }
    modifierMouseUp(args) {
        super.modifierMouseUp(args);
        console.log(`MouseUp at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }
    modifierDoubleClick(args) {
        super.modifierDoubleClick(args);
        console.log(`DoubleClick at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }
    modifierMouseWheel(args) {
        super.modifierMouseWheel(args);
        console.log(`MouseWheel delta=${args.mouseWheelDelta} at point ${args.mousePoint.x}, ${args.mousePoint.y}`);
    }
    modifierMouseEnter(args) {
        super.modifierMouseEnter(args);
        console.log(`MouseEnter!`);
    }
    modifierMouseLeave(args) {
        super.modifierMouseLeave(args);
        console.log(`MouseLeave!`);
    }
}
```

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/chart-modifier-api/custom-modifiers/custom-modifiers-overview/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/chart-modifier-api/custom-modifiers/custom-modifiers-overview/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
