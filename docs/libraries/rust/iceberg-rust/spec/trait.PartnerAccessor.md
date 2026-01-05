# Trait PartnerAccessor Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/visitor.rs.html#191-202" class="src">Source</a>

``` rust
pub trait PartnerAccessor<P> {
    // Required methods
    fn struct_partner<'a>(&self, schema_partner: &'a P) -> Result<&'a P>;
    fn field_partner<'a>(
        &self,
        struct_partner: &'a P,
        field: &NestedField,
    ) -> Result<&'a P>;
    fn list_element_partner<'a>(&self, list_partner: &'a P) -> Result<&'a P>;
    fn map_key_partner<'a>(&self, map_partner: &'a P) -> Result<&'a P>;
    fn map_value_partner<'a>(&self, map_partner: &'a P) -> Result<&'a P>;
}
```

Expand description

Accessor used to get child partner from parent partner.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.struct_partner" class="fn">struct_partner</a>\<'a\>(&self, schema_partner: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>\>

Get the struct partner from schema partner.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.field_partner" class="fn">field_partner</a>\<'a\>( &self, struct_partner: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>, field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>\>

Get the field partner from struct partner.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.list_element_partner" class="fn">list_element_partner</a>\<'a\>(&self, list_partner: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>\>

Get the list element partner from list partner.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.map_key_partner" class="fn">map_key_partner</a>\<'a\>(&self, map_partner: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>\>

Get the map key partner from map partner.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.map_value_partner" class="fn">map_value_partner</a>\<'a\>(&self, map_partner: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a P</a>\>

Get the map value partner from map partner.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#impl-PartnerAccessor%3CArc%3Cdyn+Array%3E%3E-for-ArrowArrayAccessor" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html" class="trait" title="trait iceberg::spec::PartnerAccessor">PartnerAccessor</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html" class="trait" title="trait arrow_array::array::Array">Array</a>\>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html" class="struct" title="struct iceberg::arrow::ArrowArrayAccessor">ArrowArrayAccessor</a>
