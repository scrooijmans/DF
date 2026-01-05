# Struct RecordBatchOptions Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/record_batch.rs.html#790" class="src">Source</a>

``` rust
#[non_exhaustive]pub struct RecordBatchOptions {
    pub match_field_names: bool,
    pub row_count: Option<usize>,
}
```

Expand description

Options that control the behaviour used when creating a [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

## Fields (Non-exhaustive)<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#fields" class="anchor">§</a>

This struct is marked as non-exhaustive

Non-exhaustive structs could have additional fields added in future. Therefore, non-exhaustive structs cannot be constructed in external crates using the traditional `Struct { .. }` syntax; cannot be matched against without a wildcard `..`; and struct update syntax will not work.

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#structfield.match_field_names" class="anchor field">§</a>`match_field_names: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Match field names of structs and lists. If set to `true`, the names must match.

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#structfield.row_count" class="anchor field">§</a>`row_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Optional row count, useful for specifying a row count for a RecordBatch with no columns

## Implementations<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#impl-RecordBatchOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

Creates a new `RecordBatchOptions`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#method.with_row_count" class="fn">with_row_count</a>(self, row_count: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

Sets the row_count of RecordBatchOptions and returns self

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#method.with_match_field_names" class="fn">with_match_field_names</a>( self, match_field_names: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

Sets the match_field_names of RecordBatchOptions and returns self

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#impl-Debug-for-RecordBatchOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#impl-Default-for-RecordBatchOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatchOptions.html#blanket-implementations" class="anchor">§</a>
