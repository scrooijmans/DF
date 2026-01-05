Apache Arrow Official Native Rust Implementation
crates.io docs.rs

This crate contains the official Native Rust implementation of Apache Arrow in memory format, governed by the Apache Software Foundation.

The API documentation contains examples and full API. There are several examples to start from as well.

The API documentation for most recent, unreleased code is available here.

Rust Version Compatibility
This crate is tested with the latest stable version of Rust. We do not currently test against other, older versions.

Versioning / Releases
The arrow crate follows the SemVer standard defined by Cargo and works well within the Rust crate ecosystem. See the repository README for more details on the release schedule, version and deprecation policy.

Note that for historical reasons, this crate uses versions with major numbers greater than 0.x (e.g. 19.0.0), unlike many other crates in the Rust ecosystem which spend extended time releasing versions 0.x to signal planned ongoing API changes. Minor arrow releases contain only compatible changes, while major releases may contain breaking API changes.

Feature Flags
The arrow crate provides the following features which may be enabled in your Cargo.toml:

csv (default) - support for reading and writing Arrow arrays to/from csv files
json (default) - support for reading and writing Arrow array to/from json files
ipc (default) - support for reading Arrow IPC Format, also used as the wire protocol in arrow-flight
ipc_compression - Enables reading and writing compressed IPC streams (also enables ipc)
prettyprint - support for formatting record batches as textual columns implementations of some compute
chrono-tz - support of parsing timezone using chrono-tz
ffi - bindings for the Arrow C C Data Interface
pyarrow - bindings for pyo3 to call arrow-rs from python
canonical_extension_types - definitions for canonical extension types
Arrow Feature Status
The Apache Arrow Status page lists which features of Arrow this crate supports.

Safety
Arrow seeks to uphold the Rust Soundness Pledge as articulated eloquently here. Specifically:

The intent of this crate is to be free of soundness bugs. The developers will do their best to avoid them, and welcome help in analyzing and fixing them

Where soundness in turn is defined as:

Code is unable to trigger undefined behavior using safe APIs

One way to ensure this would be to not use unsafe, however, as described in the opening chapter of the Rustonomicon this is not a requirement, and flexibility in this regard is one of Rust's great strengths.

In particular there are a number of scenarios where unsafe is largely unavoidable:

Invariants that cannot be statically verified by the compiler and unlock non-trivial performance wins, e.g. values in a StringArray are UTF-8, TrustedLen iterators, etc...
FFI
Additionally, this crate exposes a number of unsafe APIs, allowing downstream crates to explicitly opt-out of potentially expensive invariant checking where appropriate.

We have a number of strategies to help reduce this risk:

Provide strongly-typed Array and ArrayBuilder APIs to safely and efficiently interact with arrays
Extensive validation logic to safely construct ArrayData from untrusted sources
All commits are verified using MIRI to detect undefined behaviour
Use a force_validate feature that enables additional validation checks for use in test/debug builds
There is ongoing work to reduce and better document the use of unsafe, and we welcome contributions in this space
Building for WASM
Arrow can compile to WebAssembly using the wasm32-unknown-unknown and wasm32-wasi targets.

In order to compile Arrow for wasm32-unknown-unknown you will need to disable default features, then include the desired features, but exclude test dependencies (the test_utils feature). For example, use this snippet in your Cargo.toml:

[dependencies]
arrow = { version = "5.0", default-features = false, features = ["csv", "ipc"] }
Examples
The examples folder shows how to construct some different types of Arrow arrays, including dynamic arrays:

Examples can be run using the cargo run --example command. For example:

cargo run --example builders
cargo run --example dynamic_types
cargo run --example read_csv
Performance Tips
Arrow aims to be as fast as possible out of the box, whilst not compromising on safety. However, it relies heavily on LLVM auto-vectorisation to achieve this. Unfortunately the LLVM defaults, particularly for x86_64, favour portability over performance, and LLVM will consequently avoid using more recent instructions that would result in errors on older CPUs.

To address this it is recommended that you override the LLVM defaults either by setting the RUSTFLAGS environment variable, or by setting rustflags in your Cargo configuration

Enable all features supported by the current CPU

RUSTFLAGS="-C target-cpu=native"
Enable all features supported by the current CPU, and enable full use of AVX512

RUSTFLAGS="-C target-cpu=native -C target-feature=-prefer-256-bit"
Enable all features supported by CPUs more recent than haswell (2013)

RUSTFLAGS="-C target-cpu=haswell"
For a full list of features and target CPUs use

$ rustc --print target-cpus
$ rustc --print target-features
