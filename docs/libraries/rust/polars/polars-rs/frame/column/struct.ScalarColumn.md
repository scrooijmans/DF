# Struct ScalarColumn Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/column/scalar.rs.html#13" class="src">Source</a>

``` rust
pub struct ScalarColumn { /* private fields */ }
```

Expand description

A [`Column`](https://docs.rs/polars/latest/polars/prelude/enum.Column.html "enum polars::prelude::Column") that consists of a repeated [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar")

This is lazily materialized into a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-ScalarColumn" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, scalar: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.new_empty" class="fn">new_empty</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.full_null" class="fn">full_null</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.scalar" class="fn">scalar</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.to_series" class="fn">to_series</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Materialize the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") into a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.lazy_as_materialized_series" class="fn">lazy_as_materialized_series</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

Get the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") as [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") if it was already materialized.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.as_materialized_series" class="fn">as_materialized_series</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Get the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") as [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series")

This needs to materialize upon the first call. Afterwards, this is cached.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.take_materialized_series" class="fn">take_materialized_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") and materialize as a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") if not already done.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.as_single_value_series" class="fn">as_single_value_series</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") as a series with a single value.

If the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") has `length=0` the resulting `Series` will also have `length=0`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.as_n_values_series" class="fn">as_n_values_series</a>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") as a series with a `n` values.

If the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") has `length=0` the resulting `Series` will also have `length=0`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.unit_scalar_from_series" class="fn">unit_scalar_from_series</a>(series: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

Create a new [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") from a `length=1` Series and expand it `length`.

This will panic if the value cannot be made static or if the series has length `0`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.from_single_value_series" class="fn">from_single_value_series</a>(series: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

Create a new [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") from a `length<=1` Series and expand it `length`.

If `series` is empty and `length` is non-zero, a full-NULL column of `length` will be returned.

This will panic if the value cannot be made static.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.resize" class="fn">resize</a>(&self, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

Resize the [`ScalarColumn`](https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html "struct polars::frame::column::ScalarColumn") to new `length`.

This reuses the materialized [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series"), if `length <= self.length`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.strict_cast" class="fn">strict_cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.cast_unchecked" class="fn">cast_unchecked</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

##### <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#safety" class="doc-anchor">§</a>Safety

This can lead to invalid memory access in downstream code.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.rename" class="fn">rename</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> &mut <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.has_nulls" class="fn">has_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.drop_nulls" class="fn">drop_nulls</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.into_nulls" class="fn">into_nulls</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.map_scalar" class="fn">map_scalar</a>(&mut self, map_scalar: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.with_value" class="fn">with_value</a>(&mut self, value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>) -\> &mut <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-Clone-for-ScalarColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-Debug-for-ScalarColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-Deserialize%3C&#39;de%3E-for-ScalarColumn" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>( deserializer: D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>, \<D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-From%3CScalarColumn%3E-for-Column" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-IntoColumn-for-ScalarColumn" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.into_column" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html#tymethod.into_column" class="fn">into_column</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-Serialize-for-ScalarColumn" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>( &self, serializer: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#impl-TryFrom%3CSerializeWrap%3E-for-ScalarColumn" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<SerializeWrap\> for <a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( value: SerializeWrap, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a>, \<<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html" class="struct" title="struct polars::frame::column::ScalarColumn">ScalarColumn</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<SerializeWrap\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/column/struct.ScalarColumn.html#blanket-implementations" class="anchor">§</a>
