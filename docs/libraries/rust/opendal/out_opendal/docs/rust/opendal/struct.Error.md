# Struct Error Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/error.rs.html#223-233" class="src">Source</a>

``` rust
pub struct Error { /* private fields */ }
```

Expand description

Error is the error struct returned by all opendal functions.

### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#display" class="doc-anchor">Â§</a>Display

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
   0: opendal::error::Error::new
             at ./src/error.rs:197:24
   1: opendal::error::tests::generate_error
             at ./src/error.rs:241:9
   2: opendal::error::tests::test_error_debug_with_backtrace::{{closure}}
             at ./src/error.rs:305:41
   ...
```

- For conventional struct-style Debug representation, like `format!("{err:#?}")`:

``` shell
Error {
    kind: Unexpected,
    message: "something wrong happened",
    status: Permanent,
    operation: "Read",
    context: [
        (
            "path",
            "/path/to/file",
        ),
        (
            "called",
            "send_async",
        ),
    ],
    source: Some(
        "networking error",
    ),
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#impl-Error" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.new" class="fn">new</a>(kind: <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>, message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Create a new Error with error kind and message.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.with_operation" class="fn">with_operation</a>(self, operation: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> Self

Update errorâ€™s operation.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#notes" class="doc-anchor">Â§</a>Notes

If the error already carries an operation, we will push a new context `(called, operation)`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.with_context" class="fn">with_context</a>(self, key: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>) -\> Self

Add more context in error.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.set_source" class="fn">set_source</a>(self, src: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/anyhow/1.0.100/anyhow/struct.Error.html" class="struct" title="struct anyhow::Error">Error</a>\>) -\> Self

Set source for error.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#notes-1" class="doc-anchor">Â§</a>Notes

If the source has been set, we will raise a panic here.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.map" class="fn">map</a>\<F\>(self, f: F) -\> Self

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(Self) -\> Self,

Operate on error with map.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.set_permanent" class="fn">set_permanent</a>(self) -\> Self

Set permanent status for error.

By set permanent, we indicate the retry must be stopped.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.set_temporary" class="fn">set_temporary</a>(self) -\> Self

Set temporary status for error.

By set temporary, we indicate this error is retryable.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.set_persistent" class="fn">set_persistent</a>(self) -\> Self

Set persistent status for error.

By setting persistent, we indicate the retry should be stopped.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.with_permanent" class="fn">with_permanent</a>(self, permanent: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set permanent status for error by given permanent.

By set permanent, we indicate the retry must be stopped.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.with_temporary" class="fn">with_temporary</a>(self, temporary: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set temporary status for error by given temporary.

By set temporary, we indicate this error is retryable.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.with_persistent" class="fn">with_persistent</a>(self, persistent: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set persistent status for error by given persistent.

By set persistent, we indicate the retry should be stopped.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.kind" class="fn">kind</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

Return errorâ€™s kind.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.is_permanent" class="fn">is_permanent</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this error is permanent.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.is_temporary" class="fn">is_temporary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this error is temporary.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.is_persistent" class="fn">is_persistent</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this error is persistent.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.backtrace" class="fn">backtrace</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/backtrace/struct.Backtrace.html" class="struct" title="struct std::backtrace::Backtrace">Backtrace</a>\>

Return errorâ€™s backtrace.

Note: the standard way of exposing backtrace is the unstable feature [`error_generic_member_access`](https://github.com/rust-lang/rust/issues/99301). We donâ€™t provide it as it requires nightly rust.

If you just want to print error with backtrace, use `Debug`, like `format!("{err:?}")`.

If you use nightly rust, and want to access `opendal::Error`â€™s backtrace in the standard way, you can implement a newtype like this:

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
// assume you already have `#![feature(error_generic_member_access)]` on the top of your crate

#[derive(::std::fmt::Debug)]
pub struct OpendalError(opendal::Error);

impl std::fmt::Display for OpendalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for OpendalError {
    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {
        if let Some(bt) = self.0.backtrace() {
            request.provide_ref::<std::backtrace::Backtrace>(bt);
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
```

Additionally, you can add a clippy lint to prevent usage of the original `opendal::Error` type.

``` toml
disallowed-types = [
    { path = "opendal::Error", reason = "Please use `my_crate::OpendalError` instead." },
]
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#impl-Debug-for-Error" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#impl-Display-for-Error" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#impl-Error-for-Error" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.source" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.description" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

ðŸ‘ŽDeprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.cause" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

ðŸ‘ŽDeprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.provide" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

ðŸ”¬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#impl-From%3CError%3E-for-Error" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>\> for <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(err: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html#blanket-implementations" class="anchor">Â§</a>
