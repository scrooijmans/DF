<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- [index](https://opendal.apache.org/docs/nodejs/modules/index.html)
- <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html" aria-current="page">StatOptions</a>

# Interface StatOptions

interface StatOptions {  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmatch" class="tsd-kind-property">ifMatch</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmodifiedsince" class="tsd-kind-property">ifModifiedSince</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifnonematch" class="tsd-kind-property">ifNoneMatch</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifunmodifiedsince" class="tsd-kind-property">ifUnmodifiedSince</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecachecontrol" class="tsd-kind-property">overrideCacheControl</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontentdisposition" class="tsd-kind-property">overrideContentDisposition</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontenttype" class="tsd-kind-property">overrideContentType</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#version" class="tsd-kind-property">version</a>?: string;  
}

- Defined in [generated.d.ts:1005](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1005)

![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

##### Index

### Properties

<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmatch" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifMatch?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmodifiedsince" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifModifiedSince?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifnonematch" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifNoneMatch?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifunmodifiedsince" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifUnmodifiedSince?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecachecontrol" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />overrideCacheControl?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontentdisposition" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />overrideContentDisposition?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontenttype" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />overrideContentType?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#version" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />version?</a>

![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Properties

### `Optional`ifMatch<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmatch" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

ifMatch?: string

Sets if-match condition for this operation. If file exists and its etag doesn't match, an error will be returned.

- Defined in [generated.d.ts:1013](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1013)

### `Optional`ifModifiedSince<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmodifiedsince" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

ifModifiedSince?: string

Sets if-modified-since condition for this operation. If file exists and hasn't been modified since the specified time, an error will be returned. ISO 8601 formatted date string <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString>

- Defined in [generated.d.ts:1023](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1023)

### `Optional`ifNoneMatch<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifnonematch" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

ifNoneMatch?: string

Sets if-none-match condition for this operation. If file exists and its etag matches, an error will be returned.

- Defined in [generated.d.ts:1017](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1017)

### `Optional`ifUnmodifiedSince<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifunmodifiedsince" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

ifUnmodifiedSince?: string

Sets if-unmodified-since condition for this operation. If file exists and has been modified since the specified time, an error will be returned. ISO 8601 formatted date string <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString>

- Defined in [generated.d.ts:1029](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1029)

### `Optional`overrideCacheControl<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecachecontrol" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

overrideCacheControl?: string

Specifies the cache-control header for presigned operations. Only meaningful when used along with presign.

- Defined in [generated.d.ts:1037](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1037)

### `Optional`overrideContentDisposition<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontentdisposition" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

overrideContentDisposition?: string

Specifies the content-disposition header for presigned operations. Only meaningful when used along with presign.

- Defined in [generated.d.ts:1041](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1041)

### `Optional`overrideContentType<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontenttype" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

overrideContentType?: string

Specifies the content-type header for presigned operations. Only meaningful when used along with presign.

- Defined in [generated.d.ts:1033](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1033)

### `Optional`version<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#version" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

version?: string

Sets version for this operation. Retrieves data of a specified version of the given path.

- Defined in [generated.d.ts:1009](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L1009)

![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

![](out_opendal/docs/nodejs/interfaces/index.StatOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Properties

[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifMatch](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmatch)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifModifiedSince](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifmodifiedsince)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifNoneMatch](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifnonematch)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />ifUnmodifiedSince](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#ifunmodifiedsince)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />overrideCacheControl](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecachecontrol)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />overrideContentDisposition](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontentdisposition)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />overrideContentType](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#overridecontenttype)[<img src="out_opendal/docs/nodejs/interfaces/index.StatOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />version](https://opendal.apache.org/docs/nodejs/interfaces/index.StatOptions.html#version)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
