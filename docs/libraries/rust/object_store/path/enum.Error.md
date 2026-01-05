# Enum Error Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/path/mod.rs.html#39-89" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum Error {
    EmptySegment {
        path: String,
    },
    BadSegment {
        path: String,
        source: InvalidPart,
    },
    Canonicalize {
        path: PathBuf,
        source: Error,
    },
    InvalidPath {
        path: PathBuf,
    },
    NonUnicode {
        path: String,
        source: Utf8Error,
    },
    PrefixMismatch {
        path: String,
        prefix: String,
    },
}
```

Expand description

Error returned by [`Path::parse`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.parse "associated function object_store::path::Path::parse")

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.EmptySegment" class="anchor">§</a>

### EmptySegment

Error when there’s an empty segment between two slashes `/` in the path

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.EmptySegment.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The source path

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.BadSegment" class="anchor">§</a>

### BadSegment

Error when an invalid segment is encountered in the given path

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.BadSegment.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The source path

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.BadSegment.field.source" class="anchor field">§</a>`source: `<a href="https://docs.rs/object_store/latest/object_store/path/struct.InvalidPart.html" class="struct" title="struct object_store::path::InvalidPart"><code>InvalidPart</code></a>

The part containing the error

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.Canonicalize" class="anchor">§</a>

### Canonicalize

Error when path cannot be canonicalized

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.Canonicalize.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf"><code>PathBuf</code></a>

The source path

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.Canonicalize.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error"><code>Error</code></a>

The underlying error

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.InvalidPath" class="anchor">§</a>

### InvalidPath

Error when the path is not a valid URL

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.InvalidPath.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf"><code>PathBuf</code></a>

The source path

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.NonUnicode" class="anchor">§</a>

### NonUnicode

Error when a path contains non-unicode characters

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.NonUnicode.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The source path

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.NonUnicode.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/core/str/error/struct.Utf8Error.html" class="struct" title="struct core::str::error::Utf8Error"><code>Utf8Error</code></a>

The underlying `UTF8Error`

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.PrefixMismatch" class="anchor">§</a>

### PrefixMismatch

Error when the a path doesn’t start with given prefix

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.PrefixMismatch.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The source path

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#variant.PrefixMismatch.field.prefix" class="anchor field">§</a>`prefix: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The mismatched prefix

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#impl-Debug-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#impl-Display-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, \_\_formatter: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#impl-Error-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#131" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#141" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#impl-From%3CError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\> for <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html#blanket-implementations" class="anchor">§</a>
