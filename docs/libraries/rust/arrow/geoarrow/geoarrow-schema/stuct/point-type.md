# PointType in geoarrow_schema - Rust

```
pub struct PointType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#127-142)
[§](#impl-PointType-1)

[Source](about:blank/src/geoarrow_schema/type.rs.html#139-141)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{CoordType, Dimension, PointType};

let geom_type = PointType::new(Dimension::XY, Default::default()).with_coord_type(CoordType::Interleaved);
let expected_type =
    DataType::FixedSizeList(Field::new("xy", DataType::Float64, false).into(), 2);
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-Clone-for-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-Debug-for-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#144-186)
[§](#impl-ExtensionType-for-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#145)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#147)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#149-151)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#153-155)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#157-159)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#161-176)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#178-185)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#406)
[§](#impl-From%3CPointType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#406)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-Hash-for-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-PartialEq-for-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-Eq-for-PointType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#77-83)
[§](#impl-StructuralPartialEq-for-PointType)

[§](#impl-Freeze-for-PointType)

[§](#impl-RefUnwindSafe-for-PointType)

[§](#impl-Send-for-PointType)

[§](#impl-Sync-for-PointType)

[§](#impl-Unpin-for-PointType)

[§](#impl-UnwindSafe-for-PointType)
