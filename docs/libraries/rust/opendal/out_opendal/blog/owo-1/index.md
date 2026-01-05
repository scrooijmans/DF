# OwO \#1: The v0.40 Release

September 15, 2023 Â· 5 min read

<a href="https://github.com/Xuanwo" rel="noopener noreferrer" target="_blank">Xuanwo</a>

> OwO (Outcome, Working, Outlook) is an Apache OpenDALâ„¢ release blog series, where we share the current work status and future plans.

Hello! It's been a while since our last update. We've been hard at work determining the optimal way to implement new features and improvements. We're thrilled to announce that we'll soon be releasing v0.40.

This post is structured into three main sections:

- Outcome (1st `O` in `OwO`): Summarizes the key accomplishments in the v0.40 release.
- Working (the `w` in `OwO`): Provides an update on our current work.
- Outlook (2nd `O` in `OwO`): Discusses what lies ahead for OpenDAL.

## Outcome<a href="https://opendal.apache.org/blog/owo-1/#outcome" class="hash-link" aria-label="Direct link to Outcome" translate="no" title="Direct link to Outcome">â€‹</a>

OpenDAL now comprises four primary components:

- Core: The core library written in Rust.
- Bindings: Language bindings powered by the OpenDAL Rust core.
- Applications: Applications built using the OpenDAL Rust core.
- Integrations: Collaborations with other projects.

### Core<a href="https://opendal.apache.org/blog/owo-1/#core" class="hash-link" aria-label="Direct link to Core" translate="no" title="Direct link to Core">â€‹</a>

#### Unifying Append and Write Functions<a href="https://opendal.apache.org/blog/owo-1/#unifying-append-and-write-functions" class="hash-link" aria-label="Direct link to Unifying Append and Write Functions" translate="no" title="Direct link to Unifying Append and Write Functions">â€‹</a>

OpenDAL has supported `append` operations since `v0.36`. We've found, however, that this led to significant duplication between append and write. As a result, we've streamlined the two functionalities into a single write function. Our users can now:

``` prism-code
let mut w = op.writer_with("test.txt").append(true).await?;
w.write(content_a).await?;
w.write(content_b).await?;
w.close().await?;
```

This way, users can reuse the `Writer` in their own logic without handling `append` separately.

#### New Lister API<a href="https://opendal.apache.org/blog/owo-1/#new-lister-api" class="hash-link" aria-label="Direct link to New Lister API" translate="no" title="Direct link to New Lister API">â€‹</a>

To improve API consistency, we've made some adjustments to our listing functions. We've added `list` and `list_with` methods that perform single operations and renamed the original `list` to `lister` and `lister_with`.

``` prism-code
// Old API
let lister: Lister = op.list("dir").await?;

// New API
let entries: Vec<Entry> = op.list("dir").await?;
let lister: Lister = op.lister("dir").await?;
```

This brings uniformity to our API offerings.

#### List With Metakey<a href="https://opendal.apache.org/blog/owo-1/#list-with-metakey" class="hash-link" aria-label="Direct link to List With Metakey" translate="no" title="Direct link to List With Metakey">â€‹</a>

To speed up list operations, OpenDAL can now fetch and store metadata during the listing process. This eliminates the need for separate metadata calls:

``` prism-code
let entries: Vec<Entry> = op
  .list_with("dir/")
  .metakey(Metakey::ContentLength | Metakey::ContentType).await?;

// Use the metadata directly!
let meta: &Metadata = entries[0].metadata();
```

This makes metadata retrieval more intuitive.

#### Buffered Writer<a href="https://opendal.apache.org/blog/owo-1/#buffered-writer" class="hash-link" aria-label="Direct link to Buffered Writer" translate="no" title="Direct link to Buffered Writer">â€‹</a>

We've added general buffer support to optimize writing operations.

``` prism-code
let w = op.writer_with("path/to/file").buffer(8 * 1024 * 1024).await?
```

#### Others<a href="https://opendal.apache.org/blog/owo-1/#others" class="hash-link" aria-label="Direct link to Others" translate="no" title="Direct link to Others">â€‹</a>

Other improvements in the core library can be found in our <a href="https://github.com/apache/opendal/blob/main/CHANGELOG.md" rel="noopener noreferrer" target="_blank">CHANGELOG</a>.

### Bindings<a href="https://opendal.apache.org/blog/owo-1/#bindings" class="hash-link" aria-label="Direct link to Bindings" translate="no" title="Direct link to Bindings">â€‹</a>

#### C++<a href="https://opendal.apache.org/blog/owo-1/#c" class="hash-link" aria-label="Direct link to C++" translate="no" title="Direct link to C++">â€‹</a>

<a href="https://github.com/apache/opendal/tree/main/bindings/cpp" rel="noopener noreferrer" target="_blank"><code>opendal-cpp</code></a> is ready for its first release! Welcome to check it out and give us some feedback.

#### Haskell<a href="https://opendal.apache.org/blog/owo-1/#haskell" class="hash-link" aria-label="Direct link to Haskell" translate="no" title="Direct link to Haskell">â€‹</a>

<a href="https://github.com/apache/opendal/tree/main/bindings/haskell" rel="noopener noreferrer" target="_blank"><code>opendal-hs</code></a> is ready for its first release! Welcome to check it out and give us some feedback.

#### Java<a href="https://opendal.apache.org/blog/owo-1/#java" class="hash-link" aria-label="Direct link to Java" translate="no" title="Direct link to Java">â€‹</a>

<a href="https://github.com/apache/opendal/tree/main/bindings/java" rel="noopener noreferrer" target="_blank"><code>opendal-java</code></a> enabled more available services in this release, allowing user to visit services like `redis` that not enabled by default in rust core. And `opendal-java` enabled blocking layer to allow users visit services like `s3` in blocking way.

Welcome to integrate `opendal-java` into your project and give us some feedback.

#### New bindings\!<a href="https://opendal.apache.org/blog/owo-1/#new-bindings" class="hash-link" aria-label="Direct link to New bindings!" translate="no" title="Direct link to New bindings!">â€‹</a>

- <a href="https://github.com/apache/opendal/tree/main/bindings/dotnet" rel="noopener noreferrer" target="_blank"><code>opendal-dotnet</code></a>
- <a href="https://github.com/apache/opendal/tree/main/bindings/php" rel="noopener noreferrer" target="_blank"><code>opendal-php</code></a>

### Applications<a href="https://opendal.apache.org/blog/owo-1/#applications" class="hash-link" aria-label="Direct link to Applications" translate="no" title="Direct link to Applications">â€‹</a>

#### oay<a href="https://opendal.apache.org/blog/owo-1/#oay" class="hash-link" aria-label="Direct link to oay" translate="no" title="Direct link to oay">â€‹</a>

<a href="https://github.com/apache/opendal/tree/main/bin/oay" rel="noopener noreferrer" target="_blank">oay</a> is OpenDAL Gateway that allows users to access OpenDAL services via existing protocols like `s3` and `webdav`. It works like a proxy that forwarding requests to OpenDAL services.

In this release, we implement basic `webdav` support. Users can convert any storage services to a webdav server!

#### oli<a href="https://opendal.apache.org/blog/owo-1/#oli" class="hash-link" aria-label="Direct link to oli" translate="no" title="Direct link to oli">â€‹</a>

<a href="https://github.com/apache/opendal/tree/main/bin/oay" rel="noopener noreferrer" target="_blank">oli</a> is OpenDAL CLI that allows users to access storage services via CLI like `s3cmd` and `gcloud` does.

We fixed some experience issues in this release and improved some docs. Welcome to try it out and give us some feedback.

### Integrations<a href="https://opendal.apache.org/blog/owo-1/#integrations" class="hash-link" aria-label="Direct link to Integrations" translate="no" title="Direct link to Integrations">â€‹</a>

#### object_store<a href="https://opendal.apache.org/blog/owo-1/#object_store" class="hash-link" aria-label="Direct link to object_store" translate="no" title="Direct link to object_store">â€‹</a>

<a href="https://github.com/apache/opendal/tree/main/integrations/object_store" rel="noopener noreferrer" target="_blank">object_store</a> instead to implement <a href="https://github.com/apache/arrow-rs/tree/master/object_store" rel="noopener noreferrer" target="_blank"><code>object_store</code></a>'s trait over OpenDAL Operator so that users can use OpenDAL as a backend for `object_store`.

`object_store` is mostly functional, but there are some edge use cases that OpenDAL has yet to support.

So far, this release hasn't seen progress in this area; we are awaiting the resolution of the issue <a href="https://github.com/apache/opendal/issues/2762" rel="noopener noreferrer" target="_blank">Allow list paths that do not end with <code>/</code></a>.

## Working<a href="https://opendal.apache.org/blog/owo-1/#working" class="hash-link" aria-label="Direct link to Working" translate="no" title="Direct link to Working">â€‹</a>

We are working on the following things:

- `object_store` support: Make `object_store` integration works and find a user for it.
- Remove the `/` limitation for path, so we can list a path without ending with `/`.
- Expand the `start-after` support to more services (Address <a href="https://github.com/apache/opendal/issues/2786" rel="noopener noreferrer" target="_blank">#2786</a>).

## Outlook<a href="https://opendal.apache.org/blog/owo-1/#outlook" class="hash-link" aria-label="Direct link to Outlook" translate="no" title="Direct link to Outlook">â€‹</a>

We are exploring some innovative ideas:

- <a href="https://github.com/apache/opendal/discussions/2951" rel="noopener noreferrer" target="_blank">OpenDAL REST/gRPC API</a>: A REST/gRPC Server for OpenDAL.
- <a href="https://github.com/apache/opendal/discussions/2953" rel="noopener noreferrer" target="_blank">OpenDAL Cache</a>: OpenDAL native cache libs that allowing users to access data more efficiently.
- <a href="https://github.com/apache/opendal/discussions/2952" rel="noopener noreferrer" target="_blank">OpenDAL File System</a>: A read-only file system that built upon OpenDAL in rust!
- <a href="https://github.com/apache/opendal/discussions/3042" rel="noopener noreferrer" target="_blank">kio-opendal</a>: A kio plugin powered by OpenDAL that allows users to visit different storage services in <a href="https://apps.kde.org/dolphin/" rel="noopener noreferrer" target="_blank">KDE Dolphin</a>.
- gvfs-opendal: A gvfs plugin powered by OpenDAL that allows users to visit different storage services in <a href="https://wiki.gnome.org/Apps/Files" rel="noopener noreferrer" target="_blank">GNOME Files</a>

Feel free to join in the discussion!

## Summary<a href="https://opendal.apache.org/blog/owo-1/#summary" class="hash-link" aria-label="Direct link to Summary" translate="no" title="Direct link to Summary">â€‹</a>

This marks our first OpenDAL `OwO` post. We welcome your feedback.

**Tags:**

- <a href="https://opendal.apache.org/blog/tags/owo/" class="tag_f4_J tagRegular_MJrJ" rel="tag">owo</a>

<a href="https://github.com/apache/opendal/tree/main/website/blog/2023-09-14-owo-1/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/blog/owo-1/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/blog/apache-opendal-graduated/" class="pagination-nav__link pagination-nav__link--prev"></a>

Newer post

Apache OpenDALâ„¢ is now Graduated

<a href="https://opendal.apache.org/blog/how-opendal-read-data/" class="pagination-nav__link pagination-nav__link--next"></a>

Older post

Apache OpenDALâ„¢ Internal: Data Reading
