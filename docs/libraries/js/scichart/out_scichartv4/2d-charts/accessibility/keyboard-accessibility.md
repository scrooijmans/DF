On this page

# Keyboard Accessibility

A chart behavior is easily extendable with the use of Chart Modifiers. So they can be used to add a keyboard accessibility to the chart. See alsoÂ [What is the ChartModifier API](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/chart-modifier-api-overview/),Â [Common ChartModifiers Features](https://www.scichart.com/documentation/js/v4/2d-charts/chart-modifier-api/common-features/).

## Custom Keyboard Interaction Modifier<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/keyboard-accessibility/#custom-keyboard-interaction-modifier" class="hash-link" aria-label="Direct link to Custom Keyboard Interaction Modifier" translate="no" title="Direct link to Custom Keyboard Interaction Modifier">â€‹</a>

For this example we created a simple custom modifier - **KeyboardZoomPanModifier**, which adds an ability to scroll the chart with arrow buttons and to zoom with **"+"**/**"-"** buttons. The same approach can be used to add more custom keyboard bindings.

Here is the definition of the **KeyboardZoomPanModifier**:

- JS
- TS

``` prism-code
import {
    ChartModifierBase2D,
    EClipMode,
    Point,
    EXyDirection
} from "scichart";

// ...
const DEFAULT_SCROLL_DELTA = 100;
const DEFAULT_ZOOM_DELTA = 120;
class KeyboardZoomPanModifier extends ChartModifierBase2D {
    constructor(options) {
        super(options);
        this.type = "KeyboardZoomPan";
        /**
         * Modifies the speed of  zoom, for example growFactor = 0.001 means each 'click'
         * zooms the chart 0.1%
         */
        this.growFactor = options && options.growFactor || 0.001;

        /**
         * Modifies the speed of scroll, for example scrollFactor = 0.001 means each 'click'
         * scrolls the chart 0.1%
         */
        this.scrollFactor = options && options.scrollFactor || 0.001;
        this.handleKeyDown = this.handleKeyDown.bind(this);
    }
    scroll(xDelta, yDelta) {
        // Scroll the X,YAxis by the number of pixels since the last update
        const token = this.parentSurface.suspendUpdates();
        if ([EXyDirection.XDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.xAxes.asArray().forEach(axis => {
                const delta = (axis.isHorizontalAxis ? xDelta : -yDelta) * this.scrollFactor;
                axis.scroll(axis.flippedCoordinates ? -delta : delta, EClipMode.None);
            });
        }
        if ([EXyDirection.YDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.yAxes.asArray().forEach(axis => {
                const delta = (axis.isHorizontalAxis ? -xDelta : yDelta) * this.scrollFactor;
                axis.scroll(axis.flippedCoordinates ? -delta : delta, EClipMode.None);
            });
        }
        token.resume();
    }
    onAttach() {
        // set tabIndex attribute of the chart root element if it was not set externally
        this.parentSurface.domChartRoot.tabIndex = this.parentSurface.domChartRoot.tabIndex || 0;
        // subscribe to keyboard input event
        this.parentSurface.domChartRoot.addEventListener("keydown", this.handleKeyDown);
    }
    onDetach() {
        // unsubscribe from keyboard input event
        this.parentSurface.domChartRoot.removeEventListener("keydown", this.handleKeyDown);
    }
    /**
     * Performs the zoom operation around the mouse point
     * @param mousePoint The X,Y location of the mouse at the time of the zoom
     * @param delta the delta factor of zoom
     */
    performZoom(mousePoint, delta) {
        const fraction = this.growFactor * delta;
        if ([EXyDirection.XDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.xAxes.asArray().forEach(axis => {
                this.growBy(mousePoint, axis, fraction);
            });
        }
        if ([EXyDirection.YDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.yAxes.asArray().forEach(axis => {
                this.growBy(mousePoint, axis, fraction);
            });
        }
    }
    handleKeyDown(event) {
        // ignore key combinations
        if (event.ctrlKey || event.altKey || event.metaKey) {
            return;
        }
        switch (event.key) {
            case "ArrowUp":
                this.scroll(0, DEFAULT_SCROLL_DELTA);
                break;
            case "ArrowDown":
                this.scroll(0, -DEFAULT_SCROLL_DELTA);
                break;
            case "ArrowRight":
                this.scroll(DEFAULT_SCROLL_DELTA, 0);
                break;
            case "ArrowLeft":
                this.scroll(-DEFAULT_SCROLL_DELTA, 0);
                break;
            case "+": {
                const zoomPoint = new Point(
                    this.parentSurface.seriesViewRect.width / 2,
                    this.parentSurface.seriesViewRect.height / 2
                );
                this.performZoom(zoomPoint, -DEFAULT_ZOOM_DELTA);
                break;
            }
            case "-": {
                const zoomPoint = new Point(
                    this.parentSurface.seriesViewRect.width / 2,
                    this.parentSurface.seriesViewRect.height / 2
                );
                this.performZoom(zoomPoint, DEFAULT_ZOOM_DELTA);
                break;
            }
            default:
                return;
        }
        // prevent default behavior if the key is used by the modifier
        event.preventDefault();
    }
}
```

``` prism-code
import { ChartModifierBase2D, IChartModifierBaseOptions, EClipMode, Point, EXyDirection} from "scichart";

// ...
const DEFAULT_SCROLL_DELTA = 100;
const DEFAULT_ZOOM_DELTA = 120;
interface IKeyboardZoomPanModifierOptions extends IChartModifierBaseOptions {
    growFactor?: number;
    scrollFactor?: number;
}
class KeyboardZoomPanModifier extends ChartModifierBase2D {
    public type = "KeyboardZoomPan";
    /**
     * Modifies the speed of zoom, for example growFactor = 0.001 means each 'click'
     * zooms the chart 0.1%
     */
    public growFactor: number = 0.001;
    /**
     * Modifies the speed of scroll, for example scrollFactor = 0.001 means each 'click'
     * scrolls the chart 0.1%
     */
    public scrollFactor: number = 0.001;
    constructor(options?: IKeyboardZoomPanModifierOptions) {
        super(options);
        this.growFactor = options?.growFactor ?? this.growFactor;
        this.scrollFactor = options?.scrollFactor ?? this.scrollFactor;
        this.handleKeyDown = this.handleKeyDown.bind(this);
    }
    public scroll(xDelta: number, yDelta: number) {
        const token = this.parentSurface.suspendUpdates();
        // Scroll the X,YAxis by the number of pixels since the last update
        if ([EXyDirection.XDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.xAxes.asArray().forEach(axis => {
                const delta = (axis.isHorizontalAxis ? xDelta : -yDelta) * this.scrollFactor;
                axis.scroll(axis.flippedCoordinates ? -delta : delta, EClipMode.None);
            });
        }
        if ([EXyDirection.YDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.yAxes.asArray().forEach(axis => {
                const delta = (y.isHorizontalAxis ? -xDelta : yDelta) * this.scrollFactor;
                axis.scroll(axis.flippedCoordinates ? -delta : delta, EClipMode.None);
            });
        }
        token.resume();
    }
    public onAttach() {
        // set tabIndex attribute of the chart root element if it was not set externally
        this.parentSurface.domChartRoot.tabIndex = this.parentSurface.domChartRoot.tabIndex ?? 0;
        // subscribe to keyboard input event
        this.parentSurface.domChartRoot.addEventListener("keydown", this.handleKeyDown);
    }
    public onDetach() {
        // unsubscribe from keyboard input event
        this.parentSurface.domChartRoot.removeEventListener("keydown", this.handleKeyDown);
    }
    /**
     * Performs the zoom operation around the mouse point
     * @param mousePoint The X,Y location of the mouse at the time of the zoom
     * @param delta the delta factor of zoom
     */
    protected performZoom(mousePoint: Point, delta: number) {
        const fraction = this.growFactor * delta;
        if ([EXyDirection.XDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.xAxes.asArray().forEach(axis => {
                this.growBy(mousePoint, axis, fraction);
            });
        }
        if ([EXyDirection.YDirection, EXyDirection.XyDirection].includes(this.xyDirection)) {
            this.parentSurface.yAxes.asArray().forEach(axis => {
                this.growBy(mousePoint, axis, fraction);
            });
        }
    }
    private handleKeyDown(event: KeyboardEvent) {
        // ignore key combinations
        if (event.ctrlKey || event.altKey || event.metaKey) {
            return;
        }
        switch (event.key) {
            case "ArrowUp":
                this.scroll(0, DEFAULT_SCROLL_DELTA);
                break;
            case "ArrowDown":
                this.scroll(0, -DEFAULT_SCROLL_DELTA);
                break;
            case "ArrowRight":
                this.scroll(DEFAULT_SCROLL_DELTA, 0);
                break;
            case "ArrowLeft":
                this.scroll(-DEFAULT_SCROLL_DELTA, 0);
                break;
            case "+": {
                const zoomPoint = new Point(
                    this.parentSurface.seriesViewRect.width / 2,
                    this.parentSurface.seriesViewRect.height / 2
                );
                this.performZoom(zoomPoint, -DEFAULT_ZOOM_DELTA);
                break;
            }
            case "-": {
                const zoomPoint = new Point(
                    this.parentSurface.seriesViewRect.width / 2,
                    this.parentSurface.seriesViewRect.height / 2
                );
                this.performZoom(zoomPoint, DEFAULT_ZOOM_DELTA);
                break;
            }
            default:
                return;
        }
        // prevent default behavior if the key is used by the modifier
        event.preventDefault();
    }
```

Â Now let's look closer at the properties and methods of this class.

First of all the class is extended fromÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/chartmodifierbase2d.html" rel="noopener noreferrer" target="_blank">ChartModifierBase2DðŸ“˜</a>, which provides it with some properties and methods required on a chart modifier.

Also we defined several optional properties on the class which may be useful for configuring the modifier: **type**, **growFactor**, **scrollFactor**.

To make it possible for a chart to respond to key press events we need to make sure that it is focusable by keyboard navigation and add appropriate event listeners.

We can do this inside the **onAttach**/**onDetach** methods, which are called when the modifier is added to the chart:

- JS
- TS

``` prism-code
onAttach() {
    // set tabIndex attribute of the chart root element if it was not set externally
    this.parentSurface.domChartRoot.tabIndex = this.parentSurface.domChartRoot.tabIndex || 0;
    // subscribe to keyboard input event
    this.parentSurface.domChartRoot.addEventListener("keydown", this.handleKeyDown);
}
onDetach() {
    // unsubscribe from keyboard input event
    this.parentSurface.domChartRoot.removeEventListener("keydown", this.handleKeyDown);
}
```

``` prism-code
public onAttach() {
    // set tabIndex attribute of the chart root element if it was not set externally
    this.parentSurface.domChartRoot.tabIndex = this.parentSurface.domChartRoot.tabIndex ?? 0;
    this.parentSurface.domChartRoot.addEventListener("keydown", this.handleKeyDown);
}
public onDetach() {
    this.parentSurface.domChartRoot.removeEventListener("keydown", this.handleKeyDown);
}
```

Â **KeyboardZoomPanModifier.handleKeyDown** method is the one responsible for handling different keyboard inputs.

Depending on the key pressed it will call **KeyboardZoomPanModifier.scroll** or **KeyboardZoomPanModifier.performZoom** to updated the visible ranges.

![](out_scichartv4/2d-charts/accessibility/keyboard-accessibility/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

**KeyboardZoomPanModifier.scroll** uses the **SuspendUpdates API** for more info checkÂ [Batching updates or Temporary Suspending Drawing](https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/batching-updates-or-temporary-suspending-drawing/).

Finally, the usage of the modifier simply looks like:

``` prism-code
sciChartSurface.chartModifiers.add(new KeyboardZoomPanModifier({ scrollFactor: 0.1 }));
```

Â Make sure the chart root element is focused by keyboard navigation for modifier to work. E.g. you can make the element focused by default:

``` prism-code
// focus on scichart root to allow scichart detect keyboard events

sciChartSurface.domChartRoot.focus();
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/keyboard-accessibility/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Voice Over](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/voice-over/)
- [Color and Contrast](https://www.scichart.com/documentation/js/v4/2d-charts/accessibility/color-and-contrast/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/accessibility/keyboard-accessibility/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/accessibility/keyboard-accessibility/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
