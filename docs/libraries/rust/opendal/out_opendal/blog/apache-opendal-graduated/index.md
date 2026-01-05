# Apache OpenDALâ„¢ is now Graduated

January 22, 2024 Â· 7 min read

<a href="https://github.com/Xuanwo" rel="noopener noreferrer" target="_blank">Xuanwo</a>

Hello, everyone! I'm happy to announce that <a href="https://opendal.apache.org/" rel="noopener noreferrer" target="_blank">Apache OpenDALâ„¢</a> has graduated from the <a href="https://incubator.apache.org/" rel="noopener noreferrer" target="_blank">Apache Incubator</a> to become a Top-Level Project of <a href="https://apache.org/" rel="noopener noreferrer" target="_blank">the Apache Software Foundation</a>.

## What's Apache OpenDAL?<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#whats-apache-opendal" class="hash-link" aria-label="Direct link to What&#39;s Apache OpenDAL?" translate="no" title="Direct link to What&#39;s Apache OpenDAL?">â€‹</a>

**Apache OpenDAL** is a data access layer that allows users to easily and efficiently retrieve data from various storage services in a unified way. Our VISION is **access data freely**.

OpenDAL could be used as a **better** SDK for your storage services: A SDK with native integration of <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.RetryLayer.html" rel="noopener noreferrer" target="_blank">retry</a>, <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.LoggingLayer.html" rel="noopener noreferrer" target="_blank">logging</a>, <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.MetricsLayer.html" rel="noopener noreferrer" target="_blank">metrics</a>, <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TracingLayer.html" rel="noopener noreferrer" target="_blank">tracing</a>, <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TimeoutLayer.html" rel="noopener noreferrer" target="_blank">timeout</a>, <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.ThrottleLayer.html" rel="noopener noreferrer" target="_blank">throttle</a>, and <a href="https://opendal.apache.org/docs/rust/opendal/layers/index.html" rel="noopener noreferrer" target="_blank">more</a>.

OpenDAL could be used as a **super** connector for your storage services: A connector that supports all kinds of storage services from Object Storage (s3, gcs, azblob), File Storage (fs, azdls, hdfs), Consumer Cloud Storage (gdrive, onedrive), Key-Value Storage (rocksdb, sled) to Cache Storage (memcached, moka).

OpenDAL could be used as an **elegant** client for your storage services: A client with well designed API and many language bindings: <a href="https://crates.io/crates/opendal" rel="noopener noreferrer" target="_blank">Rust</a>, C, Cpp, Dotnet, Go, Haskell, <a href="https://mvnrepository.com/artifact/org.apache.opendal/opendal-java" rel="noopener noreferrer" target="_blank">Java</a>, Lua, <a href="https://www.npmjs.com/package/opendal" rel="noopener noreferrer" target="_blank">Node.js</a>, OCaml, PHP, <a href="https://pypi.org/project/opendal/" rel="noopener noreferrer" target="_blank">Python</a>, Ruby, Swift and Zig.

Need to access data? Give OpenDAL a try!

``` prism-code
async fn main() -> Result<()> {
    // Init s3 service.
    let mut builder = services::S3::default();
    builder.bucket("test");

    // Init an operator
    let op = Operator::via_map(builder)?
        // Add logging
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

## What's the ASF?<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#whats-the-asf" class="hash-link" aria-label="Direct link to What&#39;s the ASF?" translate="no" title="Direct link to What&#39;s the ASF?">â€‹</a>

The Apache Software Foundation (ASF) is a nonprofit corporation to support a number of open-source software projects. The Apache Software Foundation exists to provide software for the public good. We believe in the power of community over code, known as The Apache Way. Thousands of people around the world contribute to ASF open source projects every day.

The OpenDAL Community believes <a href="https://www.apache.org/theapacheway/" rel="noopener noreferrer" target="_blank">the apache way</a> that:

- *Earned Authority*: all individuals are given the opportunity to participate, but their influence is based on publicly earned merit â€“ what they contribute to the community.
- *Community of Peers*: individuals participate at the ASF, not organizations.
- *Open Communications*: as a virtual organization, the ASF requires all communications related to code and decision-making to be publicly accessible to ensure asynchronous collaboration, as necessitated by a globally-distributed community.
- *Consensus Decision Making*: Apache Projects are overseen by a self-selected team of active volunteers who are contributing to their respective projects.
- *Responsible Oversight*: The ASF governance model is based on trust and delegated oversight.

The original creators <a href="https://github.com/datafuselabs/databend/" rel="noopener noreferrer" target="_blank">Databend</a> chosen to contribute OpenDAL to the ASF, embracing the Apache way through <a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator" rel="noopener noreferrer" target="_blank">joining the incubator program</a>.

## What's graduation?<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#whats-graduation" class="hash-link" aria-label="Direct link to What&#39;s graduation?" translate="no" title="Direct link to What&#39;s graduation?">â€‹</a>

In the <a href="https://incubator.apache.org/" rel="noopener noreferrer" target="_blank">Apache Incubator</a>, the OpenDAL community is learning the Apache Way through daily development activities, growing its community and producing Apache releases.

During the incubation, we:

- Consist of 19 committers, including mentors, with 12 serving as PPMC members.
- Boast 164 contributors.
- Made 9 releasesâ€”averaging at least one per month.
- Had 7 different release managers to date.
- Used by 10 known entities and is a dependency for 263 GitHub projects and 18 crates.io packages.
- Opened 1,200+ issues with 1,100+ successfully resolved.
- Submitted a total of 2,400+ PRs, most of them have been merged or closed.

The graduation signifies that the OpenDAL Community is recognized as a <a href="https://opendal.apache.org/community/maturity" rel="noopener noreferrer" target="_blank">mature</a> community, which entails:

- CODE: OpenDAL is an <a href="https://github.com/apache/opendal/blob/main/LICENSE" rel="noopener noreferrer" target="_blank">Apache 2.0 licensed</a> open-source project with <a href="https://github.com/apache/opendal" rel="noopener noreferrer" target="_blank">accessible, buildable code</a> on GitHub, featuring <a href="https://github.com/apache/opendal/commits/main/" rel="noopener noreferrer" target="_blank">a traceable history and authenticated code provenance</a>.
- LICENSE: OpenDAL maintains <a href="https://github.com/apache/opendal/blob/main/DEPENDENCIES.md" rel="noopener noreferrer" target="_blank">open-source compliance</a> for all code and dependencies, requires contributor agreements, and clearly documents copyright ownership.
- Releases: OpenDAL offers standardized, committee-approved <a href="https://downloads.apache.org/opendal/" rel="noopener noreferrer" target="_blank">source code releases</a> with secure signatures, provides convenience binaries, and has <a href="https://opendal.apache.org/community/committers/release" rel="noopener noreferrer" target="_blank">a well-documented, repeatable release process</a>.
- Quality: OpenDAL is committed to code quality transparency, prioritizes security with quick issue responses, ensures backward compatibility with clear documentation, and actively addresses bug reports in a timely manner.
- Community: OpenDAL offers <a href="https://opendal.apache.org/" rel="noopener noreferrer" target="_blank">a comprehensive homepage</a>, welcomes diverse contributions, promotes a meritocratic approach for active contributors, operates on community consensus, and ensures timely responses to user queries through various channels.
- Consensus: OpenDAL has a <a href="https://projects.apache.org/committee.html?opendal" rel="noopener noreferrer" target="_blank">public list of key decision-makers</a> and uses a consensus approach for decisions, documented on its <a href="https://lists.apache.org/list.html?dev@opendal.apache.org" rel="noopener noreferrer" target="_blank">main communication channel</a>. It follows standard voting rules and records all important discussions in writing.
- Independence: OpenDAL is independent, with contributors from various companies acting on their own, not as representatives of any organization.

## What's next?<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#whats-next" class="hash-link" aria-label="Direct link to What&#39;s next?" translate="no" title="Direct link to What&#39;s next?">â€‹</a>

After graduation, OpenDAL Community will continue to focus on the following aspects under the VISION: **access data freely**.

### More Stable Services<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#more-stable-services" class="hash-link" aria-label="Direct link to More Stable Services" translate="no" title="Direct link to More Stable Services">â€‹</a>

OpenDAL now supports 59 services, although only some of them are *stable*.

*stable* for OpenDAL means that

- Have integration tests covered.
- Have at least one production user.

The *stable* service established a feedback loop between the OpenDAL community and its users. Users can submit bug reports or feature requests to the OpenDAL community, which in turn can enhance the service using this feedback while ensuring existing features remain intact.

After graduation, the OpenDAL community will focus on improving the stability of current services instead of just expanding our offerings.

We plan to:

- Add features users wanted to services like <a href="https://github.com/apache/opendal/issues/2611" rel="noopener noreferrer" target="_blank">file version</a>, <a href="https://github.com/apache/opendal/issues/3977" rel="noopener noreferrer" target="_blank">concurrently list</a> and <a href="https://github.com/apache/opendal/issues/1251" rel="noopener noreferrer" target="_blank">glob pattern</a>.
- Add integration tests for newly added services.

### More Useful Documents<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#more-useful-documents" class="hash-link" aria-label="Direct link to More Useful Documents" translate="no" title="Direct link to More Useful Documents">â€‹</a>

OpenDAL have good docs for its rust core, but not for other language bindings.

The lack of comprehensive documentation makes OpenDAL challenging for users to operate in Java or Python. Without user feedback, the community is unable to enhance this documentation, leading to a detrimental cycle that must be broken.

After graduation, the OpenDAL community will improve the documentation of other language bindings.

We plan to:

- Introduce code generation to automatically create documentation for the service builder due to its numerous configurations.
- Add more API Docs and examples for other language bindings.

OpenDAL have good docs for its public API, but not for its internal design.

OpenDAL is proud of its elegant design, but it is not well documented. This makes it difficult for new contributors to understand the codebase and make contributions.

After graduation, the OpenDAL community will improve the documentation of its internal design.

We plan to:

- Optimize the codebase to make it easier to understand.
- Add more blog posts to explain the design of OpenDAL.

### More Production Users<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#more-production-users" class="hash-link" aria-label="Direct link to More Production Users" translate="no" title="Direct link to More Production Users">â€‹</a>

OpenDAL requires more production users, as they are vital to the success of our project. Increased user production leads to more valuable feedback, a more engaged contributor base, and a stronger community. We've started the initial loop; let's expand it!

After graduation, the OpenDAL community will focus on attracting more production users.

We plan to:

- Optimize the feature set for adoption like <a href="https://github.com/apache/opendal/issues/3022" rel="noopener noreferrer" target="_blank">uri initiation</a> and <a href="https://github.com/apache/opendal/issues/3240" rel="noopener noreferrer" target="_blank">config</a>.
- Expand more ways to use OpenDAL via <a href="https://github.com/apache/opendal/tree/main/bin/ofs" rel="noopener noreferrer" target="_blank">fuse</a>, <a href="https://github.com/apache/opendal/tree/main/bin/oli" rel="noopener noreferrer" target="_blank">cli</a>, <a href="https://github.com/apache/opendal/tree/main/bin/oli" rel="noopener noreferrer" target="_blank">S3/WebDAV API</a>, <a href="https://github.com/apache/opendal/tree/main/integrations/object_store" rel="noopener noreferrer" target="_blank">object_store binding</a>.

## Conclusion<a href="https://opendal.apache.org/blog/apache-opendal-graduated/#conclusion" class="hash-link" aria-label="Direct link to Conclusion" translate="no" title="Direct link to Conclusion">â€‹</a>

The OpenDAL Community aims to create a world where users can freely access data across any storage service in any manner they choose. Graduation is just the beginningâ€”let's work together to make our VISION a reality!

**Tags:**

- <a href="https://opendal.apache.org/blog/tags/announcement/" class="tag_f4_J tagRegular_MJrJ" rel="tag">announcement</a>

<a href="https://github.com/apache/opendal/tree/main/website/blog/2024-01-22-apache-opendal-graduated/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/blog/apache-opendal-graduated/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/blog/apache-opendal-participates-in-gsoc-2024/" class="pagination-nav__link pagination-nav__link--prev"></a>

Newer post

Apache OpenDALâ„¢ participates in Google Summer of Code 2024

<a href="https://opendal.apache.org/blog/owo-1/" class="pagination-nav__link pagination-nav__link--next"></a>

Older post

OwO \#1: The v0.40 Release
