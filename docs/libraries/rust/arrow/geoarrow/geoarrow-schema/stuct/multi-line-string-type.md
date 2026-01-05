# MultiLineStringType in geoarrow_schema - Rust

## Struct MultiLineStringType

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)

```
pub struct MultiLineStringType { /* private fields */ }
```

Expand description

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#469-498)
[§](#impl-MultiLineStringType-1)

[Source](about:blank/src/geoarrow_schema/type.rs.html#492-497)

Convert to the corresponding [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

```
use arrow_schema::{DataType, Field};
use geoarrow_schema::{Dimension, MultiLineStringType};

let geom_type =
    MultiLineStringType::new(Dimension::XYZ, Default::default());

let expected_coord_type = DataType::Struct(
    vec![
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
        Field::new("z", DataType::Float64, false),
    ]
    .into(),
);
let vertices_field = Field::new("vertices", expected_coord_type, false);
let linestrings_field = Field::new_list("linestrings", vertices_field, false);
let expected_type = DataType::List(linestrings_field.into());
assert_eq!(geom_type.data_type(), expected_type);
```

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-Clone-for-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-Debug-for-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#500-542)
[§](#impl-ExtensionType-for-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#501)
[§](#associatedconstant.NAME)

The name identifying this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedconstant.NAME)

[Source](about:blank/src/geoarrow_schema/type.rs.html#503)
[§](#associatedtype.Metadata)

The metadata type of this extension type. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#associatedtype.Metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#505-507)
[§](#method.metadata-1)

Returns a reference to the metadata of this extension type, or `&()` if if this extension type defines no metadata (`Self::Metadata=()`).

[Source](about:blank/src/geoarrow_schema/type.rs.html#509-511)
[§](#method.serialize_metadata)

Returns the serialized representation of the metadata of this extension type, or `None` if this extension type defines no metadata (`Self::Metadata=()`). [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.serialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#513-515)
[§](#method.deserialize_metadata)

Deserialize the metadata of this extension type from the serialized representation of the metadata. An extension type that defines no metadata should expect `None` for the serialized metadata and return `Ok(())`. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.deserialize_metadata)

[Source](about:blank/src/geoarrow_schema/type.rs.html#517-532)
[§](#method.supports_data_type)

Returns `Ok(())` iff the given data type is supported by this extension type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#534-541)
[§](#method.try_new)

Construct this extension type for a field with the given data type and metadata. [Read more](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html#tymethod.try_new)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#410)
[§](#impl-From%3CMultiLineStringType%3E-for-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#410)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-Hash-for-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-PartialEq-for-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#265)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-Eq-for-MultiLineStringType)

[Source](about:blank/src/geoarrow_schema/type.rs.html#105-111)
[§](#impl-StructuralPartialEq-for-MultiLineStringType)

[§](#impl-Freeze-for-MultiLineStringType)

[§](#impl-RefUnwindSafe-for-MultiLineStringType)

[§](#impl-Send-for-MultiLineStringType)

[§](#impl-Sync-for-MultiLineStringType)

[§](#impl-Unpin-for-MultiLineStringType)

[§](#impl-UnwindSafe-for-MultiLineStringType)
