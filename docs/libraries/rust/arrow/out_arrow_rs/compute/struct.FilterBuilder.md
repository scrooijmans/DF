# Struct FilterBuilder Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/filter.rs.html#194" class="src">Source</a>

``` rust
pub struct FilterBuilder { /* private fields */ }
```

Expand description

A builder to construct [`FilterPredicate`](https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html "struct arrow::compute::FilterPredicate")

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#impl-FilterBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html" class="struct" title="struct arrow::compute::FilterBuilder">FilterBuilder</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#method.new" class="fn">new</a>(filter: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html" class="struct" title="struct arrow::compute::FilterBuilder">FilterBuilder</a>

Create a new [`FilterBuilder`](https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html "struct arrow::compute::FilterBuilder") that can be used to construct a [`FilterPredicate`](https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html "struct arrow::compute::FilterPredicate")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#method.optimize" class="fn">optimize</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html" class="struct" title="struct arrow::compute::FilterBuilder">FilterBuilder</a>

Compute an optimised representation of the provided `filter` mask that can be applied to an array more quickly.

Note: There is limited benefit to calling this to then filter a single array Note: This will likely have a larger memory footprint than the original mask

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterPredicate.html" class="struct" title="struct arrow::compute::FilterPredicate">FilterPredicate</a>

Construct the final `FilterPredicate`

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#impl-Debug-for-FilterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html" class="struct" title="struct arrow::compute::FilterBuilder">FilterBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/struct.FilterBuilder.html#blanket-implementations" class="anchor">§</a>
