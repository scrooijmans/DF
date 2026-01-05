# Trait SchemaVisitor Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/visitor.rs.html#23-72" class="src">Source</a>

``` rust
pub trait SchemaVisitor {
    type T;

Show 14 methods    // Required methods
    fn schema(&mut self, schema: &Schema, value: Self::T) -> Result<Self::T>;
    fn field(
        &mut self,
        field: &NestedFieldRef,
        value: Self::T,
    ) -> Result<Self::T>;
    fn struct(
        &mut self,
        struct: &StructType,
        results: Vec<Self::T>,
    ) -> Result<Self::T>;
    fn list(&mut self, list: &ListType, value: Self::T) -> Result<Self::T>;
    fn map(
        &mut self,
        map: &MapType,
        key_value: Self::T,
        value: Self::T,
    ) -> Result<Self::T>;
    fn primitive(&mut self, p: &PrimitiveType) -> Result<Self::T>;

    // Provided methods
    fn before_struct_field(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn after_struct_field(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn before_list_element(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn after_list_element(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn before_map_key(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn after_map_key(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn before_map_value(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
    fn after_map_value(&mut self, _field: &NestedFieldRef) -> Result<()> { ... }
}
```

Expand description

A post order schema visitor.

For order of methods called, please refer to [`visit_schema`](https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.visit_schema.html "fn iceberg::spec::visit_schema").

## Required Associated Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype">T</a>

Return type of this visitor.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#tymethod.schema" class="fn">schema</a>(&mut self, schema: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>, value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>

Called after schema’s type visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#tymethod.field" class="fn">field</a>(&mut self, field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>, value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>

Called after struct’s field type visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#tymethod.struct" class="fn">struct</a>( &mut self, struct: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>, results: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>

Called after struct’s fields visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#tymethod.list" class="fn">list</a>(&mut self, list: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>, value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>

Called after list fields visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#tymethod.map" class="fn">map</a>( &mut self, map: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>, key_value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>, value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>

Called after map’s key and value fields visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#tymethod.primitive" class="fn">primitive</a>(&mut self, p: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::spec::SchemaVisitor::T">T</a>\>

Called when see a primitive type.

## Provided Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.before_struct_field" class="fn">before_struct_field</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before struct field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.after_struct_field" class="fn">after_struct_field</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after struct field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.before_list_element" class="fn">before_list_element</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before list field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.after_list_element" class="fn">after_list_element</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after list field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.before_map_key" class="fn">before_map_key</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before map key field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.after_map_key" class="fn">after_map_key</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after map key field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.before_map_value" class="fn">before_map_value</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before map value field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#method.after_map_value" class="fn">after_map_value</a>(&mut self, \_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after map value field.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html#implementors" class="anchor">§</a>
