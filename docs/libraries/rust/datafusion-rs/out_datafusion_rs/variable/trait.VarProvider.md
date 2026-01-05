# Trait VarProvider Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/var_provider.rs.html#33" class="src">Source</a>

``` rust
pub trait VarProvider: Debug {
    // Required methods
    fn get_value(
        &self,
        var_names: Vec<String>,
    ) -> Result<ScalarValue, DataFusionError>;
    fn get_type(&self, var_names: &[String]) -> Option<DataType>;
}
```

Expand description

A var provider for `@variable` and `@@variable` runtime values.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#tymethod.get_value" class="fn">get_value</a>( &self, var_names: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get variable value

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#tymethod.get_type" class="fn">get_type</a>(&self, var_names: &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>

Return the type of the given variable

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#impl-VarProvider-for-SystemVar" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.SystemVar.html" class="struct" title="struct datafusion::test::variable::SystemVar">SystemVar</a>

Available on **non-WebAssembly** only.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html#impl-VarProvider-for-UserDefinedVar" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/struct.UserDefinedVar.html" class="struct" title="struct datafusion::test::variable::UserDefinedVar">UserDefinedVar</a>

Available on **non-WebAssembly** only.
