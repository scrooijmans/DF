# Struct NestedField Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/datatypes.rs.html#527-542" class="src">Source</a>

``` rust
pub struct NestedField {
    pub id: i32,
    pub name: String,
    pub required: bool,
    pub field_type: Box<Type>,
    pub doc: Option<String>,
    pub initial_default: Option<Literal>,
    pub write_default: Option<Literal>,
}
```

Expand description

A struct is a tuple of typed values. Each field in the tuple is named and has an integer id that is unique in the table schema. Each field can be either optional or required, meaning that values can (or cannot) be null. Fields may be any type. Fields may have an optional comment or doc string. Fields can have default values.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.id" class="anchor field">§</a>`id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

Id unique in table schema

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Field Name

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.required" class="anchor field">§</a>`required: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Optional or required

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.field_type" class="anchor field">§</a>`field_type: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type"><code>Type</code></a>`>`

Datatype

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.doc" class="anchor field">§</a>`doc: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Fields may have an optional comment or doc string.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.initial_default" class="anchor field">§</a>`initial_default: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal"><code>Literal</code></a>`>`

Used to populate the field’s value for all records that were written before the field was added to the schema

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#structfield.write_default" class="anchor field">§</a>`write_default: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal"><code>Literal</code></a>`>`

Used to populate the field’s value for any records written after the field was added to the schema, if the writer does not supply the field’s value

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-NestedField" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.new" class="fn">new</a>( id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, name: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>, field_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>, required: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> Self

Construct a new field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.required" class="fn">required</a>(id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, name: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>, field_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> Self

Construct a required field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.optional" class="fn">optional</a>(id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, name: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>, field_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> Self

Construct an optional field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.list_element" class="fn">list_element</a>(id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, field_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>, required: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Construct list type’s element field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.map_key_element" class="fn">map_key_element</a>(id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, field_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> Self

Construct map type’s key field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.map_value_element" class="fn">map_value_element</a>(id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, field_type: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>, required: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Construct map type’s value field.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.with_doc" class="fn">with_doc</a>(self, doc: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>) -\> Self

Set the field’s doc.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.with_initial_default" class="fn">with_initial_default</a>(self, value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>) -\> Self

Set the field’s initial default value.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.with_write_default" class="fn">with_write_default</a>(self, value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>) -\> Self

Set the field’s initial default value.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-Clone-for-NestedField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-Debug-for-NestedField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-Deserialize%3C&#39;de%3E-for-NestedField" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-Display-for-NestedField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-PartialEq-for-NestedField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-Serialize-for-NestedField" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-Eq-for-NestedField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#impl-StructuralPartialEq-for-NestedField" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html#blanket-implementations" class="anchor">§</a>
