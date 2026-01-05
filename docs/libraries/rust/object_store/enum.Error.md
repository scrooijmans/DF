# Enum Error Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1318-1426" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum Error {
    Generic {
        store: &'static str,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    NotFound {
        path: String,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    InvalidPath {
        source: Error,
    },
    JoinError {
        source: JoinError,
    },
    NotSupported {
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    AlreadyExists {
        path: String,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    Precondition {
        path: String,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    NotModified {
        path: String,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    NotImplemented,
    PermissionDenied {
        path: String,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    Unauthenticated {
        path: String,
        source: Box<dyn Error + Send + Sync + 'static>,
    },
    UnknownConfigurationKey {
        store: &'static str,
        key: String,
    },
}
```

Expand description

A specialized `Error` for object store-related errors

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Generic" class="anchor">§</a>

### Generic

A fallback error type when no variant matches

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Generic.field.store" class="anchor field">§</a>`store: &'static `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

The store this error originated from

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Generic.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotFound" class="anchor">§</a>

### NotFound

Error when the object is not found at given location

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotFound.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path to file

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotFound.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.InvalidPath" class="anchor">§</a>

### InvalidPath

Error for invalid path

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.InvalidPath.field.source" class="anchor field">§</a>`source: `<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error"><code>Error</code></a>

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.JoinError" class="anchor">§</a>

### JoinError

Error when `tokio::spawn` failed

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.JoinError.field.source" class="anchor field">§</a>`source: `<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError"><code>JoinError</code></a>

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotSupported" class="anchor">§</a>

### NotSupported

Error when the attempted operation is not supported

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotSupported.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.AlreadyExists" class="anchor">§</a>

### AlreadyExists

Error when the object already exists

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.AlreadyExists.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path to the

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.AlreadyExists.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Precondition" class="anchor">§</a>

### Precondition

Error when the required conditions failed for the operation

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Precondition.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path to the file

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Precondition.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotModified" class="anchor">§</a>

### NotModified

Error when the object at the location isn’t modified

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotModified.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path to the file

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotModified.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotImplemented" class="anchor">§</a>

### NotImplemented

Error when an operation is not implemented

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.PermissionDenied" class="anchor">§</a>

### PermissionDenied

Error when the used credentials don’t have enough permission to perform the requested operation

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.PermissionDenied.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path to the file

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.PermissionDenied.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Unauthenticated" class="anchor">§</a>

### Unauthenticated

Error when the used credentials lack valid authentication

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Unauthenticated.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path to the file

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Unauthenticated.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<dyn `<a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error"><code>Error</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + 'static>`

The wrapped error

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.UnknownConfigurationKey" class="anchor">§</a>

### UnknownConfigurationKey

Error when a configuration key is invalid for the store used

#### Fields

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.UnknownConfigurationKey.field.store" class="anchor field">§</a>`store: &'static `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

The object store used

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.UnknownConfigurationKey.field.key" class="anchor field">§</a>`key: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The configuration key used

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#impl-Debug-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#impl-Display-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, \_\_formatter: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#impl-Error-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#131" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#141" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#impl-From%3CError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\> for <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#impl-From%3CError%3E-for-Error-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\> for <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#impl-From%3CJoinError%3E-for-Error" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\> for <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(source: <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html#blanket-implementations" class="anchor">§</a>
