# Module cmp Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/lib.rs.html#52" class="src">Source</a>

Expand description

Comparison kernels for `Array`s.

These kernels can leverage SIMD if available on your system. Currently no runtime detection is provided, you should enable the specific SIMD intrinsics using `RUSTFLAGS="-C target-feature=+avx2"` for example. See the documentation [here](https://doc.rust-lang.org/stable/core/arch/) for more information.

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.compare_byte_view.html" class="fn" title="fn arrow::compute::kernels::cmp::compare_byte_view">compare_byte_view</a>  
Compares two [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") at index `left_idx` and `right_idx`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.distinct.html" class="fn" title="fn arrow::compute::kernels::cmp::distinct">distinct</a>  
Perform `left IS DISTINCT FROM right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.eq.html" class="fn" title="fn arrow::compute::kernels::cmp::eq">eq</a>  
Perform `left == right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.gt.html" class="fn" title="fn arrow::compute::kernels::cmp::gt">gt</a>  
Perform `left > right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.gt_eq.html" class="fn" title="fn arrow::compute::kernels::cmp::gt_eq">gt_eq</a>  
Perform `left >= right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.lt.html" class="fn" title="fn arrow::compute::kernels::cmp::lt">lt</a>  
Perform `left < right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.lt_eq.html" class="fn" title="fn arrow::compute::kernels::cmp::lt_eq">lt_eq</a>  
Perform `left <= right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.neq.html" class="fn" title="fn arrow::compute::kernels::cmp::neq">neq</a>  
Perform `left != right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.not_distinct.html" class="fn" title="fn arrow::compute::kernels::cmp::not_distinct">not_distinct</a>  
Perform `left IS NOT DISTINCT FROM right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum")
