# Module error Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#45" class="src">Source</a>

Expand description

DataFusion error types

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._config_datafusion_err.html" class="macro" title="macro datafusion::common::error::_config_datafusion_err">_config_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._config_err.html" class="macro" title="macro datafusion::common::error::_config_err">_config_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._exec_datafusion_err.html" class="macro" title="macro datafusion::common::error::_exec_datafusion_err">_exec_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._exec_err.html" class="macro" title="macro datafusion::common::error::_exec_err">_exec_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._internal_datafusion_err.html" class="macro" title="macro datafusion::common::error::_internal_datafusion_err">_internal_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._internal_err.html" class="macro" title="macro datafusion::common::error::_internal_err">_internal_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._not_impl_datafusion_err.html" class="macro" title="macro datafusion::common::error::_not_impl_datafusion_err">_not_impl_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._not_impl_err.html" class="macro" title="macro datafusion::common::error::_not_impl_err">_not_impl_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._plan_datafusion_err.html" class="macro" title="macro datafusion::common::error::_plan_datafusion_err">_plan_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._plan_err.html" class="macro" title="macro datafusion::common::error::_plan_err">_plan_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._resources_datafusion_err.html" class="macro" title="macro datafusion::common::error::_resources_datafusion_err">_resources_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._resources_err.html" class="macro" title="macro datafusion::common::error::_resources_err">_resources_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._schema_err.html" class="macro" title="macro datafusion::common::error::_schema_err">_schema_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._substrait_datafusion_err.html" class="macro" title="macro datafusion::common::error::_substrait_datafusion_err">_substrait_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/macro._substrait_err.html" class="macro" title="macro datafusion::common::error::_substrait_err">_substrait_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>  
A builder for [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html" class="enum" title="enum datafusion::common::error::DataFusionError">DataFusionError</a>  
DataFusion error

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.SchemaError.html" class="enum" title="enum datafusion::common::error::SchemaError">SchemaError</a>  
Schema-related errors

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/fn.add_possible_columns_to_diag.html" class="fn" title="fn datafusion::common::error::add_possible_columns_to_diag">add_possible_columns_to_diag</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/fn.field_not_found.html" class="fn" title="fn datafusion::common::error::field_not_found">field_not_found</a>  
Create a “field not found” DataFusion::SchemaError

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/fn.unqualified_field_not_found.html" class="fn" title="fn datafusion::common::error::unqualified_field_not_found">unqualified_field_not_found</a>  
Convenience wrapper over [`field_not_found`](https://docs.rs/datafusion/50.2.0/datafusion/common/fn.field_not_found.html "fn datafusion::common::field_not_found") for when there is no qualifier

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/type.GenericError.html" class="type" title="type datafusion::common::error::GenericError">GenericError</a>  
Error type for generic operations that could result in DataFusionError::External

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/type.Result.html" class="type" title="type datafusion::common::error::Result">Result</a>  
Result type for operations that could result in an [DataFusionError](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/type.SharedResult.html" class="type" title="type datafusion::common::error::SharedResult">SharedResult</a>  
Result type for operations that could result in an [DataFusionError](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") and needs to be shared (wrapped into `Arc`).
