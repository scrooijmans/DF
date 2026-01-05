# Trait SeriesTrait Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/series/series_trait.rs.html#208-209" class="src">Source</a>

``` rust
pub trait SeriesTrait:
    Send
    + Sync
    + PrivateSeries
    + PrivateSeriesNumeric {
Show 70 methods    // Required methods
    fn rename(&mut self, name: PlSmallStr);
    fn chunk_lengths(
        &self,
    ) -> Map<Iter<'_, Box<dyn Array>>, fn(&Box<dyn Array>) -> usize> ⓘ;
    fn name(&self) -> &PlSmallStr;
    fn chunks(&self) -> &Vec<Box<dyn Array>>;
    unsafe fn chunks_mut(&mut self) -> &mut Vec<Box<dyn Array>>;
    fn slice(&self, _offset: i64, _length: usize) -> Series;
    fn split_at(&self, _offset: i64) -> (Series, Series);
    fn append(&mut self, other: &Series) -> Result<(), PolarsError>;
    fn append_owned(&mut self, other: Series) -> Result<(), PolarsError>;
    fn filter(
        &self,
        _filter: &ChunkedArray<BooleanType>,
    ) -> Result<Series, PolarsError>;
    fn take(
        &self,
        _indices: &ChunkedArray<UInt32Type>,
    ) -> Result<Series, PolarsError>;
    unsafe fn take_unchecked(&self, _idx: &ChunkedArray<UInt32Type>) -> Series;
    fn take_slice(&self, _indices: &[u32]) -> Result<Series, PolarsError>;
    unsafe fn take_slice_unchecked(&self, _idx: &[u32]) -> Series;
    fn len(&self) -> usize;
    fn rechunk(&self) -> Series;
    fn new_from_index(&self, _index: usize, _length: usize) -> Series;
    fn find_validity_mismatch(&self, other: &Series, idxs: &mut Vec<u32>);
    fn cast(
        &self,
        _dtype: &DataType,
        options: CastOptions,
    ) -> Result<Series, PolarsError>;
    unsafe fn get_unchecked(&self, _index: usize) -> AnyValue<'_>;
    fn null_count(&self) -> usize;
    fn has_nulls(&self) -> bool;
    fn is_null(&self) -> ChunkedArray<BooleanType>;
    fn is_not_null(&self) -> ChunkedArray<BooleanType>;
    fn reverse(&self) -> Series;
    fn shift(&self, _periods: i64) -> Series;
    fn clone_inner(&self) -> Arc<dyn SeriesTrait>;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
    fn as_phys_any(&self) -> &(dyn Any + 'static);
    fn as_arc_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;

    // Provided methods
    fn field(&self) -> Cow<'_, Field> { ... }
    fn dtype(&self) -> &DataType { ... }
    fn n_chunks(&self) -> usize { ... }
    fn shrink_to_fit(&mut self) { ... }
    fn limit(&self, num_elements: usize) -> Series { ... }
    fn is_empty(&self) -> bool { ... }
    fn rechunk_validity(&self) -> Option<Bitmap> { ... }
    fn drop_nulls(&self) -> Series { ... }
    fn _sum_as_f64(&self) -> f64 { ... }
    fn mean(&self) -> Option<f64> { ... }
    fn std(&self, _ddof: u8) -> Option<f64> { ... }
    fn var(&self, _ddof: u8) -> Option<f64> { ... }
    fn median(&self) -> Option<f64> { ... }
    fn trim_lists_to_normalized_offsets(&self) -> Option<Series> { ... }
    fn propagate_nulls(&self) -> Option<Series> { ... }
    fn get(&self, index: usize) -> Result<AnyValue<'_>, PolarsError> { ... }
    fn sort_with(&self, _options: SortOptions) -> Result<Series, PolarsError> { ... }
    fn arg_sort(&self, options: SortOptions) -> ChunkedArray<UInt32Type> { ... }
    fn unique(&self) -> Result<Series, PolarsError> { ... }
    fn n_unique(&self) -> Result<usize, PolarsError> { ... }
    fn arg_unique(&self) -> Result<ChunkedArray<UInt32Type>, PolarsError> { ... }
    fn as_single_ptr(&mut self) -> Result<usize, PolarsError> { ... }
    fn sum_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn max_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn min_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn median_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn var_reduce(&self, _ddof: u8) -> Result<Scalar, PolarsError> { ... }
    fn std_reduce(&self, _ddof: u8) -> Result<Scalar, PolarsError> { ... }
    fn quantile_reduce(
        &self,
        _quantile: f64,
        _method: QuantileMethod,
    ) -> Result<Scalar, PolarsError> { ... }
    fn and_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn or_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn xor_reduce(&self) -> Result<Scalar, PolarsError> { ... }
    fn first(&self) -> Scalar { ... }
    fn last(&self) -> Scalar { ... }
    fn approx_n_unique(&self) -> Result<u32, PolarsError> { ... }
    fn get_object(
        &self,
        _index: usize,
    ) -> Option<&(dyn PolarsObjectSafe + 'static)> { ... }
    unsafe fn get_object_chunked_unchecked(
        &self,
        _chunk: usize,
        _index: usize,
    ) -> Option<&(dyn PolarsObjectSafe + 'static)> { ... }
    fn checked_div(&self, _rhs: &Series) -> Result<Series, PolarsError> { ... }
    fn rolling_map(
        &self,
        _f: &dyn Fn(&Series) -> Result<Series, PolarsError>,
        _options: RollingOptionsFixedWindow,
    ) -> Result<Series, PolarsError> { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.rename" class="fn">rename</a>(&mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

Rename the Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunk_lengths" class="fn">chunk_lengths</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html" class="struct" title="struct core::iter::adapters::map::Map">Map</a>\<<a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(&<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#" class="tooltip" data-notable-ty="Map&lt;Iter&lt;&#39;_, Box&lt;dyn Array&gt;&gt;, fn(&amp;Box&lt;dyn Array&gt;) -&gt; usize&gt;">ⓘ</a>

Get the lengths of the underlying chunks

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

Name of series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunks" class="fn">chunks</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

Underlying chunks.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.chunks_mut" class="fn">chunks_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

Underlying chunks.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure the length and the data types of `ArrayRef` does not change.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.slice" class="fn">slice</a>(&self, \_offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, \_length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Get a zero copy view of the data.

When offset is negative the offset is counted from the end of the array

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.split_at" class="fn">split_at</a>(&self, \_offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>)

Get a zero copy view of the data.

When offset is negative the offset is counted from the end of the array

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.append" class="fn">append</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.append_owned" class="fn">append_owned</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.filter" class="fn">filter</a>( &self, \_filter: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Filter by boolean mask. This operation clones data.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take" class="fn">take</a>( &self, \_indices: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take from `self` at the indexes given by `idx`.

Null values in `idx` because null values in the output array.

This operation is clone.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_unchecked" class="fn">take_unchecked</a>(&self, \_idx: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take from `self` at the indexes given by `idx`.

Null values in `idx` because null values in the output array.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#safety-1" class="doc-anchor">§</a>Safety

This doesn’t check any bounds.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_slice" class="fn">take_slice</a>(&self, \_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take from `self` at the indexes given by `idx`.

This operation is clone.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.take_slice_unchecked" class="fn">take_slice_unchecked</a>(&self, \_idx: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take from `self` at the indexes given by `idx`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#safety-2" class="doc-anchor">§</a>Safety

This doesn’t check any bounds.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get length of series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.rechunk" class="fn">rechunk</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Aggregate all chunks to a contiguous array of memory.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.new_from_index" class="fn">new_from_index</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Create a new Series filled with values from the given index.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
let s = Series::new("a".into(), [0i32, 1, 8]);
let s2 = s.new_from_index(2, 4);
assert_eq!(Vec::from(s2.i32().unwrap()), &[Some(8), Some(8), Some(8), Some(8)])
```

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.find_validity_mismatch" class="fn">find_validity_mismatch</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, idxs: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

Find the indices of elements where the null masks are different recursively.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.cast" class="fn">cast</a>( &self, \_dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, options: <a href="https://docs.rs/polars/latest/polars/chunked_array/cast/enum.CastOptions.html" class="enum" title="enum polars::chunked_array::cast::CastOptions">CastOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.get_unchecked" class="fn">get_unchecked</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

Get a single value by index. Don’t use this operation for loops as a runtime cast is needed for every iteration.

This may refer to physical types

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#safety-3" class="doc-anchor">§</a>Safety

Does not do any bounds checking

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Count the null values.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.has_nulls" class="fn">has_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return if any the chunks in this [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") have nulls.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.is_null" class="fn">is_null</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Get a mask of the null values.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.is_not_null" class="fn">is_not_null</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

Get a mask of the non-null values.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.reverse" class="fn">reverse</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

return a Series in reversed order

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift" class="fn">shift</a>(&self, \_periods: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Shift the values by a given period and fill the parts that will be empty due to this operation with `Nones`.

*NOTE: If you want to fill the Nones with a value use the [`shift` operation on `ChunkedArray<T>`](https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkShift.html).*

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#example-1" class="doc-anchor">§</a>Example

``` rust
fn example() -> PolarsResult<()> {
    let s = Series::new("series".into(), &[1, 2, 3]);

    let shifted = s.shift(1);
    assert_eq!(Vec::from(shifted.i32()?), &[None, Some(1), Some(2)]);

    let shifted = s.shift(-1);
    assert_eq!(Vec::from(shifted.i32()?), &[Some(2), Some(3), None]);

    let shifted = s.shift(2);
    assert_eq!(Vec::from(shifted.i32()?), &[None, None, Some(1)]);

    Ok(())
}
example();
```

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.clone_inner" class="fn">clone_inner</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a>\>

Clone inner ChunkedArray and wrap in a new Arc

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Get a hold of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), [`Logical`](https://docs.rs/polars/latest/polars/prelude/struct.Logical.html "struct polars::prelude::Logical") or `NullChunked` as an `Any` trait reference.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Get a hold of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), [`Logical`](https://docs.rs/polars/latest/polars/prelude/struct.Logical.html "struct polars::prelude::Logical") or `NullChunked` as an `Any` trait mutable reference.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_phys_any" class="fn">as_phys_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Get a hold of the [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") or `NullChunked` as an `Any` trait reference. This pierces through `Logical` types to get the underlying physical array.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.as_arc_any" class="fn">as_arc_any</a>(self: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<Self\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.field" class="fn">field</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>

Get field (used in schema)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get datatype of series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.n_chunks" class="fn">n_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Number of chunks in this Series

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrink the capacity of this array to fit its length.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.limit" class="fn">limit</a>(&self, num_elements: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Take `num_elements` from the top as a zero copy view.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if Series is empty.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.rechunk_validity" class="fn">rechunk_validity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.drop_nulls" class="fn">drop_nulls</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

Drop all null values and return a new Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method._sum_as_f64" class="fn">_sum_as_f64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

Returns the sum of the array as an f64.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.mean" class="fn">mean</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the mean value in the array Returns an option because the array is nullable.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.std" class="fn">std</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the std value in the array Returns an option because the array is nullable.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.var" class="fn">var</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the var value in the array Returns an option because the array is nullable.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Returns the median value in the array Returns an option because the array is nullable.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.trim_lists_to_normalized_offsets" class="fn">trim_lists_to_normalized_offsets</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

Trim all lists of unused start and end elements recursively.

- `None` if nothing needed to be done.
- `Some(series)` if something changed.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.propagate_nulls" class="fn">propagate_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

Propagate down nulls in nested types.

- `None` if nothing needed to be done.
- `Some(series)` if something changed.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get" class="fn">get</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a single value by index. Don’t use this operation for loops as a runtime cast is needed for every iteration.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.sort_with" class="fn">sort_with</a>(&self, \_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.arg_sort" class="fn">arg_sort</a>(&self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Retrieve the indexes needed for a sort.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.unique" class="fn">unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get unique values in the Series.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.n_unique" class="fn">n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get unique values in the Series.

A `null` value also counts as a unique value.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.arg_unique" class="fn">arg_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get first indexes of unique values.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.as_single_ptr" class="fn">as_single_ptr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Rechunk and return a pointer to the start of the Series. Only implemented for numeric types

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.sum_reduce" class="fn">sum_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the sum of the Series as a new Scalar.

If the [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is one of `{Int8, UInt8, Int16, UInt16}` the `Series` is first cast to `Int64` to prevent overflow issues.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.max_reduce" class="fn">max_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the max of the Series as a new Series of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.min_reduce" class="fn">min_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the min of the Series as a new Series of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.median_reduce" class="fn">median_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the median of the Series as a new Series of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.var_reduce" class="fn">var_reduce</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the variance of the Series as a new Series of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.std_reduce" class="fn">std_reduce</a>(&self, \_ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the standard deviation of the Series as a new Series of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.quantile_reduce" class="fn">quantile_reduce</a>( &self, \_quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, \_method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the quantile of the ChunkedArray as a new Series of length 1.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.and_reduce" class="fn">and_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the bitwise AND of the Series as a new Series of length 1,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.or_reduce" class="fn">or_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the bitwise OR of the Series as a new Series of length 1,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.xor_reduce" class="fn">xor_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the bitwise XOR of the Series as a new Series of length 1,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.first" class="fn">first</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the first element of the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") as a [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar")

If the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") is empty, a [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar") with a [`AnyValue::Null`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Null "variant polars::prelude::AnyValue::Null") is returned.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.last" class="fn">last</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

Get the last element of the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") as a [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar")

If the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") is empty, a [`Scalar`](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html "struct polars::prelude::Scalar") with a [`AnyValue::Null`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#variant.Null "variant polars::prelude::AnyValue::Null") is returned.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.approx_n_unique" class="fn">approx_n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get_object" class="fn">get_object</a>(&self, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)\>

Get the value at this index as a downcastable Any trait ref.

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.get_object_chunked_unchecked" class="fn">get_object_chunked_unchecked</a>( &self, \_chunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)\>

Get the value at this index as a downcastable Any trait ref.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#safety-4" class="doc-anchor">§</a>Safety

This function doesn’t do any bound checks.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.checked_div" class="fn">checked_div</a>(&self, \_rhs: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.rolling_map" class="fn">rolling_map</a>( &self, \_f: &dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>, \_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingOptionsFixedWindow.html" class="struct" title="struct polars::prelude::RollingOptionsFixedWindow">RollingOptionsFixedWindow</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Apply a custom function over a rolling/ moving window of the array. This has quite some dynamic dispatch, so prefer rolling_min, max, mean, sum over this.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#impl-dyn+SeriesTrait" class="anchor">§</a>

### impl dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + '\_

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.unpack" class="fn">unpack</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#impl-AsMut%3CChunkedArray%3CT%3E%3E-for-dyn+SeriesTrait" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html" class="trait" title="trait core::convert::AsMut">AsMut</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\> for dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + '\_

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.as_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut" class="fn">as_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

Converts this type into a mutable reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#impl-AsRef%3CChunkedArray%3CT%3E%3E-for-dyn+SeriesTrait" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\> for dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + '\_

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsPhysicalType.html" class="trait" title="trait polars::prelude::PolarsPhysicalType">PolarsPhysicalType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.as_ref-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#impl-AsRef%3Cdyn+SeriesTrait%3E-for-Series" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + 'a\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &(dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> + 'a)

Converts this type into a shared reference of the (usually inferred) input type.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#impl-SeriesTrait-for-NullChunked" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html" class="trait" title="trait polars::prelude::SeriesTrait">SeriesTrait</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullChunked.html" class="struct" title="struct polars::prelude::NullChunked">NullChunked</a>
