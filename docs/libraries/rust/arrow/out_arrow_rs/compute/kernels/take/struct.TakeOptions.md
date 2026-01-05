# Struct TakeOptions Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/take.rs.html#357" class="src">Source</a>

``` rust
pub struct TakeOptions {
    pub check_bounds: bool,
}
```

Expand description

Options that define how `take` should behave

## Fields<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#structfield.check_bounds" class="anchor field">§</a>`check_bounds: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Perform bounds check before taking indices from values. If enabled, an `ArrowError` is returned if the indices are out of bounds. If not enabled, and indices exceed bounds, the kernel will panic.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#impl-Clone-for-TakeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.TakeOptions.html" class="struct" title="struct arrow::compute::TakeOptions">TakeOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.TakeOptions.html" class="struct" title="struct arrow::compute::TakeOptions">TakeOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#impl-Debug-for-TakeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.TakeOptions.html" class="struct" title="struct arrow::compute::TakeOptions">TakeOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#impl-Default-for-TakeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.TakeOptions.html" class="struct" title="struct arrow::compute::TakeOptions">TakeOptions</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.TakeOptions.html" class="struct" title="struct arrow::compute::TakeOptions">TakeOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/struct.TakeOptions.html#blanket-implementations" class="anchor">§</a>
