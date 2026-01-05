# Struct PartitionTargetContext Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/sink.rs.html#198" class="src">Source</a>

``` rust
pub struct PartitionTargetContext {
    pub file_idx: usize,
    pub part_idx: usize,
    pub in_part_idx: usize,
    pub keys: Vec<PartitionTargetContextKey>,
    pub file_path: String,
    pub full_path: PlPath,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#structfield.file_idx" class="anchor field">§</a>`file_idx: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#structfield.part_idx" class="anchor field">§</a>`part_idx: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#structfield.in_part_idx" class="anchor field">§</a>`in_part_idx: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#structfield.keys" class="anchor field">§</a>`keys: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContextKey.html" class="struct" title="struct polars::prelude::PartitionTargetContextKey"><code>PartitionTargetContextKey</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#structfield.file_path" class="anchor field">§</a>`file_path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#structfield.full_path" class="anchor field">§</a>`full_path: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath"><code>PlPath</code></a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionTargetContext.html#blanket-implementations" class="anchor">§</a>
