<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/classes/index.Reader_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- [index](https://opendal.apache.org/docs/nodejs/modules/index.html)
- <a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html" aria-current="page">Reader</a>

# Class Reader

Reader is designed to read data from a given path in an asynchronous manner.

- Defined in [generated.d.ts:676](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L676)
- Defined in [index.d.ts:27](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/index.d.ts#L27)

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

##### Index

### Constructors

<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#constructor" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/e0403e228d1fd1a0024bf19598bd4fe914f2967c.svg" class="tsd-kind-icon" />constructor</a>

### Methods

<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#createreadstream" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />createReadStream</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#read" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />read</a>

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Constructors

### constructor<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#constructor" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- new Reader(): <a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html" class="tsd-signature-type tsd-kind-class">Reader</a><a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#constructorreader" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  #### Returns <a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html" class="tsd-signature-type tsd-kind-class">Reader</a>

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Methods

### createReadStream<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#createreadstream" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- createReadStream(options?: ReadableOptions\<Readable\>): Readable<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#createreadstream-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  Create a readable stream from the underlying reader.

  #### Parameters

  - `Optional`options: ReadableOptions\<Readable\>

  #### Returns Readable

  - Defined in [index.d.ts:29](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/index.d.ts#L29)

### read<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#read" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- read(buf: Buffer): Promise\<bigint\><a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#read-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  # Safety<a href="https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#safety" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Reader_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  > &mut self in async napi methods should be marked as unsafe

  Read bytes from this reader into given buffer.

  #### Parameters

  - buf: Buffer

  #### Returns Promise\<bigint\>

  - Defined in [generated.d.ts:684](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L684)

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/classes/index.Reader_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Constructors

[<img src="out_opendal/docs/nodejs/classes/index.Reader_media/e0403e228d1fd1a0024bf19598bd4fe914f2967c.svg" class="tsd-kind-icon" />constructor](https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#constructor)

![](out_opendal/docs/nodejs/classes/index.Reader_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Methods

[<img src="out_opendal/docs/nodejs/classes/index.Reader_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />createReadStream](https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#createreadstream)[<img src="out_opendal/docs/nodejs/classes/index.Reader_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />read](https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#read)

- [Safety](https://opendal.apache.org/docs/nodejs/classes/index.Reader.html#safety)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
