# Struct FixedLexicographicalComparator Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#1027" class="src">Source</a>

``` rust
pub struct FixedLexicographicalComparator<const N: usize> { /* private fields */ }
```

Expand description

A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data at given two indices. This version of the comparator is for compile-time constant number of columns. The lifetime is the same at the data wrapped.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html#impl-FixedLexicographicalComparator%3CN%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FixedLexicographicalComparator.html" class="struct" title="struct arrow::compute::FixedLexicographicalComparator">FixedLexicographicalComparator</a>\<N\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html#method.compare" class="fn">compare</a>(&self, a_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, b_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

lexicographically compare values at the wrapped columns with given indices.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html#method.try_new" class="fn">try_new</a>( columns: &\[<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html" class="struct" title="struct arrow::compute::SortColumn">SortColumn</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FixedLexicographicalComparator.html" class="struct" title="struct arrow::compute::FixedLexicographicalComparator">FixedLexicographicalComparator</a>\<N\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new lex comparator that will wrap the given sort columns and give comparison results with two indices. The number of columns should be equal to the compile-time constant N.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html#blanket-implementations" class="anchor">§</a>
