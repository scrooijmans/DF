# Module type_coercion Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#73" class="src">Source</a>

Expand description

Type coercion rules for DataFusion

Coercion is performed automatically by DataFusion when the types of arguments passed to a function or needed by operators do not exactly match the types required by that function / operator. In this case, DataFusion will attempt to *coerce* the arguments to types accepted by the function by inserting CAST operations.

CAST operations added by coercion are lossless and never discard information.

For example coercion from i32 -\> i64 might be performed because all valid i32 values can be represented using an i64. However, i64 -\> i32 is never performed as there are i64 values which can not be represented by i32 values.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/aggregates/index.html" class="mod" title="mod datafusion::logical_expr::type_coercion::aggregates">aggregates</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/binary/index.html" class="mod" title="mod datafusion::logical_expr::type_coercion::binary">binary</a>

Coercion rules for matching argument types for binary operators

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/functions/index.html" class="mod" title="mod datafusion::logical_expr::type_coercion::functions">functions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/other/index.html" class="mod" title="mod datafusion::logical_expr::type_coercion::other">other</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_datetime.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_datetime">is_datetime</a>  
Determine whether the given data type `dt` is a `Date` or `Timestamp`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_decimal.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_decimal">is_decimal</a>  
Determine whether the given data type `dt` is a `Decimal`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_interval.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_interval">is_interval</a>  
Determine whether the given data type ‘dt’ is a `Interval`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_null.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_null">is_null</a>  
Determine whether the given data type `dt` is `Null`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_signed_numeric.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_signed_numeric">is_signed_numeric</a>  
Determine whether the given data type `dt` represents signed numeric values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_timestamp.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_timestamp">is_timestamp</a>  
Determine whether the given data type `dt` is a `Timestamp`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/fn.is_utf8_or_utf8view_or_large_utf8.html" class="fn" title="fn datafusion::logical_expr::type_coercion::is_utf8_or_utf8view_or_large_utf8">is_utf8_or_utf8view_or_large_utf8</a>  
Determine whether the given data type `dt` is a `Utf8` or `Utf8View` or `LargeUtf8`.
