# Struct JsonLineReader Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ndjson/core.rs.html#22" class="src">Source</a>

``` rust
pub struct JsonLineReader<'a, R>where
    R: MmapBytesReader,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#impl-JsonLineReader%3C&#39;a,+R%3E" class="anchor">§</a>

### impl\<'a, R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

where R: 'a + <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_n_rows" class="fn">with_n_rows</a>(self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_schema" class="fn">with_schema</a>(self, schema: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_schema_overwrite" class="fn">with_schema_overwrite</a>( self, schema: &'a <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_rechunk" class="fn">with_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_predicate" class="fn">with_predicate</a>( self, predicate: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/predicates/trait.PhysicalIoExpr.html" class="trait" title="trait polars_io::predicates::PhysicalIoExpr">PhysicalIoExpr</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_projection" class="fn">with_projection</a>( self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\]\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_row_index" class="fn">with_row_index</a>( self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a mut <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.infer_schema_len" class="fn">infer_schema_len</a>( self, infer_schema_len: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_n_threads" class="fn">with_n_threads</a>(self, n: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_path" class="fn">with_path</a>\<P\>(self, path: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_chunk_size" class="fn">with_chunk_size</a>( self, chunk_size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

Sets the chunk size used by the parser. This influences performance

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.low_memory" class="fn">low_memory</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

Reduce memory consumption at the expense of performance

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.with_ignore_errors" class="fn">with_ignore_errors</a>(self, ignore_errors: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'a, R\>

Set values as `Null` if parsing fails because of schema mismatches.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.count" class="fn">count</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#impl-JsonLineReader%3C&#39;_,+File%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.from_path" class="fn">from_path</a>\<P\>(path: P) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>,

This is the recommended way to create a json reader as this allows for fastest parsing.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#impl-SerReader%3CR%3E-for-JsonLineReader%3C&#39;_,+R%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html" class="trait" title="trait polars::prelude::SerReader">SerReader</a>\<R\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'\_, R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.new" class="fn">new</a>(reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html" class="struct" title="struct polars::prelude::JsonLineReader">JsonLineReader</a>\<'\_, R\>

Create a new JsonLineReader from a file/ stream

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take the SerReader and return a parsed DataFrame.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#method.set_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#method.set_rechunk" class="fn">set_rechunk</a>(self, \_rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Make sure that all columns are contiguous in memory by aggregating the chunks into a single array.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonLineReader.html#blanket-implementations" class="anchor">§</a>
