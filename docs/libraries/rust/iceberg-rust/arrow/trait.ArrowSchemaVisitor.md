# Trait ArrowSchemaVisitor Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/schema.rs.html#51-112" class="src">Source</a>

``` rust
pub trait ArrowSchemaVisitor {
    type T;
    type U;

Show 13 methods    // Required methods
    fn schema(
        &mut self,
        schema: &ArrowSchema,
        values: Vec<Self::T>,
    ) -> Result<Self::U>;
    fn struct(
        &mut self,
        fields: &Fields,
        results: Vec<Self::T>,
    ) -> Result<Self::T>;
    fn list(&mut self, list: &DataType, value: Self::T) -> Result<Self::T>;
    fn map(
        &mut self,
        map: &DataType,
        key_value: Self::T,
        value: Self::T,
    ) -> Result<Self::T>;
    fn primitive(&mut self, p: &DataType) -> Result<Self::T>;

    // Provided methods
    fn before_field(&mut self, _field: &Field) -> Result<()> { ... }
    fn after_field(&mut self, _field: &Field) -> Result<()> { ... }
    fn before_list_element(&mut self, _field: &Field) -> Result<()> { ... }
    fn after_list_element(&mut self, _field: &Field) -> Result<()> { ... }
    fn before_map_key(&mut self, _field: &Field) -> Result<()> { ... }
    fn after_map_key(&mut self, _field: &Field) -> Result<()> { ... }
    fn before_map_value(&mut self, _field: &Field) -> Result<()> { ... }
    fn after_map_value(&mut self, _field: &Field) -> Result<()> { ... }
}
```

Expand description

A post order arrow schema visitor.

For order of methods called, please refer to \[`visit_schema`\].

## Required Associated Types<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype">T</a>

Return type of this visitor on arrow field.

#### type <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.U" class="associatedtype">U</a>

Return type of this visitor on arrow schema.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#tymethod.schema" class="fn">schema</a>( &mut self, schema: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/schema/struct.Schema.html" class="struct" title="struct arrow_schema::schema::Schema">ArrowSchema</a>, values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.U" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::U">U</a>\>

Called after schema’s type visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#tymethod.struct" class="fn">struct</a>(&mut self, fields: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/fields/struct.Fields.html" class="struct" title="struct arrow_schema::fields::Fields">Fields</a>, results: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>\>

Called after struct’s fields visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#tymethod.list" class="fn">list</a>(&mut self, list: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html" class="enum" title="enum arrow_schema::datatype::DataType">DataType</a>, value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>\>

Called after list fields visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#tymethod.map" class="fn">map</a>( &mut self, map: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html" class="enum" title="enum arrow_schema::datatype::DataType">DataType</a>, key_value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>, value: Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>\>

Called after map’s key and value fields visited.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#tymethod.primitive" class="fn">primitive</a>(&mut self, p: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html" class="enum" title="enum arrow_schema::datatype::DataType">DataType</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self::<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#associatedtype.T" class="associatedtype" title="type iceberg::arrow::ArrowSchemaVisitor::T">T</a>\>

Called when see a primitive type.

## Provided Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.before_field" class="fn">before_field</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before struct/list/map field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.after_field" class="fn">after_field</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after struct/list/map field.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.before_list_element" class="fn">before_list_element</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before list element.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.after_list_element" class="fn">after_list_element</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after list element.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.before_map_key" class="fn">before_map_key</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before map key.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.after_map_key" class="fn">after_map_key</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after map key.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.before_map_value" class="fn">before_map_value</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called before map value.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#method.after_map_value" class="fn">after_map_value</a>(&mut self, \_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html" class="struct" title="struct arrow_schema::field::Field">Field</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Called after map value.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html#implementors" class="anchor">§</a>
