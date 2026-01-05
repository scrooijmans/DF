# LineStringType in geoarrow_schema - Rust

## Struct LineStringType

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)

```
pub struct LineStringType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#213-236)
[§](#impl-LineStringType-1)

[Source](about:blank/src/geoarrow_schema/type.rs.html#231-235)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{Dimension, LineStringType};

let geom_type = LineStringType::new(Dimension::XY, Default::default());
let expected_coord_type = DataType::Struct(
    vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
    ]
    .into(),
);
let expected_type = DataType::List(Field::new("vertices", expected_coord_type, false).into());
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-Clone-for-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-Debug-for-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#238-280)
[§](#impl-ExtensionType-for-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#239)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#241)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#243-245)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#247-249)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#251-253)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#255-270)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#272-279)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#407)
[§](#impl-From%3CLineStringType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#407)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-Hash-for-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-PartialEq-for-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-Eq-for-LineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#84-90)
[§](#impl-StructuralPartialEq-for-LineStringType)

[§](#impl-Freeze-for-LineStringType)

[§](#impl-RefUnwindSafe-for-LineStringType)

[§](#impl-Send-for-LineStringType)

[§](#impl-Sync-for-LineStringType)

[§](#impl-Unpin-for-LineStringType)

[§](#impl-UnwindSafe-for-LineStringType)
