# Struct Field Copy item path

<a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/src/arrow_schema/field.rs.html#49" class="src">Source</a>

``` rust
pub struct Field { /* private fields */ }
```

Expand description

Describes a single column in a [`Schema`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema").

A [`Schema`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema") is an ordered collection of [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") objects. Fields contain:

- `name`: the name of the field
- `data_type`: the type of the field
- `nullable`: if the field is nullable
- `metadata`: a map of key-value pairs containing additional custom metadata

Arrow Extension types, are encoded in `Field`s metadata. See [`Self::try_extension_type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_extension_type "method arrow::datatypes::Field::try_extension_type") to retrieve the [`ExtensionType`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType"), if any.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Field" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

#### pub const <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#associatedconstant.LIST_FIELD_DEFAULT_NAME" class="constant">LIST_FIELD_DEFAULT_NAME</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "item"

Default list member field name

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new" class="fn">new</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Creates a new field with the given name, data type, and nullability

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#example" class="doc-anchor">§</a>Example

``` rust
Field::new("field_name", DataType::Int32, true);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_list_field" class="fn">new_list_field</a>(data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Creates a new `Field` suitable for [`DataType::List`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.List "variant arrow::datatypes::DataType::List") and [`DataType::LargeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.LargeList "variant arrow::datatypes::DataType::LargeList")

While not required, this method follows the convention of naming the `Field` `"item"`.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#example-1" class="doc-anchor">§</a>Example

``` rust
assert_eq!(
  Field::new("item", DataType::Int32, true),
  Field::new_list_field(DataType::Int32, true)
);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_dict" class="fn">new_dict</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, dict_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, dict_is_ordered: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

👎Deprecated since 54.0.0: The ability to preserve dictionary IDs will be removed. With the dict_id field disappearing this function signature will change by removing the dict_id parameter.

Creates a new field that has additional dictionary information

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_dictionary" class="fn">new_dictionary</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, key: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, value: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::Dictionary`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Dictionary "variant arrow::datatypes::DataType::Dictionary")

Use [`Self::new_dict`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_dict "associated function arrow::datatypes::Field::new_dict") for more advanced dictionary options

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#panics" class="doc-anchor">§</a>Panics

Panics if [`!key.is_dictionary_key_type`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_dictionary_key_type "method arrow::datatypes::DataType::is_dictionary_key_type")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_struct" class="fn">new_struct</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, fields: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html" class="struct" title="struct arrow::datatypes::Fields">Fields</a>\>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::Struct`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Struct "variant arrow::datatypes::DataType::Struct")

- `name`: the name of the [`DataType::Struct`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Struct "variant arrow::datatypes::DataType::Struct") field
- `fields`: the description of each struct element
- `nullable`: if the [`DataType::Struct`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Struct "variant arrow::datatypes::DataType::Struct") array is nullable

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_list" class="fn">new_list</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::List`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.List "variant arrow::datatypes::DataType::List")

- `name`: the name of the [`DataType::List`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.List "variant arrow::datatypes::DataType::List") field
- `value`: the description of each list element
- `nullable`: if the [`DataType::List`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.List "variant arrow::datatypes::DataType::List") array is nullable

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_large_list" class="fn">new_large_list</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::LargeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.LargeList "variant arrow::datatypes::DataType::LargeList")

- `name`: the name of the [`DataType::LargeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.LargeList "variant arrow::datatypes::DataType::LargeList") field
- `value`: the description of each list element
- `nullable`: if the [`DataType::LargeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.LargeList "variant arrow::datatypes::DataType::LargeList") array is nullable

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_fixed_size_list" class="fn">new_fixed_size_list</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::FixedSizeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.FixedSizeList "variant arrow::datatypes::DataType::FixedSizeList")

- `name`: the name of the [`DataType::FixedSizeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.FixedSizeList "variant arrow::datatypes::DataType::FixedSizeList") field
- `value`: the description of each list element
- `size`: the size of the fixed size list
- `nullable`: if the [`DataType::FixedSizeList`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.FixedSizeList "variant arrow::datatypes::DataType::FixedSizeList") array is nullable

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_map" class="fn">new_map</a>( name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, entries: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, keys: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, values: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, sorted: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::Map`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Map "variant arrow::datatypes::DataType::Map")

- `name`: the name of the [`DataType::Map`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Map "variant arrow::datatypes::DataType::Map") field
- `entries`: the name of the inner [`DataType::Struct`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Struct "variant arrow::datatypes::DataType::Struct") field
- `keys`: the map keys
- `values`: the map values
- `sorted`: if the [`DataType::Map`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Map "variant arrow::datatypes::DataType::Map") array is sorted
- `nullable`: if the [`DataType::Map`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Map "variant arrow::datatypes::DataType::Map") array is nullable

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.new_union" class="fn">new_union</a>\<S, F, T\>( name: S, type_ids: T, fields: F, mode: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.UnionMode.html" class="enum" title="enum arrow::datatypes::UnionMode">UnionMode</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, F: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, \<F as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>\>, T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>,

Create a new [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with [`DataType::Union`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Union "variant arrow::datatypes::DataType::Union")

- `name`: the name of the [`DataType::Union`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Union "variant arrow::datatypes::DataType::Union") field
- `type_ids`: the union type ids
- `fields`: the union fields
- `mode`: the union mode

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.set_metadata" class="fn">set_metadata</a>(&mut self, metadata: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

Sets the `Field`’s optional custom metadata.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.with_metadata" class="fn">with_metadata</a>(self, metadata: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Sets the metadata of this `Field` to be `metadata` and returns self

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.metadata" class="fn">metadata</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns the immutable reference to the `Field`’s optional custom metadata.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.metadata_mut" class="fn">metadata_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns a mutable reference to the `Field`’s optional custom metadata.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns an immutable reference to the `Field`’s name.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.set_name" class="fn">set_name</a>(&mut self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

Set the name of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.with_name" class="fn">with_name</a>(self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Set the name of the [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") and returns self.

``` rust
let field = Field::new("c1", DataType::Int64, false)
   .with_name("c2");

assert_eq!(field.name(), "c2");
```

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns an immutable reference to the [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field")’s [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType").

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.set_data_type" class="fn">set_data_type</a>(&mut self, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>)

Set [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of the [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field")

``` rust
let mut field = Field::new("c1", DataType::Int64, false);
field.set_data_type(DataType::Utf8);

assert_eq!(field.data_type(), &DataType::Utf8);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.with_data_type" class="fn">with_data_type</a>(self, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Set [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of the [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") and returns self.

``` rust
let field = Field::new("c1", DataType::Int64, false)
   .with_data_type(DataType::Utf8);

assert_eq!(field.data_type(), &DataType::Utf8);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.extension_type_name" class="fn">extension_type_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the extension type name of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field"), if set.

This returns the value of [`EXTENSION_TYPE_NAME_KEY`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/constant.EXTENSION_TYPE_NAME_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_NAME_KEY"), if set in [`Field::metadata`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.metadata "method arrow::datatypes::Field::metadata"). If the key is missing, there is no extension type name and this returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#example-2" class="doc-anchor">§</a>Example

``` rust

let field = Field::new("", DataType::Null, false);
assert_eq!(field.extension_type_name(), None);

let field = Field::new("", DataType::Null, false).with_metadata(
   [(EXTENSION_TYPE_NAME_KEY.to_owned(), "example".to_owned())]
       .into_iter()
       .collect(),
);
assert_eq!(field.extension_type_name(), Some("example"));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.extension_type_metadata" class="fn">extension_type_metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the extension type metadata of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field"), if set.

This returns the value of [`EXTENSION_TYPE_METADATA_KEY`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/constant.EXTENSION_TYPE_METADATA_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY"), if set in [`Field::metadata`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.metadata "method arrow::datatypes::Field::metadata"). If the key is missing, there is no extension type metadata and this returns `None`.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#example-3" class="doc-anchor">§</a>Example

``` rust

let field = Field::new("", DataType::Null, false);
assert_eq!(field.extension_type_metadata(), None);

let field = Field::new("", DataType::Null, false).with_metadata(
   [(EXTENSION_TYPE_METADATA_KEY.to_owned(), "example".to_owned())]
       .into_iter()
       .collect(),
);
assert_eq!(field.extension_type_metadata(), Some("example"));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_extension_type" class="fn">try_extension_type</a>\<E\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<E, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where E: <a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html" class="trait" title="trait arrow_schema::extension::ExtensionType">ExtensionType</a>,

Returns an instance of the given [`ExtensionType`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType") of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field"), if set in the [`Field::metadata`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.metadata "method arrow::datatypes::Field::metadata").

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#error" class="doc-anchor">§</a>Error

Returns an error if

- this field does not have the name of this extension type ([`ExtensionType::NAME`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME "associated constant arrow_schema::extension::ExtensionType::NAME")) in the [`Field::metadata`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.metadata "method arrow::datatypes::Field::metadata") (mismatch or missing)
- the deserialization of the metadata ([`ExtensionType::deserialize_metadata`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata "associated function arrow_schema::extension::ExtensionType::deserialize_metadata")) fails
- the construction of the extension type ([`ExtensionType::try_new`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new "associated function arrow_schema::extension::ExtensionType::try_new")) fail (for example when the [`Field::data_type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.data_type "method arrow::datatypes::Field::data_type") is not supported by the extension type ([`ExtensionType::supports_data_type`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.supports_data_type "method arrow_schema::extension::ExtensionType::supports_data_type")))

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.extension_type" class="fn">extension_type</a>\<E\>(&self) -\> E

where E: <a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html" class="trait" title="trait arrow_schema::extension::ExtensionType">ExtensionType</a>,

Returns an instance of the given [`ExtensionType`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType") of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field"), panics if this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") does not have this extension type.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#panic" class="doc-anchor">§</a>Panic

This calls [`Field::try_extension_type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_extension_type "method arrow::datatypes::Field::try_extension_type") and panics when it returns an error.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_with_extension_type" class="fn">try_with_extension_type</a>\<E\>( &mut self, extension_type: E, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

where E: <a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html" class="trait" title="trait arrow_schema::extension::ExtensionType">ExtensionType</a>,

Updates the metadata of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with the [`ExtensionType::NAME`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME "associated constant arrow_schema::extension::ExtensionType::NAME") and [`ExtensionType::metadata`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.metadata "method arrow_schema::extension::ExtensionType::metadata") of the given [`ExtensionType`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType"), if the given extension type supports the [`Field::data_type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.data_type "method arrow::datatypes::Field::data_type") of this field ([`ExtensionType::supports_data_type`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.supports_data_type "method arrow_schema::extension::ExtensionType::supports_data_type")).

If the given extension type defines no metadata, a previously set value of [`EXTENSION_TYPE_METADATA_KEY`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/constant.EXTENSION_TYPE_METADATA_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY") is cleared.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#error-1" class="doc-anchor">§</a>Error

This functions returns an error if the data type of this field does not match any of the supported storage types of the given extension type.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.with_extension_type" class="fn">with_extension_type</a>\<E\>(self, extension_type: E) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

where E: <a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html" class="trait" title="trait arrow_schema::extension::ExtensionType">ExtensionType</a>,

Updates the metadata of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") with the [`ExtensionType::NAME`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME "associated constant arrow_schema::extension::ExtensionType::NAME") and [`ExtensionType::metadata`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.metadata "method arrow_schema::extension::ExtensionType::metadata") of the given [`ExtensionType`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType").

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#panics-1" class="doc-anchor">§</a>Panics

This calls [`Field::try_with_extension_type`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_with_extension_type "method arrow::datatypes::Field::try_with_extension_type") and panics when it returns an error.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_canonical_extension_type" class="fn">try_canonical_extension_type</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/canonical/enum.CanonicalExtensionType.html" class="enum" title="enum arrow_schema::extension::canonical::CanonicalExtensionType">CanonicalExtensionType</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Returns the [`CanonicalExtensionType`](https://docs.rs/arrow-schema/56.2.0/x86_64-unknown-linux-gnu/arrow_schema/extension/canonical/enum.CanonicalExtensionType.html "enum arrow_schema::extension::canonical::CanonicalExtensionType") of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field"), if set.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#error-2" class="doc-anchor">§</a>Error

Returns an error if

- this field does not have a canonical extension type (mismatch or missing)
- the canonical extension is not supported
- the construction of the extension type fails

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Indicates whether this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") supports null values.

If true, the field *may* contain null values.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.set_nullable" class="fn">set_nullable</a>(&mut self, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Set the `nullable` of this [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field").

``` rust
let mut field = Field::new("c1", DataType::Int64, false);
field.set_nullable(true);

assert_eq!(field.is_nullable(), true);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.with_nullable" class="fn">with_nullable</a>(self, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Set `nullable` of the [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") and returns self.

``` rust
let field = Field::new("c1", DataType::Int64, false)
   .with_nullable(true);

assert_eq!(field.is_nullable(), true);
```

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.dict_id" class="fn">dict_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

👎Deprecated since 54.0.0: The ability to preserve dictionary IDs will be removed. With it, all fields related to it.

Returns the dictionary ID, if this is a dictionary type.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.dict_is_ordered" class="fn">dict_is_ordered</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Returns whether this `Field`’s dictionary is ordered, if this is a dictionary type.

##### <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#example-4" class="doc-anchor">§</a>Example

``` rust
// non dictionaries do not have a dict is ordered flat
let field = Field::new("c1", DataType::Int64, false);
assert_eq!(field.dict_is_ordered(), None);
// by default dictionary is not ordered
let field = Field::new("c1", DataType::Dictionary(Box::new(DataType::Int64), Box::new(DataType::Utf8)), false);
assert_eq!(field.dict_is_ordered(), Some(false));
let field = field.with_dict_is_ordered(true);
assert_eq!(field.dict_is_ordered(), Some(true));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.with_dict_is_ordered" class="fn">with_dict_is_ordered</a>(self, dict_is_ordered: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Set the is ordered field for this `Field`, if it is a dictionary.

Does nothing if this is not a dictionary type.

See [`Field::dict_is_ordered`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.dict_is_ordered "method arrow::datatypes::Field::dict_is_ordered") for more information.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_merge" class="fn">try_merge</a>(&mut self, from: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Merge this field into self if it is compatible.

Struct fields are merged recursively.

NOTE: `self` may be updated to a partial / unexpected state in case of merge failure.

Example:

``` rust
let mut field = Field::new("c1", DataType::Int64, false);
assert!(field.try_merge(&Field::new("c1", DataType::Int64, true)).is_ok());
assert!(field.is_nullable());
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.contains" class="fn">contains</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check to see if `self` is a superset of `other` field. Superset is defined as:

- if nullability doesn’t match, self needs to be nullable
- self.metadata is a superset of other.metadata
- all other fields are equal

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return size of this instance in bytes.

Includes the size of `Self`.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Clone-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Debug-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Display-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Extend%3CField%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-From%3CField%3C&#39;_%3E%3E-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow-ipc/56.2.0/x86_64-unknown-linux-gnu/arrow_ipc/gen/Schema/struct.Field.html" class="struct" title="struct arrow_ipc::gen::Schema::Field">Field</a>\<'\_\>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Convert an IPC Field to Arrow Field

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(field: <a href="https://docs.rs/arrow-ipc/56.2.0/x86_64-unknown-linux-gnu/arrow_ipc/gen/Schema/struct.Field.html" class="struct" title="struct arrow_ipc::gen::Schema::Field">Field</a>\<'\_\>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-FromIterator%3CField%3E-for-Fields" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html" class="struct" title="struct arrow::datatypes::Fields">Fields</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Fields.html" class="struct" title="struct arrow::datatypes::Fields">Fields</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-FromPyArrow-for-Field" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.FromPyArrow.html" class="trait" title="trait arrow_pyarrow::FromPyArrow">FromPyArrow</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.from_pyarrow_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound" class="fn">from_pyarrow_bound</a>(value: &<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'\_, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert a Python object to an arrow-rs type. [Read more](https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Hash-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Ord-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-PartialEq-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-PartialOrd-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-ToPyArrow-for-Field" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.ToPyArrow.html" class="trait" title="trait arrow_pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.to_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.ToPyArrow.html#tymethod.to_pyarrow" class="fn">to_pyarrow</a>(&self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object without consuming it.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(c_schema: &<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-TryFrom%3C%26Field%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(field: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-TryFrom%3CField%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#method.try_from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(field: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#impl-Eq-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html" class="struct" title="struct arrow::datatypes::Field">Field</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html#blanket-implementations" class="anchor">§</a>
