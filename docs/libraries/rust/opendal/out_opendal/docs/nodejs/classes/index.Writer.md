<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/classes/index.Writer_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- [index](https://opendal.apache.org/docs/nodejs/modules/index.html)
- <a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html" aria-current="page">Writer</a>

# Class Writer

Writer is designed to write data into a given path in an asynchronous manner.

- Defined in [generated.d.ts:762](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L762)
- Defined in [index.d.ts:37](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/index.d.ts#L37)

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

##### Index

### Constructors

<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#constructor" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/e0403e228d1fd1a0024bf19598bd4fe914f2967c.svg" class="tsd-kind-icon" />constructor</a>

### Methods

<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#close" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />close</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#createwritestream" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />createWriteStream</a> <a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#write" class="tsd-index-link"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />write</a>

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Constructors

### constructor<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#constructor" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- new Writer(): <a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html" class="tsd-signature-type tsd-kind-class">Writer</a><a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#constructorwriter" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  #### Returns <a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html" class="tsd-signature-type tsd-kind-class">Writer</a>

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Methods

### close<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#close" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- close(): Promise\<void\><a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#close-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  # Safety<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#safety" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  > &mut self in async napi methods should be marked as unsafe

  Close this writer.

  ### Example<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#example" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  ``` javascript
  const writer = await op.writer("path/to/file");
  await writer.write(Buffer.from("hello world"));
  await writer.close();
  Copy
  ```

  #### Returns Promise\<void\>

  - Defined in [generated.d.ts:792](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L792)

### createWriteStream<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#createwritestream" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- createWriteStream(options?: WritableOptions\<Writable\>): Writable<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#createwritestream-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  Create a writable stream from the underlying writer.

  #### Parameters

  - `Optional`options: WritableOptions\<Writable\>

  #### Returns Writable

  - Defined in [index.d.ts:39](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/index.d.ts#L39)

### write<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#write" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

- write(content: string \| Buffer\<ArrayBufferLike\>): Promise\<void\><a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#write-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>
  # Safety<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#safety-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  > &mut self in async napi methods should be marked as unsafe

  Write bytes into this writer.

  ### Example<a href="https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#example-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/classes/index.Writer_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

  ``` javascript
  const writer = await op.writer("path/to/file");
  await writer.write(Buffer.from("hello world"));
  await writer.close();
  Copy
  ```

  #### Parameters

  - content: string \| Buffer\<ArrayBufferLike\>

  #### Returns Promise\<void\>

  - Defined in [generated.d.ts:777](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L777)

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/classes/index.Writer_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Constructors

[<img src="out_opendal/docs/nodejs/classes/index.Writer_media/e0403e228d1fd1a0024bf19598bd4fe914f2967c.svg" class="tsd-kind-icon" />constructor](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#constructor)

![](out_opendal/docs/nodejs/classes/index.Writer_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Methods

[<img src="out_opendal/docs/nodejs/classes/index.Writer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />close](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#close)

- [Safety](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#safety)
- - - [Example](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#example)

[<img src="out_opendal/docs/nodejs/classes/index.Writer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />createWriteStream](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#createwritestream)[<img src="out_opendal/docs/nodejs/classes/index.Writer_media/c06389ef9336ebbae6d5cfd4c0bfac7fa18ffd0a.svg" class="tsd-kind-icon" />write](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#write)

- [Safety](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#safety-1)
- - [Example](https://opendal.apache.org/docs/nodejs/classes/index.Writer.html#example-1)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
