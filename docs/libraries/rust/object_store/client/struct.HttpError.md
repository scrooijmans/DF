# Struct HttpError Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#36-40" class="src">Source</a>

``` rust
pub struct HttpError { /* private fields */ }
```

Available on **crate feature `cloud`** only.

Expand description

An HTTP protocol error

Clients should return this when an HTTP request fails to be completed, e.g. because of a connection issue. This does **not** include HTTP requests that are return non 2xx Status Codes, as these should instead be returned as an [`HttpResponse`](https://docs.rs/object_store/latest/object_store/client/type.HttpResponse.html "type object_store::client::HttpResponse") with the appropriate status code set.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#impl-HttpError" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.new" class="fn">new</a>\<E\>(kind: <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>, e: E) -\> Self

where E: <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Create a new [`HttpError`](https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html "struct object_store::client::HttpError") with the optional status code

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.kind" class="fn">kind</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

Returns the [`HttpErrorKind`](https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html "enum object_store::client::HttpErrorKind")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#impl-Debug-for-HttpError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#impl-Display-for-HttpError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, \_\_formatter: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#impl-Error-for-HttpError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html" class="struct" title="struct object_store::client::HttpError">HttpError</a>

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#131" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#141" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html#blanket-implementations" class="anchor">§</a>
