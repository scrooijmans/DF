# MultiPointType in geoarrow_schema - Rust

## Struct MultiPointType

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)

```
pub struct MultiPointType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#387-413)
[§](#impl-MultiPointType-1)

[Source](about:blank/src/geoarrow_schema/type.rs.html#408-412)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{Dimension, MultiPointType};

let geom_type = MultiPointType::new(Dimension::XYZ, Default::default());

let expected_coord_type = DataType::Struct(
    vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
        Field::new("z", DataType::Float64, false),
    ]
    .into(),
);
let vertices_field = Field::new("points", expected_coord_type, false);
let expected_type = DataType::List(vertices_field.into());
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-Clone-for-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-Debug-for-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#415-457)
[§](#impl-ExtensionType-for-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#416)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#418)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#420-422)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#424-426)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#428-430)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#432-447)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#449-456)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#409)
[§](#impl-From%3CMultiPointType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#409)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-Hash-for-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-PartialEq-for-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-Eq-for-MultiPointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#98-104)
[§](#impl-StructuralPartialEq-for-MultiPointType)

[§](#impl-Freeze-for-MultiPointType)

[§](#impl-RefUnwindSafe-for-MultiPointType)

[§](#impl-Send-for-MultiPointType)

[§](#impl-Sync-for-MultiPointType)

[§](#impl-Unpin-for-MultiPointType)

[§](#impl-UnwindSafe-for-MultiPointType)
