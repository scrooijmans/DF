# Struct CategoricalMapping Copy item path

<a href="https://docs.rs/polars-dtype/0.51.0/x86_64-unknown-linux-gnu/src/polars_dtype/categorical/mapping.rs.html#14" class="src">Source</a>

``` rust
pub struct CategoricalMapping { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#impl-CategoricalMapping" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.new" class="fn">new</a>(max_categories: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.with_hasher" class="fn">with_hasher</a>( max_categories: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, hasher: <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.SeedableRandomState.html" class="struct" title="struct foldhash::quality::SeedableRandomState">SeedableRandomState</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.hasher" class="fn">hasher</a>(&self) -\> &<a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.SeedableRandomState.html" class="struct" title="struct foldhash::quality::SeedableRandomState">SeedableRandomState</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.set_max_categories" class="fn">set_max_categories</a>(&mut self, max_categories: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.get_cat" class="fn">get_cat</a>(&self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

Try to convert a string to a categorical id, but don’t insert it if it is missing.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.get_cat_with_hash" class="fn">get_cat_with_hash</a>(&self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, hash: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

Same as get_cat, but with the hash pre-computed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.insert_cat" class="fn">insert_cat</a>(&self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Convert a string to a categorical id.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.insert_cat_with_hash" class="fn">insert_cat_with_hash</a>( &self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, hash: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Same as to_cat, but with the hash pre-computed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.cat_to_str" class="fn">cat_to_str</a>(&self, cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Try to convert a categorical id to its corresponding string, returning None if the string is not in the data structure.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.cat_to_str_unchecked" class="fn">cat_to_str_unchecked</a>(&self, cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the string corresponding to a categorical id.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#safety" class="doc-anchor">§</a>Safety

The categorical id must have been returned from `to_cat`, and you must have synchronized with the call which inserted it.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.num_cats_upper_bound" class="fn">num_cats_upper_bound</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns an upper bound such that all strings inserted into the CategoricalMapping have a categorical id less than it. Note that due to parallel inserts which you have not synchronized with, there may be gaps when using `from_cat`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.len" class="fn">len</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of categories in this mapping.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.is_empty" class="fn">is_empty</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.to_arrow" class="fn">to_arrow</a>(&self, as_views: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#impl-Debug-for-CategoricalMapping" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.CategoricalMapping.html#blanket-implementations" class="anchor">§</a>
