# Struct ArrowArrayAccessor Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/value.rs.html#460-462" class="src">Source</a>

``` rust
pub struct ArrowArrayAccessor { /* private fields */ }
```

Expand description

Partner type representing accessing and walking arrow arrays alongside iceberg schema

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#impl-ArrowArrayAccessor" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html" class="struct" title="struct iceberg::arrow::ArrowArrayAccessor">ArrowArrayAccessor</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.new" class="fn">new</a>() -\> Self

Creates a new instance of ArrowArrayAccessor with the default ID matching mode

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.new_with_match_mode" class="fn">new_with_match_mode</a>(match_mode: <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>) -\> Self

Creates a new instance of ArrowArrayAccessor with the specified matching mode

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#impl-Default-for-ArrowArrayAccessor" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html" class="struct" title="struct iceberg::arrow::ArrowArrayAccessor">ArrowArrayAccessor</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#impl-PartnerAccessor%3CArc%3Cdyn+Array%3E%3E-for-ArrowArrayAccessor" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html" class="trait" title="trait iceberg::spec::PartnerAccessor">PartnerAccessor</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html" class="trait" title="trait arrow_array::array::Array">Array</a>\>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html" class="struct" title="struct iceberg::arrow::ArrowArrayAccessor">ArrowArrayAccessor</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.struct_partner" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.struct_partner" class="fn">struct_partner</a>\<'a\>( &self, schema_partner: &'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>\>

Get the struct partner from schema partner.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.field_partner" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.field_partner" class="fn">field_partner</a>\<'a\>( &self, struct_partner: &'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>, field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>\>

Get the field partner from struct partner.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.list_element_partner" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.list_element_partner" class="fn">list_element_partner</a>\<'a\>( &self, list_partner: &'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>\>

Get the list element partner from list partner.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.map_key_partner" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.map_key_partner" class="fn">map_key_partner</a>\<'a\>(&self, map_partner: &'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>\>

Get the map key partner from map partner.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#method.map_value_partner" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html#tymethod.map_value_partner" class="fn">map_value_partner</a>\<'a\>( &self, map_partner: &'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<&'a <a href="https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html" class="type" title="type arrow_array::array::ArrayRef">ArrayRef</a>\>

Get the map value partner from map partner.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html#blanket-implementations" class="anchor">§</a>
