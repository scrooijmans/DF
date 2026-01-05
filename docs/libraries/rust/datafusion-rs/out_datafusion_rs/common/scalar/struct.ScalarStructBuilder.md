# Struct ScalarStructBuilder Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/scalar/struct_builder.rs.html#29" class="src">Source</a>

``` rust
pub struct ScalarStructBuilder { /* private fields */ }
```

Expand description

Builder for [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct").

See examples on [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#impl-ScalarStructBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

Create a new `ScalarStructBuilder`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.new_null" class="fn">new_null</a>(fields: impl IntoFields) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Return a new [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct") with a single `null` value.

Note this is different from a struct where each of the specified fields are null (e.g. `{a: NULL}`)

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
let fields = vec![
   Field::new("a", DataType::Int32, false),
];
let sv = ScalarStructBuilder::new_null(fields);
// Note this is `NULL`, not `{a: NULL}`
assert_eq!(format!("{sv}"), "NULL");
```

To create a struct where the *fields* are null, use `Self::new()` and pass null values for each field:

``` rust
// make a nullable field
let field = Field::new("a", DataType::Int32, true);
// add a null value for the "a" field
let sv = ScalarStructBuilder::new()
  .with_scalar(field, ScalarValue::Int32(None))
  .build()
  .unwrap();
// value is not null, but field is
assert_eq!(format!("{sv}"), "{a:}");
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.with_array" class="fn">with_array</a>( self, field: impl IntoFieldRef, value: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

Add the specified field and [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") to the struct.

Note the array should have a single row.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.with_scalar" class="fn">with_scalar</a>( self, field: impl IntoFieldRef, value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

Add the specified field and `ScalarValue` to the struct.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.with_name_and_scalar" class="fn">with_name_and_scalar</a>( self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

Add a field with the specified name and value to the struct. the field is created with the specified data type and as non nullable

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct") with the fields and values added so far

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#errors" class="doc-anchor">§</a>Errors

If the [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") cannot be created (for example if there is a mismatch between field types and arrays) or the arrays do not have exactly one element.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#impl-Debug-for-ScalarStructBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#impl-Default-for-ScalarStructBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html" class="struct" title="struct datafusion::common::scalar::ScalarStructBuilder">ScalarStructBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html#blanket-implementations" class="anchor">§</a>
