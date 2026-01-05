# arrow_schema::extension - Rust

[arrow_schema](../index.html)

## Module extension 

[Source](about:blank/src/arrow_schema/extension/mod.rs.html#18-260)

Expand description

Extension types.

This module is experimental. There might be breaking changes between minor releases.

## Structs[§](#structs)

[Bool8](struct.Bool8.html "struct arrow_schema::extension::Bool8")`canonical_extension_types`

The extension type for `8-bit Boolean`.

[FixedShapeTensor](struct.FixedShapeTensor.html "struct arrow_schema::extension::FixedShapeTensor")`canonical_extension_types`

The extension type for fixed shape tensor.

[FixedShapeTensorMetadata](struct.FixedShapeTensorMetadata.html "struct arrow_schema::extension::FixedShapeTensorMetadata")`canonical_extension_types`

Extension type metadata for [`FixedShapeTensor`](struct.FixedShapeTensor.html "struct arrow_schema::extension::FixedShapeTensor").

[Json](struct.Json.html "struct arrow_schema::extension::Json")`canonical_extension_types`

The extension type for `JSON`.

[JsonMetadata](struct.JsonMetadata.html "struct arrow_schema::extension::JsonMetadata")`canonical_extension_types`

Extension type metadata for [`Json`](struct.Json.html "struct arrow_schema::extension::Json").

[Opaque](struct.Opaque.html "struct arrow_schema::extension::Opaque")`canonical_extension_types`

The extension type for `Opaque`.

[OpaqueMetadata](struct.OpaqueMetadata.html "struct arrow_schema::extension::OpaqueMetadata")`canonical_extension_types`

Extension type metadata for [`Opaque`](struct.Opaque.html "struct arrow_schema::extension::Opaque").

[Uuid](struct.Uuid.html "struct arrow_schema::extension::Uuid")`canonical_extension_types`

The extension type for `UUID`.

[VariableShapeTensor](struct.VariableShapeTensor.html "struct arrow_schema::extension::VariableShapeTensor")`canonical_extension_types`

The extension type for `VariableShapeTensor`.

[VariableShapeTensorMetadata](struct.VariableShapeTensorMetadata.html "struct arrow_schema::extension::VariableShapeTensorMetadata")`canonical_extension_types`

Extension type metadata for [`VariableShapeTensor`](struct.VariableShapeTensor.html "struct arrow_schema::extension::VariableShapeTensor").

## Enums[§](#enums)

[CanonicalExtensionType](enum.CanonicalExtensionType.html "enum arrow_schema::extension::CanonicalExtensionType")`canonical_extension_types`

Canonical extension types.

## Constants[§](#constants)

[EXTENSION_TYPE_METADATA_KEY](constant.EXTENSION_TYPE_METADATA_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY")

The metadata key for a serialized representation of the [`ExtensionType`](trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType") necessary to reconstruct the custom type.

[EXTENSION_TYPE_NAME_KEY](constant.EXTENSION_TYPE_NAME_KEY.html "constant arrow_schema::extension::EXTENSION_TYPE_NAME_KEY")

The metadata key for the string name identifying an [`ExtensionType`](trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType").

## Traits[§](#traits)

[ExtensionType](trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType")

Extension types.
