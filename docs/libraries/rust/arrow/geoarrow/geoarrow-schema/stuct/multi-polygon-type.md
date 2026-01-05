# MultiPolygonType in geoarrow_schema - Rust

## Struct MultiPolygonType

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)

```
pub struct MultiPolygonType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#564-594)
[§](#impl-MultiPolygonType-1)

[Source](about:blank/src/geoarrow_schema/type.rs.html#587-593)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{Dimension, MultiPolygonType};

let geom_type = MultiPolygonType::new(Dimension::XYM, Default::default());

let expected_coord_type = DataType::Struct(
    vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
        Field::new("m", DataType::Float64, false),
    ]
    .into(),
);
let vertices_field = Field::new("vertices", expected_coord_type, false);
let rings_field = Field::new_list("rings", vertices_field, false);
let polygons_field = Field::new_list("polygons", rings_field, false);
let expected_type = DataType::List(polygons_field.into());
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-Clone-for-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-Debug-for-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#596-638)
[§](#impl-ExtensionType-for-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#597)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#599)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#601-603)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#605-607)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#609-611)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#613-628)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#630-637)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#411)
[§](#impl-From%3CMultiPolygonType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#411)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-Hash-for-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-PartialEq-for-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-Eq-for-MultiPolygonType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#112-118)
[§](#impl-StructuralPartialEq-for-MultiPolygonType)

[§](#impl-Freeze-for-MultiPolygonType)

[§](#impl-RefUnwindSafe-for-MultiPolygonType)

[§](#impl-Send-for-MultiPolygonType)

[§](#impl-Sync-for-MultiPolygonType)

[§](#impl-Unpin-for-MultiPolygonType)

[§](#impl-UnwindSafe-for-MultiPolygonType)
