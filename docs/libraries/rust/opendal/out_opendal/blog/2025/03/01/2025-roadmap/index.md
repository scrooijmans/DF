# Apache OpenDAL 2025 Roadmap: Perfecting Production Adoption

March 1, 2025 Â· 5 min read

<a href="https://github.com/Xuanwo" rel="noopener noreferrer" target="_blank">Xuanwo</a>

Hi everyone, long time no see. Although we have been actively developing OpenDAL and consistently releasing updates, we havenâ€™t provided a clear overview of where OpenDAL is heading. This post aims to address that. Here, we will discuss OpenDAL's current position and the future directions we are moving toward.

I hope this post serves as a guide for our development, encourages more people to contribute, and ultimately helps achieve the vision of OpenDAL.

## What's OpenDAL?<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#whats-opendal" class="hash-link" aria-label="Direct link to What&#39;s OpenDAL?" translate="no" title="Direct link to What&#39;s OpenDAL?">â€‹</a>

Apache OpenDAL (`/ËˆoÊŠ.pÉ™n.dÃ¦l/`, pronounced "OH-puhn-dal") is an Open Data Access Layer that enables seamless interaction with diverse storage services. Our VISION is <a href="https://opendal.apache.org/vision" rel="noopener noreferrer" target="_blank"><strong>One Layer, All Storage</strong></a>, and our core principles are Open Community, Solid Foundation, Fast Access, Object Storage First, and Extensible Architecture.

We are building:

- <a href="https://crates.io/crates/opendal" rel="noopener noreferrer" target="_blank">A core library built in Rust</a> that supports all services at zero cost and offers unified retry, concurrency, logging, tracing, metrics, timeout and more layers.
- Language bindings for <a href="https://pypi.org/project/opendal/" rel="noopener noreferrer" target="_blank">Python</a>, <a href="https://central.sonatype.com/artifact/org.apache.opendal/opendal" rel="noopener noreferrer" target="_blank">Java</a>, <a href="https://www.npmjs.com/package/opendal" rel="noopener noreferrer" target="_blank">Node.js</a>, C/C++, and more.
- Integrations with various frameworks, including <a href="https://crates.io/crates/parquet-opendal" rel="noopener noreferrer" target="_blank">Parquet</a>, <a href="https://crates.io/crates/fuse3_opendal" rel="noopener noreferrer" target="_blank">FUSE</a>, <a href="https://github.com/messense/dav-server-rs" rel="noopener noreferrer" target="_blank">DAV server</a>, and others.
- Binaries for different use cases, such as <a href="https://crates.io/crates/oli" rel="noopener noreferrer" target="_blank">CLI</a> and <a href="https://crates.io/crates/ofs" rel="noopener noreferrer" target="_blank">FUSE</a>.

In short, through OpenDAL, users can access ALL storage services within ONE layer.

## Where's OpenDAL?<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#wheres-opendal" class="hash-link" aria-label="Direct link to Where&#39;s OpenDAL?" translate="no" title="Direct link to Where&#39;s OpenDAL?">â€‹</a>

OpenDAL's rust core has already released 143 versions, has <a href="https://crates.io/crates/opendal/reverse_dependencies" rel="noopener noreferrer" target="_blank">67 reverse dependencies</a> listed on <a href="http://crates.io/" rel="noopener noreferrer" target="_blank">crates.io</a>, and is used by <a href="https://github.com/apache/opendal/network/dependents" rel="noopener noreferrer" target="_blank">612 projects</a> as recorded on GitHub.

OpenDAL's production users include databases such as <a href="https://github.com/databendlabs/databend" rel="noopener noreferrer" target="_blank">Databend</a>, <a href="https://github.com/GreptimeTeam/greptimedb" rel="noopener noreferrer" target="_blank">GreptimeDB</a>, and <a href="https://github.com/risingwavelabs/risingwave" rel="noopener noreferrer" target="_blank">RisingWave</a>, as well as tools like <a href="https://loco.rs/" rel="noopener noreferrer" target="_blank">Loco</a>, <a href="https://github.com/mozilla/sccache" rel="noopener noreferrer" target="_blank">sccache</a>, and <a href="https://vector.dev/" rel="noopener noreferrer" target="_blank">Vector</a>.

<img src="out_opendal/blog/2025/03/01/2025-roadmap/index_media/059d86af86fe8633fcf5975763d75a5e45356a22.png" class="img_KBPg" decoding="async" loading="lazy" width="1928" height="1300" />

Apart from OpenDAL's Rust core, its various language bindings have also seen significant growth over the past year. Take python binding as an example. <a href="https://github.com/langgenius/dify/" rel="noopener noreferrer" target="_blank">Dify</a>, an LLM app development platform, is using OpenDAL to access different storage services.

<img src="out_opendal/blog/2025/03/01/2025-roadmap/index_media/8ad2982591eb67e5db7f5bba623140dfaebf0cb3.png" class="img_KBPg" decoding="async" loading="lazy" width="2502" height="796" />

## What's next for OpenDAL?<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#whats-next-for-opendal" class="hash-link" aria-label="Direct link to What&#39;s next for OpenDAL?" translate="no" title="Direct link to What&#39;s next for OpenDAL?">â€‹</a>

The following is the famous technology adoption lifecycle curve. If I were to indicate the position of OpenDAL, I would say it is at the end of the Innovators stage and moving toward the Early Adopters stage.

<img src="out_opendal/blog/2025/03/01/2025-roadmap/index_media/9a87ebb7c2654ee7742af973adb128b36c646f3b.png" class="img_KBPg" decoding="async" loading="lazy" width="1200" height="720" /> (picture from TechTarget <a href="https://www.techtarget.com/searchcio/definition/technology-adoption-lifecycle" rel="noopener noreferrer" target="_blank">technology adoption lifecycle</a>)

Innovators are adopting OpenDAL. Projects like Databend, RisingWave, GreptimeDB, and sccache have been using OpenDAL in production for years. However, early adopters are still hesitant to use OpenDALâ€”and they have valid reasons.

For examples:

- OpenDAL hasn't reached version 1.0 yet and still introduces breaking changes from time to time, sometimes even requiring code modifications for an OpenDAL upgrade. This also creates a burden for libraries that depend on OpenDAL, as every breaking change affects them as well.
- OpenDAL lacks comprehensive documentation, particularly for its bindings in Python and Java. Users need to invest significant effort and love to integrate OpenDAL bindings into their projects.
- OpenDAL lacks some important features, such as checksum support, caching, metrics of underlying http requests and initialization from a URI.

I believe we should perfect production adoption in 2025 to get OpenDAL ready for early adopters. Only in this way can we implement our vision: **One Layer, All Storage.**

## What to do in 2025?<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#what-to-do-in-2025" class="hash-link" aria-label="Direct link to What to do in 2025?" translate="no" title="Direct link to What to do in 2025?">â€‹</a>

Our plans for 2025 include the following:

### Features Needed in Production<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#features-needed-in-production" class="hash-link" aria-label="Direct link to Features Needed in Production" translate="no" title="Direct link to Features Needed in Production">â€‹</a>

In 2025, we plan to implement the following features that are importmant for production adoption:

- <a href="https://github.com/apache/opendal/issues/5480" rel="noopener noreferrer" target="_blank">Context</a>: Introduce context in OpenDAL so that services and layers can share the same context. This enables users to gain deeper insights into OpenDAL's internal operations by incorporating metrics, logging, and tracing into the underlying HTTP client we use.
- <a href="https://github.com/apache/opendal/issues/2611" rel="noopener noreferrer" target="_blank">Versioning</a>: Introduce full file versioning support in OpenDAL, enabling users to read, write, delete, list, and restore versioned files. This functionality will allow users to recover mistakenly deleted files and facilitate disaster recovery.
- <a href="https://github.com/apache/opendal/issues/5549" rel="noopener noreferrer" target="_blank">Checksum</a>: Introduce end-to-end checksum support in OpenDAL, enabling users to perform checksums during reading and writing without worrying about bit flips in memory or over the network.
- <a href="https://github.com/apache/opendal/issues/5678" rel="noopener noreferrer" target="_blank">Caching</a>: Provide high-quality built-in cache support in OpenDAL while ensuring users have the flexibility to implement their own caching logic.
- <a href="https://github.com/apache/opendal/issues/3022" rel="noopener noreferrer" target="_blank">Initialization From URI</a>: Allow users to initialize OpenDAL from a URI string, making it easier to configure and use OpenDAL in various environments.

### Improvements Needed for Production<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#improvements-needed-for-production" class="hash-link" aria-label="Direct link to Improvements Needed for Production" translate="no" title="Direct link to Improvements Needed for Production">â€‹</a>

In 2025, we plan to improve the following aspects of OpenDAL to ensure users can confidently use it in production.

- Documentation: Improve the documentation for OpenDAL, particularly for bindings like Python, Node.js, and Java. The first step is to generate well-structured documentation for the configuration values of each service.
- Communitation: Bring Back Our Community Meetings. OpenDAL used to hold regular tri-weekly meetings, but we have been unable to maintain them in the last year. We now plan to revive these meetings and encourage more face-to-face discussions in the coming years.

## Conclusion<a href="https://opendal.apache.org/blog/2025/03/01/2025-roadmap/#conclusion" class="hash-link" aria-label="Direct link to Conclusion" translate="no" title="Direct link to Conclusion">â€‹</a>

2025 marks the third year of the OpenDAL community. A huge thanks to all OpenDAL contributors and users for helping us reach this milestone. There's still a long way to go to achieve our vision, and we invite you all to join us on this incredible journey!

<a href="https://github.com/apache/opendal/discussions/5679" rel="noopener noreferrer" target="_blank">Discuss about this post here</a>

**Tags:**

- <a href="https://opendal.apache.org/blog/tags/announcement/" class="tag_f4_J tagRegular_MJrJ" rel="tag">announcement</a>

<a href="https://github.com/apache/opendal/tree/main/website/blog/2025-03-01-2025-roadmap/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/blog/2025/03/01/2025-roadmap/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/blog/2025/09/15/apache-opendal-reqsign/" class="pagination-nav__link pagination-nav__link--prev"></a>

Newer post

reqsign is now Apache OpenDAL Reqsign

<a href="https://opendal.apache.org/blog/2024/09/09/apache-opendal-meetup-beijing-1st/" class="pagination-nav__link pagination-nav__link--next"></a>

Older post

Apache OpenDALâ„¢ Beijing Meetup 1st
