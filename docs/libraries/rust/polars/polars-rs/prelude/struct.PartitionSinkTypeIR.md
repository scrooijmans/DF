# Struct PartitionSinkTypeIR Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/sink.rs.html#488" class="src">Source</a>

``` rust
pub struct PartitionSinkTypeIR {
    pub base_path: Arc<PlPath>,
    pub file_path_cb: Option<PartitionTargetCallback>,
    pub file_type: FileType,
    pub sink_options: SinkOptions,
    pub variant: PartitionVariantIR,
    pub cloud_options: Option<CloudOptions>,
    pub per_partition_sort_by: Option<Vec<SortColumnIR>>,
    pub finish_callback: Option<SinkFinishCallback>,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.base_path" class="anchor field">§</a>`base_path: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath"><code>PlPath</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.file_path_cb" class="anchor field">§</a>`file_path_cb: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallback.html" class="enum" title="enum polars::prelude::PartitionTargetCallback"><code>PartitionTargetCallback</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.file_type" class="anchor field">§</a>`file_type: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.FileType.html" class="enum" title="enum polars::prelude::FileType"><code>FileType</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.sink_options" class="anchor field">§</a>`sink_options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions"><code>SinkOptions</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.variant" class="anchor field">§</a>`variant: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariantIR.html" class="enum" title="enum polars::prelude::PartitionVariantIR"><code>PartitionVariantIR</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.cloud_options" class="anchor field">§</a>`cloud_options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions"><code>CloudOptions</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.per_partition_sort_by" class="anchor field">§</a>`per_partition_sort_by: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortColumnIR.html" class="struct" title="struct polars::prelude::SortColumnIR"><code>SortColumnIR</code></a>`>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#structfield.finish_callback" class="anchor field">§</a>`finish_callback: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkFinishCallback.html" class="enum" title="enum polars::prelude::SinkFinishCallback"><code>SinkFinishCallback</code></a>`>`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#impl-Clone-for-PartitionSinkTypeIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html" class="struct" title="struct polars::prelude::PartitionSinkTypeIR">PartitionSinkTypeIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html" class="struct" title="struct polars::prelude::PartitionSinkTypeIR">PartitionSinkTypeIR</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#impl-Debug-for-PartitionSinkTypeIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html" class="struct" title="struct polars::prelude::PartitionSinkTypeIR">PartitionSinkTypeIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#impl-PartialEq-for-PartitionSinkTypeIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html" class="struct" title="struct polars::prelude::PartitionSinkTypeIR">PartitionSinkTypeIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html" class="struct" title="struct polars::prelude::PartitionSinkTypeIR">PartitionSinkTypeIR</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#impl-StructuralPartialEq-for-PartitionSinkTypeIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html" class="struct" title="struct polars::prelude::PartitionSinkTypeIR">PartitionSinkTypeIR</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.PartitionSinkTypeIR.html#blanket-implementations" class="anchor">§</a>
