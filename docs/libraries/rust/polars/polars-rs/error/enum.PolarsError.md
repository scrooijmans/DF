# Enum PolarsError Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/lib.rs.html#82" class="src">Source</a>

``` rust
pub enum PolarsError {
Show 16 variants    AssertionError(ErrString),
    ColumnNotFound(ErrString),
    ComputeError(ErrString),
    Duplicate(ErrString),
    InvalidOperation(ErrString),
    IO {
        error: Arc<Error>,
        msg: Option<ErrString>,
    },
    NoData(ErrString),
    OutOfBounds(ErrString),
    SchemaFieldNotFound(ErrString),
    SchemaMismatch(ErrString),
    ShapeMismatch(ErrString),
    SQLInterface(ErrString),
    SQLSyntax(ErrString),
    StringCacheMismatch(ErrString),
    StructFieldNotFound(ErrString),
    Context {
        error: Box<PolarsError>,
        msg: ErrString,
    },
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.AssertionError" class="anchor">§</a>

### AssertionError(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.ColumnNotFound" class="anchor">§</a>

### ColumnNotFound(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.ComputeError" class="anchor">§</a>

### ComputeError(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.Duplicate" class="anchor">§</a>

### Duplicate(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.InvalidOperation" class="anchor">§</a>

### InvalidOperation(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.IO" class="anchor">§</a>

### IO

#### Fields

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.IO.field.error" class="anchor field">§</a>`error: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error"><code>Error</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.IO.field.msg" class="anchor field">§</a>`msg: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString"><code>ErrString</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.NoData" class="anchor">§</a>

### NoData(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.OutOfBounds" class="anchor">§</a>

### OutOfBounds(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.SchemaFieldNotFound" class="anchor">§</a>

### SchemaFieldNotFound(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.SchemaMismatch" class="anchor">§</a>

### SchemaMismatch(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.ShapeMismatch" class="anchor">§</a>

### ShapeMismatch(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.SQLInterface" class="anchor">§</a>

### SQLInterface(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.SQLSyntax" class="anchor">§</a>

### SQLSyntax(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.StringCacheMismatch" class="anchor">§</a>

### StringCacheMismatch(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.StructFieldNotFound" class="anchor">§</a>

### StructFieldNotFound(<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.Context" class="anchor">§</a>

### Context

#### Fields

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.Context.field.error" class="anchor field">§</a>`error: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError"><code>PolarsError</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#variant.Context.field.msg" class="anchor field">§</a>`msg: `<a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString"><code>ErrString</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-PolarsError" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.context_trace" class="fn">context_trace</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.wrap_msg" class="fn">wrap_msg</a>\<F\>(&self, func: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.context" class="fn">context</a>(self, msg: <a href="https://docs.rs/polars/latest/polars/error/struct.ErrString.html" class="struct" title="struct polars::error::ErrString">ErrString</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.remove_context" class="fn">remove_context</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-Clone-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-Debug-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-Display-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-Error-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#105" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#131" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#141" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CError%3E-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CError%3E-for-PolarsError-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/regex/1.11.1/x86_64-unknown-linux-gnu/regex/error/enum.Error.html" class="enum" title="enum regex::error::Error">Error</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(err: <a href="https://docs.rs/regex/1.11.1/x86_64-unknown-linux-gnu/regex/error/enum.Error.html" class="enum" title="enum regex::error::Error">Error</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CError%3E-for-PolarsError-2" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(err: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CError%3E-for-PolarsError-3" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/planus/1.1.1/x86_64-unknown-linux-gnu/planus/errors/struct.Error.html" class="struct" title="struct planus::errors::Error">Error</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(err: <a href="https://docs.rs/planus/1.1.1/x86_64-unknown-linux-gnu/planus/errors/struct.Error.html" class="struct" title="struct planus::errors::Error">Error</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CInfallible%3E-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html" class="enum" title="enum core::convert::Infallible">Infallible</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(\_: <a href="https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html" class="enum" title="enum core::convert::Infallible">Infallible</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CParquetError%3E-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/error/enum.ParquetError.html" class="enum" title="enum polars_parquet::parquet::error::ParquetError">ParquetError</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/error/enum.ParquetError.html" class="enum" title="enum polars_parquet::parquet::error::ParquetError">ParquetError</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CTryReserveError%3E-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html" class="struct" title="struct alloc::collections::TryReserveError">TryReserveError</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html" class="struct" title="struct alloc::collections::TryReserveError">TryReserveError</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#impl-From%3CUtf8Error%3E-for-PolarsError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/simdutf8/0.1.5/x86_64-unknown-linux-gnu/simdutf8/basic/struct.Utf8Error.html" class="struct" title="struct simdutf8::basic::Utf8Error">Utf8Error</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/simdutf8/0.1.5/x86_64-unknown-linux-gnu/simdutf8/basic/struct.Utf8Error.html" class="struct" title="struct simdutf8::basic::Utf8Error">Utf8Error</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/error/enum.PolarsError.html#blanket-implementations" class="anchor">§</a>
