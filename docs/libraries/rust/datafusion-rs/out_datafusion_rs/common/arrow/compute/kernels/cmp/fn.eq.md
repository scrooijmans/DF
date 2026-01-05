# Function eq Copy item path

<a href="https://docs.rs/arrow-ord/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ord/cmp.rs.html#79" class="src">Source</a>

``` rust
pub fn eq(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Expand description

Perform `left == right` operation on two [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum").

Comparing null values on either side will yield a null in the corresponding slot of the resulting [`BooleanArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html "struct datafusion::common::arrow::array::BooleanArray").

For floating values like f32 and f64, this comparison produces an ordering in accordance to the totalOrder predicate as defined in the IEEE 754 (2008 revision) floating point standard. Note that totalOrder treats positive and negative zeros as different. If it is necessary to treat them as equal, please normalize zeros before calling this kernel. See [`f32::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.total_cmp "method f32::total_cmp") and [`f64::total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp "method f64::total_cmp").

Nested types, such as lists, are not supported as the null semantics are not well-defined. For comparisons involving nested types see [`crate::ord::make_comparator`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/fn.make_comparator.html "fn datafusion::common::arrow::array::make_comparator")
