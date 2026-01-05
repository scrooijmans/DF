# Enum SortOrderBuilderError Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/sort.rs.html#99" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum SortOrderBuilderError {
    UninitializedField(&'static str),
    ValidationError(String),
}
```

Expand description

Error type for SortOrderBuilder

## Variants (Non-exhaustive)<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#variant.UninitializedField" class="anchor">§</a>

### UninitializedField(&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Uninitialized field

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#variant.ValidationError" class="anchor">§</a>

### ValidationError(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Custom validation error

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#impl-Debug-for-SortOrderBuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html" class="enum" title="enum iceberg::spec::SortOrderBuilderError">SortOrderBuilderError</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#impl-Display-for-SortOrderBuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html" class="enum" title="enum iceberg::spec::SortOrderBuilderError">SortOrderBuilderError</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#impl-Error-for-SortOrderBuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html" class="enum" title="enum iceberg::spec::SortOrderBuilderError">SortOrderBuilderError</a>

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#111" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#impl-From%3CString%3E-for-SortOrderBuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html" class="enum" title="enum iceberg::spec::SortOrderBuilderError">SortOrderBuilderError</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#impl-From%3CUninitializedFieldError%3E-for-SortOrderBuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/derive_builder/0.20.2/x86_64-unknown-linux-gnu/derive_builder/error/struct.UninitializedFieldError.html" class="struct" title="struct derive_builder::error::UninitializedFieldError">UninitializedFieldError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html" class="enum" title="enum iceberg::spec::SortOrderBuilderError">SortOrderBuilderError</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://docs.rs/derive_builder/0.20.2/x86_64-unknown-linux-gnu/derive_builder/error/struct.UninitializedFieldError.html" class="struct" title="struct derive_builder::error::UninitializedFieldError">UninitializedFieldError</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html#blanket-implementations" class="anchor">§</a>
