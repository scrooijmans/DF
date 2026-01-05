# Way to Go: OpenDAL successfully entered Apache Incubator

March 16, 2023 Â· 5 min read

<a href="https://github.com/PsiACE" rel="noopener noreferrer" target="_blank">PsiACE</a>

<img src="out_opendal/blog/opendal-entered-apache-incubator/index_media/ea383b1957768c25a0f71f5519e9ba6b440abd86.png" class="img_KBPg" decoding="async" loading="lazy" width="1876" height="799" alt="OpenDAL successfully entered Apache Incubator" />

On February 27th, 2023, the <a href="https://github.com/apache/opendal" rel="noopener noreferrer" target="_blank">OpenDAL</a> project achieved a milestone by winning the approval to join the incubator of the <a href="https://www.apache.org/" rel="noopener noreferrer" target="_blank">Apache Software Foundation</a> (ASF), the world's leading open source software organization. On March 15th, the OpenDAL project was officially transferred to the Apache Software Foundation.

This is a significant moment for <a href="https://github.com/datafuselabs/databend" rel="noopener noreferrer" target="_blank">Databend</a>, as it means that OpenDAL's technology and vision have received wider recognition and support from the open source community.

> The Apache Incubator was established in October 2002 to provide a path for projects and codebases that aspire to become part of the Apache Software Foundation. Incubating projects need to follow ASF's governance and operational practices, and use ASF's infrastructure and resources. Incubating projects need to go through a series of stages and evaluations before they can graduate to become top-level projects (TLPs) of ASF.

<img src="out_opendal/blog/opendal-entered-apache-incubator/index_media/cc236bb69895e3efd0e34899b273004f645b7847.png" class="img_KBPg" decoding="async" loading="lazy" width="1373" height="1006" alt="Apache OpenDAL Project Incubation Status - Apache Incubator" />

*<a href="https://incubator.apache.org/projects/opendal.html" rel="noopener noreferrer" target="_blank">https://incubator.apache.org/projects/opendal.html</a>*

## What is OpenDAL?<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#what-is-opendal" class="hash-link" aria-label="Direct link to What is OpenDAL?" translate="no" title="Direct link to What is OpenDAL?">â€‹</a>

Data is one of the most important assets in the future, and data access is the key for realizing data value.

There are various kinds of storage services in the market, each with its own unique interfaces and features, which bring a lot of complexity and inconvenience to data access.

OpenDAL provides a unified, simple, efficient, reliable, and observable data access layer that allows developers to seamlessly use different storage services and enjoy the best user experience.

<img src="out_opendal/blog/opendal-entered-apache-incubator/index_media/c40e2bc0aca53a7988cde5d5de26cf495d8ce7f8.png" class="img_KBPg" decoding="async" loading="lazy" width="5724" height="1518" alt="M*N to M+N with OpenDAL" />

OpenDAL simplifies the process of interfacing different storage services, and provides features such as automatic retry, request optimization, and observability. With OpenDAL, developers can directly access a bunch of storage services, without having to understand and master the details of specific SDKs.

OpenDAL's features include but are not limited to:

- Support for dozens of storage services, including local file system, HDFS, S3, OSS, etc.
- Provide a unified data access interface, without worrying about the underlying storage details.
- Support for various common data operations, including `read`, `write`, `list`, etc.
- Support for automatic retry, request optimization, and observability mechanisms.
- Zero cost, directly mapped to API calls.
- Cross-language bindings: Python, Node.js, C (being worked on), etc.

## The Story about OpenDAL<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#the-story-about-opendal" class="hash-link" aria-label="Direct link to The Story about OpenDAL" translate="no" title="Direct link to The Story about OpenDAL">â€‹</a>

### Born for Universal Data Access<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#born-for-universal-data-access" class="hash-link" aria-label="Direct link to Born for Universal Data Access" translate="no" title="Direct link to Born for Universal Data Access">â€‹</a>

OpenDAL originated from the vision of creating a universal, unified and user-friendly data access layer. It came into being in late 2021, initially as a component of the Databend project.

- On December 21, 2021, <a href="http://github.com/Xuanwo" rel="noopener noreferrer" target="_blank">Xuanwo</a> embarked on the design and re-implementation of Databend's storage access layer, <a href="https://github.com/datafuselabs/databend/pull/3575" rel="noopener noreferrer" target="_blank">dal2: Add basic operations of <code>read</code>, <code>write</code>, <code>stat</code>, <code>delete</code></a>.
- On December 27, 2021, the <a href="https://github.com/datafuselabs/databend/discussions/3662" rel="noopener noreferrer" target="_blank">proposal: Vision of Databend DAL</a> was put forward and discussed. On December 29th, dal2's implementation was integrated into Databend.
- On February 14th 2022 , dal2 officially separated from Databend's code repository and became a standalone top-level project. It was formally renamed OpenDAL.

### From One to Multiple<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#from-one-to-multiple" class="hash-link" aria-label="Direct link to From One to Multiple" translate="no" title="Direct link to From One to Multiple">â€‹</a>

Thanks to Xuanwo, <a href="https://github.com/ClSlaid" rel="noopener noreferrer" target="_blank">ClSlaid</a> and many other contributors, OpenDAL quickly became a data access layer that supports mainstream storage services such as AWS S3 / Azure Blob / GCS / HDFS, and provided cross-cloud native storage and access support for Databend's `COPY INTO`, Stage, Storage.

<a href="https://github.com/GreptimeTeam/greptimedb" rel="noopener noreferrer" target="_blank">GreptimeDB</a> was the first large-scale Rust database project to actively use OpenDAL after Databend. Later, with Xuanwo's efforts, <a href="https://github.com/mozilla/sccache" rel="noopener noreferrer" target="_blank">sccache</a> under Mozilla also tried to use OpenDAL to take over the storage layer. In order to provide more comprehensive support, OpenDAL soon added support for GitHub Action Cache.

Then, <a href="https://github.com/risingwavelabs/risingwave" rel="noopener noreferrer" target="_blank">RisingWave</a> and <a href="https://github.com/vectordotdev/vector" rel="noopener noreferrer" target="_blank">Vector</a> were supported as well. The number of OpenDAL users started to grow. More and more users choose OpenDAL as their storage access layer.

### Sky's the Limit<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#skys-the-limit" class="hash-link" aria-label="Direct link to Sky&#39;s the Limit" translate="no" title="Direct link to Sky&#39;s the Limit">â€‹</a>

OpenDAL has established a small community and formed a product matrix. In addition to the <a href="https://crates.io/crates/opendal" rel="noopener noreferrer" target="_blank">Rust - opendal</a>, it also provides <a href="https://pypi.org/project/opendal/" rel="noopener noreferrer" target="_blank">Python - opendal</a> and <a href="https://www.npmjs.com/package/opendal" rel="noopener noreferrer" target="_blank">Node.js - opendal</a> bindings.

OpenDAL has released 99 versions since its open source, with 700+ GitHub stars, 349K downloads, and 48 developers. The project has been actively updated. We sincerely thank every contributor for their efforts and dedication.

Being a part of Apache incubator is an important milestone in OpenDAL's development history. We hope to leverage ASF's platform and resources to let OpenDAL focus on providing a neutral, vendor-free, painless, and efficient storage access layer, and better serve the developers.

We expect OpenDAL to be widely used in the following application scenarios:

- Data analysis: OpenDAL can help data analysts quickly read or write data from different storage services, and perform various format conversions and operations.
- Data science: OpenDAL can help data scientists easily get or save data from different storage services, and perform various preprocessing and postprocessing.
- Data engineering: OpenDAL can help data engineers efficiently build and manage data pipelines between different storage services, and perform various monitoring and tuning.

## Acknowledgements<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#acknowledgements" class="hash-link" aria-label="Direct link to Acknowledgements" translate="no" title="Direct link to Acknowledgements">â€‹</a>

***From Xuanwo***

Hello everyone, I'm Xuanwo, the Committer of Apache OpenDAL.

The OpenDAL project embodies my personal dream. Now it has entered the Apache incubator with the collaboration of the community. I feel very happy at this moment. Thank you all contributors for your contributions, thank Databend Labs for your support, thank Champion tison for your guidance, thank Mentors ningjiang, wusheng, tedliu and hexiaoqiao for your advice.

Let us follow the guidance of Apache Way to build a community together and create value for users by providing free, painless and efficient data access experience!

## Join Apache OpenDAL Community<a href="https://opendal.apache.org/blog/opendal-entered-apache-incubator#join-apache-opendal-community" class="hash-link" aria-label="Direct link to Join Apache OpenDAL Community" translate="no" title="Direct link to Join Apache OpenDAL Community">â€‹</a>

We welcome developers and users who are interested in participating in OpenDAL project to join OpenDAL community and follow OpenDAL's latest news. You can get more information through the following ways:

- Visit OpenDAL official website: <a href="https://opendal.apache.org" rel="noopener noreferrer" target="_blank">https://opendal.apache.org</a>
- Explore OpenDAL GitHub repository: <a href="https://github.com/apache/opendal" rel="noopener noreferrer" target="_blank">https://github.com/apache/opendal</a>
- Join OpenDAL Discord channel: <a href="https://opendal.apache.org/discord" rel="noopener noreferrer" target="_blank">https://opendal.apache.org/discord</a>
- Subscribe to OpenDAL mailing list: <a href="mailto:dev@opendal.apache.org" rel="noopener noreferrer" target="_blank">dev@opendal.apache.org</a>

**Tags:**

- <a href="https://opendal.apache.org/blog/tags/announcement/" class="tag_f4_J tagRegular_MJrJ" rel="tag">announcement</a>

<a href="https://github.com/apache/opendal/tree/main/website/blog/2023-03-16-opendal-entered-apache-incubator/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/blog/opendal-entered-apache-incubator/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/blog/opendal-access-data-freely/" class="pagination-nav__link pagination-nav__link--prev"></a>

Newer post

Apache OpenDALâ„¢: Access Data Freely
