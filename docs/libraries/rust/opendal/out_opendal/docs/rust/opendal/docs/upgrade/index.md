# Module upgrade Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/mod.rs.html#43" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Upgrade and migrate procedures while OpenDAL meets breaking changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v054" class="doc-anchor">Â§</a>Upgrade to v0.54

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-6189-remove-native-blocking-support" class="doc-anchor">Â§</a>RFC-6189: Remove Native Blocking Support

OpenDAL v0.54 implements [RFC-6189](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6189_remove_native_blocking/index.html), which removes all native blocking support in favor of using `block_on` from async runtimes.

The following breaking changes have been made:

- `blocking::Operator` can no longer be used within async contexts
- Using blocking APIs now requires an async runtime
- All `Blocking*` types have been moved to the `opendal::blocking` module

To migrate:

``` diff
- use opendal::BlockingOperator;
+ use opendal::blocking::Operator;
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-6213-options-based-api" class="doc-anchor">Â§</a>RFC-6213: Options Based API

OpenDAL v0.54 implements [RFC-6213](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6213_options_api/index.html), which introduces options-based APIs for more structured and extensible operation configuration.

New APIs added:

- `read_options(path, ReadOptions)`
- `write_options(path, data, WriteOptions)`
- `list_options(path, ListOptions)`
- `stat_options(path, StatOptions)`
- `delete_options(path, DeleteOptions)`

Example usage:

``` rust
// Read with options
let options = ReadOptions::new()
    .range(0..1024)
    .if_match("etag");
let data = op.read_options("path/to/file", options).await?;

// Write with options  
let options = WriteOptions::new()
    .content_type("text/plain")
    .cache_control("max-age=3600");
op.write_options("path/to/file", data, options).await?;
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#remove-stat_has_xxx-and-list_has_xxx-apis" class="doc-anchor">Â§</a>Remove `stat_has_xxx` and `list_has_xxx` APIs

All `stat_has_*` and `list_has_*` capability check APIs have been removed. Instead, check capabilities directly on the `Capability` struct:

``` diff
- if op.info().full_capability().stat_has_content_length() {
+ if op.info().full_capability().stat.content_length {
    // ...
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#fix-with_user_metadata-signature" class="doc-anchor">Â§</a>Fix `with_user_metadata` signature

The signature of `with_user_metadata` has been changed. Please update your code accordingly if you use this method.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#services-removed-due-to-lack-of-maintainer" class="doc-anchor">Â§</a>Services removed due to lack of maintainer

The following services have been removed due to lack of maintainers:

- `atomicserver`
- `icloud`
- `nebula-graph`

If you need these services, please consider maintaining them or use alternative services.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#httpclientlayer-replaces-update_http_client" class="doc-anchor">Â§</a>HttpClientLayer replaces `update_http_client`

The `Operator::update_http_client()` method has been replaced by `HttpClientLayer`:

``` diff
- op.update_http_client(client);
+ op = op.layer(HttpClientLayer::new(client));
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#expose-presign_xxx_options-api" class="doc-anchor">Â§</a>Expose `presign_xxx_options` API

New options-based presign APIs have been exposed:

``` rust
let options = PresignOptions::new()
    .expire(Duration::from_secs(3600));
    
let url = op.presign_read_options("path/to/file", options).await?;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#remove-native-blocking-support" class="doc-anchor">Â§</a>Remove native blocking support

All native blocking implementations have been removed from the raw API. Services and layers no longer need to implement blocking-specific methods.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v053" class="doc-anchor">Â§</a>Upgrade to v0.53

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-1" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#supabase-service-is-now-an-s3-compatible-servcice" class="doc-anchor">Â§</a>Supabase service is now an S3-compatible servcice

Supabase Storage is now an S3-compatible service instead: https://github.com/supabase/storage.

We removed the supabase native service support in OpenDAL v0.53. Users who want to access Supabase Storage can use the S3 service instead.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#all-metrics-related-layers-have-been-refactored" class="doc-anchor">Â§</a>All metrics related layers have been refactored

All metrics layers have been refactored:

- `PrometheusLayer`
- `PrometheusClientLayer`
- `MetricsLayer`

They are now provides more metrics and more detailed information. All their public API have been redesigned.

For more details, please refer to `opendal::layers::observe`â€™s module documentation.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#operatordefault_executor-has-been-replaced-by-operatorexecutor" class="doc-anchor">Â§</a>`Operator::default_executor` has been replaced by `Operator::executor`

In opendal v0.53, we introduced a new concept of `Context` which is used to store the context of the current operator. Thanks to this design, we can now get and set the `executor` and `http_client` for given Operator instead.

All services `http_client` API has been deprecated and replaced by `Operator::update_http_client` API.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#opendal-msrv-bumped-to-182" class="doc-anchor">Â§</a>OpenDAL MSRV bumped to `1.82`

Since v0.53, OpenDAL will require Rust 1.82.0 or later to build.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-1" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#operation-enum-merge" class="doc-anchor">Â§</a>Operation enum merge

To reduce the complexity of the `Operation`, we have merged the duplicated `Operation`.

For example:

- `Operation::ReaderRead` has been merged into `Operation::Read`
- `Operation::BlockingRead` has been merged into `Operation::Read`

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v052" class="doc-anchor">Â§</a>Upgrade to v0.52

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-2" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-5556-write-returns-metadata" class="doc-anchor">Â§</a>RFC-5556: Write Returns Metadata

Since v0.52, all write APIs in OpenDAL have been updated to return `Metadata` instead of `()`. This metadata includes useful information provided by the service, such as `content-length`, `etag`, `version`, and `last-modified`.

This feature is not fully ready yet, and many available metadata fields are still not returned. Please visit [Tracking Issues of RFC-5556: Write Returns Metadata](https://github.com/apache/opendal/issues/5557) for progress and contributions.

Affected API:

- `opendal::Operator::write`
- `opendal::Operator::write_with`
- `opendal::Operator::writer::close`
- `opendal::raw::oio::Write::close`

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#github-actions-cache-ghac-service-v2" class="doc-anchor">Â§</a>Github Actions Cache (ghac) service v2

As [requested](https://github.com/apache/opendal/issues/5620) by GitHub, we have upgraded our GHAC service to ensure compatibility with the latest GitHub Actions cache API.

By upgrading to OpenDAL v0.52, your services will continue functioning after the deprecation of the legacy service (2025/03/01). GHES does not yet support GHAC v2, but OpenDAL has handled this properly to prevent any disruptions.

ghac service doesnâ€™t support `delete` anymore, please use githubâ€™s API to delete cache instead.

This upgrade is mandatory and enabled by default using an environment variable in the GitHub CI environment. No changes are required at the code level.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#breaking-changes-in-dependencies" class="doc-anchor">Â§</a>Breaking Changes in Dependencies

- `OtelTraceLayer` and `OtelMetricsLayer`â€™s dependence `opentelemetry` bumped to `0.28`
- `PrometheusClientLayer`â€™s dependence `prometheus-client` bumped to `0.23.1`

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v051" class="doc-anchor">Â§</a>Upgrade to v0.51

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-3" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#new-vision-one-layer-all-storage" class="doc-anchor">Â§</a>New VISION: One Layer, All Storage

OpenDAL has refined its vision to **One Layer, All Storage**, driven by the following core principles: **Open Community**, **Solid Foundation**, **Fast Access**, **Object Storage First**, and **Extensible Architecture**.

Explore the detailed vision at [OpenDAL Vision](https://opendal.apache.org/vision).

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-5313-remove-metakey" class="doc-anchor">Â§</a>RFC-5313: Remove Metakey

OpenDAL v0.51 implements [RFC-5313](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html), which removes the concept of metakey.

The following structs have been removed:

- `Metakey`

The following APIs have been removed:

- `list_with(path).metakey()`

Users no longer need to pass the metakey into the list. Instead, services will make their best effort to return as much metadata as possible. Users can check items like `Capability::list_has_etag` before making a call.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#remove-not-used-capability-write_multi_align_size" class="doc-anchor">Â§</a>Remove not used capability: `write_multi_align_size`

The capability `write_multi_align_size` is not utilized by any services, and we have no plans to support it in the future; therefore, we have removed it.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#capabilitychecklayer-and-correctnesschecklayer" class="doc-anchor">Â§</a>CapabilityCheckLayer and CorrectnessCheckLayer

OpenDAL used to perform capability checks for all services, but since v0.51, it only conducts checks that impact data correctness like `write_with_if_not_exists` or `delete_with_version` by default in the `CorrectnessCheckLayer`. If users wish to verify other non-critical capabilities like `write_with_content_type` or `write_with_cache_control`, they should manually enable the `CapabilityCheckLayer`.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-3911-deleter-api" class="doc-anchor">Â§</a>RFC-3911: Deleter API

OpenDAL v0.51 implements [RFC-3911](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3911_deleter_api/index.html), which adds `Deleter` in OpenDAL to replace `batch` operation.

The following new APIs have been added:

- \[`Operator::delete_iter`\]
- \[`Operator::delete_try_iter`\]
- \[`Operator::delete_stream`\]
- \[`Operator::delete_try_stream`\]
- \[`Operator::deleter`\]
- \[`Deleter::delete`\]
- \[`Deleter::delete_iter`\]
- \[`Deleter::delete_try_iter`\]
- \[`Deleter::delete_stream`\]
- \[`Deleter::delete_try_stream`\]
- \[`Deleter::flush`\]
- \[`Deleter::close`\]
- \[`Deleter::into_sink`\]
- \[`DeleteInput`\]
- \[`IntoDeleteInput`\]
- \[`FuturesDeleteSink`\]

The following APIs have been deprecated and will be removed in the future releases:

- `Operator::remove` (replace with \[`Operator::delete_iter`\])
- `Operator::remove_via` (replace with \[`Operator::delete_stream`\])

As a result of this change, the `limit` and `with_limit` APIs on `Operator` have also been deprecated; they are currently no-ops.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-2" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#adapterkv-now-returns-scanner-instead-of-vecstring" class="doc-anchor">Â§</a>`adapter::kv` now returns `Scanner` instead of `Vec<String>`

To support returning key-value entries in a streaming manner instead of loading them all into memory, OpenDAL updated its adapter API to return a `Scanner` instead of a `Vec<String>`.

``` diff
- async fn scan(&self, path: &str) -> Result<Vec<String>>
+ async fn scan(&self, path: &str) -> Result<Self::Scanner>
```

All services intending to implement `kv::Adapter` should adhere to this API change.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#align-metadata-api-to-info" class="doc-anchor">Â§</a>Align `metadata` API to `info`

OpenDAL changes itâ€™s old `metadata` API to `info` to align with the new `AccessorInfo` struct.

``` diff
- fn metadata(&self) -> Arc<AccessorInfo>
+ fn info(&self) -> Arc<AccessorInfo>
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#remove-not-used-struct-rangewriter" class="doc-anchor">Â§</a>Remove not used struct: `RangeWriter`

The struct `RangeWriter` is not utilized by any services, and we have no plans to support it in the future; therefore, we have removed it.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v050" class="doc-anchor">Â§</a>Upgrade to v0.50

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-4" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#services-postgresqls-connect-string-now-supports-only-url-format" class="doc-anchor">Â§</a>`services-postgresql`â€™s connect string now supports only URL format

Previously, it supports both URL format and key-value format. After switching the implementation from `tokio-postgres` to `sqlx`, the service now supports only the URL format.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#list-now-returns-path-itself" class="doc-anchor">Â§</a>`list` now returns path itself

Previously, `list("a/b")` would not return `a/b` even if it does exist. Since v0.50.0, this behavior has been changed. OpenDAL will now return the path itself if it exists. This change applies to all cases, whether the path is a directory or a file.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#refactoring-of-the-metrics-related-layer" class="doc-anchor">Â§</a>Refactoring of the metrics-related layer

In OpenDAL v0.50.0, we did a refactor on all metrics-related layers. They are now sharing the same underlying implementations. `PrometheusLayer`, `PrometheusClientLayer` and `MetricsLayer` are now have similar public APIs and exactly the same metrics value.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v049" class="doc-anchor">Â§</a>Upgrade to v0.49

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-5" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#configurator-now-returns-associated-builder-instead" class="doc-anchor">Â§</a>`Configurator` now returns associated builder instead

`Configurator` used to return `impl Builder`, but now it returns associated builder type directly. This will allow users to use the builder in a more flexible way.

``` diff
impl Configurator for MemoryConfig {
-    fn into_builder(self) -> impl Builder {
+    type Builder = MemoryBuilder;
+    fn into_builder(self) -> Self::Builder {
        MemoryBuilder { config: self }
    }
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#logginglayer-now-accepts-logginginterceptor" class="doc-anchor">Â§</a>`LoggingLayer` now accepts `LoggingInterceptor`

`LoggingLayer` now accepts `LoggingInterceptor` trait instead of configuration. This change will allow users to customize the logging behavior more flexibly.

``` diff
pub trait LoggingInterceptor: Debug + Clone + Send + Sync + Unpin + 'static {
    fn log(
        &self,
        info: &AccessorInfo,
        operation: Operation,
        context: &[(&str, &str)],
        message: &str,
        err: Option<&Error>,
    );
}
```

Users can now implement the log in the way they want.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v048" class="doc-anchor">Â§</a>Upgrade to v0.48

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-6" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#typo-in-customized_credential_load" class="doc-anchor">Â§</a>Typo in `customized_credential_load`

Since v0.48, the `customed_credential_load` function has been renamed to `customized_credential_load` to fix the typo of `customized`.

``` diff
- builder.customed_credential_load(v);
+ builder.customized_credential_load(v);
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#s3-service-rename-security_token-to-session_token" class="doc-anchor">Â§</a>S3 service rename `security_token` to `session_token`

[In 2014 Amazon switched](https://aws.amazon.com/blogs/security/a-new-and-standardized-way-to-manage-credentials-in-the-aws-sdks/) from `AWS_SECURITY_TOKEN` to `AWS_SESSION_TOKEN`. To be consistent with the naming of AWS STS, we have renamed the `security_token` field to `session_token` in the S3 service.

``` diff
- builder.security_token(v);
+ builder.session_token(v);
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#operator-from_iter-and-via_iter-replaces-from_map-and-via_map" class="doc-anchor">Â§</a>Operator `from_iter` and `via_iter` replaces `from_map` and `via_map`

Since v0.48, Operatorâ€™s new APIs `from_iter` and `via_iter` methods have deprecated the `from_map` and `via_map` methods.

``` diff
- Operator::from_map::<Fs>(map)?.finish();
+ Operator::from_iter::<Fs>(map)?.finish();
```

New API `from_iter` and `via_iter` should cover all use cases of `from_map` and `via_map`.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#service-builder-now-takes-ownership" class="doc-anchor">Â§</a>Service builder now takes ownership

Since v0.48, all service builder now takes ownership `self` instead of `&mut self`. This change will allow users to configure the service in a more flexible way.

``` diff
- let mut builder = S3::default();
- builder.bucket("test");
- builder.root("/path/to/root");
+ let builder = S3::default().bucket("test").root("/path/to/root");
  let op = Operator::new(builder)?.finish();
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-3" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#oiowritewrite-will-write-the-whole-buffer" class="doc-anchor">Â§</a>`oio::Write::write` will write the whole buffer

Starting from version 0.48, `oio::Write::write` now writes the entire buffer. This update aligns the API more closely with `oio::Read::read` and simplifies the implementation of concurrent writing.

``` diff
  trait Write {
-     fn write(&mut self, bs: Buffer) -> impl Future<Output = Result<usize>>;
+     fn write(&mut self, bs: Buffer) -> impl Future<Output = Result<()>>;
  }
```

`write` will now return `Result<()>` instead of `Result<usize>`. The number of bytes written can be obtained from the bufferâ€™s length.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#accessmetadata-will-return-arcaccessinfo" class="doc-anchor">Â§</a>`Access::metadata()` will return `Arc<AccessInfo>`

Starting from version 0.48, `Access::metadata()` will return `Arc<AccessInfo>` instead of `AccessInfo`. This change is intended to improve performance and reduce memory usage.

``` diff
  trait Access {
-     fn metadata(&self) -> AccessInfo;
+     fn metadata(&self) -> Arc<AccessInfo>;
  }
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#minitracelayer-renamed-to-fastracelayer" class="doc-anchor">Â§</a>`MinitraceLayer` renamed to `FastraceLayer`

The `MinitraceLayer` has been renamed to `FastraceLayer` to respond to the [transition from `minitrace` to `fastrace`](https://github.com/tikv/minitrace-rust/issues/229).

``` diff
- use opendal::layers::MinitraceLayer;
+ use opendal::layers::FastraceLayer;
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#use-configurator-to-replace-builderfrom_config" class="doc-anchor">Â§</a>Use `Configurator` to replace `Builder::from_config`

Since v0.48, the `Builder::from_config` and `Builder::from_map` method has been replaced by the `Configurator` trait. The `Configurator` trait provides a more flexible and extensible way to configure OpenDAL.

Service implementers should update their code to use the `Configurator` trait instead:

``` rust
impl Configurator for MemoryConfig {
    type Builder = MemoryBuilder;
    fn into_builder(self) -> Self::Builder {
        MemoryBuilder { config: self }
    }
}

impl Builder for MemoryBuilder {
    const SCHEME: Scheme = Scheme::Memory;
    type Config = MemoryConfig;

    fn build(self) -> Result<impl Access> {
        ...
    }
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v047" class="doc-anchor">Â§</a>Upgrade to v0.47

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-7" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#reader-into_xxx-apis" class="doc-anchor">Â§</a>Reader `into_xxx` APIs

Since v0.47, `Reader`â€™s `into_xxx` APIs requires `async` and returns `Result` instead.

``` diff
- let r = op.reader("test.txt").await?.into_futures_async_read(1024..2048);
+ let r = op.reader("test.txt").await?.into_futures_async_read(1024..2048).await?;
```

Affected API includes:

- `Reader::into_futures_async_read`
- `Reader::into_bytes_stream`
- `BlockingReader::into_std_read`
- `BlockingReader::into_bytes_iterator`

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-4" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#bring-streaming-read-back" class="doc-anchor">Â§</a>Bring Streaming Read Back

As explained in [core: Bring Streaming Read Back](https://github.com/apache/opendal/issues/4672), we do need read streaming back for better performance and low memory usage.

So our `oio::Read` changed back to streaming read instead:

``` diff
trait Read {
-  async fn read(&self, offset: u64, size: usize) -> Result<Buffer>;
+  async fn read(&mut self) -> Result<Buffer>;
}
```

All services and layers should be updated to meet this change.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v046" class="doc-anchor">Â§</a>Upgrade to v0.46

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-8" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#msrv-changed-to-175" class="doc-anchor">Â§</a>MSRV Changed to 1.75

Since 0.46, OpenDAL requires Rust 1.75.0 or later to use features like [`RPITIT`](https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html) and [`AFIT`](https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html).

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#services-feature-flag" class="doc-anchor">Â§</a>Services Feature Flag

Starting with version 0.46, OpenDAL only includes the memory service by default to prevent compiling unnecessary service code. To use other services, please activate their respective feature flags.

Additionally, we have removed all `reqwest`-related feature flags:

- Users must now directly use `reqwest`â€™s feature flags for options like `rustls`, `native-tls`, etc.
- The `rustls` feature is no longer enabled by default; it must be activated manually.
- OpenDAL no longer offers the `trust-dns` option; users should configure the client builder directly.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#range-based-read" class="doc-anchor">Â§</a>Range Based Read

Since v0.46, OpenDAL transformed itâ€™s Read IO trait to range based instead of stateful poll based IO. This change will make the IO more efficient, easier for concurrency and ready for completion based IO.

`opendal::Reader` now have APIs like:

``` rust
let r = op.reader("test.txt").await?;
let buf = r.read(1024..2048).await?;
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#buffer-based-io" class="doc-anchor">Â§</a>Buffer Based IO

Since version 0.46, OpenDAL features a native `Buffer` struct that supports both contiguous and non-contiguous buffers. This update enhances IO efficiency by minimizing unnecessary byte copying and enabling vectored IO.

OpenDALâ€™s `Reader` will return `Buffer` and `Writer` will accept `Buffer` as input. Users who have implemented their own IO traits should update their code to use the new `Buffer` struct.

``` rust
let r = op.reader("test.txt").await?;
// read returns `Buffer`
let buf: Buffer = r.read(1024..2048).await?;

let w = op.writer("test.txt").await?;

// Buffer can be created from continues bytes.
w.write("hello, world").await?;
// Buffer can also be created from non-continues bytes.
w.write(vec![Bytes::from("hello,"), Bytes::from("world!")]).await?;

// Make sure file has been written completely.
w.close().await?;
```

To enhance usability, weâ€™ve integrated bridges into `bytes::Buf` and `bytes::BufMut`, allowing users to directly interact with the bytes API.

``` rust
let r = op.reader("test.txt").await?;
let mut bs = vec![];
// read_into accepts bytes::BufMut
let buf: Buffer = r.read_into(&mut bs, 1024..2048).await?;

let w = op.writer("test.txt").await?;

// write_from accepts bytes::Buf
w.write_from("hello, world".as_bytes()).await?;

// Make sure file has been written completely.
w.close().await?;
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#bridge-api" class="doc-anchor">Â§</a>Bridge API

OpenDALâ€™s `Reader` and `Writer` previously implemented APIs such as `AsyncRead` and `AsyncWrite` directly. This design was not user-friendly, as it could lead to unexpected costs that users were unaware of in advance.

Since v0.46, OpenDAL provides bridge APIs for `Reader` and `Writer` instead.

``` rust
let r = op.reader("test.txt").await?;

// Convert into futures AsyncRead + AsyncSeek.
let reader = r.into_futures_async_read(1024..2048);
// Convert into futures bytes stream.
let stream = r.into_bytes_stream(1024..2048);

let w = op.writer("test.txt").await?;

// Convert into futures AsyncWrite
let writer = w.into_futures_async_write();
// Convert into futures bytes sink;
let sink = w.into_bytes_sink();
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-5" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#async-in-io-trait" class="doc-anchor">Â§</a>Async in IO trait

Since version 0.46, OpenDAL has adopted Rustâ€™s native [`async_in_trait`](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html) for our core IO traits, including `oio::Read`, `oio::Write`, and `oio::List`.

This update eliminates the need for manually written, poll-based state machines and simplifies the codebase. Consequently, OpenDAL now requires Rust version 1.75.0 or later.

Users who have implemented their own IO traits should update their code to use the new async trait syntax.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v045" class="doc-anchor">Â§</a>Upgrade to v0.45

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-9" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#blockinglayer-is-not-enabled-by-default" class="doc-anchor">Â§</a>BlockingLayer is not enabled by default

To further enhance the optionality of `tokio`, we have introduced a new feature called `layers-blocking`. The default usage of the blocking layer has been disabled. To utilize the `BlockingLayer`, please enable the `layers-blocking` feature.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#timeoutlayer-deprecated-with_speed" class="doc-anchor">Â§</a>TimeoutLayer deprecated `with_speed`

The `with_speed` API has been deprecated. Please use `with_io_timeout` instead.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-6" class="doc-anchor">Â§</a>Raw API

No raw API changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v044" class="doc-anchor">Â§</a>Upgrade to v0.44

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-10" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#moka-service-configuration" class="doc-anchor">Â§</a>Moka Service Configuration

- The `thread_pool_enabled` option has been removed.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#list-prefix-supported" class="doc-anchor">Â§</a>List Prefix Supported

After [RFC: List Prefix](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html "mod opendal::docs::rfcs::rfc_3243_list_prefix") landed, we have changed the behavior of `list` a path without `/`. OpenDAL used to return `NotADirectory` error, but now we will return the list of entries that start with given prefix instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v043" class="doc-anchor">Â§</a>Upgrade to v0.43

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-11" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#list-recursive" class="doc-anchor">Â§</a>List Recursive

After [RFC-3526: List Recursive](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html "mod opendal::docs::rfcs::rfc_3526_list_recursive") landed, we have changed the `list` API to accept `recursive` instead of `delimiter`:

Users will need to change the following usage:

- `op.list_with(path).delimiter("")` -\> `op.list_with(path).recursive(true)`
- `op.list_with(path).delimiter("/")` -\> `op.list_with(path).recursive(false)`

`delimiter` other than `""` and `"/"` is not supported anymore.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#stat-a-dir-path" class="doc-anchor">Â§</a>Stat a dir path

After [RFC: List Prefix](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html "mod opendal::docs::rfcs::rfc_3243_list_prefix") landed, we have changed the behavior of `stat` a dir path:

Here are the behavior list:

| Case | Path | Result |
|----|----|----|
| stat existing dir | `abc/` | Metadata with dir mode |
| stat existing file | `abc/def_file` | Metadata with file mode |
| stat dir without `/` | `abc/def_dir` | Error `NotFound` or metadata with dir mode |
| stat file with `/` | `abc/def_file/` | Error `NotFound` |
| stat not existing path | `xyz` | Error `NotFound` |

Services like s3, azblob can handle `stat("abc/")` correctly by check if there are objects with prefix `abc/`.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-7" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#lister-align" class="doc-anchor">Â§</a>Lister Align

We changed our internal `lister` implementation to align with the `list` public API for better performance and readability.

- trait `Page` =\> `List`
- struct `Pager` =\> `Lister`
- trait `BlockingPage` =\> `BlockingList`
- struct `BlockingPager` =\> `BlockingLister`

Every call to `next` will return an entry instead a page of entries. Also, we changed our async list api into poll based instead of `async_trait`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v042" class="doc-anchor">Â§</a>Upgrade to v0.42

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-12" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#msrv-changed" class="doc-anchor">Â§</a>MSRV Changed

OpenDAL bumps itâ€™s MSRV to 1.67.0.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#s3-service-configuration" class="doc-anchor">Â§</a>S3 Service Configuration

- The `enable_exact_buf_write` option has been deprecated and is superseded by `BufferedWriter`, introduced in version 0.40.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#oss-service-configuration" class="doc-anchor">Â§</a>Oss Service Configuration

- The `write_min_size` option has been deprecated and replaced by `BufferedWriter`, also introduced in version 0.40.
- A new setting, `allow_anonymous`, has been added. Since v0.41, OSS will now return an error if credential loading fails. Enabling `allow_anonymous` to fallback to request without credentials.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#ghac-service-configuration" class="doc-anchor">Â§</a>Ghac Service Configuration

- The `enable_create_simulation` option has been removed. We add this option to allow ghac simulate create empty file, but it could result in unexpected behavior when users create a file with content length `1`. So we remove it.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#wasabi-service-removed" class="doc-anchor">Â§</a>Wasabi Service Removed

`wasabi` service native support has been removed. Users who want to access wasabi can use our `s3` service instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v041" class="doc-anchor">Â§</a>Upgrade to v0.41

There is no public API and raw API changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v040" class="doc-anchor">Â§</a>Upgrade to v0.40

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-13" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-2578-merge-append-into-write" class="doc-anchor">Â§</a>RFC-2578 Merge Append Into Write

[RFC-2578](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html "mod opendal::docs::rfcs::rfc_2758_merge_append_into_write") merges `append` into `write` and removes `append` API.

- For writing a file at once, please use `op.write()` for convenience.
- For appending a file, please use `op.write_with().append(true)` instead of `op.append()`.

The same rule applies to `writer()` and `writer_with()`.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-2774-lister-api" class="doc-anchor">Â§</a>RFC-2774 Lister API

[RFC-2774](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html "mod opendal::docs::rfcs::rfc_2774_lister_api") proposes a new `lister` API to replace current `list` and `scan`. And we add a new API `list` to return entries directly.

- For listing a directory at once, please use `list()` for convenience.
- For listing a directory recursively, please use `list_with().delimiter("")` or `lister_with().delimiter("")` instead of `scan()`.
- For listing in streaming, please use `lister()` or `lister_with()` instead.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-2779-list-with-metakey" class="doc-anchor">Â§</a>RFC-2779 List With Metakey

[RFC-2779](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2779_list_with_metakey/index.html "mod opendal::docs::rfcs::rfc_2779_list_with_metakey") proposes a new `op.list_with().metakey()` API to allow list with metakey and removes `op.metadata(&entry)` API.

Please use `op.list_with().metakey()` instead of `op.metadata(&entry)`, for example:

``` rust
// Before
let entries: Vec<Entry> = op.list("dir/").await?;
for entry in entries {
  let meta = op.metadata(&entry, Metakey::ContentLength | Metakey::ContentType).await?;
  println!("{} {}", entry.name(), entry.metadata().content_length());
}

// After
let entries: Vec<Entry> = op
  .list_with("dir/")
  .metakey(Metakey::ContentLength | Metakey::ContentType).await?;
for entry in entries {
  println!("{} {}", entry.name(), entry.metadata().content_length());
}
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-2852-native-capability" class="doc-anchor">Â§</a>RFC-2852: Native Capability

[RFC-2852](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html "mod opendal::docs::rfcs::rfc_2852_native_capability") proposes new `native_capability` and `full_capability` API to allow users to check if the underlying service supports a capability natively.

- `native_capability` returns `true` if the capability is supported natively.
- `full_capability` returns `true` if the capability is supported, maybe via a layer.

Most of time, you can use `full_capability` to replace `capability` call. But to check if the capability is supported natively for better performance design, please use `native_capability` instead.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#buffered-writer" class="doc-anchor">Â§</a>Buffered Writer

OpenDAL v0.40 added buffered writer support!

Users donâ€™t need to specify the `content_length()` for writer anymore!

``` diff
- let mut w = op.writer_with("path/to/file").content_length(1024).await?;
+ let mut w = op.writer_with("path/to/file").await?;
```

Users can specify the `buffer()` to control the size we call underlying storage:

``` rust
let mut w = op.writer_with("path/to/file").buffer(8 * 1024 * 1024).await?;
```

If buffer is not specified, we will call underlying storage everytime we call `write`. Otherwise, we will make sure to call underlying storage when buffer is full or `close` is called.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rangeread-and-rangereader" class="doc-anchor">Â§</a>RangeRead and RangeReader

OpenDAL v0.40 removed the origin `range_read` and `range_reader` interfaces, please use `read_with().range()` or `reader_with().range()`.

``` diff
- op.range_read(path, range_start..range_end).await?;
+ op.read_with(path).range(range_start..range_end).await?;
```

``` diff
- let reader = op.range_reader(path, range_start..range_end).await?;
+ let reader = op.reader_with(path).range(range_start..range_end).await?;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-8" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#rfc-3017-remove-write-copy-from" class="doc-anchor">Â§</a>RFC-3017 Remove Write Copy From

[RFC-3017](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html "mod opendal::docs::rfcs::rfc_3017_remove_write_copy_from") removes `copy_from` API from the `oio::Write` trait. Users who implements services and layers by hand should remove this API.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v039" class="doc-anchor">Â§</a>Upgrade to v0.39

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-14" class="doc-anchor">Â§</a>Public API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#service-s3-role-arn-behavior" class="doc-anchor">Â§</a>Service S3 Role Arn Behavior

In PR \#2687, OpenDAL changed the behavior when `role_arn` has been specified.

OpenDAL used to override role_arn simply. But since this version, OpenDAL will make sure to use assume_role with specified `role_arn` and `external_id` (if supplied).

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#retrylayer-supports-retryinterceptor" class="doc-anchor">Â§</a>RetryLayer supports RetryInterceptor

In PR \#2666, `RetryLayer` supports `RetryInterceptor`. To implement this change, `RetryLayer` changed itâ€™s in-memory layout by adding a new generic parameter `I` to `RetryLayer<I>`.

Users who stores `RetryLayer` in struct or enum will need to change the type if they donâ€™t want to use default behavior.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-9" class="doc-anchor">Â§</a>Raw API

In PR \#2698, OpenDAL re-org the internal structure of `opendal::raw::oio` and changed some APIs name.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v038" class="doc-anchor">Â§</a>Upgrade to v0.38

There are no public API changes.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-10" class="doc-anchor">Â§</a>Raw API

OpenDAL add the `Write::sink` API to enable streaming writing. This is a breaking change for users who depend on the raw API.

For a quick fix, users who have implemented `opendal::raw::oio::Write` can return an `Unsupported` error for `Write::sink()`.

More details could be found at [RFC: Writer `sink` API](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html "mod opendal::docs::rfcs::rfc_2083_writer_sink_api").

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v037" class="doc-anchor">Â§</a>Upgrade to v0.37

In v0.37.0, OpenDAL bump the version of `reqsign` to v0.13.0.

There are no public API and raw API changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v036" class="doc-anchor">Â§</a>Upgrade to v0.36

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-15" class="doc-anchor">Â§</a>Public API

In v0.36, OpenDAL improving the `xxx_with` API by allow it to be called in chain:

After this change, all `xxx_with` alike call will be changed from

``` rust
let bs = op.read_with(
  "path/to/file",
  OpRead::new()
    .with_range(0..=1024)
    .with_if_match("<etag>")
    .with_if_none_match("<etag>")
    .with_override_cache_control("<cache_control>")
    .with_override_content_disposition("<content_disposition>")
  ).await?;
```

to

``` rust
let bs = op.read_with("path/to/file")
  .range(0..=1024)
  .if_match("<etag>")
  .if_none_match("<etag>")
  .override_cache_control("<cache_control>")
  .override_content_disposition("<content_disposition>")
  .await?;
```

For blocking API calls, we will need a `call()` at the end:

``` rust
let bs = bop.read_with("path/to/file")
  .range(0..=1024)
  .if_match("<etag>")
  .if_none_match("<etag>")
  .override_cache_control("<cache_control>")
  .override_content_disposition("<content_disposition>")
  .call()?;
```

Along with this change, users donâ€™t need to call `OpXxx` anymore so we moved it to `raw` API.

More details could be found at [RFC: Chain Based Operator API](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html "mod opendal::docs::rfcs::rfc_2299_chain_based_operator_api").

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-11" class="doc-anchor">Â§</a>Raw API

Migrated `opendal::ops` to `opendal::raw::ops`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v035" class="doc-anchor">Â§</a>Upgrade to v0.35

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-16" class="doc-anchor">Â§</a>Public API

- OpenDAL removes rarely used `Operator::from_env` and `Operator::from_iter` APIs
  - Users can use `Operator::via_map` instead.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-12" class="doc-anchor">Â§</a>Raw API

- OpenDAL adds `append` support with could break existing layers. Please make sure `append` requests have been forward correctly.
- After the merging of `scan` and `list`, OpenDAL removes the `scan` from raw API. Please use `list_without_delimiter` instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v034" class="doc-anchor">Â§</a>Upgrade to v0.34

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-17" class="doc-anchor">Â§</a>Public API

- OpenDAL raises itâ€™s MSRV to 1.65 for dependencies changes
- `OperatorInfo::can_scan` has been removed, to check if underlying services support scan a dir natively, please use `Capability::list_without_delimiter` instead.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-13" class="doc-anchor">Â§</a>Raw API

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#merged-scan-into-list" class="doc-anchor">Â§</a>Merged `scan` into `list`

After `Capability` introduced, we have added `delimiter` in `OpList`. Users can specify the delimiter to `""` or `"/"` to control the list behavior.

Along with this change, `Operator::scan()` becomes a short alias of `Operator::list_with(OpList::new().with_delimiter(""))`.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#typed-kv-adapter" class="doc-anchor">Â§</a>Typed Kv Adapter

In v0.34, OpenDAL adds a typed kv adapter for zero-copy read and write. If you are implemented kv adapter for a rust in-memory data struct, please consider migrate.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v033" class="doc-anchor">Â§</a>Upgrade to v0.33

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#public-api-18" class="doc-anchor">Â§</a>Public API

OpenDAL 0.33 has redesigned the `Writer` API, replacing all instances of `writer.append()` with `writer.write()`. For more information, please refer to [`Writer`](https://opendal.apache.org/docs/rust/opendal/struct.Writer.html "struct opendal::Writer").

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#raw-api-14" class="doc-anchor">Â§</a>Raw API

In addition to the redesign of the `Writer` API, we have removed `append` from `oio::Write`. Therefore, users who implement services and layers should also remove it.

After v0.33 landing, services should handle `OpWrite::content_length` correctly by following these guidelines:

- If the writer does not support uploading unsized data, return a response of `NotSupported` if content length is `None`.
- Otherwise, continue writing data until either `close` or `abort` has been called.

Furthermore, OpenDAL 0.33 introduces a new concept called `Capability` which replaces `AccessorCapability`. Services must adapt to this change.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v032" class="doc-anchor">Â§</a>Upgrade to v0.32

OpenDAL 0.32 doesnâ€™t have much breaking changes.

We changed `Accessor::create` into `Accessor::create_dir`. Only users who implement `Layer` need to change.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v031" class="doc-anchor">Â§</a>Upgrade to v0.31

In version v0.31 of OpenDAL, we made some internal refactoring to improve its compatibility with the ecosystem.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#msrv-bump" class="doc-anchor">Â§</a>MSRV Bump

We increased the MSRV to 1.64 from v0.31 onwards. Although it is still possible to build OpenDAL under older rustc versions, we cannot guarantee that any issues related to them will be fixed.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#accept-stdtimeduration-instead" class="doc-anchor">Â§</a>Accept `std::time::Duration` instead

Previously, OpenDAL accepted `time::Duration` as input for `presign_xxx`. However, since v0.31, we have changed this to accept `std::time::Duration` so that users do not need to depend on `time`. Internally, we migrated from `time` to `chrono` for better integration with other parts of the ecosystem.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#disable_ec2_metadata-for-services-s3" class="doc-anchor">Â§</a>`disable_ec2_metadata` for services s3

We have added a new configuration option called `disable_ec2_metadata` for the S3 service in response to a mistake where it was mixed up with another option called `disable_config_load`. Users who want to disable loading credentials from EC2 metadata should set this option instead.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#services-feature-flag-1" class="doc-anchor">Â§</a>Services Feature Flag

Starting from v0.31, all services in OpenDAL are split into different feature flags. To enable only S3 support, use the following TOML configuration:

``` toml
opendal = {
    version = "0.31",
    default-features = false,
    features = ["services-s3"]
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v030" class="doc-anchor">Â§</a>Upgrade to v0.30

In version 0.30, we made significant breaking changes by removing objects. Our goal in doing so was to provide our users with APIs that are easier to understand and maintain.

More details could be found at [RFC: Remove Object Concept](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1477_remove_object_concept/index.html "mod opendal::docs::rfcs::rfc_1477_remove_object_concept").

To upgrade to OpenDAL v0.30, users need to make the following changes:

- regex replace `object\((.*)\).reader\(\)` to `reader($1)`
  - replace the function on your case, itâ€™s recommended to do it one by one
- rename `ObjectMetakey` =\> `Metakey`
- rename `ObjectMode` =\> `EntryMode`
- replace `ErrorKind::ObjectXxx` to `ErrorKind::Xxx`
- rename `AccessorMetadata` =\> `AccessorInfo`
- rename `ObjectMetadata` =\> `Metadata`
- replace `operator.metadata()` =\> `operator.info()`

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v029" class="doc-anchor">Â§</a>Upgrade to v0.29

In v0.29, we introduced [Object Writer](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1420_object_writer/index.html "mod opendal::docs::rfcs::rfc_1420_object_writer") to replace existing Multipart related APIs.

Users can now append multiparts bytes into object via:

``` rust
let mut w = o.writer().await?;
w.write(bs1).await?;
w.write(bs2).await?;
w.close()
```

Along with this change, we cleaned up a lot of internal structs and traits. Users who used to depend on `opendal::raw::io::{input,output}` should use `opendal::raw::oio` instead.

Also, decompress related feature also removed. Users can use `async-compression` with `ObjectReader` directly.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v028" class="doc-anchor">Â§</a>Upgrade to v0.28

In v0.28, we introduced [Query Based Metadata](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1398_query_based_metadata/index.html "mod opendal::docs::rfcs::rfc_1398_query_based_metadata"). Users can query cached metadata with `ObjectMetakey` to make sure that OpenDAL always makes the best decision.

``` diff
- pub async fn metadata(&self) -> Result<ObjectMetadata>;
+ pub async fn metadata(
+        &self,
+        flags: impl Into<FlagSet<ObjectMetakey>>,
+    ) -> Result<Arc<ObjectMetadata>>;
```

Please visit `Object::metadata()`â€™s example for more details.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v027" class="doc-anchor">Â§</a>Upgrade to v0.27

In v0.27, we refactored our `list` related logic and added `scan` support. So make `Pager` and `BlockingPager` associated types in `Accessor` too!

``` diff
pub trait Accessor: Send + Sync + Debug + Unpin + 'static {
    type Reader: output::Read;
    type BlockingReader: output::BlockingRead;
+    type Pager: output::Page;
+    type BlockingPager: output::BlockingPage;
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#user-defined-layers" class="doc-anchor">Â§</a>User defined layers

Due to this change, all layers implementation should be changed. If there is not changed over pager, they can be changed like the following:

``` diff
impl<A: Accessor> LayeredAccessor for MyAccessor<A> {
    type Inner = A;
    type Reader = MyReader<A::Reader>;
    type BlockingReader = MyReader<A::BlockingReader>;
+    type Pager = A::Pager;
+    type BlockingPager = A::BlockingPager;

+    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Pager)> {
+        self.inner.list(path, args).await
+    }

+    async fn scan(&self, path: &str, args: OpScan) -> Result<(RpScan, Self::Pager)> {
+        self.inner.scan(path, args).await
+    }

+    fn blocking_list(&self, path: &str, args: OpList) -> Result<(RpList, Self::BlockingPager)> {
+        self.inner.blocking_list(path, args)
+    }

+    fn blocking_scan(&self, path: &str, args: OpScan) -> Result<(RpScan, Self::BlockingPager)> {
+        self.inner.blocking_scan(path, args)
+    }
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#usage-of-ops" class="doc-anchor">Â§</a>Usage of ops

To reduce the understanding overhead, we move all `OpXxx` into `opendal::ops` now. User may need to change:

``` diff
- use opendal::OpWrite;
+ use opendal::ops::OpWrite;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#usage-of-retrylayer" class="doc-anchor">Â§</a>Usage of RetryLayer

`backon` is the implementation detail of our `RetryLayer`, so we hide it from our public API. Users of `RetryLayer` need to change the code like:

``` diff
- RetryLayer::new(backon::ExponentialBackoff::default())
+ RetryLayer::new()
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v026" class="doc-anchor">Â§</a>Upgrade to v0.26

In v0.26 we have replaced all internal dynamic dispatch usage with static dispatch. With this change, we can ensure that all operations performed inside OpenDAL are zero cost.

Due to this change, we have to refactor the logic of `Operator`â€™s init logic. In v0.26, we added `opendal::Builder` trait and `opendal::OperatorBuilder`. For the first glance, the only change to existing code will be like:

``` diff
- let op = Operator::new(builder.build()?);
+ let op = Operator::new(builder.build()?).finish();
```

By adding a `finish()` call, we will erase all generic types so that `Operator` can still be easily used everywhere as before.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#accessor" class="doc-anchor">Â§</a>Accessor

In v0.26, `Accessor` has been changed into trait with associated types.

All services need to declare the types returned as `Reader` or `BlockingReader`:

``` rust
pub trait Accessor: Send + Sync + Debug + Unpin + 'static {
    type Reader: output::Read;
    type BlockingReader: output::BlockingRead;
}
```

If your service doesnâ€™t support `read` or `blocking_read`, we can use `()` to represent a dummy reader:

``` rust
impl Accessor for MyDummyAccessor {
    type Reader = ();
    type BlockingReader = ();
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#layer" class="doc-anchor">Â§</a>Layer

As described before, OpenDAL prefer to use static dispatch. Layers are required to implement the new `Layer` and `LayeredAccessor` trait:

``` rust
pub trait Layer<A: Accessor> {
    type LayeredAccessor: Accessor;

    fn layer(&self, inner: A) -> Self::LayeredAccessor;
}

#[async_trait]
pub trait LayeredAccessor: Send + Sync + Debug + Unpin + 'static {
    type Inner: Accessor;
    type Reader: output::Read;
    type BlockingReader: output::BlockingRead;
}
```

`LayeredAccessor` is a wrapper of `Accessor` with the typed `Innder`. All methods that not implemented will be forward to inner instead.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#builder" class="doc-anchor">Â§</a>Builder

Since v0.26, we implement `opendal::Builder` for all services, and servicesâ€™ mod will not be exported.

``` diff
- use opendal::services::s3::Builder;
+ use opendal::services::S3;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#conclusion" class="doc-anchor">Â§</a>Conclusion

Sorry again for the big changes in this release. Itâ€™s a big step for OpenDAL to work in more critical systems.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v025" class="doc-anchor">Â§</a>Upgrade to v0.25

In v0.25, we bring the same feature sets from `ObjectReader` to `BlockingObjectReader`.

Due to this change, all code that depends on `BlockingBytesReader` should be refactored.

- `BlockingBytesReader` =\> `input::BlockingReader`
- `BlockingOutputBytesReader` =\> `output::BlockingReader`

Most changes only happen inside. Users not using `opendal::raw::*` will not be affected.

Apart from this change, we refactored s3 credential loading logic. After this change, we can disable the config load instead of the credential methods.

- `builder.disable_credential_loader` =\> `builder.disable_config_load`

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v024" class="doc-anchor">Â§</a>Upgrade to v0.24

In v0.24, we made a big refactor on our internal IO-related traits. In this version, we split our IO traits into `input` and `output` versions:

Take `Reader` as an example:

`input::Reader` is the user input reader, which only requires `futures::AsyncRead + Send`.

`output::Reader` is the reader returned by `OpenDAL`, which implements `futures::AsyncRead`, `futures::AsyncSeek`, and `futures::Stream<Item=io::Result<Bytes>>`. Besides, `output::Reader` also implements `Send + Sync`, which makes it useful for users.

Due to this change, all code that depends on `BytesReader` should be refactored.

- `BytesReader` =\> `input::Reader`
- `OutputBytesReader` =\> `output::Reader`

Thanks to the change of IO trait split, we make `ObjectReader` implements all needed traits:

- `futures::AsyncRead`
- `futures::AsyncSeek`
- `futures::Stream<Item=io::Result<Bytes>>`

Thus, we removed the `seekable_reader` API. They can be replaced by `range_reader`:

- `o.seekable_reader` =\> `o.range_reader`

Most changes only happen inside. Users not using `opendal::raw::*` will not be affected.

Sorry for the inconvenience. I think those changes are required and make OpenDAL better! Welcome any comments at [Discussion](https://github.com/apache/opendal/discussions).

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v021" class="doc-anchor">Â§</a>Upgrade to v0.21

v0.21 is an internal refactor version of OpenDAL. In this version, we refactored our error handling and our `Accessor` APIs. Thanks to those internal changes, we added an object-level metadata cache, making it nearly zero cost to reuse existing metadata continuously.

Letâ€™s start with our errors.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#error-handling" class="doc-anchor">Â§</a>Error Handling

As described in [RFC-0977: Refactor Error](https://opendal.apache.org/rfcs/0977-refactor-error.html), we refactor opendal error by a new error called [`opendal::Error`](https://opendal.apache.org/opendal/struct.Error.html).

This change will affect all APIs that are used to return `io::Error`.

To migrate this, please replace `std::io::Error` with `opendal::Error`:

``` diff
- use std::io::Result;
+ use opendal::Result;
```

And the following error kinds should be updated:

- `std::io::ErrorKind::NotFound` =\> `opendal::ErrorKind::ObjectNotFound`
- `std::io::ErrorKind::PermissionDenied` =\> `opendal::ErrorKind::ObjectPermissionDenied`

And since v0.21, we will return errors `ObjectIsADirectory` and `ObjectNotADirectory` instead of `anyhow::Error`.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#accessor-api" class="doc-anchor">Â§</a>Accessor API

In v0.21, we refactor the whole `Accessor`â€™s API:

``` diff
- async fn write(&self, path: &str, args: OpWrite, r: BytesReader) -> Result<u64>
+ async fn write(&self, path: &str, args: OpWrite, r: BytesReader) -> Result<RpWrite>
```

Since v0.21, we will return a reply struct for different operations called `RpWrite` instead of an exact type. We can split OpenDALâ€™s public API and raw API with this change.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#objectlist-and-page" class="doc-anchor">Â§</a>ObjectList and Page

Since v0.21, `Accessor` will return `Pager` for `List`:

``` diff
- async fn list(&self, path: &str, args: OpList) -> Result<ObjectStreamer>
+ async fn list(&self, path: &str, args: OpList) -> Result<(RpList, output::Pager)>
```

And `Object` will return an `ObjectLister` which is built upon `Page`:

``` rust
pub async fn list(&self) -> Result<ObjectLister> { ... }
```

`ObjectLister` can be used as an object stream as before. It also provides the function `next_page` to get the underlying pages directly:

``` rust
impl ObjectLister {
    pub async fn next_page(&mut self) -> Result<Option<Vec<Object>>>;
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#code-layout" class="doc-anchor">Â§</a>Code Layout

Since v0.21, we have categorized all APIs into `public` and `raw`.

Public APIs are exposed under `opendal::Xxx`; they are user-face APIs that are easy to use and understand.

Raw APIs are exposed under `opendal::raw::Xxx`; they are implementation details for underlying services and layers.

Please replace all usage of `opendal::io_util::*` and `opendal::http_util::*` to `opendal::raw::*` instead.

With this change, new users of OpenDAL maybe be it easier to get started.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#summary" class="doc-anchor">Â§</a>Summary

Sorry for introducing too much breaking change in a single version. This version can be a solid version for preparing OpenDAL v1.0.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v020" class="doc-anchor">Â§</a>Upgrade to v0.20

v0.20 is a big release that we introduce a lot of performance related changes.

To make the best of information from `read` operation, we propose and implemented [RFC-0926: Object Reader](https://opendal.apache.org/rfcs/0926-object-reader.html). By this RFC, we can fetch content length from `ObjectReader` now!

``` rust
pub struct ObjectReader {
    inner: BytesReader
    meta: ObjectMetadata,
}

impl ObjectReader {
    pub fn content_length(&self) -> u64 {}
    pub fn last_modified(&self) -> Option<OffsetDateTime> {}
    pub fn etag(&self) -> Option<String> {}
}
```

To make this happen, we changed our `Accessor` API:

``` diff
- async fn read(&self, path: &str, args: OpRead) -> Result<BytesReader> {}
+ async fn read(&self, path: &str, args: OpRead) -> Result<ObjectReader> {}
```

All layers should be updated to meet this change. Also, itâ€™s required to return `content_length` while building `ObjectReader`. Please make sure the returning `ObjectMetadata` is used correctly.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v019" class="doc-anchor">Â§</a>Upgrade to v0.19

OpenDAL deprecate some features:

- `serde`: We will enable it by default.
- `layers-retry`: We will enable retry support by default.
- `layers-metadata-cache`: We will enable it by default.

Deprecated types like `DirEntry` has been removed.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v018" class="doc-anchor">Â§</a>Upgrade to v0.18

OpenDAL v0.18 introduces the following breaking changes:

- Deprecated feature flag `services-http` has been removed.
- All `DirXxx` items have been renamed to `ObjectXxx` to make them more consistent.
  - `DirEntry` -\> `Entry`
  - `DirStream` -\> `ObjectStream`
  - `DirStreamer` -\> `ObjectStream`
  - `DirIterate` -\> `ObjectIterate`
  - `DirIterator` -\> `ObjectIterator`

Besides, we also make a big change to our `Entry` API. Since v0.18, we can fully reuse the metadata that fetched during `list`. Take `entry.content_length()` for example:

- If `content_length` is already known, we will return directly.
- If not, we will check if the object entry is `complete`:
  - If `complete`, the entry already fetched all metadata that it could have, return directly.
  - If not, we will send a `stat` call to get the `metadata` and refresh our cache.

This change means:

- All API like `content_length` will be changed into async functions.
- `metadata` and `blocking_metadata` will not return errors anymore.
- To retrieve the latest meta, please use `entry.into_object().metadata()` instead.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v017" class="doc-anchor">Â§</a>Upgrade to v0.17

OpenDAL v0.17 refactor the `Accessor` to make space for future features.

We move `path String` out of the `OpXxx` to function args so that we donâ€™t need to clone twice.

``` diff
- async fn read(&self, args: OpRead) -> Result<BytesReader>
+ async fn read(&self, path: &str, args: OpRead) -> Result<BytesReader>
```

For more information about this change, please refer to [RFC-0661: Path In Accessor](https://opendal.apache.org/rfcs/0661-path-in-accessor.html).

And since OpenDAL v0.17, we will use `rustls` as default tls engine for our underlying http client. Since this release, we will not depend on `openssl` anymore.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v016" class="doc-anchor">Â§</a>Upgrade to v0.16

OpenDAL v0.16 refactor the internal implementation of `http` service. Since v0.16, http service can be used directly without enabling `services-http` feature. Accompany by these changes, http service has the following breaking changes:

- `services-http` feature has been deprecated. Enabling `services-http` is a no-op now.
- http service is read only services and canâ€™t be used to `list` or `write`.

OpenDAL introduces a new layer `ImmutableIndexLayer` that can add `list` capability for services:

``` rust
use opendal::layers::ImmutableIndexLayer;
use opendal::Operator;
use opendal::Scheme;

async fn main() {
    let mut iil = ImmutableIndexLayer::default();

    for i in ["file", "dir/", "dir/file", "dir_without_prefix/file"] {
        iil.insert(i.to_string())
    }

    let op = Operator::from_env(Scheme::Http)?.layer(iil);
}
```

For more information about this change, please refer to [RFC-0627: Split Capabilities](https://opendal.apache.org/rfcs/0627-split-capabilities.html).

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v014" class="doc-anchor">Â§</a>Upgrade to v0.14

OpenDAL v0.14 removed all deprecated APIs in previous versions, including:

- `Operator::with_backoff` in v0.13
- All services `Builder::finish()` in v0.12
- All services `Backend::build()` in v0.12

Please visit related versionâ€™s upgrade guide for migration.

And in OpenDAL v0.14, we introduce a break change for `write` operations.

``` diff
pub trait Accessor {
    - async fn write(&self, args: &OpWrite) -> Result<BytesWriter> {}
    + async fn write(&self, args: &OpWrite, r: BytesReader) -> Result<u64> {}
}
```

The following APIs have affected by this change:

- `Object::write` now accept `impl Into<Vec<u8>>` instead of `AsRef<&[u8]>`
- `Object::writer` has been removed.
- `Object::write_from` has been added to support write from a reader.
- All layers should be refactored to adapt new `Accessor` trait.

For more information about this change, please refer to [RFC-0554: Write Refactor](https://opendal.apache.org/rfcs/0554-write-refactor.html).

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v013" class="doc-anchor">Â§</a>Upgrade to v0.13

OpenDAL deprecate `Operator::with_backoff` since v0.13.

Please use [`RetryLayer`](https://opendal.apache.org/opendal/layers/struct.RetryLayer.html) instead:

``` rust
use anyhow::Result;
use backon::ExponentialBackoff;
use opendal::layers::RetryLayer;
use opendal::Operator;
use opendal::Scheme;

let _ = Operator::from_env(Scheme::Fs)
    .expect("must init")
    .layer(RetryLayer::new(ExponentialBackoff::default()));
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v012" class="doc-anchor">Â§</a>Upgrade to v0.12

OpenDAL introduces breaking changes for services initiation.

Since v0.12, `Operator::new` will accept `impl Accessor + 'static` instead of `Arc<dyn Accessor>`:

``` rust
impl Operator {
    pub fn new(accessor: impl Accessor + 'static) -> Self { .. }
}
```

Every serviceâ€™s `Builder` now have a `build()` API which can be run without async:

``` rust
let mut builder = fs::Builder::default();
let op: Operator = Operator::new(builder.build()?);
```

Along with these changes, `Operator::from_iter` and `Operator::from_env` now is a blocking API too.

For more information about this change, please refer to [RFC-0501: New Builder](https://opendal.apache.org/rfcs/0501-new-builder.html).

The following APIs have been deprecated:

- All services `Builder::finish()` (replaced by `Builder::build()`)
- All services `Backend::build()` (replace by `Builder::default()`)

The following APIs have been removed:

- public struct `Metadata` (deprecated in v0.8, replaced by `ObjectMetadata`)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v08" class="doc-anchor">Â§</a>Upgrade to v0.8

OpenDAL introduces a breaking change of `list` related operations in v0.8.

Since v0.8, `list` will return `DirStreamer` instead:

``` rust
pub trait Accessor: Send + Sync + Debug {
    async fn list(&self, args: &OpList) -> Result<DirStreamer> {}
}
```

`DirStreamer` streams `DirEntry` which carries `ObjectMode`, so that we donâ€™t need an extra call to get object mode:

``` rust
impl DirEntry {
    pub fn mode(&self) -> ObjectMode {
        self.mode
    }
}
```

And `DirEntry` can be converted into `Object` without overhead:

``` rust
let o: Object = de.into()
```

Since `v0.8`, `opendal::Metadata` has been deprecated by `opendal::ObjectMetadata`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v07" class="doc-anchor">Â§</a>Upgrade to v0.7

OpenDAL introduces a breaking change of `decompress_read` related in v0.7.

Since v0.7, `decompress_read` and `decompress_reader` will return `Ok(None)` while OpenDAL canâ€™t detect the correct compress algorithm.

``` rust
impl Object {
    pub async fn decompress_read(&self) -> Result<Option<Vec<u8>>> {}
    pub async fn decompress_reader(&self) -> Result<Option<impl BytesRead>> {}
}
```

So users should match and check the `None` case:

``` rust
let bs = o.decompress_read().await?.expect("must have valid compress algorithm");
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#upgrade-to-v04" class="doc-anchor">Â§</a>Upgrade to v0.4

OpenDAL introduces many breaking changes in v0.4.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#objectreader-is-not-asyncseek-anymore" class="doc-anchor">Â§</a>Object::reader() is not `AsyncSeek` anymore

Since v0.4, `Object::reader()` will return `impl BytesRead` instead of `Reader` that implements `AsyncRead` and `AsyncSeek`. Users who want `AsyncSeek` please wrapped with `opendal::io_util::seekable_read`:

``` rust
use opendal::io_util::seekable_read;

let o = op.object("test");
let mut r = seekable_read(&o, 10..);
r.seek(SeekFrom::Current(10)).await?;
let mut bs = vec![0;10];
r.read(&mut bs).await?;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#use-rangebounds-instead" class="doc-anchor">Â§</a>Use RangeBounds instead

Since v0.4, the following APIs will be removed.

- `Object::limited_reader(size: u64)`
- `Object::offset_reader(offset: u64)`
- `Object::range_reader(offset: u64, size: u64)`

Instead, OpenDAL is providing a more general `range_reader` powered by `RangeBounds`:

``` rust
pub async fn range_reader(&self, range: impl RangeBounds<u64>) -> Result<impl BytesRead>
```

Users can use their familiar rust range syntax:

``` rust
let r = o.range_reader(1024..2048).await?;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#return-ioresult-instead" class="doc-anchor">Â§</a>Return io::Result instead

Since v0.4, all functions in OpenDAL will return `std::io::Result` instead.

Please check via `std::io::ErrorKind` directly:

``` rust
use std::io::ErrorKind;

if let Err(e) = op.object("test_file").metadata().await {
    if e.kind() == ErrorKind::NotFound {
        println!("object not exist")
    }
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#removing-credential" class="doc-anchor">Â§</a>Removing Credential

Since v0.4, `Credential` has been removed, please use the API provided by `Builder` directly.

``` rust
builder.access_key_id("access_key_id");
builder.secret_access_key("secret_access_key");
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#write-returns-byteswriter-instead" class="doc-anchor">Â§</a>Write returns `BytesWriter` instead

Since v0.4, `Accessor::write` will return a `BytesWriter` instead accepting a `BoxedAsyncReader`.

Along with this change, the old `Writer` has been replaced by a new set of write functions:

``` rust
pub async fn write(&self, bs: impl AsRef<[u8]>) -> Result<()> {}
pub async fn writer(&self, size: u64) -> Result<impl BytesWrite> {}
```

Users can write into an object more easily:

``` rust
let _ = op.object("path/to/file").write("Hello, World!").await?;
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#io_util-replaces-readers" class="doc-anchor">Â§</a>`io_util` replaces `readers`

Since v0.4, mod `io_util` will replace `readers`. In `io_utils`, OpenDAL provides helpful functions like:

- `into_reader`: Convert `BytesStream` into `BytesRead`
- `into_sink`: Convert `BytesWrite` into `BytesSink`
- `into_stream`: Convert `BytesRead` into `BytesStream`
- `into_writer`: Convert `BytesSink` into `BytesWrite`
- `observe_read`: Add callback for `BytesReader`
- `observe_write`: Add callback for `BytesWrite`

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/upgrade/index.html#new-type-alias" class="doc-anchor">Â§</a>New type alias

For better naming, types that OpenDAL returns have been renamed:

- `AsyncRead + Unpin + Send` =\> `BytesRead`
- `BoxedAsyncReader` =\> `BytesReader`
- `AsyncWrite + Unpin + Send` =\> `BytesWrite`
- `BoxedAsyncWriter` =\> `BytesWriter`
- `ObjectStream` =\> `ObjectStreamer`
