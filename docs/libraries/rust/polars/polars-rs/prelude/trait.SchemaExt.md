# Trait SchemaExt Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/schema/mod.rs.html#14" class="src">Source</a>

``` rust
pub trait SchemaExt {
    // Required methods
    fn from_arrow_schema(value: &Schema<Field>) -> Self;
    fn get_field(&self, name: &str) -> Option<Field>;
    fn try_get_field(&self, name: &str) -> Result<Field, PolarsError>;
    fn to_arrow(&self, compat_level: CompatLevel) -> Schema<Field>;
    fn iter_fields(&self) -> impl ExactSizeIterator;
    fn to_supertype(
        &mut self,
        other: &Schema<DataType>,
    ) -> Result<bool, PolarsError>;
    fn project_select(&self, select: &Bitmap) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.from_arrow_schema" class="fn">from_arrow_schema</a>(value: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>) -\> Self

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.get_field" class="fn">get_field</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.try_get_field" class="fn">try_get_field</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.to_arrow" class="fn">to_arrow</a>(&self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>) -\> <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.iter_fields" class="fn">iter_fields</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.to_supertype" class="fn">to_supertype</a>( &mut self, other: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.project_select" class="fn">project_select</a>(&self, select: &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>) -\> Self

Select fields using a bitmap.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#impl-SchemaExt-for-Schema%3CDataType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html" class="trait" title="trait polars::prelude::SchemaExt">SchemaExt</a> for <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.get_field" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.get_field" class="fn">get_field</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>

Look up the name in the schema and return an owned [`Field`](https://docs.rs/polars/latest/polars/prelude/struct.Field.html "struct polars::prelude::Field") by cloning the data.

Returns `None` if the field does not exist.

This method constructs the `Field` by cloning the name and dtype. For a version that returns references, see [`get`](https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html#method.get "method polars_schema::schema::Schema::get") or [`get_full`](https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html#method.get_full "method polars_schema::schema::Schema::get_full").

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.try_get_field" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.try_get_field" class="fn">try_get_field</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Look up the name in the schema and return an owned [`Field`](https://docs.rs/polars/latest/polars/prelude/struct.Field.html "struct polars::prelude::Field") by cloning the data.

Returns `Err(PolarsErr)` if the field does not exist.

This method constructs the `Field` by cloning the name and dtype. For a version that returns references, see [`get`](https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html#method.get "method polars_schema::schema::Schema::get") or [`get_full`](https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html#method.get_full "method polars_schema::schema::Schema::get_full").

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.to_arrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.to_arrow" class="fn">to_arrow</a>(&self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>) -\> <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>

Convert self to `ArrowSchema` by cloning the fields.

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.iter_fields" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.iter_fields" class="fn">iter_fields</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

Iterates the [`Field`](https://docs.rs/polars/latest/polars/prelude/struct.Field.html "struct polars::prelude::Field")s in this schema, constructing them anew by cloning each `(&name, &dtype)` pair.

Note that this clones each name and dtype in order to form an owned [`Field`](https://docs.rs/polars/latest/polars/prelude/struct.Field.html "struct polars::prelude::Field"). For a clone-free version, use [`iter`](https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html#method.iter "method polars_schema::schema::Schema::iter"), which returns `(&name, &dtype)`.

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.to_supertype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.to_supertype" class="fn">to_supertype</a>( &mut self, other: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take another [`Schema`](https://docs.rs/polars/latest/polars/prelude/type.Schema.html "type polars::prelude::Schema") and try to find the supertypes between them.

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.from_arrow_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.from_arrow_schema" class="fn">from_arrow_schema</a>(value: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>) -\> <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#method.project_select" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#tymethod.project_select" class="fn">project_select</a>(&self, select: &<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>) -\> <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaExt.html#implementors" class="anchor">§</a>
