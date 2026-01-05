<img src="out_scichartv4/typedoc/interfaces/ieventhandler_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventhandler.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [IEventHandler](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventhandler.html)

# Interface IEventHandler\<T\>

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

An EventHandler is a generic class that enables subscription, unsubscription to an event

description  
Declare an event as a property in your class like this

``` ts
public class MyCLass {
    public EventHandler<string> someEvent = new EventHandler<string>();
}
```

Subscribe to the event like this

``` ts
const myClass = new MyClass();
myClass.subscribe((event) => {
   console.log(event);
});
```

Publish an event like this

``` ts
const myClass = new MyClass();
myClass.raiseEvent("Hi there!");
```

### Type parameters

- #### T

### Hierarchy

- IEventHandler

### Implemented by

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/eventhandler.html" class="tsd-signature-type">EventHandler</a>

## Index

### Methods

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventhandler.html#subscribe" class="tsd-kind-icon">subscribe</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ieventhandler.html#unsubscribe" class="tsd-kind-icon">unsubscribe</a>

## Methods

### subscribe

- subscribe(handler: (data?: T) =\> void): void

<!-- -->

- Subscribes to the event

  #### Parameters

  - ##### handler: (data?: T) =\> void

    - - (data?: T): void

      <!-- -->

      - #### Parameters

        - ##### Optional data: T

        #### Returns void

  #### Returns void

### unsubscribe

- unsubscribe(handler: (data?: T) =\> void): void

<!-- -->

- Unsubscribes from the event

  #### Parameters

  - ##### handler: (data?: T) =\> void

    - - (data?: T): void

      <!-- -->

      - #### Parameters

        - ##### Optional data: T

        #### Returns void

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
