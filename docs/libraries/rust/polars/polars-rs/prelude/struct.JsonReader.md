# Struct JsonReader Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/json/mod.rs.html#212" class="src">Source</a>

``` rust
pub struct JsonReader<'a, R>where
    R: MmapBytesReader,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Reads JSON in one of the formats in [`JsonFormat`](https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html "enum polars::prelude::JsonFormat") into a DataFrame.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#impl-JsonReader%3C&#39;a,+R%3E" class="anchor">§</a>

### impl\<'a, R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.with_schema" class="fn">with_schema</a>(self, schema: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

Set the JSON file’s schema

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.with_schema_overwrite" class="fn">with_schema_overwrite</a>( self, schema: &'a <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

Overwrite parts of the inferred schema.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.infer_schema_len" class="fn">infer_schema_len</a>( self, max_records: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

Set the JSON reader to infer the schema of the file. Currently, this is only used when reading from [`JsonFormat::JsonLines`](https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#variant.JsonLines "variant polars::prelude::JsonFormat::JsonLines"), as [`JsonFormat::Json`](https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#variant.Json "variant polars::prelude::JsonFormat::Json") reads in the entire array anyway.

When using [`JsonFormat::JsonLines`](https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#variant.JsonLines "variant polars::prelude::JsonFormat::JsonLines"), `max_records = None` will read the entire buffer in order to infer the schema, `Some(1)` would look only at the first record, `Some(2)` the first two records, etc.

It is an error to pass `max_records = Some(0)`, as a schema cannot be inferred from 0 records when deserializing from JSON (unlike CSVs, there is no header row to inspect for column names).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.with_batch_size" class="fn">with_batch_size</a>(self, batch_size: <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

Set the batch size (number of records to load at one time)

This heavily influences loading time.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.with_projection" class="fn">with_projection</a>( self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

Set the reader’s column projection: the names of the columns to keep after deserialization. If `None`, all columns are kept.

Setting `projection` to the columns you want to keep is more efficient than deserializing all of the columns and then dropping the ones you don’t want.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.with_json_format" class="fn">with_json_format</a>(self, format: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html" class="enum" title="enum polars::prelude::JsonFormat">JsonFormat</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.with_ignore_errors" class="fn">with_ignore_errors</a>(self, ignore: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'a, R\>

Return a `null` if an error occurs during parsing.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#impl-SerReader%3CR%3E-for-JsonReader%3C&#39;_,+R%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html" class="trait" title="trait polars::prelude::SerReader">SerReader</a>\<R\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'\_, R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take the SerReader and return a parsed DataFrame.

Because JSON values specify their types (number, string, etc), no upcasting or conversion is performed between incompatible types in the input. In the event that a column contains mixed dtypes, is it unspecified whether an error is returned or whether elements of incompatible dtypes are replaced with `null`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.new" class="fn">new</a>(reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'\_, R\>

Create a new instance of the [`SerReader`](https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html "trait polars::prelude::SerReader")

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#method.set_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#method.set_rechunk" class="fn">set_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html" class="struct" title="struct polars::prelude::JsonReader">JsonReader</a>\<'\_, R\>

Make sure that all columns are contiguous in memory by aggregating the chunks into a single array.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonReader.html#blanket-implementations" class="anchor">§</a>
