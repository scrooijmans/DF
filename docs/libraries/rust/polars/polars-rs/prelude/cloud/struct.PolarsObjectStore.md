# Struct PolarsObjectStore Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/polars_object_store.rs.html#37" class="src">Source</a>

``` rust
pub struct PolarsObjectStore { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Polars wrapper around [`ObjectStore`](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") functionality. This struct is cheaply cloneable.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#impl-PolarsObjectStore" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html" class="struct" title="struct polars::prelude::cloud::PolarsObjectStore">PolarsObjectStore</a>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.to_dyn_object_store" class="fn">to_dyn_object_store</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>

Gets the underlying [`ObjectStore`](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementation.

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.rebuild_inner" class="fn">rebuild_inner</a>( &self, from_version: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.try_exec_rebuild_on_err" class="fn">try_exec_rebuild_on_err</a>\<Fn, Fut, O\>( &self, func: Fn, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<O, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where Fn: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>) -\> Fut, Fut: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<O, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#impl-PolarsObjectStore-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html" class="struct" title="struct polars::prelude::cloud::PolarsObjectStore">PolarsObjectStore</a>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.get_range" class="fn">get_range</a>( &self, path: &<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.get_ranges_sort" class="fn">get_ranges_sort</a>( &self, path: &<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ranges: &mut \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/mmap/private/struct.MemSlice.html" class="struct" title="struct polars_utils::mmap::private::MemSlice">MemSlice</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Fetch byte ranges into a HashMap keyed by the range start. This will mutably sort the `ranges` slice for coalescing.

##### <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#panics" class="doc-anchor">§</a>Panics

Panics if the same range start is used by more than 1 range.

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.download" class="fn">download</a>( &self, path: &<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, file: &mut <a href="https://docs.rs/tokio/1.46.1/x86_64-unknown-linux-gnu/tokio/fs/file/struct.File.html" class="struct" title="struct tokio::fs::file::File">File</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.head" class="fn">head</a>(&self, path: &<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Fetch the metadata of the parquet file, do not memoize it.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#impl-Clone-for-PolarsObjectStore" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html" class="struct" title="struct polars::prelude::cloud::PolarsObjectStore">PolarsObjectStore</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html" class="struct" title="struct polars::prelude::cloud::PolarsObjectStore">PolarsObjectStore</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#impl-Debug-for-PolarsObjectStore" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html" class="struct" title="struct polars::prelude::cloud::PolarsObjectStore">PolarsObjectStore</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html#blanket-implementations" class="anchor">§</a>
