# Struct FilterPredicate Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/filter.rs.html#291" class="src">Source</a>

``` rust
pub struct FilterPredicate { /* private fields */ }
```

Expand description

A filtering predicate that can be applied to an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#impl-FilterPredicate" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html" class="struct" title="struct arrow::compute::FilterPredicate">FilterPredicate</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#method.filter" class="fn">filter</a>(&self, values: &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Selects rows from `values` based on this [`FilterPredicate`](https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html "struct arrow::compute::FilterPredicate")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#method.count" class="fn">count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Number of rows being selected based on this [`FilterPredicate`](https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html "struct arrow::compute::FilterPredicate")

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#impl-Debug-for-FilterPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html" class="struct" title="struct arrow::compute::FilterPredicate">FilterPredicate</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html#blanket-implementations" class="anchor">§</a>
