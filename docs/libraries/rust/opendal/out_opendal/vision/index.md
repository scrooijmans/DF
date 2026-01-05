- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/vision/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- Vision

On this page

# Vision

## Charter<a href="https://opendal.apache.org/vision#charter" class="hash-link" aria-label="Direct link to Charter" translate="no" title="Direct link to Charter">â€‹</a>

**One Layer, All Storage.**

## Principles<a href="https://opendal.apache.org/vision#principles" class="hash-link" aria-label="Direct link to Principles" translate="no" title="Direct link to Principles">â€‹</a>

Principles are guiding principles. They guide how decisions are made for the whole project. Ideally, we do all of them all the time. In some cases, though, we may be forced to decide between slightly penalizing one goal or another. In that case, we tend to support those goals that come earlier in the list over those that come later (but every case is different).

### 0. Open Community<a href="https://opendal.apache.org/vision#0-open-community" class="hash-link" aria-label="Direct link to 0. Open Community" translate="no" title="Direct link to 0. Open Community">â€‹</a>

OpenDAL SHOULD be an **open** storage library.

OpenDAL is an ASF project governed by the OpenDAL PMC. At ASF, we believe in "Community Over Code" and adhere to <a href="https://www.apache.org/theapacheway/" rel="noopener noreferrer" target="_blank">the Apache Way</a>. We aim to develop OpenDAL to meet the needs of our community. We do not maintain private versions or include features that aren't useful to others.

For example, OpenDAL prefers to have clear and readable code, as this allows more people in the community to join the development.

### 1. Solid Foundation<a href="https://opendal.apache.org/vision#1-solid-foundation" class="hash-link" aria-label="Direct link to 1. Solid Foundation" translate="no" title="Direct link to 1. Solid Foundation">â€‹</a>

OpenDAL SHOULD be a **solid** storage library.

OpenDAL is a solid foundation of user projects that users can trust OpenDAL to perform operations on real-world storage services. OpenDAL SHOULD always focus on building a Solid Foundation.

For example, OpenDAL performs additional error checks for AWS S3 complete multipart operations, as S3 may return an error in response with a 200 status code, even though this may add extra costs that conflict with "Fast Access.â€?

### 2. Fast Access<a href="https://opendal.apache.org/vision#2-fast-access" class="hash-link" aria-label="Direct link to 2. Fast Access" translate="no" title="Direct link to 2. Fast Access">â€‹</a>

OpenDAL SHOULD be a **fast** storage library.

Its fast access ensures that OpenDAL implements storage support with zero overhead. Users can integrate with OpenDAL without concerns about additional costs. OpenDAL should be as fast as, or faster than, the SDK for any given storage service.

For example, OpenDAL uses Capability to describe the capabilities of different services and adopts native features of those services whenever possible.

### 3. Object Storage First<a href="https://opendal.apache.org/vision#3-object-storage-first" class="hash-link" aria-label="Direct link to 3. Object Storage First" translate="no" title="Direct link to 3. Object Storage First">â€‹</a>

OpenDAL SHOULD be an **object storage first** library.

OpenDAL supports many storage services, but it is usually optimized for modern storage services. At the time of writing, we can say OpenDAL is object storage first. When designing features, OpenDAL tends to prioritize optimization for object storage.

For example, OpenDAL's Buffer design is primarily optimized for HTTP-based services, helping to reduce extra allocation, in-memory copying, and memory usage.

### 4. Extensible Architecture<a href="https://opendal.apache.org/vision#4-extensible-architecture" class="hash-link" aria-label="Direct link to 4. Extensible Architecture" translate="no" title="Direct link to 4. Extensible Architecture">â€‹</a>

OpenDAL SHOULD be an extensible storage library.

OpenDAL can be extended to various languages, backends, and layers. Each is independent and does not depend on the others. Users can combine different layers, such as metrics, logging, tracing, and retry, and extend their own languages, backends, and layers.

For example, OpenDAL's core never relies on the behavior or dependency of a single layer. Users can stack as many layers as they want on a given operator.

## Use Cases<a href="https://opendal.apache.org/vision#use-cases" class="hash-link" aria-label="Direct link to Use Cases" translate="no" title="Direct link to Use Cases">â€‹</a>

Who are typical OpenDAL *users*? How would they use OpenDAL?

### Infrastructure Builders<a href="https://opendal.apache.org/vision#infrastructure-builders" class="hash-link" aria-label="Direct link to Infrastructure Builders" translate="no" title="Direct link to Infrastructure Builders">â€‹</a>

Examples:

- <a href="https://github.com/databendlabs/databend" rel="noopener noreferrer" target="_blank">Databend</a>
- <a href="https://github.com/risingwavelabs/risingwave" rel="noopener noreferrer" target="_blank">RisingWave</a>
- <a href="https://github.com/GreptimeTeam/greptimedb" rel="noopener noreferrer" target="_blank">GreptimeDB</a>
- <a href="https://github.com/apache/iceberg-rust" rel="noopener noreferrer" target="_blank">Apache Iceberg Rust</a>

Use Cases:

- Building storage systems like databases
- Developing data processing pipelines
- Creating backup and archive solutions

Primary Concerns:

- **Solid Foundation**: Need guaranteed consistency and predictability for storage operations
- **Fast Access**: Require minimal overhead and optimal performance
- *Why*: Infrastructure services demand both reliability and performance as foundational requirements

### Application Developers<a href="https://opendal.apache.org/vision#application-developers" class="hash-link" aria-label="Direct link to Application Developers" translate="no" title="Direct link to Application Developers">â€‹</a>

Examples:

- <a href="https://github.com/mozilla/sccache" rel="noopener noreferrer" target="_blank">Sccache</a>
- <a href="https://github.com/vectordotdev/vector" rel="noopener noreferrer" target="_blank">Vector</a>
- <a href="https://github.com/rustic-rs/rustic" rel="noopener noreferrer" target="_blank">Rustic</a>

Use Cases:

- Building end-user applications
- Developing CLI tools
- Creating web services

Primary Concerns:

- **Fast Access**: Need efficient integration and optimal performance
- **Object Storage First**: Benefit from optimizations for modern cloud storage
- *Why*: Modern applications commonly use object storage and require responsive performance

### Platform Developers<a href="https://opendal.apache.org/vision#platform-developers" class="hash-link" aria-label="Direct link to Platform Developers" translate="no" title="Direct link to Platform Developers">â€‹</a>

Examples:

- <a href="https://github.com/pantsbuild/pants" rel="noopener noreferrer" target="_blank">Pants</a>
- <a href="https://github.com/zino-rs/zino" rel="noopener noreferrer" target="_blank">Zino</a>
- <a href="https://github.com/shuttle-hq/shuttle" rel="noopener noreferrer" target="_blank">Shuttle</a>

Use Cases:

- Building AI/ML platforms
- Developing cloud services
- Creating developer tools

Primary Concerns:

- **Extensible Architecture**: Need to customize and extend storage capabilities
- **Solid Foundation**: Require dependable storage operations
- *Why*: Platforms need flexibility to adapt to various use cases while maintaining reliability

------------------------------------------------------------------------

*This documentation is inspired a lot by <a href="https://hyper.rs/contrib/vision/" rel="noopener noreferrer" target="_blank">hyperâ€™s VISION document</a>.*

<a href="https://github.com/apache/opendal/tree/main/website/docs/02-vision.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/vision/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Oct 7, 2025** by **Patrick Carlson**

<a href="https://opendal.apache.org/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Overview

<a href="https://opendal.apache.org/core/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Core

- <a href="https://opendal.apache.org/vision#charter" class="table-of-contents__link toc-highlight">Charter</a>
- <a href="https://opendal.apache.org/vision#principles" class="table-of-contents__link toc-highlight">Principles</a>
  - <a href="https://opendal.apache.org/vision#0-open-community" class="table-of-contents__link toc-highlight">0. Open Community</a>
  - <a href="https://opendal.apache.org/vision#1-solid-foundation" class="table-of-contents__link toc-highlight">1. Solid Foundation</a>
  - <a href="https://opendal.apache.org/vision#2-fast-access" class="table-of-contents__link toc-highlight">2. Fast Access</a>
  - <a href="https://opendal.apache.org/vision#3-object-storage-first" class="table-of-contents__link toc-highlight">3. Object Storage First</a>
  - <a href="https://opendal.apache.org/vision#4-extensible-architecture" class="table-of-contents__link toc-highlight">4. Extensible Architecture</a>
- <a href="https://opendal.apache.org/vision#use-cases" class="table-of-contents__link toc-highlight">Use Cases</a>
  - <a href="https://opendal.apache.org/vision#infrastructure-builders" class="table-of-contents__link toc-highlight">Infrastructure Builders</a>
  - <a href="https://opendal.apache.org/vision#application-developers" class="table-of-contents__link toc-highlight">Application Developers</a>
  - <a href="https://opendal.apache.org/vision#platform-developers" class="table-of-contents__link toc-highlight">Platform Developers</a>
