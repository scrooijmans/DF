# Module common Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#762" class="src">Source</a>

Expand description

re-export of [`datafusion_common`](https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_common/index.html "mod datafusion_common") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/index.html" class="mod" title="mod datafusion::common::alias">alias</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html" class="mod" title="mod datafusion::common::arrow">arrow</a>  
A complete, safe, native Rust implementation of [Apache Arrow](https://arrow.apache.org), a cross-language development platform for in-memory data.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/index.html" class="mod" title="mod datafusion::common::cast">cast</a>  
This module provides DataFusion specific casting functions that provide error handling. They are intended to “never fail” but provide an error message rather than a panic, as the corresponding kernels in arrow-rs such as `as_boolean_array` do.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/config/index.html" class="mod" title="mod datafusion::common::config">config</a>  
Runtime configuration, via [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/index.html" class="mod" title="mod datafusion::common::cse">cse</a>  
Common Subexpression Elimination logic implemented in [`CSE`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/struct.CSE.html "struct datafusion::common::cse::CSE") can be controlled with a [`CSEController`](https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.CSEController.html "trait datafusion::common::cse::CSEController"), that defines how to eliminate common subtrees from a particular [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") tree.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/index.html" class="mod" title="mod datafusion::common::diagnostic">diagnostic</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/display/index.html" class="mod" title="mod datafusion::common::display">display</a>  
Types for plan display

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/encryption/index.html" class="mod" title="mod datafusion::common::encryption">encryption</a>  
This module provides types and functions related to encryption in Parquet files.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/index.html" class="mod" title="mod datafusion::common::error">error</a>  
DataFusion error types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/index.html" class="mod" title="mod datafusion::common::file_options">file_options</a>  
Options related to how files should be written

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/format/index.html" class="mod" title="mod datafusion::common::format">format</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/hash_utils/index.html" class="mod" title="mod datafusion::common::hash_utils">hash_utils</a>  
Functionality used both on logical and physical plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/instant/index.html" class="mod" title="mod datafusion::common::instant">instant</a>  
WASM-compatible `Instant` wrapper.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/nested_struct/index.html" class="mod" title="mod datafusion::common::nested_struct">nested_struct</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/parsers/index.html" class="mod" title="mod datafusion::common::parsers">parsers</a>  
Interval parsing logic

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/index.html" class="mod" title="mod datafusion::common::pruning">pruning</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/index.html" class="mod" title="mod datafusion::common::rounding">rounding</a>  
Floating point rounding mode utility library TODO: Remove this custom implementation and the “libc” dependency when floating-point rounding mode manipulation functions become available in Rust.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/runtime/index.html" class="mod" title="mod datafusion::common::runtime">runtime</a>  
re-export of [`datafusion_common_runtime`](https://docs.rs/datafusion-common-runtime/50.2.0/x86_64-unknown-linux-gnu/datafusion_common_runtime/index.html "mod datafusion_common_runtime") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/index.html" class="mod" title="mod datafusion::common::scalar">scalar</a>  
[`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue"): stores single values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/spans/index.html" class="mod" title="mod datafusion::common::spans">spans</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/index.html" class="mod" title="mod datafusion::common::stats">stats</a>  
This module provides data structures to represent statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/test_util/index.html" class="mod" title="mod datafusion::common::test_util">test_util</a>  
Utility functions to make testing DataFusion based crates easier

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/index.html" class="mod" title="mod datafusion::common::tree_node">tree_node</a>  
[`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") for visiting and rewriting expression and plan trees

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/index.html" class="mod" title="mod datafusion::common::types">types</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/index.html" class="mod" title="mod datafusion::common::utils">utils</a>  
This module provides the bisect function, which implements binary search.

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._config_datafusion_err.html" class="macro" title="macro datafusion::common::_config_datafusion_err">_config_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._exec_datafusion_err.html" class="macro" title="macro datafusion::common::_exec_datafusion_err">_exec_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._internal_datafusion_err.html" class="macro" title="macro datafusion::common::_internal_datafusion_err">_internal_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._not_impl_datafusion_err.html" class="macro" title="macro datafusion::common::_not_impl_datafusion_err">_not_impl_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._plan_datafusion_err.html" class="macro" title="macro datafusion::common::_plan_datafusion_err">_plan_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._resources_datafusion_err.html" class="macro" title="macro datafusion::common::_resources_datafusion_err">_resources_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro._substrait_datafusion_err.html" class="macro" title="macro datafusion::common::_substrait_datafusion_err">_substrait_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.arrow_datafusion_err.html" class="macro" title="macro datafusion::common::arrow_datafusion_err">arrow_datafusion_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.arrow_err.html" class="macro" title="macro datafusion::common::arrow_err">arrow_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.assert_batches_eq.html" class="macro" title="macro datafusion::common::assert_batches_eq">assert_batches_eq</a>  
Compares formatted output of a record batch with an expected vector of strings, with the result of pretty formatting record batches. This is a macro so errors appear on the correct line

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.assert_batches_sorted_eq.html" class="macro" title="macro datafusion::common::assert_batches_sorted_eq">assert_batches_sorted_eq</a>  
Compares formatted output of a record batch with an expected vector of strings in a way that order does not matter. This is a macro so errors appear on the correct line

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.assert_contains.html" class="macro" title="macro datafusion::common::assert_contains">assert_contains</a>  
A macro to assert that one string is contained within another with a nice error message if they are not.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.assert_not_contains.html" class="macro" title="macro datafusion::common::assert_not_contains">assert_not_contains</a>  
A macro to assert that one string is NOT contained within another with a nice error message if they are are.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_datafusion_err.html" class="macro" title="macro datafusion::common::config_datafusion_err">config_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_err.html" class="macro" title="macro datafusion::common::config_err">config_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_field.html" class="macro" title="macro datafusion::common::config_field">config_field</a>  
Macro that generates [`ConfigField`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html "trait datafusion::config::ConfigField") for a given type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_namespace.html" class="macro" title="macro datafusion::common::config_namespace">config_namespace</a>  
A macro that wraps a configuration struct and automatically derives [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and [`ConfigField`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html "trait datafusion::config::ConfigField") for it, allowing it to be used in the [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") configuration tree.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.context.html" class="macro" title="macro datafusion::common::context">context</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.create_array.html" class="macro" title="macro datafusion::common::create_array">create_array</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.downcast_value.html" class="macro" title="macro datafusion::common::downcast_value">downcast_value</a>  
Downcast an Arrow Array to a concrete type, return an `DataFusionError::Internal` if the cast is not possible. In normal usage of DataFusion the downcast should always succeed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.exec_datafusion_err.html" class="macro" title="macro datafusion::common::exec_datafusion_err">exec_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.exec_err.html" class="macro" title="macro datafusion::common::exec_err">exec_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.extensions_options.html" class="macro" title="macro datafusion::common::extensions_options">extensions_options</a>  
Convenience macro to create [`ExtensionsOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.internal_datafusion_err.html" class="macro" title="macro datafusion::common::internal_datafusion_err">internal_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.internal_err.html" class="macro" title="macro datafusion::common::internal_err">internal_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.not_impl_datafusion_err.html" class="macro" title="macro datafusion::common::not_impl_datafusion_err">not_impl_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.not_impl_err.html" class="macro" title="macro datafusion::common::not_impl_err">not_impl_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.plan_datafusion_err.html" class="macro" title="macro datafusion::common::plan_datafusion_err">plan_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.plan_err.html" class="macro" title="macro datafusion::common::plan_err">plan_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.record_batch.html" class="macro" title="macro datafusion::common::record_batch">record_batch</a>  
Creates a record batch from literal slice of values, suitable for rapid testing and development.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.resources_datafusion_err.html" class="macro" title="macro datafusion::common::resources_datafusion_err">resources_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.resources_err.html" class="macro" title="macro datafusion::common::resources_err">resources_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.schema_datafusion_err.html" class="macro" title="macro datafusion::common::schema_datafusion_err">schema_datafusion_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.schema_err.html" class="macro" title="macro datafusion::common::schema_err">schema_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.sql_datafusion_err.html" class="macro" title="macro datafusion::common::sql_datafusion_err">sql_datafusion_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.sql_err.html" class="macro" title="macro datafusion::common::sql_err">sql_err</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.substrait_datafusion_err.html" class="macro" title="macro datafusion::common::substrait_datafusion_err">substrait_datafusion_err</a>  
Macro wraps `$ERR` to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.substrait_err.html" class="macro" title="macro datafusion::common::substrait_err">substrait_err</a>  
Macro wraps Err(`$ERR`) to add backtrace feature

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.unwrap_or_internal_err.html" class="macro" title="macro datafusion::common::unwrap_or_internal_err">unwrap_or_internal_err</a>  
Unwrap an `Option` if possible. Otherwise return an `DataFusionError::Internal`. In normal usage of DataFusion the unwrap should always succeed.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Column.html" class="struct" title="struct datafusion::common::Column">Column</a>  
A named reference to a qualified field in a schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>  
Statistics for a column within a relation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>  
This object encapsulates a list of functional constraints:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>  
DFSchema wraps an Arrow schema and adds relation names.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>  
Additional contextual information intended for end users, to help them understand what went wrong by providing human-readable messages, and locations in the source query that relate to the error in some way.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.FunctionalDependence.html" class="struct" title="struct datafusion::common::FunctionalDependence">FunctionalDependence</a>  
This object defines a functional dependence in the schema. A functional dependence defines a relationship between determinant keys and dependent columns. A determinant key is a column, or a set of columns, whose value uniquely determines values of some other (dependent) columns. If two rows have the same determinant key, dependent columns in these rows are necessarily the same. If the determinant key is unique, the set of dependent columns is equal to the entire schema and the determinant key can serve as a primary key. Note that a primary key may “downgrade” into a determinant key due to an operation such as a join, and this object is used to track dependence relationships in such cases. For more information on functional dependencies, see: <https://www.scaler.com/topics/dbms/functional-dependency-in-dbms/>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.FunctionalDependencies.html" class="struct" title="struct datafusion::common::FunctionalDependencies">FunctionalDependencies</a>  
This object encapsulates all functional dependencies in a given relation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Location.html" class="struct" title="struct datafusion::common::Location">Location</a>  
Represents a location, determined by a line and a column number, in the original SQL query.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.RecursionUnnestOption.html" class="struct" title="struct datafusion::common::RecursionUnnestOption">RecursionUnnestOption</a>  
Instruction on how to unnest a column (mostly with a list type) such as how to name the output, and how many level it should be unnested

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ResolvedTableReference.html" class="struct" title="struct datafusion::common::ResolvedTableReference">ResolvedTableReference</a>  
A fully resolved path to a table of the form “catalog.schema.table”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>  
Represents an interval of characters in the original SQL query.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Spans.html" class="struct" title="struct datafusion::common::Spans">Spans</a>  
A collection of [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html "struct datafusion::common::Span"), meant to be used as a field of entities whose location in the original SQL query is desired to be tracked. Sometimes an entity can have multiple spans. e.g. if you want to track the position of the column a that comes from SELECT 1 AS a UNION ALL SELECT 2 AS a you’ll need two spans.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>  
Statistics for a relation Fields are optional and can be inexact because the sources sometimes provide approximate estimates for performance reasons and the transformations output are not always predictable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>  
Options for unnesting a column that contains a list type, replicating values in the other, non nested rows.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Constraint.html" class="enum" title="enum datafusion::common::Constraint">Constraint</a>  
This object defines a constraint on a table.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.DataFusionError.html" class="enum" title="enum datafusion::common::DataFusionError">DataFusionError</a>  
DataFusion error

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.Dependency.html" class="enum" title="enum datafusion::common::Dependency">Dependency</a>  
Describes functional dependency mode.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinConstraint.html" class="enum" title="enum datafusion::common::JoinConstraint">JoinConstraint</a>  
Join constraint

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinSide.html" class="enum" title="enum datafusion::common::JoinSide">JoinSide</a>  
Join side. Stores the referred table side during calculations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.JoinType.html" class="enum" title="enum datafusion::common::JoinType">JoinType</a>  
Join type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.NullEquality.html" class="enum" title="enum datafusion::common::NullEquality">NullEquality</a>  
Represents the behavior for null values when evaluating equality. Currently, its primary use case is to define the behavior of joins for null values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ParamValues.html" class="enum" title="enum datafusion::common::ParamValues">ParamValues</a>  
The parameter value corresponding to the placeholder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html" class="enum" title="enum datafusion::common::ScalarValue">ScalarValue</a>  
A dynamically typed, nullable single value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html" class="enum" title="enum datafusion::common::SchemaError">SchemaError</a>  
Schema-related errors

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaReference.html" class="enum" title="enum datafusion::common::SchemaReference">SchemaReference</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>  
A multi part identifier (path) to a table that may require further resolution (e.g. `foo.bar`).

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/constant.DEFAULT_ARROW_EXTENSION.html" class="constant" title="constant datafusion::common::DEFAULT_ARROW_EXTENSION">DEFAULT_ARROW_EXTENSION</a>  
The default file extension of arrow files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/constant.DEFAULT_AVRO_EXTENSION.html" class="constant" title="constant datafusion::common::DEFAULT_AVRO_EXTENSION">DEFAULT_AVRO_EXTENSION</a>  
The default file extension of avro files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/constant.DEFAULT_CSV_EXTENSION.html" class="constant" title="constant datafusion::common::DEFAULT_CSV_EXTENSION">DEFAULT_CSV_EXTENSION</a>  
The default file extension of csv files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/constant.DEFAULT_JSON_EXTENSION.html" class="constant" title="constant datafusion::common::DEFAULT_JSON_EXTENSION">DEFAULT_JSON_EXTENSION</a>  
The default file extension of json files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/constant.DEFAULT_PARQUET_EXTENSION.html" class="constant" title="constant datafusion::common::DEFAULT_PARQUET_EXTENSION">DEFAULT_PARQUET_EXTENSION</a>  
The default file extension of parquet files

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>  
Provides schema information needed by certain methods of `Expr` (defined in the datafusion-common crate).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a>  
Define each `FileType`/`FileCompressionType`’s extension

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html" class="trait" title="trait datafusion::common::ScalarType">ScalarType</a>  
Trait used to map a NativeType to a ScalarValue

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.SchemaExt.html" class="trait" title="trait datafusion::common::SchemaExt">SchemaExt</a>  
DataFusion-specific extensions to [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ToDFSchema.html" class="trait" title="trait datafusion::common::ToDFSchema">ToDFSchema</a>  
Convenience trait to convert Schema like things to DFSchema and DFSchemaRef with fewer keystrokes

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.aggregate_functional_dependencies.html" class="fn" title="fn datafusion::common::aggregate_functional_dependencies">aggregate_functional_dependencies</a>  
Calculates functional dependencies for aggregate output, when there is a GROUP BY expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.field_not_found.html" class="fn" title="fn datafusion::common::field_not_found">field_not_found</a>  
Create a “field not found” DataFusion::SchemaError

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.get_required_group_by_exprs_indices.html" class="fn" title="fn datafusion::common::get_required_group_by_exprs_indices">get_required_group_by_exprs_indices</a>  
Returns indices for the minimal subset of GROUP BY expressions that are functionally equivalent to the original set of GROUP BY expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.get_target_functional_dependencies.html" class="fn" title="fn datafusion::common::get_target_functional_dependencies">get_target_functional_dependencies</a>  
Returns target indices, for the determinant keys that are inside group by expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.project_schema.html" class="fn" title="fn datafusion::common::project_schema">project_schema</a>  
Applies an optional projection to a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef"), returning the projected schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.qualified_name.html" class="fn" title="fn datafusion::common::qualified_name">qualified_name</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/fn.unqualified_field_not_found.html" class="fn" title="fn datafusion::common::unqualified_field_not_found">unqualified_field_not_found</a>  
Convenience wrapper over [`field_not_found`](https://docs.rs/datafusion/50.2.0/datafusion/common/fn.field_not_found.html "fn datafusion::common::field_not_found") for when there is no qualifier

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.DFSchemaRef.html" class="type" title="type datafusion::common::DFSchemaRef">DFSchemaRef</a>  
A reference-counted reference to a [DFSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.HashMap.html" class="type" title="type datafusion::common::HashMap">HashMap</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.HashSet.html" class="type" title="type datafusion::common::HashSet">HashSet</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.Result.html" class="type" title="type datafusion::common::Result">Result</a>  
Result type for operations that could result in an [DataFusionError](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/type.SharedResult.html" class="type" title="type datafusion::common::SharedResult">SharedResult</a>  
Result type for operations that could result in an [DataFusionError](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") and needs to be shared (wrapped into `Arc`).
