- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/integrations/parquet/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/integrations/" class="breadcrumbs__link">Integrations</a>
- Parquet

On this page

# Parquet

## Apache OpenDALâ„¢ parquet integration

[<img src="out_opendal/integrations/parquet/index_media/49073adb969e94cba3d1c470073bbb2ae9fb8754.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Build Status" />](https://github.com/apache/opendal/actions?query=branch%3Amain) [<img src="out_opendal/integrations/parquet/index_media/8fdee0f616c7d3f3dd696df12c2b2d75cc09f628.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Latest Version" />](https://crates.io/crates/parquet_opendal) [<img src="out_opendal/integrations/parquet/index_media/708482a7c70b0d9ca392dcaae402258a15c491fb.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate Downloads" />](https://crates.io/crates/parquet_opendal) [<img src="out_opendal/integrations/parquet/index_media/8344f1bc1d55e7813afa8d864a46394b41605b73.svg" class="img_KBPg" decoding="async" loading="lazy" alt="chat" />](https://opendal.apache.org/discord)

`parquet_opendal` provides [`parquet`](https://crates.io/crates/parquet) efficient IO utilities.

## Useful Links<a href="https://opendal.apache.org/integrations/parquet/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- Documentation: [release](https://docs.rs/parquet_opendal/) \| [dev](https://opendal.apache.org/docs/object-store-opendal/parquet_opendal/)

## Examples<a href="https://opendal.apache.org/integrations/parquet/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

Add the following dependencies to your `Cargo.toml` with correct version:

``` prism-code
[dependencies]
parquet_opendal = "0.0.1"
opendal = { version = "0.48.0", features = ["services-s3"] }
```

``` prism-code
use std::sync::Arc;

use arrow::array::{ArrayRef, Int64Array, RecordBatch};

use futures::StreamExt;
use opendal::{services::S3Config, Operator};
use parquet::arrow::{AsyncArrowWriter, ParquetRecordBatchStreamBuilder};
use parquet_opendal::{AsyncReader, AsyncWriter};

#[tokio::main]
async fn main() {
    let mut cfg = S3Config::default();
    cfg.access_key_id = Some("my_access_key".to_string());
    cfg.secret_access_key = Some("my_secret_key".to_string());
    cfg.endpoint = Some("my_endpoint".to_string());
    cfg.region = Some("my_region".to_string());
    cfg.bucket = "my_bucket".to_string();

    // Create a new operator
    let operator = Operator::from_config(cfg).unwrap().finish();
    let path = "/path/to/file.parquet";

    // Create an async writer
    let writer = AsyncWriter::new(
        operator
            .writer_with(path)
            .chunk(32 * 1024 * 1024)
            .concurrent(8)
            .await
            .unwrap(),
    );

    let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
    let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();
    let mut writer = AsyncArrowWriter::try_new(writer, to_write.schema(), None).unwrap();
    writer.write(&to_write).await.unwrap();
    writer.close().await.unwrap();

    /// `gap(512 * 1024)` - Sets the maximum gap size (in bytes) to merge small byte ranges
    ///   to 512 KB.
    /// `chunk(16 * 1024 * 1024)` - Sets the chunk size (in bytes) for reading data to 16 MB.
    /// `concurrent(16)` - Sets the number of concurrent fetch operations to 16.
    let reader = operator
        .reader_with(path)
        .gap(512 * 1024)
        .chunk(16 * 1024 * 1024)
        .concurrent(16)
        .await
        .unwrap();

    let content_len = operator.stat(path).await.unwrap().content_length();
    // `with_prefetch_footer_size(512 * 1024)` - Sets the prefetch footer size to 512 KB.
    let reader = AsyncReader::new(reader, content_len).with_prefetch_footer_size(512 * 1024);
    let mut stream = ParquetRecordBatchStreamBuilder::new(reader)
        .await
        .unwrap()
        .build()
        .unwrap();
    let read = stream.next().await.unwrap().unwrap();
    assert_eq!(to_write, read);
}
```

## Branding<a href="https://opendal.apache.org/integrations/parquet/#branding" class="hash-link" aria-label="Direct link to Branding" translate="no" title="Direct link to Branding">â€‹</a>

The first and most prominent mentions must use the full form: **Apache OpenDALâ„¢** of the name for any individual usage (webpage, handout, slides, etc.) Depending on the context and writing style, you should use the full form of the name sufficiently often to ensure that readers clearly understand the association of both the OpenDAL project and the OpenDAL software product to the ASF as the parent organization.

For more details, see the [Apache Product Name Usage Guide](https://www.apache.org/foundation/marks/guide).

## License and Trademarks<a href="https://opendal.apache.org/integrations/parquet/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/30-integrations/parquet.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/integrations/parquet/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/integrations/object_store/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Object Store

<a href="https://opendal.apache.org/integrations/spring/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Spring

- <a href="https://opendal.apache.org/integrations/parquet/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/integrations/parquet/#examples" class="table-of-contents__link toc-highlight">Examples</a>
- <a href="https://opendal.apache.org/integrations/parquet/#branding" class="table-of-contents__link toc-highlight">Branding</a>
- <a href="https://opendal.apache.org/integrations/parquet/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
