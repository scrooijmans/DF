# Struct Logical Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/mod.rs.html#34" class="src">Source</a>

``` rust
pub struct Logical<Logical, Physical>where
    Logical: PolarsDataType,
    Physical: PolarsDataType,{
    pub phys: ChunkedArray<Physical>,
    pub dtype: DataType,
    /* private fields */
}
```

Expand description

Maps a logical type to a chunked array implementation of the physical type. This saves a lot of compiler bloat and allows us to reuse functionality.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<Physical>`<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.sort_with" class="fn">sort_with</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.sort" class="fn">sort</a>( &self, descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

Returned a sorted `ChunkedArray`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.arg_sort" class="fn">arg_sort</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Retrieve the indexes needed to sort this array.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.precision" class="fn">precision</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.scale" class="fn">scale</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.to_scale" class="fn">to_scale</a>( &self, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E-1" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_enum" class="fn">is_enum</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.uses_lexical_ordering" class="fn">uses_lexical_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return whether or not the [`CategoricalChunked`](https://docs.rs/polars/latest/polars/prelude/type.CategoricalChunked.html "type polars::prelude::CategoricalChunked") uses the lexical order of the string values when sorting.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.full_null_with_dtype" class="fn">full_null_with_dtype</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_cats_and_dtype" class="fn">from_cats_and_dtype</a>( cat_ids: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

Create a [`CategoricalChunked`](https://docs.rs/polars/latest/polars/prelude/type.CategoricalChunked.html "type polars::prelude::CategoricalChunked") from a physical array and dtype.

Checks that all the category ids are valid, mapping invalid ones to nulls.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_cats_and_dtype_unchecked" class="fn">from_cats_and_dtype_unchecked</a>( cat_ids: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

Create a [`CategoricalChunked`](https://docs.rs/polars/latest/polars/prelude/type.CategoricalChunked.html "type polars::prelude::CategoricalChunked") from a physical array and dtype.

##### <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#safety" class="doc-anchor">§</a>Safety

It’s not checked that the indices are in-bounds or that the dtype is correct.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_mapping" class="fn">get_mapping</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>

Get a reference to the mapping of categorical types to the string values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.iter_str" class="fn">iter_str</a>(&self) -\> impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIterator.html" class="trait" title="trait polars::prelude::PolarsIterator">PolarsIterator</a>

Create an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") that iterates over the `&str` values of the [`CategoricalChunked`](https://docs.rs/polars/latest/polars/prelude/type.CategoricalChunked.html "type polars::prelude::CategoricalChunked").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_str_iter" class="fn">from_str_iter</a>\<'a, I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, strings: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>,

Converts from strings to this CategoricalChunked.

If this dtype is an Enum any non-existing strings get mapped to null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.to_arrow" class="fn">to_arrow</a>( &self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/dictionary/struct.DictionaryArray.html" class="struct" title="struct polars_arrow::array::dictionary::DictionaryArray">DictionaryArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CK,+T%3E" class="anchor">§</a>

### impl\<K, T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>

where K: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>,

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new_logical" class="fn">new_logical</a>( phys: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>

##### <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#safety-1" class="doc-anchor">§</a>Safety

You must uphold the logical types’ invariants.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CK,+T%3E-1" class="anchor">§</a>

### impl\<K, T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>

where K: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.rename" class="fn">rename</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.has_nulls" class="fn">has_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_not_null" class="fn">is_not_null</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.field" class="fn">field</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.physical" class="fn">physical</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.physical_mut" class="fn">physical_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_physical" class="fn">into_physical</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.as_date_iter" class="fn">as_date_iter</a>(&self) -\> impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_naive_date" class="fn">from_naive_date</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>,

Construct a new [`DateChunked`](https://docs.rs/polars/latest/polars/prelude/type.DateChunked.html "type polars::prelude::DateChunked") from an iterator over [`NaiveDate`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html "struct chrono::naive::date::NaiveDate").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.to_string" class="fn">to_string</a>( &self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert from Date into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.strftime" class="fn">strftime</a>( &self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert from Date into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

Alias for `to_string`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_naive_date_options" class="fn">from_naive_date_options</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\>,

Construct a new [`DateChunked`](https://docs.rs/polars/latest/polars/prelude/type.DateChunked.html "type polars::prelude::DateChunked") from an iterator over optional [`NaiveDate`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html "struct chrono::naive::date::NaiveDate").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.as_datetime_iter" class="fn">as_datetime_iter</a>(&self) -\> impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.time_unit" class="fn">time_unit</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.time_zone" class="fn">time_zone</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.to_string-1" class="fn">to_string</a>( &self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert from Datetime into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.strftime-1" class="fn">strftime</a>( &self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert from Datetime into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

Alias for `to_string`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_naive_datetime" class="fn">from_naive_datetime</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>,

Construct a new [`DatetimeChunked`](https://docs.rs/polars/latest/polars/prelude/type.DatetimeChunked.html "type polars::prelude::DatetimeChunked") from an iterator over [`NaiveDateTime`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_naive_datetime_options" class="fn">from_naive_datetime_options</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_time_unit" class="fn">cast_time_unit</a>(&self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit"). And update the data accordingly.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.set_time_unit" class="fn">set_time_unit</a>(&mut self, time_unit: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit"). This does not modify the data.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.set_time_zone" class="fn">set_time_zone</a>(&mut self, time_zone: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Change the underlying [`TimeZone`](https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html "struct polars::prelude::TimeZone"). This does not modify the data. This does not validate the time zone - it’s up to the caller to verify that it’s already been validated.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.set_time_unit_and_time_zone" class="fn">set_time_unit_and_time_zone</a>( &mut self, time_unit: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, time_zone: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit") and [`TimeZone`](https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html "struct polars::prelude::TimeZone"). This does not modify the data. This does not validate the time zone - it’s up to the caller to verify that it’s already been validated.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.time_unit-1" class="fn">time_unit</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_time_unit-1" class="fn">cast_time_unit</a>(&self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit"). And update the data accordingly.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.set_time_unit-1" class="fn">set_time_unit</a>(&mut self, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

Change the underlying [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html "enum polars::prelude::TimeUnit"). This does not modify the data.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.to_string-2" class="fn">to_string</a>( &self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert from [`Duration`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Duration "variant polars::prelude::DataType::Duration") to String; note that `strftime` format strings are not supported, only the specifiers ‘iso’ and ‘polars’.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_duration" class="fn">from_duration</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>,

Construct a new [`DurationChunked`](https://docs.rs/polars/latest/polars/prelude/type.DurationChunked.html "type polars::prelude::DurationChunked") from an iterator over [`ChronoDuration`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_duration_options" class="fn">from_duration_options</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\>,

Construct a new [`DurationChunked`](https://docs.rs/polars/latest/polars/prelude/type.DurationChunked.html "type polars::prelude::DurationChunked") from an iterator over optional [`ChronoDuration`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.to_string-3" class="fn">to_string</a>(&self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Convert from Time into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.strftime-2" class="fn">strftime</a>(&self, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Convert from Time into String with the given format. See [chrono strftime/strptime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html).

Alias for `to_string`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.as_time_iter" class="fn">as_time_iter</a>(&self) -\> impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_naive_time" class="fn">from_naive_time</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>,

Construct a new [`TimeChunked`](https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html "type polars::prelude::TimeChunked") from an iterator over [`NaiveTime`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from_naive_time_options" class="fn">from_naive_time_options</a>\<I\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: I, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\>,

Construct a new [`TimeChunked`](https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html "type polars::prelude::TimeChunked") from an iterator over optional [`NaiveTime`](https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime").

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Add-for-%26Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.add" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>( self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, ) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkAggSeries-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAggSeries.html" class="trait" title="trait polars::prelude::ChunkAggSeries">ChunkAggSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAgg.html" class="trait" title="trait polars::prelude::ChunkAgg">ChunkAgg</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.min_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAggSeries.html#method.min_reduce" class="fn">min_reduce</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the min of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.max_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAggSeries.html#method.max_reduce" class="fn">max_reduce</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the max of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.sum_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAggSeries.html#method.sum_reduce" class="fn">sum_reduce</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the sum of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.prod_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkAggSeries.html#method.prod_reduce" class="fn">prod_reduce</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the product of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") as a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of length 1.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkCompareEq%3C%26ChunkedArray%3CStringType%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.equal-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.equal_missing-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.not_equal-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.not_equal_missing-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkCompareEq%3C%26Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.equal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.equal_missing" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.not_equal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.not_equal_missing" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkCompareEq%3C%26str%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.equal-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.equal_missing-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.not_equal-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.not_equal_missing-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkCompareIneq%3C%26ChunkedArray%3CStringType%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Item-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.gt-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.gt_eq-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.lt-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.lt_eq-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkCompareIneq%3C%26Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>: for\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.gt" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.gt_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.lt" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.lt_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-ChunkCompareIneq%3C%26str%3E-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Item-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.gt-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.gt_eq-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.lt-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.lt_eq-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>( &self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Clone-for-Logical%3CK,+T%3E" class="anchor">§</a>

### impl\<K, T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>

where K: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<K, T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-DateMethods-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html" class="trait" title="trait polars::prelude::DateMethods">DateMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.parse_from_str_slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#tymethod.parse_from_str_slice" class="fn">parse_from_str_slice</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.year" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.year" class="fn">year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract month from underlying NaiveDate representation. Returns the year number in the calendar date.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_leap_year" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.is_leap_year" class="fn">is_leap_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Extract year from underlying NaiveDate representation. Returns whether the year is a leap year.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.iso_year" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.iso_year" class="fn">iso_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

This year number might not match the calendar year number.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.quarter" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.quarter" class="fn">quarter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract month from underlying NaiveDateTime representation. Quarters range from 1 to 4.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.month" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.month" class="fn">month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract month from underlying NaiveDateTime representation. Returns the month number starting from 1. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.month)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.days_in_month" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.days_in_month" class="fn">days_in_month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the number of days in the month of the underlying NaiveDate representation.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.week" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.week" class="fn">week</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the ISO week number starting from 1. The return value ranges from 1 to 53. (The last week of year differs by years.)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.day" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.day" class="fn">day</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract day from underlying NaiveDate representation. Returns the day of month starting from 1. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.day)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.ordinal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.ordinal" class="fn">ordinal</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

Returns the day of year starting from 1. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.ordinal)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new_from_parts" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DateMethods.html#method.new_from_parts" class="fn">new_from_parts</a>( year: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, month: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, day: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a date ChunkedArray from individual time components.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-DatetimeMethods-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html" class="trait" title="trait polars::prelude::DatetimeMethods">DatetimeMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.year-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.year" class="fn">year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract month from underlying NaiveDateTime representation. Returns the year number in the calendar date.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_leap_year-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.is_leap_year" class="fn">is_leap_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Extract year from underlying NaiveDate representation. Returns whether the year is a leap year.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.iso_year-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.iso_year" class="fn">iso_year</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.quarter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.quarter" class="fn">quarter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract quarter from underlying NaiveDateTime representation. Quarters range from 1 to 4.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.month-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.month" class="fn">month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract month from underlying NaiveDateTime representation. Returns the month number starting from 1. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.month)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.days_in_month-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.days_in_month" class="fn">days_in_month</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the number of days in the month of the underlying NaiveDateTime representation.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.weekday" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.weekday" class="fn">weekday</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract ISO weekday from underlying NaiveDateTime representation. Returns the weekday number where monday = 1 and sunday = 7

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.week-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.week" class="fn">week</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Returns the ISO week number starting from 1. The return value ranges from 1 to 53. (The last week of year differs by years.)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.day-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.day" class="fn">day</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract day from underlying NaiveDateTime representation. Returns the day of month starting from 1. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.day)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.hour" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.hour" class="fn">hour</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract hour from underlying NaiveDateTime representation. Returns the hour number from 0 to 23.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.minute" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.minute" class="fn">minute</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract minute from underlying NaiveDateTime representation. Returns the minute number from 0 to 59.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.second" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.second" class="fn">second</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the second number from 0 to 59.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.nanosecond" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.nanosecond" class="fn">nanosecond</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.ordinal-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.ordinal" class="fn">ordinal</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>

Returns the day of year starting from 1. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.ordinal)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.parse_from_str_slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.parse_from_str_slice" class="fn">parse_from_str_slice</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, tu: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new_from_parts-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DatetimeMethods.html#method.new_from_parts" class="fn">new_from_parts</a>( year: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, month: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, day: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, hour: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, minute: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, second: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, nanosecond: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, ambiguous: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, time_unit: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, time_zone: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a datetime ChunkedArray from individual time components.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Div-for-%26Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.div" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>( self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, ) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-DurationMethods-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html" class="trait" title="trait polars::prelude::DurationMethods">DurationMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.hours" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.hours" class="fn">hours</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the hours from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.days" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.days" class="fn">days</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the days from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.minutes" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.minutes" class="fn">minutes</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the seconds from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.seconds" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.seconds" class="fn">seconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the seconds from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.milliseconds" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.milliseconds" class="fn">milliseconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the milliseconds from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.microseconds" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.microseconds" class="fn">microseconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the microseconds from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.nanoseconds" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DurationMethods.html#tymethod.nanoseconds" class="fn">nanoseconds</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Extract the nanoseconds from a `Duration`

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-From%3CLogical%3CDateType,+Int32Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-From%3CLogical%3CDatetimeType,+Int64Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-From%3CLogical%3CDurationType,+Int64Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-From%3CLogical%3CTimeType,+Int64Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-IntoSeries-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_series-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_series-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-IntoSeries-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_series-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_series-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-IntoSeries-for-Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_series-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_series-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-IntoSeries-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_series-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_series-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-IntoSeries-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-IntoSeries-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.into_series-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.is_series-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-LogicalType-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_with_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-LogicalType-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.dtype-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_with_options-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-LogicalType-for-Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.dtype-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value_unchecked-2" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_with_options-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-LogicalType-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.dtype-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value_unchecked-3" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_with_options-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-LogicalType-for-Logical%3CT,+%3CT+as+PolarsCategoricalType%3E::PolarsPhysical%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.dtype-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value_unchecked-4" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_with_options-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-LogicalType-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html" class="trait" title="trait polars::prelude::LogicalType">LogicalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.dtype-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &'static <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get data type of [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value" class="fn">get_any_value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Gets [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") from [`LogicalType`](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html "trait polars::prelude::LogicalType")

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.get_any_value_unchecked-5" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked" class="fn">get_any_value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.get_any_value_unchecked)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast_with_options-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#tymethod.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, cast_options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.cast-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LogicalType.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Mul-for-%26Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.mul" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>( self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, ) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BNaiveDate%5D%3E-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BNaiveDateTime%5D%3E-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BNaiveTime%5D%3E-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDate%3E%5D%3E-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDateTime%3E%5D%3E-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveTime%3E%5D%3E-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BOption%3CTimeDelta%3E%5D%3E-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-NamedFrom%3CT,+%5BTimeDelta%5D%3E-for-Logical%3CDurationType,+Int64Type%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.new-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-PolarsRound-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html" class="trait" title="trait polars::prelude::PolarsRound">PolarsRound</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.round-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#tymethod.round" class="fn">round</a>( &self, every: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, \_tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-PolarsRound-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html" class="trait" title="trait polars::prelude::PolarsRound">PolarsRound</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.round" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsRound.html#tymethod.round" class="fn">round</a>( &self, every: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-PolarsTruncate-for-Logical%3CDateType,+Int32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTruncate.html" class="trait" title="trait polars::prelude::PolarsTruncate">PolarsTruncate</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.truncate-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTruncate.html#tymethod.truncate" class="fn">truncate</a>( &self, \_tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, every: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-PolarsTruncate-for-Logical%3CDatetimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTruncate.html" class="trait" title="trait polars::prelude::PolarsTruncate">PolarsTruncate</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.truncate" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTruncate.html#tymethod.truncate" class="fn">truncate</a>( &self, tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>\>, every: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-Sub-for-%26Logical%3CDecimalType,+Int128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>( self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, ) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#impl-TimeMethods-for-Logical%3CTimeType,+Int64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html" class="trait" title="trait polars::prelude::TimeMethods">TimeMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.hour-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html#tymethod.hour" class="fn">hour</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract hour from underlying NaiveDateTime representation. Returns the hour number from 0 to 23.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.minute-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html#tymethod.minute" class="fn">minute</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract minute from underlying NaiveDateTime representation. Returns the minute number from 0 to 59.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.second-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html#tymethod.second" class="fn">second</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the second number from 0 to 59.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.nanosecond-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html#tymethod.nanosecond" class="fn">nanosecond</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

Extract second from underlying NaiveDateTime representation. Returns the number of nanoseconds since the whole non-leap second. The range from 1,000,000,000 to 1,999,999,999 represents the leap second.

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#method.parse_from_str_slice-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TimeMethods.html#tymethod.parse_from_str_slice" class="fn">parse_from_str_slice</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\], fmt: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/struct.Logical.html#blanket-implementations" class="anchor">§</a>
