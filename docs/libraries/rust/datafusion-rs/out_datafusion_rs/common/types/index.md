# Module types Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#60" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/struct.LogicalField.html" class="struct" title="struct datafusion::common::types::LogicalField">LogicalField</a>  
A record of a logical type, its name and its nullability.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/struct.LogicalFields.html" class="struct" title="struct datafusion::common::types::LogicalFields">LogicalFields</a>  
A cheaply cloneable, owned collection of [`LogicalFieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/type.LogicalFieldRef.html "type datafusion::common::types::LogicalFieldRef").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/struct.LogicalUnionFields.html" class="struct" title="struct datafusion::common::types::LogicalUnionFields">LogicalUnionFields</a>  
A cheaply cloneable, owned collection of [`LogicalFieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/type.LogicalFieldRef.html "type datafusion::common::types::LogicalFieldRef") and their corresponding type ids.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>  
Representation of a type that DataFusion can handle natively. It is a subset of the physical variants in Arrow’s native [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.TypeParameter.html" class="enum" title="enum datafusion::common::types::TypeParameter">TypeParameter</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.TypeSignature.html" class="enum" title="enum datafusion::common::types::TypeSignature">TypeSignature</a>  
Signature that uniquely identifies a type among other types.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html" class="trait" title="trait datafusion::common::types::LogicalType">LogicalType</a>  
Representation of a logical type with its signature and its native backing type.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_binary.html" class="fn" title="fn datafusion::common::types::logical_binary">logical_binary</a>  
Getter for singleton instance of a logical type representing [`NativeType::Binary`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Binary "variant datafusion::common::types::NativeType::Binary").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_boolean.html" class="fn" title="fn datafusion::common::types::logical_boolean">logical_boolean</a>  
Getter for singleton instance of a logical type representing [`NativeType::Boolean`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Boolean "variant datafusion::common::types::NativeType::Boolean").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_date.html" class="fn" title="fn datafusion::common::types::logical_date">logical_date</a>  
Getter for singleton instance of a logical type representing [`NativeType::Date`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Date "variant datafusion::common::types::NativeType::Date").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_float16.html" class="fn" title="fn datafusion::common::types::logical_float16">logical_float16</a>  
Getter for singleton instance of a logical type representing [`NativeType::Float16`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Float16 "variant datafusion::common::types::NativeType::Float16").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_float32.html" class="fn" title="fn datafusion::common::types::logical_float32">logical_float32</a>  
Getter for singleton instance of a logical type representing [`NativeType::Float32`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Float32 "variant datafusion::common::types::NativeType::Float32").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_float64.html" class="fn" title="fn datafusion::common::types::logical_float64">logical_float64</a>  
Getter for singleton instance of a logical type representing [`NativeType::Float64`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Float64 "variant datafusion::common::types::NativeType::Float64").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_int8.html" class="fn" title="fn datafusion::common::types::logical_int8">logical_int8</a>  
Getter for singleton instance of a logical type representing [`NativeType::Int8`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Int8 "variant datafusion::common::types::NativeType::Int8").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_int16.html" class="fn" title="fn datafusion::common::types::logical_int16">logical_int16</a>  
Getter for singleton instance of a logical type representing [`NativeType::Int16`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Int16 "variant datafusion::common::types::NativeType::Int16").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_int32.html" class="fn" title="fn datafusion::common::types::logical_int32">logical_int32</a>  
Getter for singleton instance of a logical type representing [`NativeType::Int32`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Int32 "variant datafusion::common::types::NativeType::Int32").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_int64.html" class="fn" title="fn datafusion::common::types::logical_int64">logical_int64</a>  
Getter for singleton instance of a logical type representing [`NativeType::Int64`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Int64 "variant datafusion::common::types::NativeType::Int64").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_null.html" class="fn" title="fn datafusion::common::types::logical_null">logical_null</a>  
Getter for singleton instance of a logical type representing [`NativeType::Null`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.Null "variant datafusion::common::types::NativeType::Null").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_string.html" class="fn" title="fn datafusion::common::types::logical_string">logical_string</a>  
Getter for singleton instance of a logical type representing [`NativeType::String`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.String "variant datafusion::common::types::NativeType::String").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_uint8.html" class="fn" title="fn datafusion::common::types::logical_uint8">logical_uint8</a>  
Getter for singleton instance of a logical type representing [`NativeType::UInt8`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.UInt8 "variant datafusion::common::types::NativeType::UInt8").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_uint16.html" class="fn" title="fn datafusion::common::types::logical_uint16">logical_uint16</a>  
Getter for singleton instance of a logical type representing [`NativeType::UInt16`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.UInt16 "variant datafusion::common::types::NativeType::UInt16").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_uint32.html" class="fn" title="fn datafusion::common::types::logical_uint32">logical_uint32</a>  
Getter for singleton instance of a logical type representing [`NativeType::UInt32`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.UInt32 "variant datafusion::common::types::NativeType::UInt32").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/fn.logical_uint64.html" class="fn" title="fn datafusion::common::types::logical_uint64">logical_uint64</a>  
Getter for singleton instance of a logical type representing [`NativeType::UInt64`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#variant.UInt64 "variant datafusion::common::types::NativeType::UInt64").

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/type.LogicalFieldRef.html" class="type" title="type datafusion::common::types::LogicalFieldRef">LogicalFieldRef</a>  
A reference counted [`LogicalField`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/struct.LogicalField.html "struct datafusion::common::types::LogicalField").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/type.LogicalTypeRef.html" class="type" title="type datafusion::common::types::LogicalTypeRef">LogicalTypeRef</a>  
A reference counted [`LogicalType`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/trait.LogicalType.html "trait datafusion::common::types::LogicalType").
