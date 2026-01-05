<img src="out_scichartv4/typedoc/interfaces/ieventlistenersource_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventlistenersource.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IEventListenerSource](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventlistenersource.html)

# Interface IEventListenerSource

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Defines the interface to a type which sources events, such as an HTML5 element

### Hierarchy

- IEventListenerSource

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventlistenersource.html#addeventlistener" class="tsd-kind-icon">addEventListener</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventlistenersource.html#removeeventlistener" class="tsd-kind-icon">removeEventListener</a>

## Methods

### addEventListener

- addEventListener\<K\>(type: K, listener: (this: HTMLElement, ev: HTMLElementEventMap\[K\]) =\> any, options?: boolean \| AddEventListenerOptions): void

<!-- -->

- #### Type parameters

  - #### K: keyof HTMLElementEventMap

  #### Parameters

  - ##### type: K

  - ##### listener: (this: HTMLElement, ev: HTMLElementEventMap\[K\]) =\> any

    - - (this: HTMLElement, ev: HTMLElementEventMap\[K\]): any

      <!-- -->

      - #### Parameters

        - ##### this: HTMLElement

        - ##### ev: HTMLElementEventMap\[K\]

        #### Returns any

  - ##### Optional options: boolean \| AddEventListenerOptions

  #### Returns void

### removeEventListener

- removeEventListener\<K\>(type: K, listener: (this: HTMLElement, ev: HTMLElementEventMap\[K\]) =\> any, options?: boolean \| EventListenerOptions): void

<!-- -->

- #### Type parameters

  - #### K: keyof HTMLElementEventMap

  #### Parameters

  - ##### type: K

  - ##### listener: (this: HTMLElement, ev: HTMLElementEventMap\[K\]) =\> any

    - - (this: HTMLElement, ev: HTMLElementEventMap\[K\]): any

      <!-- -->

      - #### Parameters

        - ##### this: HTMLElement

        - ##### ev: HTMLElementEventMap\[K\]

        #### Returns any

  - ##### Optional options: boolean \| EventListenerOptions

  #### Returns void

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
