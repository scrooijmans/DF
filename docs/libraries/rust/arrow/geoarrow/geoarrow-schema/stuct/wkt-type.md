# WktType in geoarrow_schema - Rust

```
pub struct WktType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-Clone-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-Debug-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-Default-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1413-1444)
[§](#impl-ExtensionType-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1414)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1416)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1418-1420)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#1422-1424)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1426-1428)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1430-1437)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1439-1443)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-Hash-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-PartialEq-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-Eq-for-WktType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1391)
[§](#impl-StructuralPartialEq-for-WktType)

[§](#impl-Freeze-for-WktType)

[§](#impl-RefUnwindSafe-for-WktType)

[§](#impl-Send-for-WktType)

[§](#impl-Sync-for-WktType)

[§](#impl-Unpin-for-WktType)

[§](#impl-UnwindSafe-for-WktType)
