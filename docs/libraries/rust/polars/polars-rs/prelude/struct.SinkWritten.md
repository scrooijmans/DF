# Struct SinkWritten Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/sink.rs.html#280" class="src">Source</a>

``` rust
pub struct SinkWritten {
    pub file_idx: usize,
    pub part_idx: usize,
    pub in_part_idx: usize,
    pub keys: Vec<PartitionTargetContextKey>,
    pub file_path: PathBuf,
    pub full_path: PathBuf,
    pub num_rows: usize,
    pub file_size: usize,
    pub gathered: Option<DataFrame>,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.file_idx" class="anchor field">§</a>`file_idx: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.part_idx" class="anchor field">§</a>`part_idx: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.in_part_idx" class="anchor field">§</a>`in_part_idx: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.keys" class="anchor field">§</a>`keys: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContextKey.html" class="struct" title="struct polars::prelude::PartitionTargetContextKey"><code>PartitionTargetContextKey</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.file_path" class="anchor field">§</a>`file_path: `<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf"><code>PathBuf</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.full_path" class="anchor field">§</a>`full_path: `<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf"><code>PathBuf</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.num_rows" class="anchor field">§</a>`num_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.file_size" class="anchor field">§</a>`file_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#structfield.gathered" class="anchor field">§</a>`gathered: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame"><code>DataFrame</code></a>`>`

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkWritten.html#blanket-implementations" class="anchor">§</a>
