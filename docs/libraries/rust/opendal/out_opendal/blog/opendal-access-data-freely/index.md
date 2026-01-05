# Apache OpenDALâ„¢: Access Data Freely

July 7, 2023 Â· 5 min read

<a href="https://github.com/PsiACE" rel="noopener noreferrer" target="_blank">PsiACE</a>

<a href="https://github.com/Xuanwo" rel="noopener noreferrer" target="_blank">Xuanwo</a>

If you're committed to building cloud-native, cross-cloud-first applications and services, or you want to support configurable storage backends to meet complex data access needs, or if you're tired of juggling various SDKs and hoping for a unified abstraction and development experience, Apache OpenDALâ„¢ will be your perfect partner.

<img src="out_opendal/blog/opendal-access-data-freely/index_media/841a60e6d4bc3274b1106e1a26e6b71a849bb2e2.png" class="img_KBPg" decoding="async" loading="lazy" width="3932" height="2662" alt="OpenDAL Arch" />

## What is OpenDAL?<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#what-is-opendal" class="hash-link" aria-label="Direct link to What is OpenDAL?" translate="no" title="Direct link to What is OpenDAL?">â€‹</a>

**OpenDAL** is a data access layer that allows users to easily and efficiently retrieve data from various storage services in a unified way.

**Data Access Layer** means: OpenDAL is in a critical position in the data read-write process. We shield the implementation details of different storage backends and provide a set of unified interface abstractions externally.

Next, let's try to answer *"What OpenDAL is not"* and deconstruct OpenDAL from another perspective:

### Opendal Is Not a Proxy Service<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#opendal-is-not-a-proxy-service" class="hash-link" aria-label="Direct link to Opendal Is Not a Proxy Service" translate="no" title="Direct link to Opendal Is Not a Proxy Service">â€‹</a>

OpenDAL is provided in the form of a library, not as a service or application that proxies various storage backends.

If you want to integrate OpenDAL into an existing project, you need to call OpenDAL's interface directly through the bindings supported by OpenDAL to access the storage services.

### Opendal Is Not an SDK Aggregator<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#opendal-is-not-an-sdk-aggregator" class="hash-link" aria-label="Direct link to Opendal Is Not an SDK Aggregator" translate="no" title="Direct link to Opendal Is Not an SDK Aggregator">â€‹</a>

Although OpenDAL replaces various SDKs in the application architecture, it is not implemented as an SDK aggregator.

In other words, OpenDAL does not simply call various storage service SDKs. We have developed our own docking with various storage services based on a unified Rust core to ensure that the differences between services are smoothed out.

For example, for S3, OpenDAL manually constructs HTTP requests and parses HTTP responses to ensure that all behaviors comply with API specifications and are fully under the control of OpenDAL. Due to OpenDAL's native takeover of the data access process, we can easily implement unified retry and logging mechanisms for various storage backends and ensure behavioral consistency.

For compatible services with S3, due to the limitations of native storage services and differences in API implementation, compatibility and behavioral details may differ from S3. For example, OSS needs to set an independent header to ensure consistent behavior for `Range`. In addition to docking with native storage services, OpenDAL will also perform targeted processing for compatible services to ensure users' data access experience.

## Advantages of OpenDAL<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#advantages-of-opendal" class="hash-link" aria-label="Direct link to Advantages of OpenDAL" translate="no" title="Direct link to Advantages of OpenDAL">â€‹</a>

OpenDAL is not the only project dedicated to providing data access abstraction, but compared to other similar projects, OpenDAL has the following advantages:

### Rich Service Support<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#rich-service-support" class="hash-link" aria-label="Direct link to Rich Service Support" translate="no" title="Direct link to Rich Service Support">â€‹</a>

OpenDAL supports dozens of storage services, covering a wide range of scenarios and supporting on-demand selection:

- Standard Storage Protocols: FTP, HTTP, SFTP, WebDAV, etc.
- Object Storage Services: azblob, gcs, obs, oss, s3, etc.
- File Storage Services: fs, azdls, hdfs, webhdfs, ipfs, etc.
- Consumer Cloud Storage Service: Google Drive, OneDrive, Dropbox, etc.
- Key-Value Storage Service: Memory, Redis, Rocksdb, etc.
- Cache Storage Service: Ghac, Memcached, etc.

### Complete Cross-Language Bindings<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#complete-cross-language-bindings" class="hash-link" aria-label="Direct link to Complete Cross-Language Bindings" translate="no" title="Direct link to Complete Cross-Language Bindings">â€‹</a>

With Rust as the core, OpenDAL now provides binding support for multiple languages such as Python/Node.js/Java/C and is also actively developing bindings for other languages.

Cross-language bindings not only provide unified storage access abstractions for other languages but also follow naming conventions and development habits that are common in various languages as much as possible to pave the way for quick use.

### Powerful Middleware Support<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#powerful-middleware-support" class="hash-link" aria-label="Direct link to Powerful Middleware Support" translate="no" title="Direct link to Powerful Middleware Support">â€‹</a>

OpenDAL offers native layer support, enabling users to implement middleware or intercept for all operations.

- Error Retry: OpenDAL supports fine-grained error retry capabilities. In addition to common request retries, it supports breakpoint resumable transmission without having to re-read the entire file.
- Observability: OpenDAL implements logging,tracing,and metrics support for all operations. Turning on middleware can directly obtain observability capabilities for storage.
- Concurrency control, flow control, fuzz testing, and more.

### Easy to Use<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#easy-to-use" class="hash-link" aria-label="Direct link to Easy to Use" translate="no" title="Direct link to Easy to Use">â€‹</a>

OpenDAL's API has been well designed and polished in actual use. The documentation covers everything and is easy to get started with. Here's an example of using Python bindings to access HDFS:

``` prism-code
import opendal
    
op = opendal.Operator("hdfs", name_node="hdfs://192.16.8.10.103")
op.read("path/to/file")
```

### Use Cases of OpenDAL<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#use-cases-of-opendal" class="hash-link" aria-label="Direct link to Use Cases of OpenDAL" translate="no" title="Direct link to Use Cases of OpenDAL">â€‹</a>

Currently, OpenDAL is widely used in various scenarios that require cloud-native capabilities, including but not limited to databases, data pipelines, and caches. The main user cases include:

- <a href="https://github.com/datafuselabs/databend/" rel="noopener noreferrer" target="_blank">Databend</a>: A modern Elasticity and Performance cloud data warehouse. Using OpenDAL to read and write persistent data (s3, azblob, gcs, hdfs, etc.) and cache data (fs, redis, rocksdb, moka, etc.).
- <a href="https://github.com/GreptimeTeam/greptimedb" rel="noopener noreferrer" target="_blank">GreptimeDB</a>: An open-source, cloud-native, distributed time-series database. Using OpenDAL to read and write persistent data (s3, azblob, etc.).
- <a href="https://github.com/mozilla/sccache/" rel="noopener noreferrer" target="_blank">mozilla/sccache</a>: `sccache` is <a href="https://github.com/ccache/ccache" rel="noopener noreferrer" target="_blank"><code>ccache</code></a> with cloud storage. Using OpenDAL to read and write cache data (s3 and ghac, etc.).
- <a href="https://github.com/risingwavelabs/risingwave" rel="noopener noreferrer" target="_blank">RisingWave</a>: A Distributed SQL Database for Stream Processing. Using OpenDAL to read and write persistent data (s3, azblob, hdfs, etc.).
- <a href="https://github.com/vectordotdev/vector" rel="noopener noreferrer" target="_blank">Vector</a>: A high-performance observability data pipeline. Using OpenDAL to write persistent data (currently mainly using hdfs).

## Future Plans of OpenDAL<a href="https://opendal.apache.org/blog/opendal-access-data-freely/#future-plans-of-opendal" class="hash-link" aria-label="Direct link to Future Plans of OpenDAL" translate="no" title="Direct link to Future Plans of OpenDAL">â€‹</a>

In addition to further meeting the needs of cloud-native data access, OpenDAL will continue to expand user scenarios and actively explore its use in data science and mobile applications. At the same time, OpenDAL will continue to polish its existing implementations and bindings to provide users with a better integration experience.

OpenDAL will also explore how to improve users' workflows in data management and service integration:

- Polish the `oli` command-line tool to help users manage data painlessly.
- Implement the `oay` proxy service to provide users with high-quality compatible APIs.

In addition, since OpenDAL is currently a cross-language project, we also plan to write a series of introductory tutorials to help everyone learn OpenDAL from scratch while learning the language.

**Tags:**

- <a href="https://opendal.apache.org/blog/tags/announcement/" class="tag_f4_J tagRegular_MJrJ" rel="tag">announcement</a>

<a href="https://github.com/apache/opendal/tree/main/website/blog/2023-07-07-apache-opendal-access-data-freely/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/blog/opendal-access-data-freely/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/blog/how-opendal-read-data/" class="pagination-nav__link pagination-nav__link--prev"></a>

Newer post

Apache OpenDALâ„¢ Internal: Data Reading

<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator/" class="pagination-nav__link pagination-nav__link--next"></a>

Older post

Way to Go: OpenDAL successfully entered Apache Incubator
