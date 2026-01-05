# BoxType in geoarrow_schema - Rust

[![](https://github.com/geoarrow.png)](../geoarrow_schema/index.html)

```
pub struct BoxType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#1165-1264)
[§](#impl-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1167-1169)

Construct a new type from parts.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1172-1174)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1177-1179)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1182-1184)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1187-1189)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1214-1258)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{BoxType, Dimension};

let geom_type = BoxType::new(Dimension::XYZM, Default::default());

let expected_type = DataType::Struct(
    vec![
        Field::new("xmin", DataType::Float64, false),
        Field::new("ymin", DataType::Float64, false),
        Field::new("zmin", DataType::Float64, false),
        Field::new("mmin", DataType::Float64, false),
        Field::new("xmax", DataType::Float64, false),
        Field::new("ymax", DataType::Float64, false),
        Field::new("zmax", DataType::Float64, false),
        Field::new("mmax", DataType::Float64, false),
    ]
    .into(),
);
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#1261-1263)

Convert this type to a [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field"), retaining extension metadata.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#impl-Clone-for-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#impl-Debug-for-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1266-1298)
[§](#impl-ExtensionType-for-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1267)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1269)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1271-1273)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#1275-1277)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1279-1281)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1283-1292)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1294-1297)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#413)
[§](#impl-From%3CBoxType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#413)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#impl-Hash-for-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#impl-PartialEq-for-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#impl-Eq-for-BoxType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1159)
[§](#impl-StructuralPartialEq-for-BoxType)

[§](#impl-Freeze-for-BoxType)

[§](#impl-RefUnwindSafe-for-BoxType)

[§](#impl-Send-for-BoxType)

[§](#impl-Sync-for-BoxType)

[§](#impl-Unpin-for-BoxType)

[§](#impl-UnwindSafe-for-BoxType)
