# Struct IntervalParseConfig Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#1049" class="src">Source</a>

``` rust
pub struct IntervalParseConfig { /* private fields */ }
```

Expand description

Config to parse interval strings

Currently stores the `default_unit` to use if the string doesn’t have one specified

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#impl-IntervalParseConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html" class="struct" title="struct arrow::compute::kernels::cast_utils::IntervalParseConfig">IntervalParseConfig</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#method.new" class="fn">new</a>(default_unit: <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/enum.IntervalUnit.html" class="enum" title="enum arrow::compute::kernels::cast_utils::IntervalUnit">IntervalUnit</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html" class="struct" title="struct arrow::compute::kernels::cast_utils::IntervalParseConfig">IntervalParseConfig</a>

Create a new [IntervalParseConfig](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html "struct arrow::compute::kernels::cast_utils::IntervalParseConfig") with the given default unit

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#impl-Clone-for-IntervalParseConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html" class="struct" title="struct arrow::compute::kernels::cast_utils::IntervalParseConfig">IntervalParseConfig</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html" class="struct" title="struct arrow::compute::kernels::cast_utils::IntervalParseConfig">IntervalParseConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#impl-Debug-for-IntervalParseConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html" class="struct" title="struct arrow::compute::kernels::cast_utils::IntervalParseConfig">IntervalParseConfig</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/struct.IntervalParseConfig.html#blanket-implementations" class="anchor">§</a>
