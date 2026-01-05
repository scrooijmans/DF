- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/community/maturity/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- Maturity

On this page

# Maturity Assessment for Apache OpenDALâ„¢

The goals of this maturity model are to describe how Apache projects operate in a concise and high-level way, and to provide a basic framework that projects may choose to use to evaluate themselves.

More details can be found <a href="https://community.apache.org/apache-way/apache-project-maturity-model.html" rel="noopener noreferrer" target="_blank">here</a>.

## Status of this assessment<a href="https://opendal.apache.org/community/maturity#status-of-this-assessment" class="hash-link" aria-label="Direct link to Status of this assessment" translate="no" title="Direct link to Status of this assessment">â€‹</a>

This assessment is evaluated during OpenDAL's graduation, which is finished on 2024-01-18.

## Maturity model assessment<a href="https://opendal.apache.org/community/maturity#maturity-model-assessment" class="hash-link" aria-label="Direct link to Maturity model assessment" translate="no" title="Direct link to Maturity model assessment">â€‹</a>

The following table is filled according to the <a href="https://community.apache.org/apache-way/apache-project-maturity-model.html" rel="noopener noreferrer" target="_blank">Apache Maturity Model</a>. Mentors and community members are welcome to comment and modify it.

### CODE<a href="https://opendal.apache.org/community/maturity#code" class="hash-link" aria-label="Direct link to CODE" translate="no" title="Direct link to CODE">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **CD10** | The project produces Open Source software for distribution to the public, at no charge. | **YES** The project source code is licensed under the `Apache License 2.0`. |
| **CD20** | Anyone can easily discover and access the project's code.. | **YES** The <a href="https://opendal.apache.org/" rel="noopener noreferrer" target="_blank">official website</a> includes `GitHub` link which can access the project's repository on GitHub directly. |
| **CD30** | Anyone using standard, widely-available tools, can build the code in a reproducible way. | **YES** Apache OpenDAL provide `how-to-build` document for every component to tell user how to compile on bare metal, such as the <a href="https://github.com/apache/opendal/blob/main/core/CONTRIBUTING.md" rel="noopener noreferrer" target="_blank">core's</a>. |
| **CD40** | The full history of the project's code is available via a source code control system, in a way that allows anyone to recreate any released version. | **YES** It depends on git, and anyone can view the full history of the project via commit logs. |
| **CD50** | The source code control system establishes the provenance of each line of code in a reliable way, based on strong authentication of the committer. When third parties contribute code, commit messages provide reliable information about the code provenance. | **YES** The project uses GitHub and managed by Apache Infra, it ensuring provenance of each line of code to a committer. And the third-party contributions are accepted in accordance with the contributing guides. |

### LICENSE<a href="https://opendal.apache.org/community/maturity#license" class="hash-link" aria-label="Direct link to LICENSE" translate="no" title="Direct link to LICENSE">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **LC10** | The Apache License, version 2.0, covers the released code. | **YES** The <a href="https://github.com/apache/opendal/blob/main/LICENSE" rel="noopener noreferrer" target="_blank">LICENSE</a> is in GitHub repository. And all source files are with APLv2 header, checked by `korandoru/hawkeye@v3.6.0`. |
| **LC20** | Libraries that are mandatory dependencies of the project's code do not create more restrictions than the Apache License does. | **YES** All dependencies are listed. |
| **LC30** | The libraries mentioned in LC20 are available as Open Source software. | **YES** All dependencies are listed are available as Open Source software |
| **LC40** | Committers are bound by an Individual Contributor Agreement (the "Apache iCLA") that defines which code they may commit and how they need to identify code that is not their own. | **YES** All committers have iCLAs. |
| **LC50** | The project clearly defines and documents the copyright ownership of everything that the project produces. | **YES** And all source files are with APLv2 header, checked by `korandoru/hawkeye@v3.6.0`. |

### Releases<a href="https://opendal.apache.org/community/maturity#releases" class="hash-link" aria-label="Direct link to Releases" translate="no" title="Direct link to Releases">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **RE10** | Releases consist of source code, distributed using standard and open archive formats that are expected to stay readable in the long term. | **YES** Source release is distributed via <a href="https://dist.apache.org/repos/dist/release/opendal/" rel="noopener noreferrer" target="_blank">dist.apache.org</a> and linked from <a href="https://opendal.apache.org/download/" rel="noopener noreferrer" target="_blank">download page</a>. |
| **RE20** | The project's PPMC (Project Management Committee, see CS10) approves each software release in order to make the release an act of the Foundation. | **YES** All releases have been voted at <a href="mailto:dev@opendal.apache.org" rel="noopener noreferrer" target="_blank">dev@opendal.apache.org</a> and <a href="mailto:general@incubator.apache.org" rel="noopener noreferrer" target="_blank">general@incubator.apache.org</a>, and have at least 3 PPMC member's votes. |
| **RE30** | Releases are signed and/or distributed along with digests that anyone can reliably use to validate the downloaded archives. | **YES** All releases are signed, and the <a href="https://dist.apache.org/repos/dist/release/opendal/KEYS" rel="noopener noreferrer" target="_blank">KEYS</a> are available. |
| **RE40** | The project can distribute convenience binaries alongside source code, but they are not Apache Releases, they are provided with no guarantee. | **YES** User can easily build binaries from source code, and we do not provide binaries as Apache Releases. |
| **RE50** | The project documents a repeatable release process so that someone new to the project can independently generate the complete set of artifacts required for a release. | **YES** We can follow the <a href="https://opendal.apache.org/community/committers/release" rel="noopener noreferrer" target="_blank">Release guide</a> to make a new Apache OpenDAL release, and so far we had 6 different release managers. |

### Quality<a href="https://opendal.apache.org/community/maturity#quality" class="hash-link" aria-label="Direct link to Quality" translate="no" title="Direct link to Quality">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **QU10** | The project is open and honest about the quality of its code. Various levels of quality and maturity for various modules are natural and acceptable as long as they are clearly communicated. | **YES** We encourage user to <a href="https://github.com/apache/opendal/issues" rel="noopener noreferrer" target="_blank">report issues</a>. |
| **QU20** | The project puts a very high priority on producing secure software. | **YES** All security issues will be addressed within 3 days. |
| **QU30** | The project provides a well-documented, secure and private channel to report security issues, along with a documented way of responding to them. | **Yes** The official website provides a <a href="https://opendal.apache.org/community/security" rel="noopener noreferrer" target="_blank">security page</a> |
| **QU40** | The project puts a high priority on backwards compatibility and aims to document any incompatible changes and provide tools and documentation to help users transition to new features. | **Yes** We follow semantic versions. As long as it's within one major version, it's backward compatible. And when any breaking changes added, we provide corresponding upgrade guides. |
| **QU50** | The project strives to respond to documented bug reports in a timely manner. | **YES** The project has resolved 1000+ issues and 2300+ pull requests so far, with very prompt response. |

### Community<a href="https://opendal.apache.org/community/maturity#community" class="hash-link" aria-label="Direct link to Community" translate="no" title="Direct link to Community">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **CO10** | The project has a well-known homepage that points to all the information required to operate according to this maturity model. | **YES** The <a href="https://opendal.apache.org/" rel="noopener noreferrer" target="_blank">official website</a> includes all information user need to run Apache OpenDAL. |
| **CO20** | The community welcomes contributions from anyone who acts in good faith and in a respectful manner, and who adds value to the project. | **Yes** We provide contributing guides for every component. And we also have a <a href="https://github.com/apache/opendal/blob/main/CONTRIBUTING.md" rel="noopener noreferrer" target="_blank">general contributing guide</a> |
| **CO30** | Contributions include source code, documentation, constructive bug reports, constructive discussions, marketing and generally anything that adds value to the project. | **YES** All good contributions including code and non-code are welcomed. |
| **CO40** | The community strives to be meritocratic and gives more rights and responsibilities to contributors who, over time, add value to the project. | **YES** The community has elected 2 new PPMC members and 7 new committers so far. |
| **CO50** | The project documents how contributors can earn more rights such as commit access or decision power, and applies these principles consistently. | **YES** The community has clear docs on nominating committers and PPMC members |
| **CO60** | The community operates based on consensus of its members (see CS10) who have decision power. Dictators, benevolent or not, are not welcome in Apache projects. | **YES** All decisions are made after vote by community members. |
| **CO70** | The project strives to answer user questions in a timely manner. | **YES** We use <a href="mailto:dev@opendal.apache.org" rel="noopener noreferrer" target="_blank">dev@opendal.apache.org</a>, <a href="https://github.com/apache/opendal/issues" rel="noopener noreferrer" target="_blank">GitHub issue</a> and <a href="https://github.com/apache/opendal/discussions" rel="noopener noreferrer" target="_blank">GitHub discussion</a> to do this in a timely manner. |

### Consensus<a href="https://opendal.apache.org/community/maturity#consensus" class="hash-link" aria-label="Direct link to Consensus" translate="no" title="Direct link to Consensus">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **CS10** | The project maintains a public list of its contributors who have decision power. The project's PPMC (Project Management Committee) consists of those contributors. | **Yes** See <a href="https://opendal.apache.org/community/#people" rel="noopener noreferrer" target="_blank">members</a> with all PPMC members and committers. |
| **CS20** | Decisions require a consensus among PPMC members and are documented on the project's main communications channel. The PPMC takes community opinions into account, but the PPMC has the final word. | **YES** All decisions are made by votes on <a href="mailto:dev@opendal.apache.org" rel="noopener noreferrer" target="_blank">dev@opendal.apache.org</a>, and with at least 3 +1 votes from PPMC. |
| **CS30** | The project uses documented voting rules to build consensus when discussion is not sufficient. | **YES** The project uses the standard ASF voting rules. |
| **CS40** | In Apache projects, vetoes are only valid for code commits. The person exercising the veto must justify it with a technical explanation, as per the Apache voting rules defined in CS30. | **YES** Apache OpenDAL community has not used the veto power yet except for code commits. |
| **CS50** | All "important" discussions happen asynchronously in written form on the project's main communications channel. Offline, face-to-face or private discussions that affect the project are also documented on that channel. | **YES** All important discussions and conclusions are recorded in written form. |

### Independence<a href="https://opendal.apache.org/community/maturity#independence" class="hash-link" aria-label="Direct link to Independence" translate="no" title="Direct link to Independence">â€‹</a>

| **ID** | **Description** | **Status** |
|----|----|----|
| **IN10** | The project is independent from any corporate or organizational influence. | **YES** The PPMC members and committer of Apache OpenDAL are from several different companies, and majority of them are NOT From the company that donated this project. |
| **IN20** | Contributors act as themselves, not as representatives of a corporation or organization. | **YES** The contributors act on their own initiative without representing a corporation or organization. |

<a href="https://github.com/apache/opendal/tree/main/website/community/maturity.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/community/maturity/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/community/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Community

<a href="https://opendal.apache.org/community/security/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Security

- <a href="https://opendal.apache.org/community/maturity#status-of-this-assessment" class="table-of-contents__link toc-highlight">Status of this assessment</a>
- <a href="https://opendal.apache.org/community/maturity#maturity-model-assessment" class="table-of-contents__link toc-highlight">Maturity model assessment</a>
  - <a href="https://opendal.apache.org/community/maturity#code" class="table-of-contents__link toc-highlight">CODE</a>
  - <a href="https://opendal.apache.org/community/maturity#license" class="table-of-contents__link toc-highlight">LICENSE</a>
  - <a href="https://opendal.apache.org/community/maturity#releases" class="table-of-contents__link toc-highlight">Releases</a>
  - <a href="https://opendal.apache.org/community/maturity#quality" class="table-of-contents__link toc-highlight">Quality</a>
  - <a href="https://opendal.apache.org/community/maturity#community" class="table-of-contents__link toc-highlight">Community</a>
  - <a href="https://opendal.apache.org/community/maturity#consensus" class="table-of-contents__link toc-highlight">Consensus</a>
  - <a href="https://opendal.apache.org/community/maturity#independence" class="table-of-contents__link toc-highlight">Independence</a>
