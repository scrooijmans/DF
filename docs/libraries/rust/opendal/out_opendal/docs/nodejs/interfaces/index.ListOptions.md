<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- [index](https://opendal.apache.org/docs/nodejs/modules/index.html)
- <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html" aria-current="page">ListOptions</a>

# Interface ListOptions

interface ListOptions {  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#deleted" class="tsd-kind-property">deleted</a>?: boolean;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#limit" class="tsd-kind-property">limit</a>?: number;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#recursive" class="tsd-kind-property">recursive</a>?: boolean;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#startafter" class="tsd-kind-property">startAfter</a>?: string;  
Â Â Â Â <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#versions" class="tsd-kind-property">versions</a>?: boolean;  
}

- Defined in [generated.d.ts:808](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L808)

![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

##### Index

### Properties

<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#deleted" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />deleted?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#limit" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />limit?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#recursive" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />recursive?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#startafter" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />startAfter?</a> <a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#versions" class="tsd-index-link"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />versions?</a>

![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

## Properties

### `Optional`deleted<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#deleted" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

deleted?: boolean

The deleted is used to control whether the deleted objects should be returned.

- If `false`, list operation will not return with deleted objects
- If `true`, list operation will return with deleted objects if object versioning is supported by the underlying service

Default to `false`

- Defined in [generated.d.ts:844](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L844)

### `Optional`limit<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#limit" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

limit?: number

The limit passed to underlying service to specify the max results that could return per-request.

Users could use this to control the memory usage of list operation.

- Defined in [generated.d.ts:814](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L814)

### `Optional`recursive<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#recursive" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

recursive?: boolean

The recursive is used to control whether the list operation is recursive.

- If `false`, list operation will only list the entries under the given path.
- If `true`, list operation will list all entries that starts with given path.

Default to `false`.

- Defined in [generated.d.ts:826](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L826)

### `Optional`startAfter<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#startafter" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

startAfter?: string

The start_after passed to underlying service to specify the specified key to start listing from.

- Defined in [generated.d.ts:818](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L818)

### `Optional`versions<a href="https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#versions" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

versions?: boolean

The versions is used to control whether the object versions should be returned.

- If `false`, list operation will not return with object versions
- If `true`, list operation will return with object versions if object versioning is supported by the underlying service

Default to `false`

- Defined in [generated.d.ts:835](https://github.com/apache/opendal/blob/6f22e6d7a66b905257ee931ced7f7315768c0943/bindings/nodejs/generated.d.ts#L835)

![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

![](out_opendal/docs/nodejs/interfaces/index.ListOptions_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)Properties

[<img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />deleted](https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#deleted)[<img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />limit](https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#limit)[<img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />recursive](https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#recursive)[<img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />startAfter](https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#startafter)[<img src="out_opendal/docs/nodejs/interfaces/index.ListOptions_media/9f84f7b937ed959160d22b928f40bbec94070062.svg" class="tsd-kind-icon" />versions](https://opendal.apache.org/docs/nodejs/interfaces/index.ListOptions.html#versions)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
