# Field in arrow_schema - Rust

```
pub struct Field { /* private fields */ }
```

Expand description

Describes a single column in a [`Schema`](struct.Schema.html "struct arrow_schema::Schema").

A [`Schema`](struct.Schema.html "struct arrow_schema::Schema") is an ordered collection of [`Field`](struct.Field.html "struct arrow_schema::Field") objects. Fields contain:

- `name`: the name of the field
- `data_type`: the type of the field
- `nullable`: if the field is nullable
- `metadata`: a map of key-value pairs containing additional custom metadata

Arrow Extension types, are encoded in `Field`s metadata. See [`Self::try_extension_type`](about:blank/struct.Field.html#method.try_extension_type "method arrow_schema::Field::try_extension_type") to retrieve the [`ExtensionType`](extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType"), if any.

[Source](about:blank/src/arrow_schema/field.rs.html#135-861)
[§](#impl-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#137)

Default list member field name

[Source](about:blank/src/arrow_schema/field.rs.html#146-156)

Creates a new field with the given name, data type, and nullability

##### [§](#example)Example

```
Field::new("field_name", DataType::Int32, true);
```

[Source](about:blank/src/arrow_schema/field.rs.html#172-174)

Creates a new `Field` suitable for [`DataType::List`](about:blank/enum.DataType.html#variant.List "variant arrow_schema::DataType::List") and [`DataType::LargeList`](about:blank/enum.DataType.html#variant.LargeList "variant arrow_schema::DataType::LargeList")

While not required, this method follows the convention of naming the `Field` `"item"`.

##### [§](#example-1)Example

```
assert_eq!(
  Field::new("item", DataType::Int32, true),
  Field::new_list_field(DataType::Int32, true)
);
```

[Source](about:blank/src/arrow_schema/field.rs.html#181-197)

👎Deprecated since 54.0.0: The ability to preserve dictionary IDs will be removed. With the dict_id field disappearing this function signature will change by removing the dict_id parameter.

Creates a new field that has additional dictionary information

[Source](about:blank/src/arrow_schema/field.rs.html#206-218)

[Source](about:blank/src/arrow_schema/field.rs.html#225-227)

[Source](about:blank/src/arrow_schema/field.rs.html#234-236)

[Source](about:blank/src/arrow_schema/field.rs.html#243-249)

[Source](about:blank/src/arrow_schema/field.rs.html#257-264)

[Source](about:blank/src/arrow_schema/field.rs.html#274-291)

[Source](about:blank/src/arrow_schema/field.rs.html#299-311)

[Source](about:blank/src/arrow_schema/field.rs.html#315-317)

Sets the `Field`’s optional custom metadata.

[Source](about:blank/src/arrow_schema/field.rs.html#320-323)

Sets the metadata of this `Field` to be `metadata` and returns self

[Source](about:blank/src/arrow_schema/field.rs.html#327-329)

Returns the immutable reference to the `Field`’s optional custom metadata.

[Source](about:blank/src/arrow_schema/field.rs.html#333-335)

Returns a mutable reference to the `Field`’s optional custom metadata.

[Source](about:blank/src/arrow_schema/field.rs.html#339-341)

Returns an immutable reference to the `Field`’s name.

[Source](about:blank/src/arrow_schema/field.rs.html#345-347)

Set the name of this [`Field`](struct.Field.html "struct arrow_schema::Field")

[Source](about:blank/src/arrow_schema/field.rs.html#358-361)

Set the name of the [`Field`](struct.Field.html "struct arrow_schema::Field") and returns self.

```
let field = Field::new("c1", DataType::Int64, false)
   .with_name("c2");

assert_eq!(field.name(), "c2");
```

[Source](about:blank/src/arrow_schema/field.rs.html#365-367)

Returns an immutable reference to the [`Field`](struct.Field.html "struct arrow_schema::Field")’s [`DataType`](enum.DataType.html "enum arrow_schema::DataType").

[Source](about:blank/src/arrow_schema/field.rs.html#379-381)

Set [`DataType`](enum.DataType.html "enum arrow_schema::DataType") of the [`Field`](struct.Field.html "struct arrow_schema::Field")

```
let mut field = Field::new("c1", DataType::Int64, false);
field.set_data_type(DataType::Utf8);

assert_eq!(field.data_type(), &DataType::Utf8);
```

[Source](about:blank/src/arrow_schema/field.rs.html#392-395)

Set [`DataType`](enum.DataType.html "enum arrow_schema::DataType") of the [`Field`](struct.Field.html "struct arrow_schema::Field") and returns self.

```
let field = Field::new("c1", DataType::Int64, false)
   .with_data_type(DataType::Utf8);

assert_eq!(field.data_type(), &DataType::Utf8);
```

[Source](about:blank/src/arrow_schema/field.rs.html#418-422)

Returns the extension type name of this [`Field`](struct.Field.html "struct arrow_schema::Field"), if set.

This returns the value of [`EXTENSION_TYPE_NAME_KEY`](extension/constant.EXTENSION_TYPE_NAME_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_NAME_KEY"), if set in [`Field::metadata`](about:blank/struct.Field.html#method.metadata "method arrow_schema::Field::metadata"). If the key is missing, there is no extension type name and this returns `None`.

##### [§](#example-2)Example

```

let field = Field::new("", DataType::Null, false);
assert_eq!(field.extension_type_name(), None);

let field = Field::new("", DataType::Null, false).with_metadata(
   [(EXTENSION_TYPE_NAME_KEY.to_owned(), "example".to_owned())]
       .into_iter()
       .collect(),
);
assert_eq!(field.extension_type_name(), Some("example"));
```

[Source](about:blank/src/arrow_schema/field.rs.html#445-449)

Returns the extension type metadata of this [`Field`](struct.Field.html "struct arrow_schema::Field"), if set.

This returns the value of [`EXTENSION_TYPE_METADATA_KEY`](extension/constant.EXTENSION_TYPE_METADATA_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY"), if set in [`Field::metadata`](about:blank/struct.Field.html#method.metadata "method arrow_schema::Field::metadata"). If the key is missing, there is no extension type metadata and this returns `None`.

##### [§](#example-3)Example

```

let field = Field::new("", DataType::Null, false);
assert_eq!(field.extension_type_metadata(), None);

let field = Field::new("", DataType::Null, false).with_metadata(
   [(EXTENSION_TYPE_METADATA_KEY.to_owned(), "example".to_owned())]
       .into_iter()
       .collect(),
);
assert_eq!(field.extension_type_metadata(), Some("example"));
```

[Source](about:blank/src/arrow_schema/field.rs.html#465-485)

[Source](about:blank/src/arrow_schema/field.rs.html#494-497)

[Source](about:blank/src/arrow_schema/field.rs.html#511-530)

[Source](about:blank/src/arrow_schema/field.rs.html#539-543)

[Source](about:blank/src/arrow_schema/field.rs.html#554-556)

Available on **crate feature `canonical_extension_types`** only.

Returns the [`CanonicalExtensionType`](extension/enum.CanonicalExtensionType.html "enum arrow_schema::extension::CanonicalExtensionType") of this [`Field`](struct.Field.html "struct arrow_schema::Field"), if set.

##### [§](#error-2)Error

Returns an error if

- this field does not have a canonical extension type (mismatch or missing)
- the canonical extension is not supported
- the construction of the extension type fails

[Source](about:blank/src/arrow_schema/field.rs.html#562-564)

Indicates whether this [`Field`](struct.Field.html "struct arrow_schema::Field") supports null values.

If true, the field _may_ contain null values.

[Source](about:blank/src/arrow_schema/field.rs.html#576-578)

Set the `nullable` of this [`Field`](struct.Field.html "struct arrow_schema::Field").

```
let mut field = Field::new("c1", DataType::Int64, false);
field.set_nullable(true);

assert_eq!(field.is_nullable(), true);
```

[Source](about:blank/src/arrow_schema/field.rs.html#589-592)

Set `nullable` of the [`Field`](struct.Field.html "struct arrow_schema::Field") and returns self.

```
let field = Field::new("c1", DataType::Int64, false)
   .with_nullable(true);

assert_eq!(field.is_nullable(), true);
```

[Source](about:blank/src/arrow_schema/field.rs.html#641-647)

👎Deprecated since 54.0.0: The ability to preserve dictionary IDs will be removed. With it, all fields related to it.

Returns the dictionary ID, if this is a dictionary type.

[Source](about:blank/src/arrow_schema/field.rs.html#664-669)

Returns whether this `Field`’s dictionary is ordered, if this is a dictionary type.

##### [§](#example-4)Example

```
// non dictionaries do not have a dict is ordered flat
let field = Field::new("c1", DataType::Int64, false);
assert_eq!(field.dict_is_ordered(), None);
// by default dictionary is not ordered
let field = Field::new("c1", DataType::Dictionary(Box::new(DataType::Int64), Box::new(DataType::Utf8)), false);
assert_eq!(field.dict_is_ordered(), Some(false));
let field = field.with_dict_is_ordered(true);
assert_eq!(field.dict_is_ordered(), Some(true));
```

[Source](about:blank/src/arrow_schema/field.rs.html#676-681)

Set the is ordered field for this `Field`, if it is a dictionary.

Does nothing if this is not a dictionary type.

See [`Field::dict_is_ordered`](about:blank/struct.Field.html#method.dict_is_ordered "method arrow_schema::Field::dict_is_ordered") for more information.

[Source](about:blank/src/arrow_schema/field.rs.html#697-828)

Merge this field into self if it is compatible.

Struct fields are merged recursively.

NOTE: `self` may be updated to a partial / unexpected state in case of merge failure.

Example:

```
let mut field = Field::new("c1", DataType::Int64, false);
assert!(field.try_merge(&Field::new("c1", DataType::Int64, true)).is_ok());
assert!(field.is_nullable());
```

[Source](about:blank/src/arrow_schema/field.rs.html#835-845)

Check to see if `self` is a superset of `other` field. Superset is defined as:

- if nullability doesn’t match, self needs to be nullable
- self.metadata is a superset of other.metadata
- all other fields are equal

[Source](about:blank/src/arrow_schema/field.rs.html#850-860)

Return size of this instance in bytes.

Includes the size of `Self`.

[Source](about:blank/src/arrow_schema/field.rs.html#47)
[§](#impl-Clone-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#47)
[§](#impl-Debug-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#48)
[§](#impl-Deserialize%3C'de%3E-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#864-868)
[§](#impl-Display-for-Field)

[Source](about:blank/src/arrow_schema/schema.rs.html#168-176)
[§](#impl-Extend%3CField%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#169-175)
[§](#method.extend)

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#417)
[§](#method.extend_one)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#425)
[§](#method.extend_reserve)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](about:blank/src/arrow_schema/fields.rs.html#267-271)
[§](#impl-FromIterator%3CField%3E-for-Fields)

[Source](about:blank/src/arrow_schema/field.rs.html#119-133)
[§](#impl-Hash-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#85-117)
[§](#impl-Ord-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#68-75)
[§](#impl-PartialEq-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#69-74)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/arrow_schema/field.rs.html#79-83)
[§](#impl-PartialOrd-for-Field)

[Source](about:blank/src/arrow_schema/field.rs.html#80-82)
[§](#method.partial_cmp)

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398)
[§](#method.lt)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416)
[§](#method.le)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434)
[§](#method.gt)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452)
[§](#method.ge)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](about:blank/src/arrow_schema/field.rs.html#48)
[§](#impl-Serialize-for-Field)

[Source](about:blank/src/arrow_schema/ffi.rs.html#618-627)
[§](#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-Field)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#619)
[§](#associatedtype.Error-1)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#621-626)
[§](#method.try_from-1)

Performs the conversion.

[Source](about:blank/src/arrow_schema/extension/canonical/mod.rs.html#82-106)
[§](#impl-TryFrom%3C%26Field%3E-for-CanonicalExtensionType)

Available on **crate feature `canonical_extension_types`** only.

[Source](about:blank/src/arrow_schema/extension/canonical/mod.rs.html#83)
[§](#associatedtype.Error)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/extension/canonical/mod.rs.html#85-105)
[§](#method.try_from)

Performs the conversion.

[Source](about:blank/src/arrow_schema/ffi.rs.html#772-791)
[§](#impl-TryFrom%3C%26Field%3E-for-FFI_ArrowSchema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#773)
[§](#associatedtype.Error-2)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#775-790)
[§](#method.try_from-2)

Performs the conversion.

[Source](about:blank/src/arrow_schema/ffi.rs.html#811-817)
[§](#impl-TryFrom%3CField%3E-for-FFI_ArrowSchema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#812)
[§](#associatedtype.Error-3)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#814-816)
[§](#method.try_from-3)

Performs the conversion.

[Source](about:blank/src/arrow_schema/field.rs.html#77)
[§](#impl-Eq-for-Field)

[§](#impl-Freeze-for-Field)

[§](#impl-RefUnwindSafe-for-Field)

[§](#impl-Send-for-Field)

[§](#impl-Sync-for-Field)

[§](#impl-Unpin-for-Field)

[§](#impl-UnwindSafe-for-Field)
