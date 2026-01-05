# Trait GetAnyValue Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/any_value.rs.html#1459" class="src">Source</a>

``` rust
pub trait GetAnyValue {
    // Required method
    unsafe fn get_unchecked(&self, index: usize) -> AnyValue<'_>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#required-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#tymethod.get_unchecked" class="fn">get_unchecked</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#safety" class="doc-anchor">§</a>Safety

Get an value without doing bound checks.

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#impl-GetAnyValue-for-Box%3Cdyn+Array%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html" class="trait" title="trait polars::prelude::GetAnyValue">GetAnyValue</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#method.get_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#tymethod.get_unchecked" class="fn">get_unchecked</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.GetAnyValue.html#implementors" class="anchor">§</a>
