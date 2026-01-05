# Struct Schema Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/mod.rs.html#58-72" class="src">Source</a>

``` rust
pub struct Schema { /* private fields */ }
```

Expand description

Defines schema in iceberg.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Schema" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.builder" class="fn">builder</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html" class="struct" title="struct iceberg::spec::SchemaBuilder">SchemaBuilder</a>

Create a schema builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html" class="struct" title="struct iceberg::spec::SchemaBuilder">SchemaBuilder</a>

Create a new schema builder from a schema.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_by_id" class="fn">field_by_id</a>(&self, field_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>\>

Get field by field id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_by_name" class="fn">field_by_name</a>(&self, field_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>\>

Get field by field name.

Both full name and short name could work here.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_by_name_case_insensitive" class="fn">field_by_name_case_insensitive</a>( &self, field_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>\>

Get field by field name, but in case-insensitive way.

Both full name and short name could work here.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_by_alias" class="fn">field_by_alias</a>(&self, alias: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>\>

Get field by alias.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.highest_field_id" class="fn">highest_field_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

Returns \[`highest_field_id`\].

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.schema_id" class="fn">schema_id</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>

Returns \[`schema_id`\].

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.as_struct" class="fn">as_struct</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>

Returns \[`r#struct`\].

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.identifier_field_ids" class="fn">identifier_field_ids</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> + '\_

Returns \[`identifier_field_ids`\].

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_id_by_name" class="fn">field_id_by_name</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Get field id by full name.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.name_by_field_id" class="fn">name_by_field_id</a>(&self, field_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get full name by field id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.accessor_by_field_id" class="fn">accessor_by_field_id</a>(&self, field_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<StructAccessor\>\>

Get an accessor for retrieving data in a struct

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_id_to_name_map" class="fn">field_id_to_name_map</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Return A HashMap matching field ids to field names.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.field_id_to_fields" class="fn">field_id_to_fields</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>\>

Return a hashmap matching field ids to nested fields.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Clone-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Debug-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Deserialize%3C&#39;de%3E-for-Schema" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Display-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-PartialEq-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Serialize-for-Schema" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-TryFrom%3C%26Schema%3E-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/schema/struct.Schema.html" class="struct" title="struct arrow_schema::schema::Schema">Schema</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(schema: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/schema/struct.Schema.html" class="struct" title="struct arrow_schema::schema::Schema">ArrowSchema</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Performs the conversion.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-TryFrom%3C%26Schema%3E-for-Schema-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>\> for <a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/schema/struct.Schema.html" class="struct" title="struct arrow_schema::schema::Schema">Schema</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(schema: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Performs the conversion.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#impl-Eq-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html#blanket-implementations" class="anchor">§</a>
