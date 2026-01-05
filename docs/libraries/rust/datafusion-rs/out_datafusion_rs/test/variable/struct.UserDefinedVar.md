# Struct UserDefinedVar Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/variable.rs.html#50" class="src">Source</a>

``` rust
pub struct UserDefinedVar {}
```

Available on **non-WebAssembly** only.

Expand description

user defined variable

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#impl-UserDefinedVar" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html" class="struct" title="struct datafusion::test::variable::UserDefinedVar">UserDefinedVar</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#method.new" class="fn">new</a>() -\> Self

new user defined variable

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#impl-Debug-for-UserDefinedVar" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html" class="struct" title="struct datafusion::test::variable::UserDefinedVar">UserDefinedVar</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#impl-Default-for-UserDefinedVar" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html" class="struct" title="struct datafusion::test::variable::UserDefinedVar">UserDefinedVar</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html" class="struct" title="struct datafusion::test::variable::UserDefinedVar">UserDefinedVar</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#impl-VarProvider-for-UserDefinedVar" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html" class="struct" title="struct datafusion::test::variable::UserDefinedVar">UserDefinedVar</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#method.get_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#tymethod.get_value" class="fn">get_value</a>(&self, var_names: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Get user defined variable value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#method.get_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#tymethod.get_type" class="fn">get_type</a>(&self, var_names: &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>

Return the type of the given variable

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html#blanket-implementations" class="anchor">§</a>
