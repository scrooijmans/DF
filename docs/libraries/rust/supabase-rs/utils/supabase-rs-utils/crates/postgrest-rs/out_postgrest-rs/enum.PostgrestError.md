# Enum PostgrestError Copy item path

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/src/supabase_rust_postgrest/lib.rs.html#59-89" class="src">Source</a>

``` rust
pub enum PostgrestError {
    ApiError {
        details: PostgrestApiErrorDetails,
        status: StatusCode,
    },
    UnparsedApiError {
        message: String,
        status: StatusCode,
    },
    NetworkError(Error),
    UrlParseError(ParseError),
    SerializationError(Error),
    InvalidParameters(String),
    TransactionError(String),
    DeserializationError(String),
}
```

Expand description

エラー型

## Variants<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.ApiError" class="anchor">§</a>

### ApiError

#### Fields

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.ApiError.field.details" class="anchor field">§</a>`details: `<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestApiErrorDetails.html" class="struct" title="struct supabase_rust_postgrest::PostgrestApiErrorDetails"><code>PostgrestApiErrorDetails</code></a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.ApiError.field.status" class="anchor field">§</a>`status: `<a href="https://docs.rs/http/0.2.12/x86_64-unknown-linux-gnu/http/status/struct.StatusCode.html" class="struct" title="struct http::status::StatusCode"><code>StatusCode</code></a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.UnparsedApiError" class="anchor">§</a>

### UnparsedApiError

#### Fields

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.UnparsedApiError.field.message" class="anchor field">§</a>`message: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.UnparsedApiError.field.status" class="anchor field">§</a>`status: `<a href="https://docs.rs/http/0.2.12/x86_64-unknown-linux-gnu/http/status/struct.StatusCode.html" class="struct" title="struct http::status::StatusCode"><code>StatusCode</code></a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.NetworkError" class="anchor">§</a>

### NetworkError(<a href="https://docs.rs/reqwest/0.11.27/x86_64-unknown-linux-gnu/reqwest/error/struct.Error.html" class="struct" title="struct reqwest::error::Error">Error</a>)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.UrlParseError" class="anchor">§</a>

### UrlParseError(<a href="https://docs.rs/url/2.5.4/x86_64-unknown-linux-gnu/url/parser/enum.ParseError.html" class="enum" title="enum url::parser::ParseError">ParseError</a>)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.SerializationError" class="anchor">§</a>

### SerializationError(<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html" class="struct" title="struct serde_json::error::Error">Error</a>)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.InvalidParameters" class="anchor">§</a>

### InvalidParameters(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.TransactionError" class="anchor">§</a>

### TransactionError(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#variant.DeserializationError" class="anchor">§</a>

### DeserializationError(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

## Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#impl-Debug-for-PostgrestError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#impl-Display-for-PostgrestError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, \_\_formatter: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#impl-Error-for-PostgrestError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#impl-From%3CError%3E-for-PostgrestError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/reqwest/0.11.27/x86_64-unknown-linux-gnu/reqwest/error/struct.Error.html" class="struct" title="struct reqwest::error::Error">Error</a>\> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/reqwest/0.11.27/x86_64-unknown-linux-gnu/reqwest/error/struct.Error.html" class="struct" title="struct reqwest::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#impl-From%3CError%3E-for-PostgrestError-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html" class="struct" title="struct serde_json::error::Error">Error</a>\> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html" class="struct" title="struct serde_json::error::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#impl-From%3CParseError%3E-for-PostgrestError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/url/2.5.4/x86_64-unknown-linux-gnu/url/parser/enum.ParseError.html" class="enum" title="enum url::parser::ParseError">ParseError</a>\> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/url/2.5.4/x86_64-unknown-linux-gnu/url/parser/enum.ParseError.html" class="enum" title="enum url::parser::ParseError">ParseError</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html#blanket-implementations" class="anchor">§</a>
