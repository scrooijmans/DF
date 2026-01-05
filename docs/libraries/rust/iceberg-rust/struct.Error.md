# Struct Error Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/error.rs.html#133-143" class="src">Source</a>

``` rust
pub struct Error { /* private fields */ }
```

Expand description

Error is the error struct returned by all iceberg functions.

### <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#display" class="doc-anchor">§</a>Display

Error can be displayed in two ways:

- Via `Display`: like `err.to_string()` or `format!("{err}")`

Error will be printed in a single line:

``` shell
Unexpected, context: { path: /path/to/file, called: send_async } => something wrong happened, source: networking error"
```

- Via `Debug`: like `format!("{err:?}")`

Error will be printed in multi lines with more details and backtraces (if captured):

``` shell
Unexpected => something wrong happened

Context:
   path: /path/to/file
   called: send_async

Source: networking error

Backtrace:
   0: iceberg::error::Error::new
             at ./src/error.rs:197:24
   1: iceberg::error::tests::generate_error
             at ./src/error.rs:241:9
   2: iceberg::error::tests::test_error_debug_with_backtrace::{{closure}}
             at ./src/error.rs:305:41
   ...
```

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-Error" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.new" class="fn">new</a>(kind: <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>, message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Create a new Error with error kind and message.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.with_retryable" class="fn">with_retryable</a>(self, retryable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set retryable of the error.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.with_context" class="fn">with_context</a>(self, key: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Add more context in error.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.with_source" class="fn">with_source</a>(self, src: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/anyhow/1.0.99/x86_64-unknown-linux-gnu/anyhow/struct.Error.html" class="struct" title="struct anyhow::Error">Error</a>\>) -\> Self

Set source for error.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#notes" class="doc-anchor">§</a>Notes

If the source has been set, we will raise a panic here.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.backtrace" class="fn">backtrace</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/backtrace/struct.Backtrace.html" class="struct" title="struct std::backtrace::Backtrace">Backtrace</a>

Return error’s backtrace.

Note: the standard way of exposing backtrace is the unstable feature [`error_generic_member_access`](https://github.com/rust-lang/rust/issues/99301). We don’t provide it as it requires nightly rust.

If you just want to print error with backtrace, use `Debug`, like `format!("{err:?}")`.

If you use nightly rust, and want to access `iceberg::Error`’s backtrace in the standard way, you can implement a new type like this:

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
// assume you already have `#![feature(error_generic_member_access)]` on the top of your crate

#[derive(::std::fmt::Debug)]
pub struct IcebergError(iceberg::Error);

impl std::fmt::Display for IcebergError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for IcebergError {
    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {
        request.provide_ref::<std::backtrace::Backtrace>(self.0.backtrace());
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
```

Additionally, you can add a clippy lint to prevent usage of the original `iceberg::Error` type.

``` toml
disallowed-types = [
    { path = "iceberg::Error", reason = "Please use `my_crate::IcebergError` instead." },
]
```

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.kind" class="fn">kind</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

Return error’s kind.

Users can use this method to check error’s kind and take actions.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.retryable" class="fn">retryable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return error’s retryable status

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.message" class="fn">message</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return error’s message.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-Debug-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-Display-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-Error-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CArrowError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/error/enum.ArrowError.html" class="enum" title="enum arrow_schema::error::ArrowError">ArrowError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/error/enum.ArrowError.html" class="enum" title="enum arrow_schema::error::ArrowError">ArrowError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/error/struct.Error.html" class="struct" title="struct uuid::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/error/struct.Error.html" class="struct" title="struct uuid::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/apache-avro/0.20.0/x86_64-unknown-linux-gnu/apache_avro/error/struct.Error.html" class="struct" title="struct apache_avro::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/apache-avro/0.20.0/x86_64-unknown-linux-gnu/apache_avro/error/struct.Error.html" class="struct" title="struct apache_avro::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error-2" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/opendal/0.54.0/x86_64-unknown-linux-gnu/opendal/types/error/struct.Error.html" class="struct" title="struct opendal::types::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/opendal/0.54.0/x86_64-unknown-linux-gnu/opendal/types/error/struct.Error.html" class="struct" title="struct opendal::types::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error-3" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/error/struct.Error.html" class="struct" title="struct reqwest::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/error/struct.Error.html" class="struct" title="struct reqwest::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error-4" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/serde_json/1.0.143/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html" class="struct" title="struct serde_json::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/serde_json/1.0.143/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html" class="struct" title="struct serde_json::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error-5" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/error/enum.Error.html" class="enum" title="enum rust_decimal::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/rust_decimal/1.38.0/x86_64-unknown-linux-gnu/rust_decimal/error/enum.Error.html" class="enum" title="enum rust_decimal::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CError%3E-for-Error-6" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CParquetError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/parquet/55.2.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CParseError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/format/struct.ParseError.html" class="struct" title="struct chrono::format::ParseError">ParseError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/format/struct.ParseError.html" class="struct" title="struct chrono::format::ParseError">ParseError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CParseError%3E-for-Error-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/parser/enum.ParseError.html" class="enum" title="enum url::parser::ParseError">ParseError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/parser/enum.ParseError.html" class="enum" title="enum url::parser::ParseError">ParseError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CParseIntError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/error/struct.ParseIntError.html" class="struct" title="struct core::num::error::ParseIntError">ParseIntError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/core/num/error/struct.ParseIntError.html" class="struct" title="struct core::num::error::ParseIntError">ParseIntError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CSendError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/futures-channel/0.3.31/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.SendError.html" class="struct" title="struct futures_channel::mpsc::SendError">SendError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/futures-channel/0.3.31/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.SendError.html" class="struct" title="struct futures_channel::mpsc::SendError">SendError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CTryFromIntError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html" class="struct" title="struct core::num::error::TryFromIntError">TryFromIntError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html" class="struct" title="struct core::num::error::TryFromIntError">TryFromIntError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CTryFromSliceError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/array/struct.TryFromSliceError.html" class="struct" title="struct core::array::TryFromSliceError">TryFromSliceError</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/core/array/struct.TryFromSliceError.html" class="struct" title="struct core::array::TryFromSliceError">TryFromSliceError</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#impl-From%3CUtf8Error%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/str/error/struct.Utf8Error.html" class="struct" title="struct core::str::error::Utf8Error">Utf8Error</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/core/str/error/struct.Utf8Error.html" class="struct" title="struct core::str::error::Utf8Error">Utf8Error</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html#blanket-implementations" class="anchor">§</a>
