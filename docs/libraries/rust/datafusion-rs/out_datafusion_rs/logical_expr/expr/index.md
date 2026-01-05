# Module expr Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#47" class="src">Source</a>

Expand description

Logical Expressions: [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunction.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunction">AggregateFunction</a>

Aggregate function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunctionParams">AggregateFunctionParams</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Alias.html" class="struct" title="struct datafusion::logical_expr::expr::Alias">Alias</a>

Alias expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Between.html" class="struct" title="struct datafusion::logical_expr::expr::Between">Between</a>

BETWEEN expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.BinaryExpr.html" class="struct" title="struct datafusion::logical_expr::expr::BinaryExpr">BinaryExpr</a>

Binary expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Case.html" class="struct" title="struct datafusion::logical_expr::expr::Case">Case</a>

CASE expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Cast.html" class="struct" title="struct datafusion::logical_expr::expr::Cast">Cast</a>

Cast expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Exists.html" class="struct" title="struct datafusion::logical_expr::expr::Exists">Exists</a>

EXISTS expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ExprListDisplay.html" class="struct" title="struct datafusion::logical_expr::expr::ExprListDisplay">ExprListDisplay</a>

Formats a list of `&Expr` with a custom separator using SQL display format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Literal metadata

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.InList.html" class="struct" title="struct datafusion::logical_expr::expr::InList">InList</a>

InList expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.InSubquery.html" class="struct" title="struct datafusion::logical_expr::expr::InSubquery">InSubquery</a>

IN subquery

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Like.html" class="struct" title="struct datafusion::logical_expr::expr::Like">Like</a>

LIKE expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Placeholder.html" class="struct" title="struct datafusion::logical_expr::expr::Placeholder">Placeholder</a>

Placeholder, representing bind parameter values such as `$1` or `$name`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.PlannedReplaceSelectItem.html" class="struct" title="struct datafusion::logical_expr::expr::PlannedReplaceSelectItem">PlannedReplaceSelectItem</a>

The planned expressions for `REPLACE`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ScalarFunction.html" class="struct" title="struct datafusion::logical_expr::expr::ScalarFunction">ScalarFunction</a>

Invoke a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") with a set of arguments

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Sort.html" class="struct" title="struct datafusion::logical_expr::expr::Sort">Sort</a>

SORT expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.TryCast.html" class="struct" title="struct datafusion::logical_expr::expr::TryCast">TryCast</a>

TryCast Expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Unnest.html" class="struct" title="struct datafusion::logical_expr::expr::Unnest">Unnest</a>

UNNEST expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

Additional options for wildcards, e.g. Snowflake `EXCLUDE`/`RENAME` and Bigquery `EXCEPT`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunction">WindowFunction</a>

Window function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunctionParams.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunctionParams">WindowFunctionParams</a>

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::expr::Expr">Expr</a>  
Represents logical expressions such as `A + 1`, or `CAST(c1 AS int)`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/enum.GetFieldAccess.html" class="enum" title="enum datafusion::logical_expr::expr::GetFieldAccess">GetFieldAccess</a>  
Access a sub field of a nested type, such as `Field` or `List`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/enum.GroupingSet.html" class="enum" title="enum datafusion::logical_expr::expr::GroupingSet">GroupingSet</a>  
Grouping sets

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/enum.WindowFunctionDefinition.html" class="enum" title="enum datafusion::logical_expr::expr::WindowFunctionDefinition">WindowFunctionDefinition</a>  
A function used as a SQL window function

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/constant.OUTER_REFERENCE_COLUMN_PREFIX.html" class="constant" title="constant datafusion::logical_expr::expr::OUTER_REFERENCE_COLUMN_PREFIX">OUTER_REFERENCE_COLUMN_PREFIX</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/constant.UNNEST_COLUMN_PREFIX.html" class="constant" title="constant datafusion::logical_expr::expr::UNNEST_COLUMN_PREFIX">UNNEST_COLUMN_PREFIX</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/fn.intersect_metadata_for_union.html" class="fn" title="fn datafusion::logical_expr::expr::intersect_metadata_for_union">intersect_metadata_for_union</a>

Intersects multiple metadata instances for UNION operations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/fn.physical_name.html" class="fn" title="fn datafusion::logical_expr::expr::physical_name">physical_name</a>

The name of the column (field) that this `Expr` will produce in the physical plan. The difference from [Expr::schema_name](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.schema_name "method datafusion::prelude::Expr::schema_name") is that top-level columns are unqualified.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/fn.schema_name_from_exprs.html" class="fn" title="fn datafusion::logical_expr::expr::schema_name_from_exprs">schema_name_from_exprs</a>

Get schema_name for Vector of expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/fn.schema_name_from_sorts.html" class="fn" title="fn datafusion::logical_expr::expr::schema_name_from_sorts">schema_name_from_sorts</a>

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/type.SchemaFieldMetadata.html" class="type" title="type datafusion::logical_expr::expr::SchemaFieldMetadata">SchemaFieldMetadata</a>  
The metadata used in [`Field::metadata`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.metadata "method datafusion::common::arrow::datatypes::Field::metadata").
