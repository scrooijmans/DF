# Module ops Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/mod.rs.html#13" class="src">Source</a>

Expand description

Traits for miscellaneous operations on ChunkedArray

## Modules<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/index.html" class="mod" title="mod polars::chunked_array::ops::arity">arity</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/fill_null/index.html" class="mod" title="mod polars::chunked_array::ops::fill_null">fill_null</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/float_sorted_arg_max/index.html" class="mod" title="mod polars::chunked_array::ops::float_sorted_arg_max">float_sorted_arg_max</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/full/index.html" class="mod" title="mod polars::chunked_array::ops::full">full</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/gather/index.html" class="mod" title="mod polars::chunked_array::ops::gather">gather</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/row_encode/index.html" class="mod" title="mod polars::chunked_array::ops::row_encode">row_encode</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/search_sorted/index.html" class="mod" title="mod polars::chunked_array::ops::search_sorted">search_sorted</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/sort/index.html" class="mod" title="mod polars::chunked_array::ops::sort">sort</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/zip/index.html" class="mod" title="mod polars::chunked_array::ops::zip">zip</a>

## Structs<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html" class="struct" title="struct polars::chunked_array::ops::SortMultipleOptions">SortMultipleOptions</a>  
Sort options for multi-series sorting.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortOptions.html" class="struct" title="struct polars::chunked_array::ops::SortOptions">SortOptions</a>  
Options for single series sorting.

## Enums<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/enum.FillNullStrategy.html" class="enum" title="enum polars::chunked_array::ops::FillNullStrategy">FillNullStrategy</a>

## Traits<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAgg.html" class="trait" title="trait polars::chunked_array::ops::ChunkAgg">ChunkAgg</a>  
Aggregation operations.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkAnyValue.html" class="trait" title="trait polars::chunked_array::ops::ChunkAnyValue">ChunkAnyValue</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html" class="trait" title="trait polars::chunked_array::ops::ChunkApply">ChunkApply</a>  
Fastest way to do elementwise operations on a [`ChunkedArray<T>`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") when the operation is cheaper than branching due to null checking.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApplyKernel.html" class="trait" title="trait polars::chunked_array::ops::ChunkApplyKernel">ChunkApplyKernel</a>  
Apply kernels on the arrow array chunks in a ChunkedArray.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApproxNUnique.html" class="trait" title="trait polars::chunked_array::ops::ChunkApproxNUnique">ChunkApproxNUnique</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkBitwiseReduce.html" class="trait" title="trait polars::chunked_array::ops::ChunkBitwiseReduce">ChunkBitwiseReduce</a>  
Bitwise Reduction Operations.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkBytes.html" class="trait" title="trait polars::chunked_array::ops::ChunkBytes">ChunkBytes</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkCast.html" class="trait" title="trait polars::chunked_array::ops::ChunkCast">ChunkCast</a>  
Cast `ChunkedArray<T>` to `ChunkedArray<N>`

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkCompareEq.html" class="trait" title="trait polars::chunked_array::ops::ChunkCompareEq">ChunkCompareEq</a>  
Compare [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")’s and get a `boolean` mask that can be used to filter rows.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkCompareIneq.html" class="trait" title="trait polars::chunked_array::ops::ChunkCompareIneq">ChunkCompareIneq</a>  
Compare [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")’s using inequality operators (`<`, `>=`, etc.) and get a `boolean` mask that can be used to filter rows.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkExpandAtIndex.html" class="trait" title="trait polars::chunked_array::ops::ChunkExpandAtIndex">ChunkExpandAtIndex</a>  
Create a new ChunkedArray filled with values at that index.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkExplode.html" class="trait" title="trait polars::chunked_array::ops::ChunkExplode">ChunkExplode</a>  
Explode/flatten a List or String Series

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkFillNullValue.html" class="trait" title="trait polars::chunked_array::ops::ChunkFillNullValue">ChunkFillNullValue</a>  
Replace None values with a value

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkFilter.html" class="trait" title="trait polars::chunked_array::ops::ChunkFilter">ChunkFilter</a>  
Filter values by a boolean mask.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkFull.html" class="trait" title="trait polars::chunked_array::ops::ChunkFull">ChunkFull</a>  
Fill a ChunkedArray with one value.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkFullNull.html" class="trait" title="trait polars::chunked_array::ops::ChunkFullNull">ChunkFullNull</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkNestingUtils.html" class="trait" title="trait polars::chunked_array::ops::ChunkNestingUtils">ChunkNestingUtils</a>  
Utility methods for dealing with nested chunked arrays.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkQuantile.html" class="trait" title="trait polars::chunked_array::ops::ChunkQuantile">ChunkQuantile</a>  
Quantile and median aggregation.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkReverse.html" class="trait" title="trait polars::chunked_array::ops::ChunkReverse">ChunkReverse</a>  
Reverse a [`ChunkedArray<T>`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray")

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkRollApply.html" class="trait" title="trait polars::chunked_array::ops::ChunkRollApply">ChunkRollApply</a>  
This differs from ChunkWindowCustom and ChunkWindow by not using a fold aggregator, but reusing a `Series` wrapper and calling `Series` aggregators. This likely is a bit slower than ChunkWindow

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html" class="trait" title="trait polars::chunked_array::ops::ChunkSet">ChunkSet</a>  
Create a `ChunkedArray` with new values by index or by boolean mask.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkShift.html" class="trait" title="trait polars::chunked_array::ops::ChunkShift">ChunkShift</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkShiftFill.html" class="trait" title="trait polars::chunked_array::ops::ChunkShiftFill">ChunkShiftFill</a>  
Shift the values of a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") by a number of periods.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSort.html" class="trait" title="trait polars::chunked_array::ops::ChunkSort">ChunkSort</a>  
Sort operations on `ChunkedArray`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTake.html" class="trait" title="trait polars::chunked_array::ops::ChunkTake">ChunkTake</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkTakeUnchecked.html" class="trait" title="trait polars::chunked_array::ops::ChunkTakeUnchecked">ChunkTakeUnchecked</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkUnique.html" class="trait" title="trait polars::chunked_array::ops::ChunkUnique">ChunkUnique</a>  
Get unique values in a `ChunkedArray`

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkVar.html" class="trait" title="trait polars::chunked_array::ops::ChunkVar">ChunkVar</a>  
Variance and standard deviation aggregation.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkZip.html" class="trait" title="trait polars::chunked_array::ops::ChunkZip">ChunkZip</a>  
Combine two [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") based on some predicate.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.IsFirstDistinct.html" class="trait" title="trait polars::chunked_array::ops::IsFirstDistinct">IsFirstDistinct</a>  
Mask the first unique values as `true`

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.IsLastDistinct.html" class="trait" title="trait polars::chunked_array::ops::IsLastDistinct">IsLastDistinct</a>  
Mask the last unique values as `true`

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.Reinterpret.html" class="trait" title="trait polars::chunked_array::ops::Reinterpret">Reinterpret</a>  
<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.SlicedArray.html" class="trait" title="trait polars::chunked_array::ops::SlicedArray">SlicedArray</a>  
Utility trait to slice concrete arrow arrays whilst keeping their concrete type. E.g. don’t return `Box<dyn Array>`.

## Functions<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/fn._set_check_length.html" class="fn" title="fn polars::chunked_array::ops::_set_check_length">_set_check_length</a><sup>⚠</sup>  
Meant for internal use. In very rare conditions this can be turned off.

## Type Aliases<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/type.FillNullLimit.html" class="type" title="type polars::chunked_array::ops::FillNullLimit">FillNullLimit</a>
