- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/integrations/dav_server/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/integrations/" class="breadcrumbs__link">Integrations</a>
- Dav Server

On this page

# Dav Server

## Apache OpenDALâ„¢ dav-server integration

[<img src="out_opendal/integrations/dav_server/index_media/49073adb969e94cba3d1c470073bbb2ae9fb8754.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Build Status" />](https://github.com/apache/opendal/actions?query=branch%3Amain) [<img src="out_opendal/integrations/dav_server/index_media/4c16779f2f24e8738d68d8c32f0c01dfd87df1e9.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Latest Version" />](https://crates.io/crates/dav-server-opendalfs) [<img src="out_opendal/integrations/dav_server/index_media/97d1e0c817e54ce2cbe4035b78921ee1670c16a6.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Crate Downloads" />](https://crates.io/crates/dav-server-opendalfs) [<img src="out_opendal/integrations/dav_server/index_media/8344f1bc1d55e7813afa8d864a46394b41605b73.svg" class="img_KBPg" decoding="async" loading="lazy" alt="chat" />](https://opendal.apache.org/discord)

`dav-server-opendalfs` is an [`dav-server`](https://github.com/messense/dav-server-rs) implementation using opendal.

This crate can help you to access ANY storage services with the same webdav API.

## Useful Links<a href="https://opendal.apache.org/integrations/dav_server/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- Documentation: [release](https://docs.rs/dav-server-opendalfs/) \| [dev](https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/)

## Examples<a href="https://opendal.apache.org/integrations/dav_server/#examples" class="hash-link" aria-label="Direct link to Examples" translate="no" title="Direct link to Examples">â€‹</a>

``` prism-code
use anyhow::Result;
use dav_server::davpath::DavPath;
use dav_server_opendalfs::OpendalFs;
use opendal::services::Memory;
use opendal::Operator;

#[tokio::test]
async fn test() -> Result<()> {
 let op = Operator::new(Memory::default())?.finish();

 let webdavfs = OpendalFs::new(op);

 let metadata = webdavfs
     .metadata(&DavPath::new("/").unwrap())
     .await
     .unwrap();
 println!("{}", metadata.is_dir());

 Ok(())
}
```

## Branding<a href="https://opendal.apache.org/integrations/dav_server/#branding" class="hash-link" aria-label="Direct link to Branding" translate="no" title="Direct link to Branding">â€‹</a>

The first and most prominent mentions must use the full form: **Apache OpenDALâ„¢** of the name for any individual usage (webpage, handout, slides, etc.) Depending on the context and writing style, you should use the full form of the name sufficiently often to ensure that readers clearly understand the association of both the OpenDAL project and the OpenDAL software product to the ASF as the parent organization.

For more details, see the [Apache Product Name Usage Guide](https://www.apache.org/foundation/marks/guide).

## License and Trademarks<a href="https://opendal.apache.org/integrations/dav_server/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/30-integrations/dav_server.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/integrations/dav_server/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/integrations/cloud_filter/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Cloud Filter (removed)

<a href="https://opendal.apache.org/integrations/fuse3/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Fuse3 (removed)

- <a href="https://opendal.apache.org/integrations/dav_server/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/integrations/dav_server/#examples" class="table-of-contents__link toc-highlight">Examples</a>
- <a href="https://opendal.apache.org/integrations/dav_server/#branding" class="table-of-contents__link toc-highlight">Branding</a>
- <a href="https://opendal.apache.org/integrations/dav_server/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
