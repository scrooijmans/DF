# Module signature Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#42" class="src">Source</a>

Expand description

Function signatures: [`Volatility`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html "enum datafusion::logical_expr::Volatility"), [`Signature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") and [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/struct.ImplicitCoercion.html" class="struct" title="struct datafusion::logical_expr_common::signature::ImplicitCoercion">ImplicitCoercion</a>  
Defines rules for implicit type coercion, specifying which source types can be coerced and the default type to use when coercing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/struct.Signature.html" class="struct" title="struct datafusion::logical_expr_common::signature::Signature">Signature</a>  
Provides information necessary for calling a function.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.ArrayFunctionArgument.html" class="enum" title="enum datafusion::logical_expr_common::signature::ArrayFunctionArgument">ArrayFunctionArgument</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.ArrayFunctionSignature.html" class="enum" title="enum datafusion::logical_expr_common::signature::ArrayFunctionSignature">ArrayFunctionSignature</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr_common::signature::Coercion">Coercion</a>  
Represents type coercion rules for function arguments, specifying both the desired type and optional implicit coercion rules for source types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr_common::signature::TypeSignature">TypeSignature</a>  
The types of arguments for which a function has implementations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr_common::signature::TypeSignatureClass">TypeSignatureClass</a>  
Represents the class of types that can be used in a function signature.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr_common::signature::Volatility">Volatility</a>  
How a function’s output changes with respect to a fixed input

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/constant.FIXED_SIZE_LIST_WILDCARD.html" class="constant" title="constant datafusion::logical_expr_common::signature::FIXED_SIZE_LIST_WILDCARD">FIXED_SIZE_LIST_WILDCARD</a>  
Constant that is used as a placeholder for any valid fixed size list. This is used where a function can accept a fixed size list type with any valid length. It exists to avoid the need to enumerate all possible fixed size list lengths.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/constant.TIMEZONE_WILDCARD.html" class="constant" title="constant datafusion::logical_expr_common::signature::TIMEZONE_WILDCARD">TIMEZONE_WILDCARD</a>  
Constant that is used as a placeholder for any valid timezone. This is used where a function can accept a timestamp type with any valid timezone, it exists to avoid the need to enumerate all possible timezones. See [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature") for more details.
