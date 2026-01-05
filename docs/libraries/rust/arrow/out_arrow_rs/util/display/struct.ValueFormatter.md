# Struct ValueFormatter Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/display.rs.html#179" class="src">Source</a>

``` rust
pub struct ValueFormatter<'a> { /* private fields */ }
```

Expand description

Implements [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for a specific array value

## Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#impl-ValueFormatter%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html" class="struct" title="struct arrow::util::display::ValueFormatter">ValueFormatter</a>\<'\_\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#method.write" class="fn">write</a>(&self, s: &mut dyn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html" class="trait" title="trait core::fmt::Write">Write</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Writes this value to the provided [`Write`](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write")

Note: this ignores [`FormatOptions::with_display_error`](https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html#method.with_display_error "method arrow::util::display::FormatOptions::with_display_error") and will return an error on formatting issue

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#method.try_to_string" class="fn">try_to_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Fallibly converts this to a string

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#impl-Display-for-ValueFormatter%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html" class="struct" title="struct arrow::util::display::ValueFormatter">ValueFormatter</a>\<'\_\>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#blanket-implementations" class="anchor">§</a>
