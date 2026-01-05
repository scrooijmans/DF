- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/core/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- Core

On this page

# Core

## Apache OpenDALâ„¢ Rust Core: One Layer, All Storage.

[<img src="out_opendal/core/index_media/49073adb969e94cba3d1c470073bbb2ae9fb8754.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Build Status" />](https://github.com/apache/opendal/actions?query=branch%3Amain) [<img src="out_opendal/core/index_media/1b610f4f7a0682a2b8e99de8878cb5246ae3692d.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Latest Version" />](https://crates.io/crates/opendal) [<img src="out_opendal/core/index_media/ee3da019d342d08430b7d4b5f0c1964da2544562.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate Downloads" />](https://crates.io/crates/opendal) [<img src="out_opendal/core/index_media/8344f1bc1d55e7813afa8d864a46394b41605b73.svg" class="img_KBPg" decoding="async" loading="lazy" alt="chat" />](https://opendal.apache.org/discord)

Apache OpenDALâ„¢ is an Open Data Access Layer that enables seamless interaction with diverse storage services.

<img src="out_opendal/core/index_media/841a60e6d4bc3274b1106e1a26e6b71a849bb2e2.png" class="img_KBPg" style="width:61.8%" decoding="async" loading="lazy" alt="OpenDAL Architectural" />

## Useful Links<a href="https://opendal.apache.org/core/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- Documentation: [release](https://docs.rs/opendal/) \| [dev](https://opendal.apache.org/docs/rust/opendal/)
- [Examples](https://github.com/apache/opendal/blob/main/core/examples)
- [Release Notes](https://docs.rs/opendal/latest/opendal/docs/changelog/index.html)
- [Upgrade Guide](https://docs.rs/opendal/latest/opendal/docs/upgrade/index.html)
- [RFC List](https://docs.rs/opendal/latest/opendal/docs/rfcs/index.html)

## Services<a href="https://opendal.apache.org/core/#services" class="hash-link" aria-label="Direct link to Services" translate="no" title="Direct link to Services">â€‹</a>

OpenDAL supports the following storage [services](https://docs.rs/opendal/latest/opendal/services/index.html):

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<thead>
<tr>
<th>Type</th>
<th>Services</th>
</tr>
</thead>
<tbody>
<tr>
<td>Standard Storage Protocols</td>
<td>ftp http <a href="https://datatracker.ietf.org/doc/html/draft-ietf-secsh-filexfer-02">sftp</a> <a href="https://datatracker.ietf.org/doc/html/rfc4918">webdav</a></td>
</tr>
<tr>
<td>Object Storage Services</td>
<td><a href="https://azure.microsoft.com/en-us/services/storage/blobs/">azblob</a> <a href="https://www.tencentcloud.com/products/cos">cos</a> <a href="https://cloud.google.com/storage">gcs</a> <a href="https://www.huaweicloud.com/intl/en-us/product/obs.html">obs</a> <a href="https://www.aliyun.com/product/oss">oss</a> <a href="https://aws.amazon.com/s3/">s3</a><br />
<a href="https://www.backblaze.com/">b2</a> <a href="https://docs.openstack.org/swift/latest/">openstack_swift</a> <a href="https://www.upyun.com/">upyun</a> <a href="https://vercel.com/docs/storage/vercel-blob">vercel-blob</a></td>
</tr>
<tr>
<td>File Storage Services</td>
<td>fs <a href="https://docs.alluxio.io/os/user/stable/en/api/REST-API.html">alluxio</a> <a href="https://azure.microsoft.com/en-us/products/storage/data-lake-storage/">azdls</a> <a href="https://learn.microsoft.com/en-us/rest/api/storageservices/file-service-rest-api">azfile</a> <a href="https://github.com/compio-rs/compio/">compfs</a><br />
<a href="https://docs.databricks.com/en/dbfs/index.html">dbfs</a> <a href="https://www.mongodb.com/docs/manual/core/gridfs/">gridfs</a> <a href="https://hadoop.apache.org/docs/r3.3.4/hadoop-project-dist/hadoop-hdfs/HdfsDesign.html">hdfs</a> <a href="https://github.com/Kimahriman/hdfs-native">hdfs-native</a> <a href="https://ipfs.tech/">ipfs</a> <a href="https://hadoop.apache.org/docs/stable/hadoop-project-dist/hadoop-hdfs/WebHDFS.html">webhdfs</a></td>
</tr>
<tr>
<td>Consumer Cloud Storage Service</td>
<td><a href="https://www.aliyundrive.com/">aliyun-drive</a> <a href="https://www.google.com/drive/">gdrive</a> <a href="https://www.microsoft.com/en-us/microsoft-365/onedrive/online-cloud-storage">onedrive</a> <a href="https://www.dropbox.com/">dropbox</a> <a href="https://koofr.eu/">koofr</a><br />
<a href="https://www.pcloud.com/">pcloud</a> <a href="https://www.seafile.com/">seafile</a> <a href="https://360.yandex.com/disk/">yandex-disk</a></td>
</tr>
<tr>
<td>Key-Value Storage Services</td>
<td><a href="https://crates.io/crates/cacache">cacache</a> <a href="https://developers.cloudflare.com/kv/">cloudflare-kv</a> <a href="https://github.com/xacrimon/dashmap">dashmap</a> memory <a href="https://etcd.io/">etcd</a><br />
<a href="https://www.foundationdb.org/">foundationdb</a> <a href="https://crates.io/crates/persy">persy</a> <a href="https://redis.io/">redis</a> <a href="http://rocksdb.org/">rocksdb</a> <a href="https://crates.io/crates/sled">sled</a><br />
<a href="https://crates.io/crates/redb">redb</a> <a href="https://tikv.org/">tikv</a></td>
</tr>
<tr>
<td>Database Storage Services</td>
<td><a href="https://developers.cloudflare.com/d1/">d1</a> <a href="https://www.mongodb.com/">mongodb</a> <a href="https://www.mysql.com/">mysql</a> <a href="https://www.postgresql.org/">postgresql</a> <a href="https://www.sqlite.org/">sqlite</a> <a href="https://surrealdb.com/">surrealdb</a></td>
</tr>
<tr>
<td>Cache Storage Services</td>
<td><a href="https://docs.github.com/en/actions/using-workflows/caching-dependencies-to-speed-up-workflows">ghac</a> <a href="https://memcached.org/">memcached</a> <a href="https://github.com/moka-rs/mini-moka">mini-moka</a> <a href="https://github.com/moka-rs/moka">moka</a> <a href="https://vercel.com/docs/concepts/monorepos/remote-caching">vercel-artifacts</a></td>
</tr>
<tr>
<td>Git Based Storage Services</td>
<td><a href="https://huggingface.co/">huggingface</a></td>
</tr>
</tbody>
</table>

## Layers<a href="https://opendal.apache.org/core/#layers" class="hash-link" aria-label="Direct link to Layers" translate="no" title="Direct link to Layers">â€‹</a>

OpenDAL supports the following storage [layers](https://docs.rs/opendal/latest/opendal/layers/index.html) to extend the behavior:

| Name | Depends | Description |
|----|----|----|
| [`AsyncBacktraceLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.AsyncBacktraceLayer.html) | [async-backtrace](https://github.com/tokio-rs/async-backtrace) | Add Efficient, logical 'stack' traces of async functions for the underlying services. |
| [`AwaitTreeLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.AwaitTreeLayer.html) | [await-tree](https://github.com/risingwavelabs/await-tree) | Add a Instrument await-tree for actor-based applications to the underlying services. |
| [`BlockingLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.BlockingLayer.html) | [tokio](https://github.com/tokio-rs/tokio) | Add blocking API support for non-blocking services. |
| [`ChaosLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.ChaosLayer.html) | [rand](https://github.com/rust-random/rand) | Inject chaos into underlying services for robustness test. |
| [`ConcurrentLimitLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.ConcurrentLimitLayer.html) | [tokio](https://github.com/tokio-rs/tokio) | Add concurrent request limit. |
| [`DtraceLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.DtraceLayer.html) | [probe](https://github.com/cuviper/probe-rs) | Support User Statically-Defined Tracing(aka USDT) on Linux |
| [`LoggingLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.LoggingLayer.html) | [log](https://github.com/rust-lang/log) | Add log for every operations. |
| [`MetricsLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.MetricsLayer.html) | [metrics](https://github.com/metrics-rs/metrics) | Add metrics for every operations. |
| [`MimeGuessLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.MimeGuessLayer.html) | [mime_guess](https://github.com/abonander/mime_guess) | Add `Content-Type` automatically based on the file extension in the operation path. |
| [`FastraceLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.FastraceLayer.html) | [fastrace](https://github.com/fastracelabs/fastrace) | Add fastrace for every operations. |
| [`OtelMetricsLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.OtelMetricsLayer.html) | \[opentelemetry::metrics\] | Add opentelemetry::metrics for every operations. |
| [`OtelTraceLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.OtelTraceLayer.html) | [opentelemetry::trace](https://docs.rs/opentelemetry/latest/opentelemetry/trace/index.html) | Add opentelemetry::trace for every operations. |
| [`PrometheusClientLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.PrometheusClientLayer.html) | [prometheus_client](https://github.com/prometheus/client_rust) | Add prometheus metrics for every operations. |
| [`PrometheusLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.PrometheusLayer.html) | [prometheus](https://github.com/tikv/rust-prometheus) | Add prometheus metrics for every operations. |
| [`RetryLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.RetryLayer.html) | [backon](https://github.com/Xuanwo/backon) | Add retry for temporary failed operations. |
| [`ThrottleLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.ThrottleLayer.html) | [governor](https://github.com/boinkor-net/governor) | Add a bandwidth rate limiter to the underlying services. |
| [`TimeoutLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.TimeoutLayer.html) | [tokio](https://github.com/tokio-rs/tokio) | Add timeout for every operations to avoid slow or unexpected hang operations. |
| [`TracingLayer`](https://docs.rs/opendal/latest/opendal/layers/struct.TracingLayer.html) | [tracing](https://github.com/tokio-rs/tracing) | Add tracing for every operations. |

## Quickstart<a href="https://opendal.apache.org/core/#quickstart" class="hash-link" aria-label="Direct link to Quickstart" translate="no" title="Direct link to Quickstart">â€‹</a>

``` prism-code
use opendal::Result;
use opendal::layers::LoggingLayer;
use opendal::services;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Pick a builder and configure it.
    let mut builder = services::S3::default();
    builder.bucket("test");

    // Init an operator
    let op = Operator::new(builder)?
        // Init with logging layer enabled.
        .layer(LoggingLayer::default())
        .finish();

    // Write data
    op.write("hello.txt", "Hello, World!").await?;

    // Read data
    let bs = op.read("hello.txt").await?;

    // Fetch metadata
    let meta = op.stat("hello.txt").await?;
    let mode = meta.mode();
    let length = meta.content_length();

    // Delete
    op.delete("hello.txt").await?;

    Ok(())
}
```

## Examples<a href="https://opendal.apache.org/core/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

| Name | Description |
|----|----|
| [Basic](https://github.com/apache/opendal/blob/main/core/examples/basic) | Show how to use opendal to operate storage service. |
| [Concurrent Upload](https://github.com/apache/opendal/blob/main/core/examples/concurrent-upload) | Show how to perform upload concurrently to a storage service. |
| [Multipart Upload](https://github.com/apache/opendal/blob/main/core/examples/multipart-upload) | Show how to perform a multipart upload to a storage service. |

## Contributing<a href="https://opendal.apache.org/core/#contributing" class="hash-link" aria-label="Direct link to Contributing" translate="no" title="Direct link to Contributing">â€‹</a>

Check out the [CONTRIBUTING](https://github.com/apache/opendal/blob/main/core/CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Used by<a href="https://opendal.apache.org/core/#used-by" class="hash-link" aria-label="Direct link to Used by" translate="no" title="Direct link to Used by">â€‹</a>

Check out the [users](https://github.com/apache/opendal/blob/main/core/users.md) list for more details on who is using OpenDAL.

## Branding<a href="https://opendal.apache.org/core/#branding" class="hash-link" aria-label="Direct link to Branding" translate="no" title="Direct link to Branding">â€‹</a>

The first and most prominent mentions must use the full form: **Apache OpenDALâ„¢** of the name for any individual usage (webpage, handout, slides, etc.) Depending on the context and writing style, you should use the full form of the name sufficiently often to ensure that readers clearly understand the association of both the OpenDAL project and the OpenDAL software product to the ASF as the parent organization.

For more details, see the [Apache Product Name Usage Guide](https://www.apache.org/foundation/marks/guide).

## License and Trademarks<a href="https://opendal.apache.org/core/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/10-core.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/core/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/vision/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Vision

<a href="https://opendal.apache.org/category/bindings/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Bindings

- <a href="https://opendal.apache.org/core/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/core/#services" class="table-of-contents__link toc-highlight">Services</a>
- <a href="https://opendal.apache.org/core/#layers" class="table-of-contents__link toc-highlight">Layers</a>
- <a href="https://opendal.apache.org/core/#quickstart" class="table-of-contents__link toc-highlight">Quickstart</a>
- <a href="https://opendal.apache.org/core/#examples" class="table-of-contents__link toc-highlight">Examples</a>
- <a href="https://opendal.apache.org/core/#contributing" class="table-of-contents__link toc-highlight">Contributing</a>
- <a href="https://opendal.apache.org/core/#used-by" class="table-of-contents__link toc-highlight">Used by</a>
- <a href="https://opendal.apache.org/core/#branding" class="table-of-contents__link toc-highlight">Branding</a>
- <a href="https://opendal.apache.org/core/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
