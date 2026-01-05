# Trait ToDFSchema Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/dfschema.rs.html#940" class="src">Source</a>

``` rust
pub trait ToDFSchema: Sized {
    // Required method
    fn to_dfschema(self) -> Result<DFSchema, DataFusionError>;

    // Provided method
    fn to_dfschema_ref(self) -> Result<Arc<DFSchema>, DataFusionError> { ... }
}
```

Expand description

Convenience trait to convert Schema like things to DFSchema and DFSchemaRef with fewer keystrokes

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#tymethod.to_dfschema" class="fn">to_dfschema</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempt to create a DSSchema

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#method.to_dfschema_ref" class="fn">to_dfschema_ref</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempt to create a DSSchemaRef

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#impl-ToDFSchema-for-Arc%3CSchema%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html" class="trait" title="trait datafusion::common::ToDFSchema">ToDFSchema</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#method.to_dfschema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#tymethod.to_dfschema" class="fn">to_dfschema</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#impl-ToDFSchema-for-Vec%3CField%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html" class="trait" title="trait datafusion::common::ToDFSchema">ToDFSchema</a> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#method.to_dfschema-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#tymethod.to_dfschema" class="fn">to_dfschema</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html#impl-ToDFSchema-for-Schema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html" class="trait" title="trait datafusion::common::ToDFSchema">ToDFSchema</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>
