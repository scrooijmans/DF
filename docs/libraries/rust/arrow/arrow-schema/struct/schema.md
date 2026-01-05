# Schema in arrow_schema - Rust

```
pub struct Schema {
    pub fields: Fields,
    pub metadata: HashMap<String, String>,
}
```

Expand description

Describes the meta-data of an ordered sequence of relative types.

Note that this information is only part of the meta-data and not part of the physical memory layout.

A sequence of fields that describe the schema.

A map of key-value pairs containing additional meta data.

[Source](about:blank/src/arrow_schema/schema.rs.html#194-518)
[§](#impl-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#196-201)

Creates an empty `Schema`

[Source](about:blank/src/arrow_schema/schema.rs.html#214-216)

Creates a new [`Schema`](struct.Schema.html "struct arrow_schema::Schema") from a sequence of [`Field`](struct.Field.html "struct arrow_schema::Field") values.

##### [§](#example)Example

```
let field_a = Field::new("a", DataType::Int64, false);
let field_b = Field::new("b", DataType::Boolean, false);

let schema = Schema::new(vec![field_a, field_b]);
```

[Source](about:blank/src/arrow_schema/schema.rs.html#236-241)

Creates a new [`Schema`](struct.Schema.html "struct arrow_schema::Schema") from a sequence of [`Field`](struct.Field.html "struct arrow_schema::Field") values and adds additional metadata in form of key value pairs.

##### [§](#example-1)Example

```

let field_a = Field::new("a", DataType::Int64, false);
let field_b = Field::new("b", DataType::Boolean, false);

let mut metadata: HashMap<String, String> = HashMap::new();
metadata.insert("row_count".to_string(), "100".to_string());

let schema = Schema::new_with_metadata(vec![field_a, field_b], metadata);
```

[Source](about:blank/src/arrow_schema/schema.rs.html#244-247)

Sets the metadata of this `Schema` to be `metadata` and returns self

[Source](about:blank/src/arrow_schema/schema.rs.html#251-265)

Returns a new schema with only the specified columns in the new schema This carries metadata from the parent schema over as well

[Source](about:blank/src/arrow_schema/schema.rs.html#295-319)

Merge schema into self if it is compatible. Struct fields will be merged recursively.

Example:

```

let merged = Schema::try_merge(vec![
    Schema::new(vec![
        Field::new("c1", DataType::Int64, false),
        Field::new("c2", DataType::Utf8, false),
    ]),
    Schema::new(vec![
        Field::new("c1", DataType::Int64, true),
        Field::new("c2", DataType::Utf8, false),
        Field::new("c3", DataType::Utf8, false),
    ]),
]).unwrap();

assert_eq!(
    merged,
    Schema::new(vec![
        Field::new("c1", DataType::Int64, true),
        Field::new("c2", DataType::Utf8, false),
        Field::new("c3", DataType::Utf8, false),
    ]),
);
```

[Source](about:blank/src/arrow_schema/schema.rs.html#323-325)

Returns an immutable reference of the vector of `Field` instances.

[Source](about:blank/src/arrow_schema/schema.rs.html#364-366)

Returns a vector with references to all fields (including nested fields)

##### [§](#example-2)Example

```
use std::sync::Arc;
use arrow_schema::{DataType, Field, Fields, Schema};

let f1 = Arc::new(Field::new("a", DataType::Boolean, false));

let f2_inner = Arc::new(Field::new("b_inner", DataType::Int8, false));
let f2 = Arc::new(Field::new("b", DataType::List(f2_inner.clone()), false));

let f3_inner1 = Arc::new(Field::new("c_inner1", DataType::Int8, false));
let f3_inner2 = Arc::new(Field::new("c_inner2", DataType::Int8, false));
let f3 = Arc::new(Field::new(
    "c",
    DataType::Struct(vec![f3_inner1.clone(), f3_inner2.clone()].into()),
    false
));

let mut schema = Schema::new(vec![
  f1.clone(), f2.clone(), f3.clone()
]);
assert_eq!(
    schema.flattened_fields(),
    vec![
        f1.as_ref(),
        f2.as_ref(),
        f2_inner.as_ref(),
        f3.as_ref(),
        f3_inner1.as_ref(),
        f3_inner2.as_ref()
   ]
);
```

[Source](about:blank/src/arrow_schema/schema.rs.html#374-376)

Returns an immutable reference of a specific [`Field`](struct.Field.html "struct arrow_schema::Field") instance selected using an offset within the internal `fields` vector.

##### [§](#panics)Panics

Panics if index out of bounds

[Source](about:blank/src/arrow_schema/schema.rs.html#379-381)

Returns an immutable reference of a specific [`Field`](struct.Field.html "struct arrow_schema::Field") instance selected by name.

[Source](about:blank/src/arrow_schema/schema.rs.html#389-395)

👎Deprecated since 54.0.0: The ability to preserve dictionary IDs will be removed. With it, all functions related to it.

Returns a vector of immutable references to all [`Field`](struct.Field.html "struct arrow_schema::Field") instances selected by the dictionary ID they use.

[Source](about:blank/src/arrow_schema/schema.rs.html#398-406)

Find the index of the column with the given name.

[Source](about:blank/src/arrow_schema/schema.rs.html#410-412)

Returns an immutable reference to the Map of custom metadata key-value pairs.

[Source](about:blank/src/arrow_schema/schema.rs.html#457-495)

Normalize a [`Schema`](struct.Schema.html "struct arrow_schema::Schema") into a flat table.

Nested [`Field`](struct.Field.html "struct arrow_schema::Field")s will generate names separated by `separator`, up to a depth of `max_level` (unlimited if `None`).

e.g. given a [`Schema`](struct.Schema.html "struct arrow_schema::Schema"):

```
    "foo": StructArray<"bar": Utf8>
```

A separator of `"."` would generate a batch with the schema:

Note that giving a depth of `Some(0)` to `max_level` is the same as passing in `None`; it will be treated as unlimited.

##### [§](#example-3)Example

```
let schema = Schema::new(vec![
    Field::new(
        "a",
        DataType::Struct(Fields::from(vec![
            Arc::new(Field::new("animals", DataType::Utf8, true)),
            Arc::new(Field::new("n_legs", DataType::Int64, true)),
        ])),
        false,
    ),
])
.normalize(".", None)
.expect("valid normalization");
let expected = Schema::new(vec![
    Field::new("a.animals", DataType::Utf8, true),
    Field::new("a.n_legs", DataType::Int64, true),
]);
assert_eq!(schema, expected);
```

[Source](about:blank/src/arrow_schema/schema.rs.html#499-502)

Look up a column by name and return a immutable reference to the column along with its index.

[Source](about:blank/src/arrow_schema/schema.rs.html#510-517)

Check to see if `self` is a superset of `other` schema.

In particular returns true if `self.metadata` is a superset of `other.metadata` and [`Fields::contains`](about:blank/struct.Fields.html#method.contains "method arrow_schema::Fields::contains") for `self.fields` and `other.fields`

In other words, any record that conforms to `other` should also conform to `self`.

[Source](about:blank/src/arrow_schema/schema.rs.html#185)
[§](#impl-Clone-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#185)
[§](#impl-Debug-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#186)
[§](#impl-Deserialize%3C'de%3E-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#520-531)
[§](#impl-Display-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#143-147)
[§](#impl-From%3C%26Schema%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#144-146)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/schema.rs.html#149-156)
[§](#impl-From%3CSchema%3E-for-SchemaBuilder)

[Source](about:blank/src/arrow_schema/schema.rs.html#150-155)
[§](#method.from-1)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/schema.rs.html#535-547)
[§](#impl-Hash-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#185)
[§](#impl-PartialEq-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#185)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/arrow_schema/schema.rs.html#186)
[§](#impl-Serialize-for-Schema)

[Source](about:blank/src/arrow_schema/ffi.rs.html#629-643)
[§](#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-Schema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#630)
[§](#associatedtype.Error)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#632-642)
[§](#method.try_from)

Performs the conversion.

[Source](about:blank/src/arrow_schema/ffi.rs.html#793-801)
[§](#impl-TryFrom%3C%26Schema%3E-for-FFI_ArrowSchema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#794)
[§](#associatedtype.Error-1)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#796-800)
[§](#method.try_from-1)

Performs the conversion.

[Source](about:blank/src/arrow_schema/ffi.rs.html#819-825)
[§](#impl-TryFrom%3CSchema%3E-for-FFI_ArrowSchema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#820)
[§](#associatedtype.Error-2)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#822-824)
[§](#method.try_from-2)

Performs the conversion.

[Source](about:blank/src/arrow_schema/schema.rs.html#185)
[§](#impl-Eq-for-Schema)

[Source](about:blank/src/arrow_schema/schema.rs.html#185)
[§](#impl-StructuralPartialEq-for-Schema)

[§](#impl-Freeze-for-Schema)

[§](#impl-RefUnwindSafe-for-Schema)

[§](#impl-Send-for-Schema)

[§](#impl-Sync-for-Schema)

[§](#impl-Unpin-for-Schema)

[§](#impl-UnwindSafe-for-Schema)
