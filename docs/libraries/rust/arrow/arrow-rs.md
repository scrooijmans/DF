Title: GitHub - apache/arrow-rs: Official Rust implementation of Apache Arrow

Description: Official Rust implementation of Apache Arrow. Contribute to apache/arrow-rs development by creating an account on GitHub.                                           

Skip to content  

You signed in with another tab or window. Reload to refresh your session. You signed out in another tab or window. Reload to refresh your session. You switched accounts on another tab or window. Reload to refresh your session. Dismiss alert

apache / **arrow-rs** Public

*   Notifications You must be signed in to change notification settings
*   Fork 960
*   Star 3k

Official Rust implementation of Apache Arrow

arrow.apache.org/

### License

Apache-2.0 license

3k stars 960 forks Branches Tags Activity

Star

Notifications You must be signed in to change notification settings

apache/arrow-rs
===============

 main

BranchesTags

Go to file

Code

Open more actions menu

Folders and files
-----------------

| Name | Name | 
Last commit message

| 

Last commit date

|
| --- | --- | --- | --- |
| 

Latest commit
-------------

History
-------

6,591 Commits

|
| 

.github

| 

.github

| 

| 

|
| 

arrow-arith

| 

arrow-arith

| 

| 

|
| 

arrow-array

| 

arrow-array

| 

| 

|
| 

arrow-avro

| 

arrow-avro

| 

| 

|
| 

arrow-buffer

| 

arrow-buffer

| 

| 

|
| 

arrow-cast

| 

arrow-cast

| 

| 

|
| 

arrow-csv

| 

arrow-csv

| 

| 

|
| 

arrow-data

| 

arrow-data

| 

| 

|
| 

arrow-flight

| 

arrow-flight

| 

| 

|
| 

arrow-integration-test

| 

arrow-integration-test

| 

| 

|
| 

arrow-integration-testing

| 

arrow-integration-testing

| 

| 

|
| 

arrow-ipc

| 

arrow-ipc

| 

| 

|
| 

arrow-json

| 

arrow-json

| 

| 

|
| 

arrow-ord

| 

arrow-ord

| 

| 

|
| 

arrow-pyarrow-integration-testing

| 

arrow-pyarrow-integration-testing

| 

| 

|
| 

arrow-pyarrow

| 

arrow-pyarrow

| 

| 

|
| 

arrow-row

| 

arrow-row

| 

| 

|
| 

arrow-schema

| 

arrow-schema

| 

| 

|
| 

arrow-select

| 

arrow-select

| 

| 

|
| 

arrow-string

| 

arrow-string

| 

| 

|
| 

arrow

| 

arrow

| 

| 

|
| 

dev

| 

dev

| 

| 

|
| 

format

| 

format

| 

| 

|
| 

parquet-testing @ b68bea4

| 

parquet-testing @ b68bea4

| 

| 

|
| 

parquet-variant

| 

parquet-variant

| 

| 

|
| 

parquet

| 

parquet

| 

| 

|
| 

parquet\_derive

| 

parquet\_derive

| 

| 

|
| 

parquet\_derive\_test

| 

parquet\_derive\_test

| 

| 

|
| 

testing @ 735ae71

| 

testing @ 735ae71

| 

| 

|
| 

.asf.yaml

| 

.asf.yaml

| 

| 

|
| 

.gitattributes

| 

.gitattributes

| 

| 

|
| 

.github\_changelog\_generator

| 

.github\_changelog\_generator

| 

| 

|
| 

.gitignore

| 

.gitignore

| 

| 

|
| 

.gitmodules

| 

.gitmodules

| 

| 

|
| 

.pre-commit-config.yaml

| 

.pre-commit-config.yaml

| 

| 

|
| 

CHANGELOG-old.md

| 

CHANGELOG-old.md

| 

| 

|
| 

CHANGELOG.md

| 

CHANGELOG.md

| 

| 

|
| 

CODE\_OF\_CONDUCT.md

| 

CODE\_OF\_CONDUCT.md

| 

| 

|
| 

CONTRIBUTING.md

| 

CONTRIBUTING.md

| 

| 

|
| 

Cargo.toml

| 

Cargo.toml

| 

| 

|
| 

LICENSE.txt

| 

LICENSE.txt

| 

| 

|
| 

NOTICE.txt

| 

NOTICE.txt

| 

| 

|
| 

README.md

| 

README.md

| 

| 

|
| 

header

| 

header

| 

| 

|
| 

pre-commit.sh

| 

pre-commit.sh

| 

| 

|
| 

rustfmt.toml

| 

rustfmt.toml

| 

| 

|
| 

View all files

|

Repository files navigation
---------------------------

Native Rust implementation of Apache Arrow and Apache Parquet
=============================================================

Welcome to the Rust implementation of Apache Arrow, the popular in-memory columnar format.

This repository contains the following crates:

| Crate | Description | Latest API Docs | README |
| --- | --- | --- | --- |
| `arrow` | Core functionality (memory layout, arrays, low level computations) | docs.rs | (README) |
| `arrow-flight` | Support for Arrow-Flight IPC protocol | docs.rs | (README) |
| `parquet` | Support for Parquet columnar file format | docs.rs | (README) |
| `parquet_derive` | A crate for deriving RecordWriter/RecordReader for arbitrary, simple structs | docs.rs | (README) |

The current development version the API documentation in this repo can be found here.

Note: previously the `object_store` crate was also part of this repository, but it has been moved to the arrow-rs-object-store repository

Release Versioning and Schedule
-------------------------------

The Arrow Rust project releases approximately monthly and follows Semantic Versioning.

Due to available maintainer and testing bandwidth, `arrow` crates (`arrow`, `arrow-flight`, etc.) are released on the same schedule with the same versions as the `parquet` and \[`parquet-derive`\] crates.

This crate releases every month. We release new major versions (with potentially breaking API changes) at most once a quarter, and release incremental minor versions in the intervening months. See ticket #5368 for more details.

To keep our maintenance burden down, we do regularly scheduled releases (major and minor) from the `main` branch. How we handle PRs with breaking API changes is described in the contributing guide.

Planned Release Schedule

| Approximate Date | Version | Notes |
| --- | --- | --- |
| Apr 2025 | `55.0.0` | Major, potentially breaking API changes |
| May 2025 | `55.1.0` | Minor, NO breaking API changes |
| June 2025 | `55.2.0` | Minor, NO breaking API changes |
| July 2025 | `56.0.0` | Major, potentially breaking API changes |

### Rust Version Compatibility Policy

arrow-rs, parquet and object\_store are built and tested with stable Rust, and will keep a rolling MSRV (minimum supported Rust version) that can only be updated in major releases on a need by basis (e.g. project dependencies bump their MSRV or a particular Rust feature is useful for us etc.). The new MSRV if selected will be at least 6 months old. The minor releases are guaranteed to have the same MSRV.

Note: If a Rust hotfix is released for the current MSRV, the MSRV will be updated to the specific minor version that includes all applicable hotfixes preceding other policies.

E.g.

in Apr 2025 we will release version 55.0.0 which might have a version bump. But the Rust version selected in this case will be at most version 1.81.

### Guidelines for `panic` vs `Result`

In general, use panics for bad states that are unreachable, unrecoverable or harmful. For those caused by invalid user input, however, we prefer to report that invalidity gracefully as an error result instead of panicking. In general, invalid input should result in an `Error` as soon as possible. It _is_ ok for code paths after validation to assume validation has already occurred and panic if not. See ticket #6737 for more nuances.

### Deprecation Guidelines

Minor releases may deprecate, but not remove APIs. Deprecating APIs allows downstream Rust programs to still compile, but generate compiler warnings. This gives downstream crates time to migrate prior to API removal.

To deprecate an API:

*   Mark the API as deprecated using `#[deprecated]` and specify the exact arrow-rs version in which it was deprecated
*   Concisely describe the preferred API to help the user transition

The deprecated version is the next version which will be released (please consult the list above). To mark the API as deprecated, use the `#[deprecated(since = "...", note = "...")]` attribute.

Foe example

#\[deprecated(since = "51.0.0", note = "Use \`date\_part\` instead")\]

In general, deprecated APIs will remain in the codebase for at least two major releases after they were deprecated (typically between 6 - 9 months later). For example, an API deprecated in `51.3.0` can be removed in `54.0.0` (or later). Deprecated APIs may be removed earlier or later than these guidelines at the discretion of the maintainers.

Related Projects
----------------

There are several related crates in different repositories

| Crate | Description | Documentation |
| --- | --- | --- |
| `object_store` | Object Storage (aws, azure, gcp, local, in-memory) interface | (README) |
| `datafusion` | In-memory query engine with SQL support | (README) |
| `ballista` | Distributed query execution | (README) |
| `parquet_opendal` | Use \[`opendal`\] for `parquet` Arrow IO | (README) |

Collectively, these crates support a wider array of functionality for analytic computations in Rust.

For example, you can write SQL queries or a `DataFrame` (using the `datafusion` crate) to read a parquet file (using the `parquet` crate), evaluate it in-memory using Arrow's columnar format (using the `arrow` crate), and send to another process (using the `arrow-flight` crate).

Generally speaking, the `arrow` crate offers functionality for using Arrow arrays, and `datafusion` offers most operations typically found in SQL, including `join`s and window functions.

You can find more details about each crate in their respective READMEs.

Arrow Rust Community
--------------------

The `[email protected]` mailing list serves as the core communication channel for the Arrow community. Instructions for signing up and links to the archives can be found on the Arrow Community page. All major announcements and communications happen there.

The Rust Arrow community also uses the official ASF Slack for informal discussions and coordination. This is a great place to meet other contributors and get guidance on where to contribute. Join us in the `#arrow-rust` channel and feel free to ask for an invite via:

1.  the `[email protected]` mailing list
2.  the GitHub Discussions
3.  the Discord channel

The Rust implementation uses GitHub issues as the system of record for new features and bug fixes and this plays a critical role in the release process.

For design discussions we generally use GitHub issues.

There is more information in the contributing guide.

About
-----

Official Rust implementation of Apache Arrow

arrow.apache.org/

### Topics

rust arrow parquet

### Resources

Readme

### License

Apache-2.0 license

### Code of conduct

Code of conduct

### Security policy

Security policy

### Uh oh!

There was an error while loading. Please reload this page.

Activity

Custom properties

### Stars

**3k** stars

### Watchers

**51** watching

### Forks

**960** forks

Report repository

Releases 15
-----------

arrow 55.2.0 Latest

Jun 22, 2025

\+ 14 releases

Packages 0
----------

No packages published  

Used by 7.9k
------------

\+ 7,925

Contributors 658
----------------

\+ 644 contributors

Languages
---------

*   Rust 99.4%
*   Other 0.6%

You can’t perform that action at this time.