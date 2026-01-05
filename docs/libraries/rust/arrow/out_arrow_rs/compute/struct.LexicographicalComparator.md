# Struct LexicographicalComparator Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#991" class="src">Source</a>

``` rust
pub struct LexicographicalComparator { /* private fields */ }
```

Expand description

A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data at given two indices. The lifetime is the same at the data wrapped.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html#impl-LexicographicalComparator" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html" class="struct" title="struct arrow::compute::LexicographicalComparator">LexicographicalComparator</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html#method.compare" class="fn">compare</a>(&self, a_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, b_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

lexicographically compare values at the wrapped columns with given indices.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html#method.try_new" class="fn">try_new</a>( columns: &\[<a href="https://docs.rs/arrow/latest/arrow/compute/struct.SortColumn.html" class="struct" title="struct arrow::compute::SortColumn">SortColumn</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html" class="struct" title="struct arrow::compute::LexicographicalComparator">LexicographicalComparator</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new lex comparator that will wrap the given sort columns and give comparison results with two indices.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.LexicographicalComparator.html#blanket-implementations" class="anchor">§</a>
