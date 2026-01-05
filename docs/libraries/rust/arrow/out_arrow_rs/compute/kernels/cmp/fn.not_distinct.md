# Function not_distinct Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/cmp.rs.html#200" class="src">Source</a>

``` rust
pub fn not_distinct(
    lhs: &dyn Datum,
    rhs: &dyn Datum,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Perform `left IS NOT DISTINCT FROM right` operation on two [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum")

[`not_distinct`](https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.not_distinct.html "fn arrow::compute::kernels::cmp::not_distinct") is similar to [`eq`](https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.eq.html "fn arrow::compute::kernels::cmp::eq"), only differing in null handling. In particular, two operands are considered `NOT DISTINCT` if they have the same value or if both of them is NULL. The result of [`not_distinct`](https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/fn.not_distinct.html "fn arrow::compute::kernels::cmp::not_distinct") is never NULL.

For floating values like f32 and f64, this comparison produces an ordering in accordance to the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard. Note that totalOrder treats positive and negative zeros as different. If it is necessary to treat them as equal, please normalize zeros before calling this kernel. See [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp") and [`f64::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp "method f64::total_cmp").

Nested types, such as lists, are not supported as the null semantics are not well-defined. For comparisons involving nested types see [`crate::ord::make_comparator`](https://docs.rs/arrow/latest/arrow/array/fn.make_comparator.html "fn arrow::array::make_comparator")
