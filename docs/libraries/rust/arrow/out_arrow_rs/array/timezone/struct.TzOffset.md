# Struct TzOffset Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/timezone.rs.html#61" class="src">Source</a>

``` rust
pub struct TzOffset { /* private fields */ }
```

Expand description

An [`Offset`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.Offset.html "trait chrono::offset::Offset") for [`Tz`](https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html "struct arrow::array::timezone::Tz")

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#impl-Clone-for-TzOffset" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#impl-Debug-for-TzOffset" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#impl-Display-for-TzOffset" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#impl-Offset-for-TzOffset" class="anchor">§</a>

### impl <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.Offset.html" class="trait" title="trait chrono::offset::Offset">Offset</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#method.fix" class="anchor">§</a>

#### fn <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/trait.Offset.html#tymethod.fix" class="fn">fix</a>(&self) -\> <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/fixed/struct.FixedOffset.html" class="struct" title="struct chrono::offset::fixed::FixedOffset">FixedOffset</a>

Returns the fixed offset from UTC to the local time stored.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#impl-Copy-for-TzOffset" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html" class="struct" title="struct arrow::array::timezone::TzOffset">TzOffset</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.TzOffset.html#blanket-implementations" class="anchor">§</a>
