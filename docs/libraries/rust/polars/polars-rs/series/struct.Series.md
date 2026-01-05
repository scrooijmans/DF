# Struct Series Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/mod.rs.html#147" class="src">Source</a>

``` rust
pub struct Series(pub Arc<dyn SeriesTrait>);
```

Expand description

## <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#series" class="doc-anchor">§</a>Series

The columnar data type for a DataFrame.

Most of the available functions are defined in the [SeriesTrait trait](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html "trait polars::prelude::SeriesTrait").

The `Series` struct consists of typed [ChunkedArray](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")’s. To quickly cast a `Series` to a `ChunkedArray` you can call the method with the name of the type:

``` rust
let s: Series = [1, 2, 3].iter().collect();
// Quickly obtain the ChunkedArray wrapped by the Series.
let chunked_array = s.i32().unwrap();
```

### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#arithmetic" class="doc-anchor">§</a>Arithmetic

You can do standard arithmetic on series.

``` rust
let s = Series::new("a".into(), [1 , 2, 3]);
let out_add = &s + &s;
let out_sub = &s - &s;
let out_div = &s / &s;
let out_mul = &s * &s;
```

Or with series and numbers.

``` rust
let s: Series = (1..3).collect();
let out_add_one = &s + 1;
let out_multiply = &s * 10;

// Could not overload left hand side operator.
let out_divide = 1.div(&s);
let out_add = 1.add(&s);
let out_subtract = 1.sub(&s);
let out_multiply = 1.mul(&s);
```

### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#comparison" class="doc-anchor">§</a>Comparison

You can obtain boolean mask by comparing series.

``` rust
let s = Series::new("dollars".into(), &[1, 2, 3]);
let mask = s.equal(1).unwrap();
let valid = [true, false, false].iter();
assert!(mask
    .into_iter()
    .map(|opt_bool| opt_bool.unwrap()) // option, because series can be null
    .zip(valid)
    .all(|(a, b)| a == *b))
```

See all the comparison operators in the [ChunkCompareEq trait](https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html "trait polars::prelude::ChunkCompareEq") and [ChunkCompareIneq trait](https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html "trait polars::prelude::ChunkCompareIneq").

### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#iterators" class="doc-anchor">§</a>Iterators

The Series variants contain differently typed [ChunkedArray](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")s. These structs can be turned into iterators, making it possible to use any function/ closure you want on a Series.

These iterators return an `Option<T>` because the values of a series may be null.

``` rust
use polars_core::prelude::*;
let pi = 3.14;
let s = Series::new("angle".into(), [2f32 * pi, pi, 1.5 * pi].as_ref());
let s_cos: Series = s.f32()
                    .expect("series was not an f32 dtype")
                    .into_iter()
                    .map(|opt_angle| opt_angle.map(|angle| angle.cos()))
                    .collect();
```

### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#creation" class="doc-anchor">§</a>Creation

Series can be create from different data structures. Below we’ll show a few ways we can create a Series object.

``` rust
// Series can be created from Vec's, slices and arrays
Series::new("boolean series".into(), &[true, false, true]);
Series::new("int series".into(), &[1, 2, 3]);
// And can be nullable
Series::new("got nulls".into(), &[Some(1), None, Some(2)]);

// Series can also be collected from iterators
let from_iter: Series = (0..10)
    .into_iter()
    .collect();
```

## Tuple Fields<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait"><code>SeriesTrait</code></a>`>`

## Implementations<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.fill_null" class="fn">fill_null</a>( &self, strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.FillNullStrategy.html" class="enum" title="enum polars::prelude::FillNullStrategy">FillNullStrategy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Replace None values with one of the following strategies:

- Forward fill (replace None with the previous value)
- Backward fill (replace None with the next value)
- Mean fill (replace None with the mean of the whole array)
- Min fill (replace None with the minimum of the whole array)
- Max fill (replace None with the maximum of the whole array)
- Zero fill (replace None with the value zero)
- One fill (replace None with the value one)

*NOTE: If you want to fill the Nones with a value use the [`fill_null` operation on `ChunkedArray<T>`](https://docs.rs/polars/latest/polars/prelude/trait.ChunkFillNullValue.html "trait polars::prelude::ChunkFillNullValue")*.

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#example" class="doc-anchor">§</a>Example

``` rust
fn example() -> PolarsResult<()> {
    let s = Column::new("some_missing".into(), &[Some(1), None, Some(2)]);

    let filled = s.fill_null(FillNullStrategy::Forward(None))?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(1), Some(2)]);

    let filled = s.fill_null(FillNullStrategy::Backward(None))?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(2), Some(2)]);

    let filled = s.fill_null(FillNullStrategy::Min)?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(1), Some(2)]);

    let filled = s.fill_null(FillNullStrategy::Max)?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(2), Some(2)]);

    let filled = s.fill_null(FillNullStrategy::Mean)?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(1), Some(2)]);

    let filled = s.fill_null(FillNullStrategy::Zero)?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(0), Some(2)]);

    let filled = s.fill_null(FillNullStrategy::One)?;
    assert_eq!(Vec::from(filled.i32()?), &[Some(1), Some(1), Some(2)]);

    Ok(())
}
example();
```

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sample_n" class="fn">sample_n</a>( &self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sample_frac" class="fn">sample_frac</a>( &self, frac: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sample a fraction between 0.0-1.0 of this [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.shuffle" class="fn">shuffle</a>(&self, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.fmt_list" class="fn">fmt_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-3" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.serialize_into_writer" class="fn">serialize_into_writer</a>( &self, writer: &mut dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.serialize_to_bytes" class="fn">serialize_to_bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.deserialize_from_reader" class="fn">deserialize_from_reader</a>( reader: &mut dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-4" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_any_values" class="fn">from_any_values</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, values: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\], strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") from a slice of AnyValues.

The data type of the resulting Series is determined by the `values` and the `strict` parameter:

- If `strict` is `true`, the data type is equal to the data type of the first non-null value. If any other non-null values do not match this data type, an error is raised.
- If `strict` is `false`, the data type is the supertype of the `values`. An error is returned if no supertype can be determined. **WARNING**: A full pass over the values is required to determine the supertype.
- If no values were passed, the resulting data type is `Null`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_any_values_and_dtype" class="fn">from_any_values_and_dtype</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, values: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\], dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") with the given `dtype` from a slice of AnyValues.

If `strict` is `true`, an error is returned if the values do not match the given data type. If `strict` is `false`, values that do not match the given data type are cast. If casting is not possible, the values are set to null instead.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-5" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.wrapping_trunc_div_scalar" class="fn">wrapping_trunc_div_scalar</a>\<T\>(&self, rhs: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-6" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_add_owned" class="fn">try_add_owned</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_sub_owned" class="fn">try_sub_owned</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_mul_owned" class="fn">try_mul_owned</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-7" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_array" class="fn">from_array</a>\<A\>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, array: A) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where A: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/static_array/trait.ParameterFreeDtypeStaticArray.html" class="trait" title="trait polars_arrow::array::static_array::ParameterFreeDtypeStaticArray">ParameterFreeDtypeStaticArray</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_chunk_and_dtype" class="fn">from_chunk_and_dtype</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, chunk: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_chunks_and_dtype_unchecked" class="fn">from_chunks_and_dtype_unchecked</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, chunks: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Takes chunks and a polars datatype and constructs the Series This is faster than creating from chunks and an arrow datatype because there is no casting involved

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure that the given `dtype`’s physical type matches all the `ArrayRef` dtypes.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method._try_from_arrow_unchecked" class="fn">_try_from_arrow_unchecked</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, chunks: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#safety-1" class="doc-anchor">§</a>Safety

The caller must ensure that the given `dtype` matches all the `ArrayRef` dtypes.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method._try_from_arrow_unchecked_with_md" class="fn">_try_from_arrow_unchecked_with_md</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, chunks: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, md: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a new Series without checking if the inner dtype of the chunks is correct

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#safety-2" class="doc-anchor">§</a>Safety

The caller must ensure that the given `dtype` matches all the `ArrayRef` dtypes.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_new" class="fn">try_new</a>\<T\>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, data: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, \<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, T) as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html" class="trait" title="trait core::convert::TryInto">TryInto</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryInto::Error">Error</a>\>

where (<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, T): <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html" class="trait" title="trait core::convert::TryInto">TryInto</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-9" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new_null" class="fn">new_null</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-10" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.array_ref" class="fn">array_ref</a>(&self, chunk_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Returns a reference to the Arrow ArrayRef

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.to_arrow" class="fn">to_arrow</a>( &self, chunk_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Convert a chunk in the Series to the correct Arrow type. This conversion is needed because polars doesn’t use a 1 on 1 mapping for logical/categoricals, etc.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-11" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/series/struct.SeriesIter.html" class="struct" title="struct polars::series::SeriesIter">SeriesIter</a>\<'\_\> <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#" class="tooltip" data-notable-ty="SeriesIter&lt;&#39;_&gt;">ⓘ</a>

iterate over [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") as [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue").

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#panics" class="doc-anchor">§</a>Panics

This will panic if the array is not rechunked first.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.phys_iter" class="fn">phys_iter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\> + '\_\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-12" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_i8" class="fn">try_i8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int8`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int8 "variant polars::prelude::DataType::Int8")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_i16" class="fn">try_i16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int16`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int16 "variant polars::prelude::DataType::Int16")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_i32" class="fn">try_i32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")

``` rust
let s = Series::new("foo".into(), [1i32 ,2, 3]);
let s_squared: Series = s.i32()
    .unwrap()
    .into_iter()
    .map(|opt_v| {
        match opt_v {
            Some(v) => Some(v * v),
            None => None, // null value
        }
}).collect();
```

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int32 "variant polars::prelude::DataType::Int32")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_i64" class="fn">try_i64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int64 "variant polars::prelude::DataType::Int64")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_i128" class="fn">try_i128</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int128`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int128 "variant polars::prelude::DataType::Int128")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_f32" class="fn">try_f32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Float32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float32 "variant polars::prelude::DataType::Float32")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_f64" class="fn">try_f64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Float64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float64 "variant polars::prelude::DataType::Float64")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_u8" class="fn">try_u8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt8`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt8 "variant polars::prelude::DataType::UInt8")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_u16" class="fn">try_u16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt16`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt16 "variant polars::prelude::DataType::UInt16")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_u32" class="fn">try_u32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt32 "variant polars::prelude::DataType::UInt32")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_u64" class="fn">try_u64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt64 "variant polars::prelude::DataType::UInt64")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_bool" class="fn">try_bool</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Boolean`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Boolean "variant polars::prelude::DataType::Boolean")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_str" class="fn">try_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::String`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.String "variant polars::prelude::DataType::String")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_binary" class="fn">try_binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Binary`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Binary "variant polars::prelude::DataType::Binary")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_binary_offset" class="fn">try_binary_offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Binary`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Binary "variant polars::prelude::DataType::Binary")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_time" class="fn">try_time</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Time`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Time "variant polars::prelude::DataType::Time")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_date" class="fn">try_date</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Date`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Date "variant polars::prelude::DataType::Date")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_datetime" class="fn">try_datetime</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Datetime`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Datetime "variant polars::prelude::DataType::Datetime")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_duration" class="fn">try_duration</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Duration`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Duration "variant polars::prelude::DataType::Duration")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_decimal" class="fn">try_decimal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Decimal`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Decimal "variant polars::prelude::DataType::Decimal")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_list" class="fn">try_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype list

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_array" class="fn">try_array</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Array`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Array "variant polars::prelude::DataType::Array")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_cat" class="fn">try_cat</a>\<T\>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_cat8" class="fn">try_cat8</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Categorical`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Categorical "variant polars::prelude::DataType::Categorical") or [`DataType::Enum`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Enum "variant polars::prelude::DataType::Enum") with a physical type of UInt8.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_cat16" class="fn">try_cat16</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_cat32" class="fn">try_cat32</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_struct" class="fn">try_struct</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Struct`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Struct "variant polars::prelude::DataType::Struct")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_null" class="fn">try_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Null`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Null "variant polars::prelude::DataType::Null")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.i8" class="fn">i8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int8`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int8 "variant polars::prelude::DataType::Int8")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.i16" class="fn">i16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int16`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int16 "variant polars::prelude::DataType::Int16")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.i32" class="fn">i32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")

``` rust
let s = Series::new("foo".into(), [1i32 ,2, 3]);
let s_squared: Series = s.i32()
    .unwrap()
    .into_iter()
    .map(|opt_v| {
        match opt_v {
            Some(v) => Some(v * v),
            None => None, // null value
        }
}).collect();
```

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int32 "variant polars::prelude::DataType::Int32")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.i64" class="fn">i64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int64 "variant polars::prelude::DataType::Int64")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.i128" class="fn">i128</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Int128`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int128 "variant polars::prelude::DataType::Int128")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.f32" class="fn">f32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Float32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float32 "variant polars::prelude::DataType::Float32")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.f64" class="fn">f64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Float64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float64 "variant polars::prelude::DataType::Float64")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.u8" class="fn">u8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt8`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt8 "variant polars::prelude::DataType::UInt8")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.u16" class="fn">u16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt16`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt16 "variant polars::prelude::DataType::UInt16")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.u32" class="fn">u32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt32 "variant polars::prelude::DataType::UInt32")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.u64" class="fn">u64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::UInt64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt64 "variant polars::prelude::DataType::UInt64")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bool" class="fn">bool</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Boolean`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Boolean "variant polars::prelude::DataType::Boolean")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.str" class="fn">str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::String`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.String "variant polars::prelude::DataType::String")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.binary" class="fn">binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Binary`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Binary "variant polars::prelude::DataType::Binary")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.binary_offset" class="fn">binary_offset</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Binary`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Binary "variant polars::prelude::DataType::Binary")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.time" class="fn">time</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Time`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Time "variant polars::prelude::DataType::Time")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.date" class="fn">date</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Date`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Date "variant polars::prelude::DataType::Date")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.datetime" class="fn">datetime</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Datetime`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Datetime "variant polars::prelude::DataType::Datetime")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.duration" class="fn">duration</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Duration`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Duration "variant polars::prelude::DataType::Duration")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.decimal" class="fn">decimal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DecimalType.html" class="struct" title="struct polars::prelude::DecimalType">DecimalType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type">Int128Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Decimal`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Decimal "variant polars::prelude::DataType::Decimal")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.list" class="fn">list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype list

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.array" class="fn">array</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Array`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Array "variant polars::prelude::DataType::Array")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cat" class="fn">cat</a>\<T\>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>,

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Categorical`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Categorical "variant polars::prelude::DataType::Categorical") or [`DataType::Enum`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Enum "variant polars::prelude::DataType::Enum").

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cat8" class="fn">cat8</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Categorical`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Categorical "variant polars::prelude::DataType::Categorical") or [`DataType::Enum`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Enum "variant polars::prelude::DataType::Enum") with a physical type of UInt8.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cat16" class="fn">cat16</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Categorical`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Categorical "variant polars::prelude::DataType::Categorical") or [`DataType::Enum`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Enum "variant polars::prelude::DataType::Enum") with a physical type of UInt16.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cat32" class="fn">cat32</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical">PolarsPhysical</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Categorical`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Categorical "variant polars::prelude::DataType::Categorical") or [`DataType::Enum`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Enum "variant polars::prelude::DataType::Enum") with a physical type of UInt32.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.struct_" class="fn">struct_</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Struct`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Struct "variant polars::prelude::DataType::Struct")

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.null" class="fn">null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unpack to [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") of dtype [`DataType::Null`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Null "variant polars::prelude::DataType::Null")

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-13" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.extend_constant" class="fn">extend_constant</a>( &self, value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extend with a constant value.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-14" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.full_null" class="fn">full_null</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-15" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.get_leaf_array" class="fn">get_leaf_array</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Recurse nested types until we are at the leaf array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.list_offsets_and_validities_recursive" class="fn">list_offsets_and_validities_recursive</a>( &self, ) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/offset/struct.OffsetsBuffer.html" class="struct" title="struct polars_arrow::offset::OffsetsBuffer">OffsetsBuffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>\>)

TODO: Move this somewhere else?

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.implode" class="fn">implode</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert the values of this Series to a ListChunked with a length of 1, so a Series of `[1, 2, 3]` becomes `[[1, 2, 3]]`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.reshape_array" class="fn">reshape_array</a>( &self, dimensions: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.ReshapeDimension.html" class="enum" title="enum polars::prelude::ReshapeDimension">ReshapeDimension</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.reshape_list" class="fn">reshape_list</a>( &self, dimensions: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.ReshapeDimension.html" class="enum" title="enum polars::prelude::ReshapeDimension">ReshapeDimension</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new_empty" class="fn">new_empty</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Create a new empty Series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.clear" class="fn">clear</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.take_inner" class="fn">take_inner</a>\<T\>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a>,

Take or clone a owned copy of the inner [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.chunks_mut" class="fn">chunks_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#safety-3" class="doc-anchor">§</a>Safety

The caller must ensure the length and the data types of `ArrayRef` does not change. And that the null_count is updated (e.g. with a `compute_len()`)

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.into_chunks" class="fn">into_chunks</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.select_chunk" class="fn">select_chunk</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_sorted_flag" class="fn">is_sorted_flag</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.set_sorted_flag" class="fn">set_sorted_flag</a>(&mut self, sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.get_flags" class="fn">get_flags</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/flags/struct.StatisticsFlags.html" class="struct" title="struct polars::chunked_array::flags::StatisticsFlags">StatisticsFlags</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.into_frame" class="fn">into_frame</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rename" class="fn">rename</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Rename series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.with_name" class="fn">with_name</a>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Return this Series with a new name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_arrow_chunks" class="fn">from_arrow_chunks</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, arrays: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_arrow" class="fn">from_arrow</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, array: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrink the capacity of this array to fit its length.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.append" class="fn">append</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Append in place. This is done by adding the chunks of `other` to this [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

See [`ChunkedArray::append`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html#method.append "method polars::prelude::ChunkedArray::append") and [`ChunkedArray::extend`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html#method.extend "method polars::prelude::ChunkedArray::extend").

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.append_owned" class="fn">append_owned</a>( &mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Append in place. This is done by adding the chunks of `other` to this [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

See [`ChunkedArray::append_owned`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html#method.append_owned "method polars::prelude::ChunkedArray::append_owned") and [`ChunkedArray::extend`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html#method.extend "method polars::prelude::ChunkedArray::extend").

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.compute_len" class="fn">compute_len</a>(&mut self)

Redo a length and null_count compute

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.extend" class="fn">extend</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extend the memory backed by this array with the values from `other`.

See [`ChunkedArray::extend`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html#method.extend "method polars::prelude::ChunkedArray::extend") and [`ChunkedArray::append`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html#method.append "method polars::prelude::ChunkedArray::append").

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sort" class="fn">sort</a>(&self, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sort the series with specific options.

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#example-1" class="doc-anchor">§</a>Example

``` rust
let s = Series::new("foo".into(), [2, 1, 3]);
let sorted = s.sort(SortOptions::default())?;
assert_eq!(sorted, Series::new("foo".into(), [1, 2, 3]));
}
```

See [`SortOptions`](https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html "struct polars::prelude::SortOptions") for more options.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.as_single_ptr" class="fn">as_single_ptr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Only implemented for numeric types

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cast" class="fn">cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cast_with_options" class="fn">cast_with_options</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") to another [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType").

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.cast_unchecked" class="fn">cast_unchecked</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast from physical to logical types without any checks on the validity of the cast.

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#safety-4" class="doc-anchor">§</a>Safety

This can lead to invalid memory access in downstream code.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_physical_unchecked" class="fn">from_physical_unchecked</a>( &self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert a non-logical series back into a logical series without casting.

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#safety-5" class="doc-anchor">§</a>Safety

This can lead to invalid memory access in downstream code.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.to_float" class="fn">to_float</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast numerical types to f64, and keep floats as is.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sum" class="fn">sum</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

Compute the sum of all values in this Series. Returns `Some(0)` if the array is empty, and `None` if the array only contains null values.

If the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is one of `{Int8, UInt8, Int16, UInt16}` the `Series` is first cast to `Int64` to prevent overflow issues.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.min" class="fn">min</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

Returns the minimum value in the array, according to the natural order. Returns an option because the array is nullable.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.max" class="fn">max</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

Returns the maximum value in the array, according to the natural order. Returns an option because the array is nullable.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.explode" class="fn">explode</a>(&self, skip_empty: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Explode a list Series. This expands every item to a new row..

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_nan" class="fn">is_nan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if numeric value is NaN (note this is different than missing/ null)

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_not_nan" class="fn">is_not_nan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if numeric value is NaN (note this is different than missing/null)

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_finite" class="fn">is_finite</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if numeric value is finite

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_infinite" class="fn">is_infinite</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if numeric value is infinite

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.zip_with" class="fn">zip_with</a>( &self, mask: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a new ChunkedArray with values from self where the mask evaluates `true` and values from `other` where the mask evaluates `false`. This function automatically broadcasts unit length inputs.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.to_physical_repr" class="fn">to_physical_repr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

Converts a Series to their physical representation, if they have one, otherwise the series is left unchanged.

- Date -\> Int32
- Datetime -\> Int64
- Duration -\> Int64
- Decimal -\> Int128
- Time -\> Int64
- Categorical -\> U8/U16/U32
- List(inner) -\> List(physical of inner)
- Array(inner) -\> Array(physical of inner)
- Struct -\> Struct with physical repr of each struct column

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gather_every" class="fn">gather_every</a>( &self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Traverse and collect every nth element in a new array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sum_reduce" class="fn">sum_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the sum of the Series as a new Series of length 1. Returns a Series with a single zeroed entry if self is an empty numeric series.

If the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is one of `{Int8, UInt8, Int16, UInt16}` the `Series` is first cast to `Int64` to prevent overflow issues.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.product" class="fn">product</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the product of an array.

If the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is one of `{Int8, UInt8, Int16, UInt16}` the `Series` is first cast to `Int64` to prevent overflow issues.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.strict_cast" class="fn">strict_cast</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Cast throws an error if conversion had overflows

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.str_value" class="fn">str_value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.head" class="fn">head</a>(&self, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Get the head of the Series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.tail" class="fn">tail</a>(&self, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Get the tail of the Series.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.mean_reduce" class="fn">mean_reduce</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.unique_stable" class="fn">unique_stable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the unique elements, but maintain order. This requires more work than a naive [`Series::unique`](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.unique "method polars::prelude::SeriesTrait::unique").

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_idx" class="fn">try_idx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.idx" class="fn">idx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.estimated_size" class="fn">estimated_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns an estimation of the total (heap) allocated size of the `Series` in bytes.

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#implementation" class="doc-anchor">§</a>Implementation

This estimation is the sum of the size of its buffers, validity, including nested arrays. Multiple arrays may share buffers and bitmaps. Therefore, the size of 2 arrays is not the sum of the sizes computed from this function. In particular, [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray")’s size is an upper bound.

When an array is sliced, its allocated size remains constant because the buffer unchanged. However, this function will yield a smaller number. This is because this function returns the visible size of the buffer, not its total capacity.

FFI buffers are included in this estimation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.as_list" class="fn">as_list</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

Packs every element into a list.

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.row_encode_unordered" class="fn">row_encode_unordered</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.row_encode_ordered" class="fn">row_encode_ordered</a>( &self, descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_last: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryOffsetType.html" class="struct" title="struct polars::prelude::BinaryOffsetType">BinaryOffsetType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Series-17" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equals" class="fn">equals</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if series are equal. Note that `None == None` evaluates to `false`

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equals_missing" class="fn">equals_missing</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if all values in series are equal where `None == None` evaluates to `true`.

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a>\><a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#deref-methods-dyn+SeriesTrait" class="anchor">§</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.unpack" class="fn">unpack</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Add%3CT%3E-for-%26Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<T\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-17" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.add-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: T) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Add%3CT%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-18" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.add-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: T) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Add-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-16" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.add" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Add-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-19" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.add-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ArgAgg-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html" class="trait" title="trait polars::prelude::ArgAgg">ArgAgg</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.arg_min" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#tymethod.arg_min" class="fn">arg_min</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the index of the minimal value

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.arg_max" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArgAgg.html#tymethod.arg_max" class="fn">arg_max</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the index of the maximal value

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-AsRef%3CSeries%3E-for-AmortSeries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/series/amortized_iter/struct.AmortSeries.html" class="struct" title="struct polars::series::amortized_iter::AmortSeries">AmortSeries</a>

We don’t implement Deref so that the caller is aware of converting to Series

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-AsRef%3Cdyn+SeriesTrait%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + 'a\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.as_ref-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &(dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + 'a)

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-AsSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/series/trait.AsSeries.html" class="trait" title="trait polars::prelude::series::AsSeries">AsSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.as_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/series/trait.AsSeries.html#tymethod.as_series" class="fn">as_series</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitAnd%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitand-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitAnd%3CSeries%3E-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitand-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitAnd-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitAnd-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitand-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitOr%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitor-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitOr%3CSeries%3E-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitor-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitOr-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitOr-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitor-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitXor%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitxor-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitXor%3CSeries%3E-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitxor-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitXor-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-BitXor-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.bitxor-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkApply%3C&#39;a,+Series%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.apply_values" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#tymethod.apply_values" class="fn">apply_values</a>\<F\>(&'a self, f: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Apply a closure `F` elementwise.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.FuncRet" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.apply" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#tymethod.apply" class="fn">apply</a>\<F\>(&'a self, f: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Apply a closure elementwise including null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.apply_to_slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#tymethod.apply_to_slice" class="fn">apply_to_slice</a>\<F, T\>(&'a self, f: F, slice: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>) -\> T,

Apply a closure elementwise and write results to a mutable slice.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkCompareEq%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Create a boolean mask by checking for equality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equal_missing" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Create a boolean mask by checking for equality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.not_equal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Create a boolean mask by checking for inequality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.not_equal_missing" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>( &self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, ) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Create a boolean mask by checking for inequality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkCompareEq%3C%26str%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equal-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check for equality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equal_missing-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.not_equal-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check for inequality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.not_equal_missing-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkCompareEq%3CRhs%3E-for-Series" class="anchor">§</a>

### impl\<Rhs\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where Rhs: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equal-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal" class="fn">equal</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.equal_missing-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.equal_missing" class="fn">equal_missing</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for equality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.not_equal-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal" class="fn">not_equal</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.not_equal_missing-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#tymethod.not_equal_missing" class="fn">not_equal_missing</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html" class="trait" title="trait polars::prelude::ChunkCompareEq">ChunkCompareEq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareEq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareEq::Item">Item</a>

Check for inequality where `None == None`.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkCompareIneq%3C%26Series%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gt" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Create a boolean mask by checking if self \> rhs.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gt_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Create a boolean mask by checking if self \>= rhs.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lt" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Create a boolean mask by checking if self \< rhs.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lt_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Create a boolean mask by checking if self \<= rhs.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkCompareIneq%3C%26str%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Item-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gt-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gt_eq-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lt-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lt_eq-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>(&self, rhs: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkCompareIneq%3CRhs%3E-for-Series" class="anchor">§</a>

### impl\<Rhs\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where Rhs: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Item-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gt-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt" class="fn">gt</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than comparison.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.gt_eq-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.gt_eq" class="fn">gt_eq</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Greater than or equal comparison.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lt-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt" class="fn">lt</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than comparison.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lt_eq-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#tymethod.lt_eq" class="fn">lt_eq</a>(&self, rhs: Rhs) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html" class="trait" title="trait polars::prelude::ChunkCompareIneq">ChunkCompareIneq</a>\<Rhs\>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkCompareIneq.html#associatedtype.Item" class="associatedtype" title="type polars::prelude::ChunkCompareIneq::Item">Item</a>

Less than or equal comparison

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkFull%3C%26Series%3E-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFull.html" class="trait" title="trait polars::prelude::ChunkFull">ChunkFull</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.full-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFull.html#tymethod.full" class="fn">full</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, value: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

Create a ChunkedArray with a single value.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkFull%3C%26Series%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFull.html" class="trait" title="trait polars::prelude::ChunkFull">ChunkFull</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.full" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFull.html#tymethod.full" class="fn">full</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, value: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

Create a ChunkedArray with a single value.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkQuantile%3CSeries%3E-for-ChunkedArray%3CFixedSizeListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html" class="trait" title="trait polars::prelude::ChunkQuantile">ChunkQuantile</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FixedSizeListType.html" class="struct" title="struct polars::prelude::FixedSizeListType">FixedSizeListType</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.median-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

Returns the mean value in the array. Returns `None` if the array is empty or only contains null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.quantile-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html#method.quantile" class="fn">quantile</a>( &self, \_quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, \_method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate a given quantile of the ChunkedArray. Returns `None` if the array is empty or only contains null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkQuantile%3CSeries%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html" class="trait" title="trait polars::prelude::ChunkQuantile">ChunkQuantile</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.median" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

Returns the mean value in the array. Returns `None` if the array is empty or only contains null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.quantile" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html#method.quantile" class="fn">quantile</a>( &self, \_quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, \_method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate a given quantile of the ChunkedArray. Returns `None` if the array is empty or only contains null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ChunkQuantile%3CSeries%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html" class="trait" title="trait polars::prelude::ChunkQuantile">ChunkQuantile</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.median-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

Returns the mean value in the array. Returns `None` if the array is empty or only contains null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.quantile-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkQuantile.html#method.quantile" class="fn">quantile</a>( &self, \_quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, \_method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate a given quantile of the ChunkedArray. Returns `None` if the array is empty or only contains null values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Clone-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Container-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html" class="trait" title="trait polars_core::utils::Container">Container</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.split_at" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.iter_chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.iter_chunks" class="fn">iter_chunks</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.should_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.should_rechunk" class="fn">should_rechunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.n_chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.n_chunks" class="fn">n_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.chunk_lengths" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.chunk_lengths" class="fn">chunk_lengths</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Debug-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Default-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Deref-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a>

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Deserialize%3C&#39;de%3E-for-Series" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>( deserializer: D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, \<D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Display-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Div%3CT%3E-for-%26Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<T\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-25" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.div-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: T) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Div%3CT%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-26" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.div-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: T) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Div-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.div" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

``` rust
let s: Series = [1, 2, 3].iter().collect();
let out = (&s / &s).unwrap();
```

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-24" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Div-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-27" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.div-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3CChunkedArray%3CT%3E%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(ca: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3CLogical%3CDateType,+Int32Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DateType.html" class="struct" title="struct polars::prelude::DateType">DateType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3CLogical%3CDatetimeType,+Int64Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DatetimeType.html" class="struct" title="struct polars::prelude::DatetimeType">DatetimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3CLogical%3CDurationType,+Int64Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DurationType.html" class="struct" title="struct polars::prelude::DurationType">DurationType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3CLogical%3CTimeType,+Int64Type%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Logical.html" class="struct" title="struct polars::prelude::Logical">Logical</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeType.html" class="struct" title="struct polars::prelude::TimeType">TimeType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3CSeries%3E-for-Column" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(series: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-From%3C_SerdeSeries%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<\_SerdeSeries\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: \_SerdeSeries) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26bool%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-33" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26f32%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-27" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26f64%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-30" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26i16%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-18" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26i32%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-21" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26i64%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-24" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26i8%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26str%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-35" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26u16%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26u32%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26u64%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3C%26u8%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3C%26str%3E%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-34" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3CString%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-36" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cbool%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-31" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cf32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-25" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cf64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-28" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Ci16%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-16" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Ci32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-19" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Ci64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-22" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Ci8%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cu16%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cu32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cu64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3COption%3Cu8%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3CSeries%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>,

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#panics-1" class="doc-anchor">§</a>Panics

Panics if Series have different lengths.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3CString%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-37" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cbool%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-32" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cf32%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-26" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cf64%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-29" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Ci16%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-17" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Ci32%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-20" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Ci64%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-23" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Ci8%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cu16%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cu32%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cu64%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-FromIterator%3Cu8%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_iter-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-IntoSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#method.is_series" class="fn">is_series</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.into_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html#tymethod.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Literal-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.lit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

[Literal](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Literal "variant polars::prelude::Expr::Literal") expression.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-MomentSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html" class="trait" title="trait polars::prelude::MomentSeries">MomentSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.skew" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#method.skew" class="fn">skew</a>(&self, bias: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the sample skewness of a data set. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#method.skew)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.kurtosis" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#method.kurtosis" class="fn">kurtosis</a>(&self, fisher: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, bias: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the kurtosis (Fisher or Pearson) of a dataset. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.MomentSeries.html#method.kurtosis)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Mul%3CT%3E-for-%26Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<T\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-21" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.mul-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: T) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Mul%3CT%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-22" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.mul-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: T) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Mul-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.mul" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

``` rust
let s: Series = [1, 2, 3].iter().collect();
let out = (&s * &s).unwrap();
```

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-20" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Mul-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-23" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.mul-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3C%26Series,+str%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-35" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, s: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CRange%3Ci32%3E,+Int32Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-29" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CRange%3Ci64%3E,+Int64Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-28" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CRange%3Cu32%3E,+UInt32Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-31" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CRange%3Cu64%3E,+UInt64Type%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-30" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5B%26%5Bu8%5D%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-39" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5B%26str%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-34" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BAnyValue%3C&#39;a%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-52" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, values: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Construct a new [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") from a collection of [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue").

##### <a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#panics-2" class="doc-anchor">§</a>Panics

Panics if the values do not all share the same data type (with the exception of [`DataType::Null`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Null "variant polars::prelude::DataType::Null"), which is always allowed).

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BCow%3C&#39;a,+%5Bu8%5D%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-41" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BCow%3C&#39;a,+str%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-37" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BNaiveDate%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-43" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BNaiveDateTime%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-45" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BNaiveTime%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-49" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3C%26%5Bu8%5D%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-40" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3C%26str%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-36" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CCow%3C&#39;a,+%5Bu8%5D%3E%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-42" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CCow%3C&#39;a,+str%3E%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-38" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDate%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-44" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveDateTime%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-46" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CNaiveTime%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-50" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CSeries%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-33" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, s: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CString%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-14" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CTimeDelta%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-48" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3CVec%3Cu8%3E%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-15" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cbool%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-16" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cf32%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-26" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cf64%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-27" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Ci128%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-25" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Ci16%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-22" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Ci32%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-23" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Ci64%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-24" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Ci8%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-21" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cu16%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-18" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cu32%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-19" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cu64%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-20" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BOption%3Cu8%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-17" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BString%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BTimeDelta%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-47" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5BVec%3Cu8%3E%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bbool%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bf32%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bf64%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bi128%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bi16%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bi32%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bi64%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bi8%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bu16%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bu32%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bu64%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+%5Bu8%5D%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+ListType%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, <a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-32" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, s: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFrom%3CT,+T%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html" class="trait" title="trait polars::prelude::NamedFrom">NamedFrom</a>\<T, T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a>,

For any [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") and [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series")

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.new-51" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFrom.html#tymethod.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, t: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Cf32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Cf64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Ci128%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Ci16%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Ci32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Ci64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Ci8%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Cu16%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Cu32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Cu64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NamedFromOwned%3CVec%3Cu8%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.from_vec-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Initialize by name and values.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-NumOpsDispatchChecked-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html" class="trait" title="trait polars::prelude::NumOpsDispatchChecked">NumOpsDispatchChecked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.checked_div" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#tymethod.checked_div" class="fn">checked_div</a>(&self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checked integer division. Computes self / rhs, returning None if rhs == 0 or the division results in overflow.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.checked_div_num" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumOpsDispatchChecked.html#tymethod.checked_div_num" class="fn">checked_div_num</a>\<T\>(&self, rhs: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-PartialEq-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Rem%3CT%3E-for-%26Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<T\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-29" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rem-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: T) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Rem%3CT%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-30" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rem-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: T) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Rem-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rem" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

``` rust
let s: Series = [1, 2, 3].iter().collect();
let out = (&s / &s).unwrap();
```

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-28" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-RoundSeries-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html" class="trait" title="trait polars::prelude::RoundSeries">RoundSeries</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.round" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.round" class="fn">round</a>(&self, decimals: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, mode: <a href="https://docs.rs/polars/latest/polars/prelude/enum.RoundMode.html" class="enum" title="enum polars::prelude::RoundMode">RoundMode</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Round underlying floating point array to given decimal.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.round_sig_figs" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.round_sig_figs" class="fn">round_sig_figs</a>(&self, digits: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.floor" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.floor" class="fn">floor</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Floor underlying floating point array to the lowest integers smaller or equal to the float value.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.ceil" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.RoundSeries.html#method.ceil" class="fn">ceil</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Ceil underlying floating point array to the highest integers smaller or equal to the float value.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Serialize-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>( &self, serializer: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-SeriesJoin-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesJoin.html" class="trait" title="trait polars::prelude::SeriesJoin">SeriesJoin</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.hash_join_semi_anti" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesJoin.html#method.hash_join_semi_anti" class="fn">hash_join_semi_anti</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, anti: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.hash_join_inner" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesJoin.html#method.hash_join_inner" class="fn">hash_join_inner</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, validate: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinValidation.html" class="enum" title="enum polars::prelude::JoinValidation">JoinValidation</a>, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<((<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>), <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.hash_join_outer" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesJoin.html#method.hash_join_outer" class="fn">hash_join_outer</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, validate: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinValidation.html" class="enum" title="enum polars::prelude::JoinValidation">JoinValidation</a>, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-SeriesMethods-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesMethods.html" class="trait" title="trait polars::prelude::SeriesMethods">SeriesMethods</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.value_counts" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesMethods.html#method.value_counts" class="fn">value_counts</a>( &self, sort: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, parallel: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, normalize: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") with the unique `values` of this [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and a column `"counts"` with dtype [`IdxType`](https://docs.rs/polars/latest/polars/prelude/type.IdxType.html "type polars::prelude::IdxType")

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.ensure_sorted_arg" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesMethods.html#method.ensure_sorted_arg" class="fn">ensure_sorted_arg</a>(&self, operation: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.is_sorted" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesMethods.html#method.is_sorted" class="fn">is_sorted</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checks if a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") is sorted. Tries to fail fast.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-SeriesOpsTime-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html" class="trait" title="trait polars::prelude::SeriesOpsTime">SeriesOpsTime</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_mean_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_mean_by" class="fn">rolling_mean_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling mean to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_mean" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_mean" class="fn">rolling_mean</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling mean to a Series. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_mean)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_sum_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_sum_by" class="fn">rolling_sum_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling sum to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_sum" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_sum" class="fn">rolling_sum</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling sum to a Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_quantile_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_quantile_by" class="fn">rolling_quantile_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling quantile to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_quantile" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_quantile" class="fn">rolling_quantile</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling quantile to a Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_min_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_min_by" class="fn">rolling_min_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling min to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_min" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_min" class="fn">rolling_min</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling min to a Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_max_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_max_by" class="fn">rolling_max_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling max to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_max" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_max" class="fn">rolling_max</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling max to a Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_var_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_var_by" class="fn">rolling_var_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling variance to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_var" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_var" class="fn">rolling_var</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling variance to a Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_std_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_std_by" class="fn">rolling_std_by</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsDynamicWindow.html" class="struct" title="struct polars::prelude::RollingOptionsDynamicWindow">RollingOptionsDynamicWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling std_dev to a Series based on another Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rolling_std" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesOpsTime.html#method.rolling_std" class="fn">rolling_std</a>( &self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a rolling std_dev to a Series.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-SeriesRank-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html" class="trait" title="trait polars::prelude::SeriesRank">SeriesRank</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.rank" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesRank.html#method.rank" class="fn">rank</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RankOptions.html" class="struct" title="struct polars::prelude::RankOptions">RankOptions</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-SeriesSealed-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesSealed.html" class="trait" title="trait polars::prelude::SeriesSealed">SeriesSealed</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.as_series-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesSealed.html#tymethod.as_series" class="fn">as_series</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ShrinkType-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html" class="trait" title="trait polars::prelude::ShrinkType">ShrinkType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.shrink_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ShrinkType.html#tymethod.shrink_type" class="fn">shrink_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Sub%3CT%3E-for-%26Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<T\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-13" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sub-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: T) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Sub%3CT%3E-for-Series" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.NumCast.html" class="trait" title="trait num_traits::cast::NumCast">NumCast</a>,

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-14" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sub-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: T) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<T\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Sub-for-%26Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-12" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-Sub-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Output-15" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.sub-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-TakeChunked-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html" class="trait" title="trait polars::prelude::TakeChunked">TakeChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.take_opt_chunked_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_opt_chunked_unchecked" class="fn">take_opt_chunked_unchecked</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, by: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], avoid_sharing: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take function that checks of null state in `ChunkIdx`.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.take_chunked_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_chunked_unchecked" class="fn">take_chunked_unchecked</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, by: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>, avoid_sharing: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Gathers elements from a ChunkedArray, specifying for each element a chunk index and index within that chunk through ChunkId. If avoid_sharing is true the returned data should not share references with the original array (like shared buffers in views). [Read more](https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_chunked_unchecked)

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-ToDummies-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html" class="trait" title="trait polars::prelude::ToDummies">ToDummies</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.to_dummies" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ToDummies.html#tymethod.to_dummies" class="fn">to_dummies</a>( &self, separator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, drop_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, drop_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-TryFrom%3C(%26ArrowField,+Box%3Cdyn+Array%3E)%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<(&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Error-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(field_arr: (&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Performs the conversion.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-TryFrom%3C(%26ArrowField,+Vec%3CBox%3Cdyn+Array%3E%3E)%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<(&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( field_arr: (&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Performs the conversion.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-TryFrom%3C(PlSmallStr,+Box%3Cdyn+Array%3E)%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( name_arr: (<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Performs the conversion.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#impl-TryFrom%3C(PlSmallStr,+Vec%3CBox%3Cdyn+Array%3E%3E)%3E-for-Series" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( name_arr: (<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>), ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Performs the conversion.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/series/struct.Series.html#blanket-implementations" class="anchor">§</a>
