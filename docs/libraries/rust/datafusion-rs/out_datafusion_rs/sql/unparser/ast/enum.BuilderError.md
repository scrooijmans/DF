# Enum BuilderError Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/unparser/ast.rs.html#697" class="src">Source</a>

``` rust
pub enum BuilderError {
    UninitializedField(&'static str),
    ValidationError(String),
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#variant.UninitializedField" class="anchor">§</a>

### UninitializedField(&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#variant.ValidationError" class="anchor">§</a>

### ValidationError(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#impl-Debug-for-BuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#impl-Display-for-BuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#impl-Error-for-BuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#111" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#impl-From%3CBuilderError%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#impl-From%3CString%3E-for-BuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#impl-From%3CUninitializedFieldError%3E-for-BuilderError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UninitializedFieldError.html" class="struct" title="struct datafusion::sql::unparser::ast::UninitializedFieldError">UninitializedFieldError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(s: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UninitializedFieldError.html" class="struct" title="struct datafusion::sql::unparser::ast::UninitializedFieldError">UninitializedFieldError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html#blanket-implementations" class="anchor">§</a>
