# Module logical_plan Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#59" class="src">Source</a>

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/builder/index.html" class="mod" title="mod datafusion::logical_expr::logical_plan::builder">builder</a>  
This module provides a builder for creating LogicalPlans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/index.html" class="mod" title="mod datafusion::logical_expr::logical_plan::display">display</a>  
This module provides logic for displaying LogicalPlans in various styles

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/dml/index.html" class="mod" title="mod datafusion::logical_expr::logical_plan::dml">dml</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/tree_node/index.html" class="mod" title="mod datafusion::logical_expr::logical_plan::tree_node">tree_node</a>  
[`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") based visiting and rewriting for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Aggregate.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Aggregate">Aggregate</a>  
Aggregates its input based on a set of grouping and aggregate expressions (e.g. SUM).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Analyze.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Analyze">Analyze</a>  
Runs the actual plan, and then prints the physical plan with with execution metrics.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.ColumnUnnestList.html" class="struct" title="struct datafusion::logical_expr::logical_plan::ColumnUnnestList">ColumnUnnestList</a>  
Represent the unnesting operation on a list column, such as the recursion depth and the output column name after unnesting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateCatalog.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateCatalog">CreateCatalog</a>  
Creates a catalog (aka “Database”).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateCatalogSchema.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateCatalogSchema">CreateCatalogSchema</a>  
Creates a schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateExternalTable">CreateExternalTable</a>  
Creates an external table.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateFunction.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateFunction">CreateFunction</a>  
Arguments passed to `CREATE FUNCTION`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateFunctionBody.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateFunctionBody">CreateFunctionBody</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateIndex.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateIndex">CreateIndex</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateMemoryTable.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateMemoryTable">CreateMemoryTable</a>  
Creates an in memory table.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateView.html" class="struct" title="struct datafusion::logical_expr::logical_plan::CreateView">CreateView</a>  
Creates a view.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Deallocate.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Deallocate">Deallocate</a>  
Deallocate a prepared statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DescribeTable.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DescribeTable">DescribeTable</a>  
Describe the schema of table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DistinctOn.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DistinctOn">DistinctOn</a>  
Removes duplicate rows from the input

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DmlStatement.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DmlStatement">DmlStatement</a>  
Modifies the content of a database

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DropCatalogSchema.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DropCatalogSchema">DropCatalogSchema</a>  
Drops a schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DropFunction.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DropFunction">DropFunction</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DropTable.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DropTable">DropTable</a>  
Drops a table.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.DropView.html" class="struct" title="struct datafusion::logical_expr::logical_plan::DropView">DropView</a>  
Drops a view.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.EmptyRelation.html" class="struct" title="struct datafusion::logical_expr::logical_plan::EmptyRelation">EmptyRelation</a>  
Relationship produces 0 or 1 placeholder rows with specified output schema In most cases the output schema for `EmptyRelation` would be empty, however, it can be non-empty typically for optimizer rules

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Execute.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Execute">Execute</a>  
Execute a prepared statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Explain.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Explain">Explain</a>  
Produces a relation with string representations of various parts of the plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.ExplainOption.html" class="struct" title="struct datafusion::logical_expr::logical_plan::ExplainOption">ExplainOption</a>  
Options for EXPLAIN

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Extension.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Extension">Extension</a>  
Extension operator defined outside of DataFusion

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Filter.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Filter">Filter</a>  
Filters rows from its input that do not match an expression (essentially a WHERE clause with a predicate expression).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Join.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Join">Join</a>  
Join two logical plans on one or more join columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Limit.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Limit">Limit</a>  
Produces the first `n` tuples from its input and discards the rest.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.LogicalPlanBuilder.html" class="struct" title="struct datafusion::logical_expr::logical_plan::LogicalPlanBuilder">LogicalPlanBuilder</a>  
Builder for logical plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.LogicalPlanBuilderOptions.html" class="struct" title="struct datafusion::logical_expr::logical_plan::LogicalPlanBuilderOptions">LogicalPlanBuilderOptions</a>  
Options for [`LogicalPlanBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html "struct datafusion::logical_expr::LogicalPlanBuilder")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.LogicalTableSource.html" class="struct" title="struct datafusion::logical_expr::logical_plan::LogicalTableSource">LogicalTableSource</a>  
Basic TableSource implementation intended for use in tests and documentation. It is expected that users will provide their own TableSource implementations or use DataFusion’s DefaultTableSource.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.OperateFunctionArg.html" class="struct" title="struct datafusion::logical_expr::logical_plan::OperateFunctionArg">OperateFunctionArg</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Prepare.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Prepare">Prepare</a>  
Prepare a statement but do not execute it. Prepare statements can have 0 or more `Expr::Placeholder` expressions that are filled in during execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Projection.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Projection">Projection</a>  
Evaluates an arbitrary list of expressions (essentially a SELECT with an expression list) on its input.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.RecursiveQuery.html" class="struct" title="struct datafusion::logical_expr::logical_plan::RecursiveQuery">RecursiveQuery</a>  
A variadic query operation, Recursive CTE.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Repartition.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Repartition">Repartition</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.SetVariable.html" class="struct" title="struct datafusion::logical_expr::logical_plan::SetVariable">SetVariable</a>  
Set a Variable’s value – value in [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Sort.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Sort">Sort</a>  
Sorts its input according to a list of sort expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.StringifiedPlan.html" class="struct" title="struct datafusion::logical_expr::logical_plan::StringifiedPlan">StringifiedPlan</a>  
Represents some sort of execution plan, in String form

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Subquery.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Subquery">Subquery</a>  
Subquery

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.SubqueryAlias.html" class="struct" title="struct datafusion::logical_expr::logical_plan::SubqueryAlias">SubqueryAlias</a>  
Aliased subquery

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.TableScan.html" class="struct" title="struct datafusion::logical_expr::logical_plan::TableScan">TableScan</a>  
Produces rows from a table provider by reference or from the context

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.TransactionEnd.html" class="struct" title="struct datafusion::logical_expr::logical_plan::TransactionEnd">TransactionEnd</a>  
Indicator that any current transaction should be terminated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.TransactionStart.html" class="struct" title="struct datafusion::logical_expr::logical_plan::TransactionStart">TransactionStart</a>  
Indicator that the following statements should be committed or rolled back atomically

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Union.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Union">Union</a>  
Union multiple inputs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Unnest.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Unnest">Unnest</a>  
Unnest a column that contains a nested list type. See [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions") for more details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Values.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Values">Values</a>  
Values expression. See [Postgres VALUES](https://www.postgresql.org/docs/current/queries-values.html) documentation for more details.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.Window.html" class="struct" title="struct datafusion::logical_expr::logical_plan::Window">Window</a>  
Window its input based on a set of window spec and window function (e.g. SUM or RANK)

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.DdlStatement.html" class="enum" title="enum datafusion::logical_expr::logical_plan::DdlStatement">DdlStatement</a>  
Various types of DDL (CREATE / DROP) catalog manipulation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.Distinct.html" class="enum" title="enum datafusion::logical_expr::logical_plan::Distinct">Distinct</a>  
Removes duplicate rows from the input

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::logical_plan::ExplainFormat">ExplainFormat</a>  
Output formats for controlling for Explain plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.FetchType.html" class="enum" title="enum datafusion::logical_expr::logical_plan::FetchType">FetchType</a>  
Different types of fetch expression in Limit plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.InvariantLevel.html" class="enum" title="enum datafusion::logical_expr::logical_plan::InvariantLevel">InvariantLevel</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.JoinConstraint.html" class="enum" title="enum datafusion::logical_expr::logical_plan::JoinConstraint">JoinConstraint</a>  
Join constraint

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.JoinType.html" class="enum" title="enum datafusion::logical_expr::logical_plan::JoinType">JoinType</a>  
Join type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::logical_plan::LogicalPlan">LogicalPlan</a>  
A `LogicalPlan` is a node in a tree of relational operators (such as Projection or Filter).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.Partitioning.html" class="enum" title="enum datafusion::logical_expr::logical_plan::Partitioning">Partitioning</a>  
Logical partitioning schemes supported by [`LogicalPlan::Repartition`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#variant.Repartition "variant datafusion::logical_expr::LogicalPlan::Repartition")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.PlanType.html" class="enum" title="enum datafusion::logical_expr::logical_plan::PlanType">PlanType</a>  
Represents which type of plan, when storing multiple for use in EXPLAIN plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.SkipType.html" class="enum" title="enum datafusion::logical_expr::logical_plan::SkipType">SkipType</a>  
Different types of skip expression in Limit plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::logical_plan::Statement">Statement</a>  
Various types of Statements.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.TransactionAccessMode.html" class="enum" title="enum datafusion::logical_expr::logical_plan::TransactionAccessMode">TransactionAccessMode</a>  
Indicates if this transaction is allowed to write

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.TransactionConclusion.html" class="enum" title="enum datafusion::logical_expr::logical_plan::TransactionConclusion">TransactionConclusion</a>  
Indicates if a transaction was committed or aborted

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.TransactionIsolationLevel.html" class="enum" title="enum datafusion::logical_expr::logical_plan::TransactionIsolationLevel">TransactionIsolationLevel</a>  
Indicates ANSI transaction isolation level

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/enum.WriteOp.html" class="enum" title="enum datafusion::logical_expr::logical_plan::WriteOp">WriteOp</a>  
The type of DML operation to perform.

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/constant.UNNAMED_TABLE.html" class="constant" title="constant datafusion::logical_expr::logical_plan::UNNAMED_TABLE">UNNAMED_TABLE</a>  
Default table name for unnamed table

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/trait.ToStringifiedPlan.html" class="trait" title="trait datafusion::logical_expr::logical_plan::ToStringifiedPlan">ToStringifiedPlan</a>  
Trait for something that can be formatted as a stringified plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::logical_plan::UserDefinedLogicalNode">UserDefinedLogicalNode</a>  
This defines the interface for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") nodes that can be used to extend DataFusion with custom relational operators.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/trait.UserDefinedLogicalNodeCore.html" class="trait" title="trait datafusion::logical_expr::logical_plan::UserDefinedLogicalNodeCore">UserDefinedLogicalNodeCore</a>  
This trait facilitates implementation of the [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode").

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.assert_expected_schema.html" class="fn" title="fn datafusion::logical_expr::logical_plan::assert_expected_schema">assert_expected_schema</a>  
Returns an error if the plan does not have the expected schema. Ignores metadata and nullability.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.build_join_schema.html" class="fn" title="fn datafusion::logical_expr::logical_plan::build_join_schema">build_join_schema</a>  
Creates a schema for a join operation. The fields from the left side are first

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.check_subquery_expr.html" class="fn" title="fn datafusion::logical_expr::logical_plan::check_subquery_expr">check_subquery_expr</a>  
Do necessary check on subquery expressions and fail the invalid plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.display_schema.html" class="fn" title="fn datafusion::logical_expr::logical_plan::display_schema">display_schema</a>  
Print the schema in a compact representation to `buf`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.projection_schema.html" class="fn" title="fn datafusion::logical_expr::logical_plan::projection_schema">projection_schema</a>  
Computes the schema of the result produced by applying a projection to the input logical plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.requalify_sides_if_needed.html" class="fn" title="fn datafusion::logical_expr::logical_plan::requalify_sides_if_needed">requalify_sides_if_needed</a>  
(Re)qualify the sides of a join if needed, i.e. if the columns from one side would otherwise conflict with the columns from the other. This is especially useful for queries that come as Substrait, since Substrait doesn’t currently allow specifying aliases, neither for columns nor for tables. DataFusion requires columns to be uniquely identifiable, in some places (see e.g. DFSchema::check_names). The function returns:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.table_scan.html" class="fn" title="fn datafusion::logical_expr::logical_plan::table_scan">table_scan</a>  
Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema. This is mostly used for testing and documentation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.union.html" class="fn" title="fn datafusion::logical_expr::logical_plan::union">union</a>  
Union two [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/fn.wrap_projection_for_join_if_necessary.html" class="fn" title="fn datafusion::logical_expr::logical_plan::wrap_projection_for_join_if_necessary">wrap_projection_for_join_if_necessary</a>  
Wrap projection for a plan, if the join keys contains normal expression.
