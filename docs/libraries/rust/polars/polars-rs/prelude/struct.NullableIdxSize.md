# Struct NullableIdxSize Copy item path

<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/src/polars_utils/index.rs.html#25" class="src">Source</a>

``` rust
#[repr(transparent)]pub struct NullableIdxSize {
    pub inner: u32,
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#structfield.inner" class="anchor field">§</a>`inner: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.is_null_idx" class="fn">is_null_idx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.null" class="fn">null</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.idx" class="fn">idx</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.to_opt" class="fn">to_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-Clone-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-Debug-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-From%3Cu32%3E-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-PartialEq-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-Zeroable-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html" class="trait" title="trait bytemuck::zeroable::Zeroable">Zeroable</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#method.zeroed" class="anchor">§</a>

#### fn <a href="https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed" class="fn">zeroed</a>() -\> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-AnyBitPattern-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/anybitpattern/trait.AnyBitPattern.html" class="trait" title="trait bytemuck::anybitpattern::AnyBitPattern">AnyBitPattern</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-Copy-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-Eq-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#impl-NoUninit-for-NullableIdxSize" class="anchor">§</a>

### impl <a href="https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/no_uninit/trait.NoUninit.html" class="trait" title="trait bytemuck::no_uninit::NoUninit">NoUninit</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html" class="struct" title="struct polars::prelude::NullableIdxSize">NullableIdxSize</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NullableIdxSize.html#blanket-implementations" class="anchor">§</a>
