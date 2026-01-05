# Struct DataFusionErrorBuilder Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#683" class="src">Source</a>

``` rust
pub struct DataFusionErrorBuilder(/* private fields */);
```

Expand description

A builder for [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

This builder can be used to collect multiple errors and return them as a [`DataFusionError::Collection`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#variant.Collection "variant datafusion::error::DataFusionError::Collection").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#example-no-errors" class="doc-anchor">§</a>Example: no errors

``` rust
let mut builder = DataFusionError::builder();
// ok_or returns the value if no errors have been added
assert_eq!(builder.error_or(42).unwrap(), 42);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#example-with-errors" class="doc-anchor">§</a>Example: with errors

``` rust
let mut builder = DataFusionError::builder();
builder.add_error(DataFusionError::Internal("foo".to_owned()));
// ok_or returns the value if no errors have been added
assert_contains!(builder.error_or(42).unwrap_err().to_string(), "Internal error: foo");
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#impl-DataFusionErrorBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

Create a new [`DataFusionErrorBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html "struct datafusion::common::error::DataFusionErrorBuilder")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#method.add_error" class="fn">add_error</a>(&mut self, error: <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>)

Add an error to the in progress list

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let mut builder = DataFusionError::builder();
builder.add_error(DataFusionError::Internal("foo".to_owned()));
assert_contains!(builder.error_or(42).unwrap_err().to_string(), "Internal error: foo");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#method.with_error" class="fn">with_error</a>(self, error: <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

Add an error to the in progress list, returning the builder

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#example-1" class="doc-anchor">§</a>Example

``` rust
let builder = DataFusionError::builder()
  .with_error(DataFusionError::Internal("foo".to_owned()));
assert_contains!(builder.error_or(42).unwrap_err().to_string(), "Internal error: foo");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#method.error_or" class="fn">error_or</a>\<T\>(self, ok: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns `Ok(ok)` if no errors were added to the builder, otherwise returns a `Result::Err`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#impl-Debug-for-DataFusionErrorBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#impl-Default-for-DataFusionErrorBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html#blanket-implementations" class="anchor">§</a>
