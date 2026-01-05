<a href="https://docs.rs/" class="pure-menu-heading pure-menu-link docsrs-logo" aria-label="Docs.rs"> Docs.rs</a>

- 
- <a href="https://docs.rs/crate/opendal/0.54.1" class="pure-menu-link crate-name" title="Apache OpenDAL™: One Layer, All Storage."> opendal-0.54.1</a>

<!-- -->

- <a href="https://docs.rs/crate/opendal/0.54.1/builds/2582519#" class="pure-menu-link" aria-label="docs.rs">docs.rs</a>
  - <a href="https://docs.rs/about" class="pure-menu-link"> About docs.rs</a>
  - <a href="https://docs.rs/about/badges" class="pure-menu-link"> Badges</a>
  - <a href="https://docs.rs/about/builds" class="pure-menu-link"> Builds</a>
  - <a href="https://docs.rs/about/metadata" class="pure-menu-link"> Metadata</a>
  - <a href="https://docs.rs/about/redirections" class="pure-menu-link"> Shorthand URLs</a>
  - <a href="https://docs.rs/about/download" class="pure-menu-link"> Download</a>
  - <a href="https://docs.rs/about/rustdoc-json" class="pure-menu-link"> Rustdoc JSON</a>
  - <a href="https://docs.rs/releases/queue" class="pure-menu-link"> Build queue</a>
  - <a href="https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs" class="pure-menu-link" target="_blank"> Privacy policy</a>

<!-- -->

- <a href="https://docs.rs/crate/opendal/0.54.1/builds/2582519#" class="pure-menu-link" aria-label="Rust">Rust</a>
  - <a href="https://www.rust-lang.org/" class="pure-menu-link" target="_blank">Rust website</a>
  - <a href="https://doc.rust-lang.org/book/" class="pure-menu-link" target="_blank">The Book</a>
  - <a href="https://doc.rust-lang.org/std/" class="pure-menu-link" target="_blank">Standard Library API Reference</a>
  - <a href="https://doc.rust-lang.org/rust-by-example/" class="pure-menu-link" target="_blank">Rust by Example</a>
  - <a href="https://doc.rust-lang.org/cargo/guide/" class="pure-menu-link" target="_blank">The Cargo Guide</a>
  - <a href="https://doc.rust-lang.org/nightly/clippy" class="pure-menu-link" target="_blank">Clippy Documentation</a>

# opendal 0.54.1 

Apache OpenDAL™: One Layer, All Storage.

- <a href="https://docs.rs/crate/opendal/0.54.1" class="pure-menu-link">  Crate</a>
- <a href="https://docs.rs/crate/opendal/0.54.1/source/" class="pure-menu-link">  Source</a>
- <a href="https://docs.rs/crate/opendal/0.54.1/builds" class="pure-menu-link pure-menu-active">  Builds</a>
- <a href="https://docs.rs/crate/opendal/0.54.1/features" class="pure-menu-link"> Feature flags</a>

**Build \#2582519 2025-10-13 03:27:21**

Build failed. If you want to re-trigger a documentation build, you can do it [here](https://crates.io/crates/opendal/0.54.1/rebuild-docs). You can find more information on **docs.rs** builds documentation on the [builds page](https://docs.rs/about/builds).

- <a href="https://docs.rs/crate/opendal/0.54.1/builds/2582519/x86_64-unknown-linux-gnu.txt" class="release"></a>
  **x86_64-unknown-linux-gnu.txt**
- <a href="https://docs.rs/crate/opendal/0.54.1/builds/2582519/x86_64-unknown-linux-gnu_json.txt" class="release"></a>
  x86_64-unknown-linux-gnu_json.txt

<!-- -->

    # rustc version
    rustc 1.92.0-nightly (2300c2aef 2025-10-12)# docs.rs version
    docsrs 0.6.0 (f42173de 2025-09-12)# build log
    [INFO] running `Command { std: "docker" "create" "-v" "/home/cratesfyi/workspace/builds/opendal-0.54.1/target:/opt/rustwide/target:rw,Z" "-v" "/home/cratesfyi/workspace/builds/opendal-0.54.1/source:/opt/rustwide/workdir:ro,Z" "-v" "/home/cratesfyi/workspace/cargo-home:/opt/rustwide/cargo-home:ro,Z" "-v" "/home/cratesfyi/workspace/rustup-home:/opt/rustwide/rustup-home:ro,Z" "-e" "SOURCE_DIR=/opt/rustwide/workdir" "-e" "CARGO_TARGET_DIR=/opt/rustwide/target" "-e" "DOCS_RS=1" "-e" "CARGO_HOME=/opt/rustwide/cargo-home" "-e" "RUSTUP_HOME=/opt/rustwide/rustup-home" "-w" "/opt/rustwide/workdir" "-m" "6442450944" "--cpus" "6" "--user" "1001:1001" "--network" "none" "ghcr.io/rust-lang/crates-build-env/linux@sha256:e90291280db7d1fac5b66fc6dad9f9662629e7365a55743daf9bdf73ebc4ea79" "/opt/rustwide/cargo-home/bin/cargo" "+nightly" "rustdoc" "--lib" "-Zrustdoc-map" "--all-features" "--config" "build.rustdocflags=[\"--cfg\", \"docsrs\", \"-Z\", \"unstable-options\", \"--emit=invocation-specific\", \"--resource-suffix\", \"-20251012-1.92.0-nightly-2300c2aef\", \"--static-root-path\", \"/-/rustdoc.static/\", \"--cap-lints\", \"warn\", \"--extern-html-root-takes-precedence\"]" "--offline" "-Zunstable-options" "--config=doc.extern-map.registries.crates-io=\"https://docs.rs/{pkg_name}/{version}/x86_64-unknown-linux-gnu\"" "-Zrustdoc-scrape-examples" "-j6" "--target" "x86_64-unknown-linux-gnu", kill_on_drop: false }`
    [INFO] [stderr] WARNING: Your kernel does not support swap limit capabilities or the cgroup is not mounted. Memory limited without swap.
    [INFO] [stdout] 70b483b87e04eb361da05e6569d3586f55eb1093664c3708f36612434ca2b961
    [INFO] running `Command { std: "docker" "start" "-a" "70b483b87e04eb361da05e6569d3586f55eb1093664c3708f36612434ca2b961", kill_on_drop: false }`
    [INFO] [stderr] warning: target filter specified, but no targets matched; this is a no-op
    [INFO] [stderr]  Documenting opendal v0.54.1 (/opt/rustwide/workdir)
    [INFO] [stderr] error[E0557]: feature has been removed
    [INFO] [stderr]   --> src/lib.rs:21:29
    [INFO] [stderr]    |
    [INFO] [stderr] 21 | #![cfg_attr(docsrs, feature(doc_auto_cfg))]
    [INFO] [stderr]    |                             ^^^^^^^^^^^^ feature has been removed
    [INFO] [stderr]    |
    [INFO] [stderr]    = note: removed in CURRENT_RUSTC_VERSION; see <https://github.com/rust-lang/rust/pull/138907> for more information
    [INFO] [stderr]    = note: merged into `doc_cfg`
    [INFO] [stderr] 
    [INFO] [stderr] error: Compilation failed, aborting rustdoc
    [INFO] [stderr] 
    [INFO] [stderr] For more information about this error, try `rustc --explain E0557`.
    [INFO] [stderr] error: could not document `opendal`
    [INFO] running `Command { std: "docker" "inspect" "70b483b87e04eb361da05e6569d3586f55eb1093664c3708f36612434ca2b961", kill_on_drop: false }`
    [INFO] running `Command { std: "docker" "rm" "-f" "70b483b87e04eb361da05e6569d3586f55eb1093664c3708f36612434ca2b961", kill_on_drop: false }`
    [INFO] [stdout] 70b483b87e04eb361da05e6569d3586f55eb1093664c3708f36612434ca2b961
