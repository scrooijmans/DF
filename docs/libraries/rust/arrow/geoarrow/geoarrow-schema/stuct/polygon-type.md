# PolygonType in geoarrow_schema - Rust

```
pub struct PolygonType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#293-321)
[§](#impl-PolygonType-1)

[Source](about:blank/src/geoarrow_schema/type.rs.html#315-320)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{Dimension, PolygonType};

let geom_type = PolygonType::new(Dimension::XYZ, Default::default());

let expected_coord_type = DataType::Struct(
    vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
        Field::new("z", DataType::Float64, false),
    ]
    .into(),
);
let vertices_field = Field::new("vertices", expected_coord_type, false);
let rings_field = Field::new_list("rings", vertices_field, false);
let expected_type = DataType::List(rings_field.into());
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-Clone-for-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-Debug-for-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#323-365)
[§](#impl-ExtensionType-for-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#324)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#326)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#328-330)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#332-334)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#336-338)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#340-355)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#357-364)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#408)
[§](#impl-From%3CPolygonType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#408)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-Hash-for-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-PartialEq-for-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-Eq-for-PolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#91-97)
[§](#impl-StructuralPartialEq-for-PolygonType)

[§](#impl-Freeze-for-PolygonType)

[§](#impl-RefUnwindSafe-for-PolygonType)

[§](#impl-Send-for-PolygonType)

[§](#impl-Sync-for-PolygonType)

[§](#impl-Unpin-for-PolygonType)

[§](#impl-UnwindSafe-for-PolygonType)
