# arrow_schema - Rust

## [Crate arrow_schema](#)

[![logo](https://arrow.apache.org/img/arrow-logo_chevrons_black-txt_white-bg.svg)](../arrow_schema/index.html)

## [arrow_schema](../arrow_schema/index.html)56.2.0

- [All Items](all.html)

### [Crate Items](#modules)

- [Modules](#modules "Modules")
- [Structs](#structs "Structs")
- [Enums](#enums "Enums")
- [Constants](#constants "Constants")
- [Type Aliases](#types "Type Aliases")

## Crate arrow_schema 

[Source](about:blank/src/arrow_schema/lib.rs.html#18-216)

Expand description

Arrow logical types

## Modules[§](#modules)

[extension](extension/index.html "mod arrow_schema::extension")

Extension types.

[ffi](ffi/index.html "mod arrow_schema::ffi")`ffi`

Contains declarations to bind to the [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html).

## Structs[§](#structs)

[Field](struct.Field.html "struct arrow_schema::Field")

Describes a single column in a [`Schema`](struct.Schema.html "struct arrow_schema::Schema").

[Fields](struct.Fields.html "struct arrow_schema::Fields")

A cheaply cloneable, owned slice of [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef")

[Schema](struct.Schema.html "struct arrow_schema::Schema")

Describes the meta-data of an ordered sequence of relative types.

[SchemaBuilder](struct.SchemaBuilder.html "struct arrow_schema::SchemaBuilder")

A builder to facilitate building a [`Schema`](struct.Schema.html "struct arrow_schema::Schema") from iteratively from [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef")

[SortOptions](struct.SortOptions.html "struct arrow_schema::SortOptions")

Options that define the sort order of a given column

[UnionFields](struct.UnionFields.html "struct arrow_schema::UnionFields")

A cheaply cloneable, owned collection of [`FieldRef`](type.FieldRef.html "type arrow_schema::FieldRef") and their corresponding type ids

## Enums[§](#enums)

[ArrowError](enum.ArrowError.html "enum arrow_schema::ArrowError")

Many different operations in the `arrow` crate return this error type.

[DataType](enum.DataType.html "enum arrow_schema::DataType")

Datatypes supported by this implementation of Apache Arrow.

[IntervalUnit](enum.IntervalUnit.html "enum arrow_schema::IntervalUnit")

YEAR_MONTH, DAY_TIME, MONTH_DAY_NANO interval in SQL style.

[TimeUnit](enum.TimeUnit.html "enum arrow_schema::TimeUnit")

An absolute length of time in seconds, milliseconds, microseconds or nanoseconds.

[UnionMode](enum.UnionMode.html "enum arrow_schema::UnionMode")

Sparse or Dense union layouts

## Constants[§](#constants)

[DECIMAL32_DEFAULT_SCALE](constant.DECIMAL32_DEFAULT_SCALE.html "constant arrow_schema::DECIMAL32_DEFAULT_SCALE")

The default scale for [DataType::Decimal32](about:blank/enum.DataType.html#variant.Decimal32 "variant arrow_schema::DataType::Decimal32") values

[DECIMAL32_MAX_PRECISION](constant.DECIMAL32_MAX_PRECISION.html "constant arrow_schema::DECIMAL32_MAX_PRECISION")

The maximum precision for [DataType::Decimal32](about:blank/enum.DataType.html#variant.Decimal32 "variant arrow_schema::DataType::Decimal32") values

[DECIMAL32_MAX_SCALE](constant.DECIMAL32_MAX_SCALE.html "constant arrow_schema::DECIMAL32_MAX_SCALE")

The maximum scale for [DataType::Decimal32](about:blank/enum.DataType.html#variant.Decimal32 "variant arrow_schema::DataType::Decimal32") values

[DECIMAL64_DEFAULT_SCALE](constant.DECIMAL64_DEFAULT_SCALE.html "constant arrow_schema::DECIMAL64_DEFAULT_SCALE")

The default scale for [DataType::Decimal64](about:blank/enum.DataType.html#variant.Decimal64 "variant arrow_schema::DataType::Decimal64") values

[DECIMAL64_MAX_PRECISION](constant.DECIMAL64_MAX_PRECISION.html "constant arrow_schema::DECIMAL64_MAX_PRECISION")

The maximum precision for [DataType::Decimal64](about:blank/enum.DataType.html#variant.Decimal64 "variant arrow_schema::DataType::Decimal64") values

[DECIMAL64_MAX_SCALE](constant.DECIMAL64_MAX_SCALE.html "constant arrow_schema::DECIMAL64_MAX_SCALE")

The maximum scale for [DataType::Decimal64](about:blank/enum.DataType.html#variant.Decimal64 "variant arrow_schema::DataType::Decimal64") values

[DECIMAL128_MAX_PRECISION](constant.DECIMAL128_MAX_PRECISION.html "constant arrow_schema::DECIMAL128_MAX_PRECISION")

The maximum precision for [DataType::Decimal128](about:blank/enum.DataType.html#variant.Decimal128 "variant arrow_schema::DataType::Decimal128") values

[DECIMAL128_MAX_SCALE](constant.DECIMAL128_MAX_SCALE.html "constant arrow_schema::DECIMAL128_MAX_SCALE")

The maximum scale for [DataType::Decimal128](about:blank/enum.DataType.html#variant.Decimal128 "variant arrow_schema::DataType::Decimal128") values

[DECIMAL256_MAX_PRECISION](constant.DECIMAL256_MAX_PRECISION.html "constant arrow_schema::DECIMAL256_MAX_PRECISION")

The maximum precision for [DataType::Decimal256](about:blank/enum.DataType.html#variant.Decimal256 "variant arrow_schema::DataType::Decimal256") values

[DECIMAL256_MAX_SCALE](constant.DECIMAL256_MAX_SCALE.html "constant arrow_schema::DECIMAL256_MAX_SCALE")

The maximum scale for [DataType::Decimal256](about:blank/enum.DataType.html#variant.Decimal256 "variant arrow_schema::DataType::Decimal256") values

[DECIMAL_DEFAULT_SCALE](constant.DECIMAL_DEFAULT_SCALE.html "constant arrow_schema::DECIMAL_DEFAULT_SCALE")

The default scale for [DataType::Decimal128](about:blank/enum.DataType.html#variant.Decimal128 "variant arrow_schema::DataType::Decimal128") and [DataType::Decimal256](about:blank/enum.DataType.html#variant.Decimal256 "variant arrow_schema::DataType::Decimal256") values

## Type Aliases[§](#types)

[FieldRef](type.FieldRef.html "type arrow_schema::FieldRef")

A reference counted [`Field`](struct.Field.html "struct arrow_schema::Field")

[SchemaRef](type.SchemaRef.html "type arrow_schema::SchemaRef")

A reference-counted reference to a [`Schema`](struct.Schema.html "struct arrow_schema::Schema").
