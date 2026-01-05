# Struct InvalidPart Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/path/parts.rs.html#31-34" class="src">Source</a>

``` rust
pub struct InvalidPart { /* private fields */ }
```

Expand description

Error returned by [`PathPart::parse`](https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html#method.parse "associated function object_store::path::PathPart::parse")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#impl-Debug-for-InvalidPart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html" class="struct" title="struct object_store::path::InvalidPart">InvalidPart</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#impl-Display-for-InvalidPart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html" class="struct" title="struct object_store::path::InvalidPart">InvalidPart</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, \_\_formatter: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#impl-Error-for-InvalidPart" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html" class="struct" title="struct object_store::path::InvalidPart">InvalidPart</a>

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#105" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#131" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#141" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html#blanket-implementations" class="anchor">§</a>
