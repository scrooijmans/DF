<a href="https://opendal.apache.org/docs/nodejs/index.html" class="title">Apache OpenDALâ„¢ - v0.49.1</a>

[Homepage](https://opendal.apache.org/)[GitHub](https://github.com/apache/opendal/tree/main/bindings/nodejs)

![](out_opendal/docs/nodejs/documents/upgrade_media/f804b37d8f9651dce021c98c3091caf59681a8c3.svg)

Preparing search index...

<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#" id="tsd-toolbar-menu-trigger" class="tsd-widget menu" aria-label="Menu" data-toggle="menu"><img src="out_opendal/docs/nodejs/documents/upgrade_media/f3fbb7bef6ec1f2e24711c5bb2ef60c9c6ecb0d5.svg" /></a>

- <a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html" aria-current="page">upgrade</a>

# Upgrade to v0.48<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#upgrade-to-v048" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

## Breaking change<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#breaking-change" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

### Public API<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#public-api" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

Now, nodejs binding `op.is_exist` changed to `op.exists` to align with nodejs API style.

# Upgrade to v0.47<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#upgrade-to-v047" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

## Breaking change<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#breaking-change-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

### Public API<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#public-api-1" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

Now, the `append` operation has been removed. You can use below code instead.

``` js
op.write("path/to/file", Buffer.from("hello world"), { append: true });
Copy
```

# Upgrade to v0.44<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#upgrade-to-v044" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

## Breaking change<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#breaking-change-2" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

### Services<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#services" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

Because of [a TLS lib issue](https://github.com/apache/opendal/issues/3650), we temporarily disable the `services-ftp` feature.

### Public API<a href="https://opendal.apache.org/docs/nodejs/documents/upgrade.html#public-api-2" class="tsd-anchor-icon" aria-label="Permalink"><img src="out_opendal/docs/nodejs/documents/upgrade_media/1d480b11402974a270d65a1008580bad01955eab.svg" /></a>

Now, the `list` operation returns `Array<Entry>` instead of a lister. Also, we removed `scan`, you can use `list('some/path', {recursive: true})`/`listSync('some/path', {recursive: true})` instead of `scan('some/path')`/`scanSync('some/path')`.

![](out_opendal/docs/nodejs/documents/upgrade_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### Settings

Member Visibility

- [x] ![](out_opendal/docs/nodejs/documents/upgrade_media/bed852a5793c1fd9f7a03de8ebab8248a23ebeaa.svg)Inherited

ThemeOSLightDark

![](out_opendal/docs/nodejs/documents/upgrade_media/3d34c9e8a5108625c9958e0712352b388901c004.svg)

### On This Page

[Upgrade to v0.48](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#upgrade-to-v048)

- [Breaking change](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#breaking-change)
- - [Public API](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#public-api)

[Upgrade to v0.47](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#upgrade-to-v047)

- [Breaking change](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#breaking-change-1)
- - [Public API](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#public-api-1)

[Upgrade to v0.44](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#upgrade-to-v044)

- [Breaking change](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#breaking-change-2)
- - [Services](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#services)
  - [Public API](https://opendal.apache.org/docs/nodejs/documents/upgrade.html#public-api-2)

<a href="https://opendal.apache.org/" class="tsd-nav-link">Homepage</a><a href="https://github.com/apache/opendal/tree/main/bindings/nodejs" class="tsd-nav-link">GitHub</a>

[Apache OpenDALâ„¢ - v0.49.1](https://opendal.apache.org/docs/nodejs/modules.html)

- Loading...

Copyright Â© 2022-2025, The Apache Software Foundation. Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.
