# WkbType in geoarrow_schema - Rust

```
pub struct WkbType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-Clone-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-Debug-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-Default-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1354-1385)
[§](#impl-ExtensionType-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1355)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1357)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1359-1361)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#1363-1365)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1367-1369)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1371-1378)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1380-1384)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-Hash-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-PartialEq-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-Eq-for-WkbType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#1332)
[§](#impl-StructuralPartialEq-for-WkbType)

[§](#impl-Freeze-for-WkbType)

[§](#impl-RefUnwindSafe-for-WkbType)

[§](#impl-Send-for-WkbType)

[§](#impl-Sync-for-WkbType)

[§](#impl-Unpin-for-WkbType)

[§](#impl-UnwindSafe-for-WkbType)
