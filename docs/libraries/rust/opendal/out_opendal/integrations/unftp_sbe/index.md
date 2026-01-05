- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/integrations/unftp_sbe/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/integrations/" class="breadcrumbs__link">Integrations</a>
- Unftp Sbe

On this page

# Unftp Sbe

## Apache OpenDALâ„¢ unftp Integration

[<img src="out_opendal/integrations/unftp_sbe/index_media/49073adb969e94cba3d1c470073bbb2ae9fb8754.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Build Status" />](https://github.com/apache/opendal/actions?query=branch%3Amain) [<img src="out_opendal/integrations/unftp_sbe/index_media/a37ecc3081bb9465bf0ec780204f4571cab03f81.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Latest Version" />](https://crates.io/crates/unftp-sbe-opendal) [<img src="out_opendal/integrations/unftp_sbe/index_media/97cb005d754205cb345f70b72b8270290ff793d1.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate Downloads" />](https://crates.io/crates/unftp-sbe-opendal) [<img src="out_opendal/integrations/unftp_sbe/index_media/8344f1bc1d55e7813afa8d864a46394b41605b73.svg" class="img_KBPg" decoding="async" loading="lazy" alt="chat" />](https://opendal.apache.org/discord)

`unftp-sbe-opendal` is an [unftp](https://crates.io/crates/unftp) `StorageBackend` implementation using opendal.

This crate can help you to access ANY storage services with the same FTP API.

## Useful Links<a href="https://opendal.apache.org/integrations/unftp_sbe/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- Documentation: [release](https://docs.rs/unftp-sbe-opendal/) \| [dev](https://opendal.apache.org/docs/unftp-sbe-opendal/unftp_sbe_opendal/)

## Examples<a href="https://opendal.apache.org/integrations/unftp_sbe/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

``` prism-code
use anyhow::Result;
use opendal::Operator;
use opendal::Scheme;
use opendal::services;
use unftp_sbe_opendal::OpendalStorage;

#[tokio::main]
async fn main() -> Result<()> {
    // Create any service desired
    let op = opendal::Operator::from_map::<services::S3>(
        [
            ("bucket".to_string(), "my_bucket".to_string()),
            ("access_key".to_string(), "my_access_key".to_string()),
            ("secret_key".to_string(), "my_secret_key".to_string()),
            ("endpoint".to_string(), "my_endpoint".to_string()),
            ("region".to_string(), "my_region".to_string()),
        ]
            .into_iter()
            .collect(),
    )?.finish();

    // Wrap the operator with `OpendalStorage`
    let backend = OpendalStorage::new(op);

    // Build the actual unftp server
    let server = libunftp::ServerBuilder::new(Box::new(move || backend.clone())).build()?;

    // Start the server
    server.listen("0.0.0.0:0").await?;

    Ok(())
}
```

## Branding<a href="https://opendal.apache.org/integrations/unftp_sbe/#branding" class="hash-link" aria-label="Direct link to Branding" translate="no" title="Direct link to Branding">â€‹</a>

The first and most prominent mentions must use the full form: **Apache OpenDALâ„¢** of the name for any individual usage (webpage, handout, slides, etc.) Depending on the context and writing style, you should use the full form of the name sufficiently often to ensure that readers clearly understand the association of both the OpenDAL project and the OpenDAL software product to the ASF as the parent organization.

For more details, see the [Apache Product Name Usage Guide](https://www.apache.org/foundation/marks/guide).

## License and Trademarks<a href="https://opendal.apache.org/integrations/unftp_sbe/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/30-integrations/unftp_sbe.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/integrations/unftp_sbe/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/integrations/spring/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Spring

<a href="https://opendal.apache.org/category/applications/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Applications

- <a href="https://opendal.apache.org/integrations/unftp_sbe/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/integrations/unftp_sbe/#examples" class="table-of-contents__link toc-highlight">Examples</a>
- <a href="https://opendal.apache.org/integrations/unftp_sbe/#branding" class="table-of-contents__link toc-highlight">Branding</a>
- <a href="https://opendal.apache.org/integrations/unftp_sbe/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
