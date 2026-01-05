# Trait CatNative Copy item path

<a href="https://docs.rs/polars-dtype/0.51.0/x86_64-unknown-linux-gnu/src/polars_dtype/categorical/catsize.rs.html#3" class="src">Source</a>

``` rust
pub trait CatNative {
    // Required methods
    fn as_cat(&self) -> u32;
    fn from_cat(cat: u32) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.as_cat" class="fn">as_cat</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.from_cat" class="fn">from_cat</a>(cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#impl-CatNative-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.CatNative.html" class="trait" title="trait polars::prelude::CatNative">CatNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#method.as_cat" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.as_cat" class="fn">as_cat</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#method.from_cat" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.from_cat" class="fn">from_cat</a>(cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#impl-CatNative-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.CatNative.html" class="trait" title="trait polars::prelude::CatNative">CatNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#method.as_cat-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.as_cat" class="fn">as_cat</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#method.from_cat-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.from_cat" class="fn">from_cat</a>(cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#impl-CatNative-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.CatNative.html" class="trait" title="trait polars::prelude::CatNative">CatNative</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#method.as_cat-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.as_cat" class="fn">as_cat</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#method.from_cat-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#tymethod.from_cat" class="fn">from_cat</a>(cat: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/datatypes/trait.CatNative.html#implementors" class="anchor">§</a>
