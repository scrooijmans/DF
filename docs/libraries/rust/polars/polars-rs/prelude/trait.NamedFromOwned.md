# Trait NamedFromOwned Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/named_from.rs.html#20" class="src">Source</a>

``` rust
pub trait NamedFromOwned<T> {
    // Required method
    fn from_vec(name: PlSmallStr, _: T) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#tymethod.from_vec" class="fn">from_vec</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, \_: T) -\> Self

Initialize by name and values.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Cf32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Cf64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Ci8%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Ci16%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Ci32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Ci64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Ci128%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Cu8%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Cu16%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Cu32%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html#impl-NamedFromOwned%3CVec%3Cu64%3E%3E-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NamedFromOwned.html" class="trait" title="trait polars::prelude::NamedFromOwned">NamedFromOwned</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
