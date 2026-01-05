<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- [index](https://opendal.apache.org/docs/nodejs/modules/index.html)
- <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html" aria-current="page">RetryLayer</a>

# Class RetryLayer

Retry layer

Add retry for temporary failed operations.

# Notes<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#notes" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

This layer will retry failed operations when \[`Error::is_temporary`\] returns true. If the operation still failed, this layer will set error to `Persistent` which means error has been retried.

`write` and `blocking_write` don't support retry so far, visit [this issue](https://github.com/apache/opendal/issues/1223) for more details.

# Examples<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#examples" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

``` javascript
const op = new Operator("file", { root: "/tmp" })

const retry = new RetryLayer();
retry.max_times = 3;
retry.jitter = true;

op.layer(retry.build());
Copy
```

- Defined in [generated.d.ts:714](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L714)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

##### Index

### Constructors

<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#constructor" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e0403e228d1fd1a0024bf19598bd4fe914f2967c.svg" class="tsd-kind-icon" />constructor</a>

### Accessors

<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#factor" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />factor</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#jitter" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />jitter</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#maxdelay" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />maxDelay</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#maxtimes" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />maxTimes</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#mindelay" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />minDelay</a>

### Methods

<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#build" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />build</a>

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Constructors

### constructor<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#constructor" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- new RetryLayer(): <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html" class="tsd-signature-type tsd-kind-class">RetryLayer</a><a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#constructorretrylayer" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  #### Returns <a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html" class="tsd-signature-type tsd-kind-class">RetryLayer</a>

  - Defined in [generated.d.ts:715](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L715)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Accessors

### factor<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#factor" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- set factor(v: number): void
  Set factor of current backoff.

  # Panics<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#panics" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  This function will panic if the input factor is smaller than `1.0`.

  #### Parameters

  - v: number

  #### Returns void

  - Defined in [generated.d.ts:736](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L736)

### jitter<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#jitter" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- set jitter(v: boolean): void
  Set jitter of current backoff.

  If jitter is enabled, ExponentialBackoff will add a random jitter in `[0, min_delay)` to current delay.

  #### Parameters

  - v: boolean

  #### Returns void

  - Defined in [generated.d.ts:722](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L722)

### maxDelay<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#maxdelay" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- set maxDelay(v: number): void
  Set max_delay of current backoff.

  Delay will not increase if the current delay is larger than max_delay.

  # Notes<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#notes-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  - The unit of max_delay is millisecond.

  #### Parameters

  - v: number

  #### Returns void

  - Defined in [generated.d.ts:746](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L746)

### maxTimes<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#maxtimes" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- set maxTimes(v: number): void
  Set max_times of current backoff.

  Backoff will return `None` if max times are reached.

  #### Parameters

  - v: number

  #### Returns void

  - Defined in [generated.d.ts:728](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L728)

### minDelay<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#mindelay" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- set minDelay(v: number): void
  Set min_delay of current backoff.

  # Notes<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#notes-2" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  - The unit of min_delay is millisecond.

  #### Parameters

  - v: number

  #### Returns void

  - Defined in [generated.d.ts:754](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L754)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Methods

### build<a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#build" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- build(): <a href="https://opendal.apache.org/docs/nodejs/classes/index.ExternalObject.html" class="tsd-signature-type tsd-kind-class">ExternalObject</a>\<<a href="https://opendal.apache.org/docs/nodejs/classes/index.Layer.html" class="tsd-signature-type tsd-kind-class">Layer</a>\><a href="https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#build-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  #### Returns <a href="https://opendal.apache.org/docs/nodejs/classes/index.ExternalObject.html" class="tsd-signature-type tsd-kind-class">ExternalObject</a>\<<a href="https://opendal.apache.org/docs/nodejs/classes/index.Layer.html" class="tsd-signature-type tsd-kind-class">Layer</a>\>

  - Defined in [generated.d.ts:755](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L755)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

[Notes](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#notes)[Examples](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#examples)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Constructors

[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e0403e228d1fd1a0024bf19598bd4fe914f2967c.svg" class="tsd-kind-icon" />constructor](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#constructor)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Accessors

[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />factor](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#factor)

- [Panics](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#panics)

[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />jitter](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#jitter)[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />maxDelay](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#maxdelay)

- [Notes](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#notes-1)

[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />maxTimes](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#maxtimes)[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/e5a9c15d446a7b4a2dd32ae515fe77469cad6fe9.svg" class="tsd-kind-icon" />minDelay](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#mindelay)

- [Notes](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#notes-2)

![](out_opendal/docs/nodejs/classes/index.RetryLayer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Methods

[<img src="out_opendal/docs/nodejs/classes/index.RetryLayer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />build](https://opendal.apache.org/docs/nodejs/classes/index.RetryLayer.html#build)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
