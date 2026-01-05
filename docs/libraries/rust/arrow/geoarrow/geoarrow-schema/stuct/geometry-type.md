# GeometryType in geoarrow_schema - Rust

## Struct GeometryType

[Source](about:blank/src/geoarrow_schema/type.rs.html#940-943)

```
pub struct GeometryType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-Clone-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-Debug-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-Default-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1053-1088)
[§](#impl-ExtensionType-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1054)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1056)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1058-1060)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#1062-1064)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1066-1068)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1070-1079)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1081-1087)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#414)
[§](#impl-From%3CGeometryType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#414)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-Hash-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-PartialEq-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-Eq-for-GeometryType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#939)
[§](#impl-StructuralPartialEq-for-GeometryType)

[§](#impl-Freeze-for-GeometryType)

[§](#impl-RefUnwindSafe-for-GeometryType)

[§](#impl-Send-for-GeometryType)

[§](#impl-Sync-for-GeometryType)

[§](#impl-Unpin-for-GeometryType)

[§](#impl-UnwindSafe-for-GeometryType)
