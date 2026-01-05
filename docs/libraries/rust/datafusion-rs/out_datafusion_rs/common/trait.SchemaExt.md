# Trait SchemaExt Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/dfschema.rs.html#1059" class="src">Source</a>

``` rust
pub trait SchemaExt {
    // Required methods
    fn equivalent_names_and_types(&self, other: &Self) -> bool;
    fn logically_equivalent_names_and_types(
        &self,
        other: &Self,
    ) -> Result<(), DataFusionError>;
}
```

Expand description

DataFusion-specific extensions to [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema").

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html#tymethod.equivalent_names_and_types" class="fn">equivalent_names_and_types</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

This is a specialized version of Eq that ignores differences in nullability and metadata.

It works the same as [`DFSchema::equivalent_names_and_types`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.equivalent_names_and_types "method datafusion::common::DFSchema::equivalent_names_and_types").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html#tymethod.logically_equivalent_names_and_types" class="fn">logically_equivalent_names_and_types</a>( &self, other: &Self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns nothing if the two schemas have the same qualified named fields with logically equivalent data types. Returns internal error otherwise.

Use [DFSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema")::equivalent_names_and_types for stricter semantic type equivalence checking.

It is only used by insert into cases.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html#impl-SchemaExt-for-Schema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html" class="trait" title="trait datafusion::common::SchemaExt">SchemaExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>
