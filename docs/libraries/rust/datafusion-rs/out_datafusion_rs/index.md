# Crate datafusion Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#18-1150" class="src">Source</a>

Expand description

[DataFusion](https://datafusion.apache.org/) is an extensible query engine written in Rust that uses [Apache Arrow](https://arrow.apache.org) as its in-memory format. DataFusion’s target users are developers building fast and feature rich database and analytic systems, customized to particular workloads. Please see the [DataFusion website](https://datafusion.apache.org) for additional documentation, [use cases](https://datafusion.apache.org/user-guide/introduction.html#use-cases) and examples.

“Out of the box,” DataFusion offers [SQL](https://datafusion.apache.org/user-guide/sql/index.html) and [`Dataframe`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") APIs, excellent [performance](https://benchmark.clickhouse.com/), built-in support for CSV, Parquet, JSON, and Avro, extensive customization, and a great community. [Python Bindings](https://github.com/apache/datafusion-python) are also available.

DataFusion features a full query planner, a columnar, streaming, multi-threaded, vectorized execution engine, and partitioned data sources. You can customize DataFusion at almost all points including additional data sources, query languages, functions, custom operators and more. See the [Architecture](https://docs.rs/datafusion/50.2.0/datafusion/index.html#architecture) section below for more details.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#examples" class="doc-anchor">§</a>Examples

The main entry point for interacting with DataFusion is the [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext"). [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s represent expressions such as `a + b`.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#dataframe" class="doc-anchor">§</a>DataFrame

To execute a query against data stored in a CSV file using a [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame"):

``` rust

let ctx = SessionContext::new();

// create the dataframe
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;

// create a plan
let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;

// execute the plan
let results: Vec<RecordBatch> = df.collect().await?;

// format the results
let pretty_results = arrow::util::pretty::pretty_format_batches(&results)?
   .to_string();

let expected = vec![
    "+---+----------------+",
    "| a | min(?table?.b) |",
    "+---+----------------+",
    "| 1 | 2              |",
    "+---+----------------+"
];

assert_eq!(pretty_results.trim().lines().collect::<Vec<_>>(), expected);
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#sql" class="doc-anchor">§</a>SQL

To execute a query against a CSV file using [SQL](https://datafusion.apache.org/user-guide/sql/index.html):

``` rust

let ctx = SessionContext::new();

ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new()).await?;

// create a plan
let df = ctx.sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100").await?;

// execute the plan
let results: Vec<RecordBatch> = df.collect().await?;

// format the results
let pretty_results = arrow::util::pretty::pretty_format_batches(&results)?
  .to_string();

let expected = vec![
    "+---+----------------+",
    "| a | min(example.b) |",
    "+---+----------------+",
    "| 1 | 2              |",
    "+---+----------------+"
];

assert_eq!(pretty_results.trim().lines().collect::<Vec<_>>(), expected);
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#more-examples" class="doc-anchor">§</a>More Examples

There are many additional annotated examples of using DataFusion in the [datafusion-examples](https://github.com/apache/datafusion/tree/main/datafusion-examples) directory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#architecture" class="doc-anchor">§</a>Architecture

You can find a formal description of DataFusion’s architecture in our [SIGMOD 2024 Paper](https://dl.acm.org/doi/10.1145/3626246.3653368).

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#design-goals" class="doc-anchor">§</a>Design Goals

DataFusion’s Architecture Goals are:

1.  Work “out of the box”: Provide a very fast, world class query engine with minimal setup or required configuration.

2.  Customizable everything: All behavior should be customizable by implementing traits.

3.  Architecturally boring 🥱: Follow industrial best practice rather than trying cutting edge, but unproven, techniques.

With these principles, users start with a basic, high-performance engine and specialize it over time to suit their needs and available engineering capacity.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#overview--presentations" class="doc-anchor">§</a>Overview Presentations

The following presentations offer high level overviews of the different components and how they interact together.

- \[Apr 2023\]: The Apache DataFusion Architecture talks
  - *Query Engine*: [recording](https://youtu.be/NVKujPxwSBA) and [slides](https://docs.google.com/presentation/d/1D3GDVas-8y0sA4c8EOgdCvEjVND4s2E7I6zfs67Y4j8/edit#slide=id.p)
  - *Logical Plan and Expressions*: [recording](https://youtu.be/EzZTLiSJnhY) and [slides](https://docs.google.com/presentation/d/1ypylM3-w60kVDW7Q6S99AHzvlBgciTdjsAfqNP85K30)
  - *Physical Plan and Execution*: [recording](https://youtu.be/2jkWU3_w6z0) and [slides](https://docs.google.com/presentation/d/1cA2WQJ2qg6tx6y4Wf8FH2WVSm9JQ5UgmBWATHdik0hg)
- \[July 2022\]: DataFusion and Arrow: Supercharge Your Data Analytical Tool with a Rusty Query Engine: [recording](https://www.youtube.com/watch?v=Rii1VTn3seQ) and [slides](https://docs.google.com/presentation/d/1q1bPibvu64k2b7LPi7Yyb0k3gA1BiUYiUbEklqW1Ckc/view#slide=id.g11054eeab4c_0_1165)
- \[March 2021\]: The DataFusion architecture is described in *Query Engine Design and the Rust-Based DataFusion in Apache Arrow*: [recording](https://www.youtube.com/watch?v=K6eCAVEk4kU) (DataFusion content starts [~ 15 minutes in](https://www.youtube.com/watch?v=K6eCAVEk4kU&t=875s)) and [slides](https://www.slideshare.net/influxdata/influxdb-iox-tech-talks-query-engine-design-and-the-rustbased-datafusion-in-apache-arrow-244161934)
- \[February 2021\]: How DataFusion is used within the Ballista Project is described in *Ballista: Distributed Compute with Rust and Apache Arrow*: [recording](https://www.youtube.com/watch?v=ZZHQaOap9pQ)

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#customization-and-extension" class="doc-anchor">§</a>Customization and Extension

DataFusion is designed to be highly extensible, so you can start with a working, full featured engine, and then specialize any behavior for your use case. For example, some projects may add custom [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") operators, or create their own query language that directly creates [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") rather than using the built in SQL planner, [`SqlToRel`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html "struct datafusion::sql::planner::SqlToRel").

In order to achieve this, DataFusion supports extension at many points:

- read from any datasource ([`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider"))
- define your own catalogs, schemas, and table lists ([`catalog`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/index.html "mod datafusion::catalog") and [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider"))
- build your own query language or plans ([`LogicalPlanBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html "struct datafusion::logical_expr::LogicalPlanBuilder"))
- declare and use user-defined functions ([`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF"), and [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF"), [`WindowUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowUDF.html "struct datafusion::logical_expr::WindowUDF"))
- add custom plan rewrite passes ([`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule"), [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule") and [`PhysicalOptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html "trait datafusion::physical_optimizer::PhysicalOptimizerRule"))
- extend the planner to use user-defined logical and physical nodes ([`QueryPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/trait.QueryPlanner.html "trait datafusion::execution::context::QueryPlanner"))

You can find examples of each of them in the [datafusion-examples](https://github.com/apache/datafusion/tree/main/datafusion-examples) directory.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#query-planning-and-execution-overview" class="doc-anchor">§</a>Query Planning and Execution Overview

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#sql-1" class="doc-anchor">§</a>SQL

``` text
                Parsed with            SqlToRel creates
                sqlparser              initial plan
┌───────────────┐           ┌─────────┐             ┌─────────────┐
│   SELECT *    │           │Query {  │             │Project      │
│   FROM ...    │──────────▶│..       │────────────▶│  TableScan  │
│               │           │}        │             │    ...      │
└───────────────┘           └─────────┘             └─────────────┘

  SQL String                 sqlparser               LogicalPlan
                             AST nodes
```

1.  The query string is parsed to an Abstract Syntax Tree (AST) [`Statement`](https://docs.rs/sqlparser/latest/sqlparser/ast/enum.Statement.html) using [sqlparser](https://docs.rs/sqlparser/latest/sqlparser).

2.  The AST is converted to a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") and logical expressions [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s to compute the desired result by [`SqlToRel`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html "struct datafusion::sql::planner::SqlToRel"). This phase also includes name and type resolution (“binding”).

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#dataframe-1" class="doc-anchor">§</a>DataFrame

When executing plans using the [`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") API, the process is identical as with SQL, except the DataFrame API builds the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") directly using [`LogicalPlanBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalPlanBuilder.html "struct datafusion::logical_expr::LogicalPlanBuilder"). Systems that have their own custom query languages typically also build [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") directly.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#planning" class="doc-anchor">§</a>Planning

``` text
            AnalyzerRules and      PhysicalPlanner          PhysicalOptimizerRules
            OptimizerRules         creates ExecutionPlan    improve performance
            rewrite plan
┌─────────────┐        ┌─────────────┐      ┌─────────────────┐        ┌─────────────────┐
│Project      │        │Project(x, y)│      │ProjectExec      │        │ProjectExec      │
│  TableScan  │──...──▶│  TableScan  │─────▶│  ...            │──...──▶│  ...            │
│    ...      │        │    ...      │      │   DataSourceExec│        │   DataSourceExec│
└─────────────┘        └─────────────┘      └─────────────────┘        └─────────────────┘

 LogicalPlan            LogicalPlan         ExecutionPlan             ExecutionPlan
```

To process large datasets with many rows as efficiently as possible, significant effort is spent planning and optimizing, in the following manner:

1.  The [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") is checked and rewritten to enforce semantic rules, such as type coercion, by [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s

2.  The [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") is rewritten by [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")s, such as projection and filter pushdown, to improve its efficiency.

3.  The [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") is converted to an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") by a [`PhysicalPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/trait.PhysicalPlanner.html "trait datafusion::physical_planner::PhysicalPlanner")

4.  The [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") is rewritten by [`PhysicalOptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html "trait datafusion::physical_optimizer::PhysicalOptimizerRule")s, such as sort and join selection, to improve its efficiency.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#data-sources" class="doc-anchor">§</a>Data Sources

``` text
Planning       │
requests       │            TableProvider::scan
information    │            creates an
such as schema │            ExecutionPlan
               │
               ▼
  ┌─────────────────────────┐         ┌───────────────┐
  │                         │         │               │
  │impl TableProvider       │────────▶│DataSourceExec │
  │                         │         │               │
  └─────────────────────────┘         └───────────────┘
        TableProvider
        (built in or user provided)    ExecutionPlan
```

A [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") provides information for planning and an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for execution. DataFusion includes two built-in table providers that support common file formats and require no runtime services, [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") and [`MemTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.MemTable.html "struct datafusion::datasource::MemTable"). You can add support for any other data source and/or file formats by implementing the [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") trait.

See also:

1.  [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable"): Reads data from one or more Parquet, JSON, CSV, or AVRO files in one or more local or remote directories. Supports HIVE style partitioning, optional compression, directly reading from remote object store, file metadata caching, and more.

2.  [`MemTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.MemTable.html "struct datafusion::datasource::MemTable"): Reads data from in memory [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es.

3.  [`StreamingTable`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/streaming/struct.StreamingTable.html "struct datafusion::catalog::streaming::StreamingTable"): Reads data from potentially unbounded inputs.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#plan-representations" class="doc-anchor">§</a>Plan Representations

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#logical-plans" class="doc-anchor">§</a>Logical Plans

Logical planning yields [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") nodes and [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") representing expressions which are [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema") aware and represent statements independent of how they are physically executed. A [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") is a Directed Acyclic Graph (DAG) of other [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s, each potentially containing embedded [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s.

[`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s can be rewritten with [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") API, see the [`tree_node module`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/tree_node/index.html "mod datafusion::logical_expr::logical_plan::tree_node") for more details.

[`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s can also be rewritten with [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") API and simplified using [`ExprSimplifier`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html "struct datafusion::optimizer::simplify_expressions::ExprSimplifier"). Examples of working with and executing [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s can be found in the [`expr_api`.rs](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs) example

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#physical-plans" class="doc-anchor">§</a>Physical Plans

An [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") (sometimes referred to as a “physical plan”) is a plan that can be executed against data. It a DAG of other [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s each potentially containing expressions that implement the [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") trait.

Compared to a [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan"), an [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") has additional concrete information about how to perform calculations (e.g. hash vs merge join), and how data flows during execution (e.g. partitioning and sortedness).

[cp_solver](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/intervals/cp_solver/index.html "mod datafusion::physical_expr::intervals::cp_solver") performs range propagation analysis on [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s and [`PruningPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html "struct datafusion::physical_optimizer::pruning::PruningPredicate") can prove certain boolean [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s used for filtering can never be `true` using additional statistical information.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#execution" class="doc-anchor">§</a>Execution

``` text
           ExecutionPlan::execute             Calling next() on the
           produces a stream                  stream produces the data

┌────────────────┐      ┌─────────────────────────┐         ┌────────────┐
│ProjectExec     │      │impl                     │    ┌───▶│RecordBatch │
│  ...           │─────▶│SendableRecordBatchStream│────┤    └────────────┘
│  DataSourceExec│      │                         │    │    ┌────────────┐
└────────────────┘      └─────────────────────────┘    ├───▶│RecordBatch │
              ▲                                        │    └────────────┘
ExecutionPlan │                                        │         ...
              │                                        │
              │                                        │    ┌────────────┐
            PhysicalOptimizerRules                     ├───▶│RecordBatch │
            request information                        │    └────────────┘
            such as partitioning                       │    ┌ ─ ─ ─ ─ ─ ─
                                                       └───▶ None        │
                                                            └ ─ ─ ─ ─ ─ ─
```

[`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s process data using the [Apache Arrow](https://arrow.apache.org) memory format, making heavy use of functions from the [arrow](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html "mod datafusion::common::arrow") crate. Values are represented with [`ColumnarValue`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html "enum datafusion::logical_expr::ColumnarValue"), which are either [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") (single constant values) or [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") (Arrow Arrays).

Calling [`execute`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.execute "method datafusion::physical_plan::ExecutionPlan::execute") produces 1 or more partitions of data, as a [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream"), which implements a pull based execution API. Calling [`next()`](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.next "method futures_util::stream::stream::StreamExt::next")`.await` will incrementally compute and return the next [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch"). Balanced parallelism is achieved using [Volcano style](https://doi.org/10.1145/93605.98720) “Exchange” operations implemented by [`RepartitionExec`](https://docs.rs/datafusion/latest/datafusion/physical_plan/repartition/struct.RepartitionExec.html).

While some recent research such as [Morsel-Driven Parallelism](https://db.in.tum.de/~leis/papers/morsels.pdf) describes challenges with the pull style Volcano execution model on NUMA architectures, in practice DataFusion achieves similar scalability as systems that use push driven schedulers [such as DuckDB](https://github.com/duckdb/duckdb/issues/1583). See the [DataFusion paper in SIGMOD 2024](https://github.com/apache/datafusion/files/15149988/DataFusion_Query_Engine___SIGMOD_2024-FINAL-mk4.pdf) for more details.

See the [implementors of `ExecutionPlan`](https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html#implementors) for a list of physical operators available.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#streaming-execution" class="doc-anchor">§</a>Streaming Execution

DataFusion is a “streaming” query engine which means [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s incrementally read from their input(s) and compute output one [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") at a time by continually polling [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream")s. Output and intermediate [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")s each have approximately `batch_size` rows, which amortizes per-batch overhead of execution.

Note that certain operations, sometimes called “pipeline breakers”, (for example full sorts or hash aggregations) are fundamentally non streaming and must read their input fully before producing **any** output. As much as possible, other operators read a single [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") from their input to produce a single [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") as output.

For example, given this SQL query:

``` sql
SELECT date_trunc('month', time) FROM data WHERE id IN (10,20,30);
```

The diagram below shows the call sequence when a consumer calls [`next()`](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.next "method futures_util::stream::stream::StreamExt::next") to get the next [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") of output. While it is possible that some steps run on different threads, typically tokio will use the same thread that called [`next()`](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.next "method futures_util::stream::stream::StreamExt::next") to read from the input, apply the filter, and return the results without interleaving any other operations. This results in excellent cache locality as the same CPU core that produces the data often consumes it immediately as well.

``` text

Step 3: FilterExec calls next()       Step 2: ProjectionExec calls
        on input Stream                  next() on input Stream
        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─      ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                           │                                               Step 1: Consumer
        ▼                        ▼                           │               calls next()
┏━━━━━━━━━━━━━━━━┓     ┏━━━━━┻━━━━━━━━━━━━━┓      ┏━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                ┃     ┃                   ┃      ┃                        ◀ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┃  DataSource    ┃     ┃                   ┃      ┃                        ┃
┃    (e.g.       ┃     ┃    FilterExec     ┃      ┃     ProjectionExec     ┃
┃ ParquetSource) ┃     ┃id IN (10, 20, 30) ┃      ┃date_bin('month', time) ┃
┃                ┃     ┃                   ┃      ┃                        ┣ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ▶
┃                ┃     ┃                   ┃      ┃                        ┃
┗━━━━━━━━━━━━━━━━┛     ┗━━━━━━━━━━━┳━━━━━━━┛      ┗━━━━━━━━━━━━━━━━━━━━━━━━┛
        │                  ▲                                 ▲          Step 6: ProjectionExec
                           │     │                           │        computes date_trunc into a
        └ ─ ─ ─ ─ ─ ─ ─ ─ ─       ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─          new RecordBatch returned
             ┌─────────────────────┐                ┌─────────────┐          from client
             │     RecordBatch     │                │ RecordBatch │
             └─────────────────────┘                └─────────────┘

          Step 4: DataSource returns a        Step 5: FilterExec returns a new
               single RecordBatch            RecordBatch with only matching rows
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#thread-scheduling-cpu--io-thread-pools-and-tokio-runtimes" class="doc-anchor">§</a>Thread Scheduling, CPU / IO Thread Pools, and [Tokio](https://tokio.rs) [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime")s

DataFusion automatically runs each plan with multiple CPU cores using a [Tokio](https://tokio.rs) [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") as a thread pool. While tokio is most commonly used for asynchronous network I/O, the combination of an efficient, work-stealing scheduler, and first class compiler support for automatic continuation generation (`async`) also makes it a compelling choice for CPU intensive applications as explained in the [Using Rustlang’s Async Tokio Runtime for CPU-Bound Tasks](https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/) blog.

The number of cores used is determined by the `target_partitions` configuration setting, which defaults to the number of CPU cores. While preparing for execution, DataFusion tries to create this many distinct `async` [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")s for each [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan"). The [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")s for certain [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")s, such as [`RepartitionExec`](https://docs.rs/datafusion/latest/datafusion/physical_plan/repartition/struct.RepartitionExec.html) and [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec"), spawn [Tokio](https://tokio.rs) [`task`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/index.html "mod tokio::task")s, that run on threads managed by the [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime"). Many DataFusion [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")s perform CPU intensive processing.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#cooperative-scheduling" class="doc-anchor">§</a>Cooperative Scheduling

DataFusion uses cooperative scheduling, which means that each [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") is responsible for yielding control back to the [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") after some amount of work is done. Please see the [`coop`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html "mod datafusion::physical_plan::coop") module documentation for more details.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#network-io-and-cpu-intensive-tasks" class="doc-anchor">§</a>Network I/O and CPU intensive tasks

Using `async` for CPU intensive tasks makes it easy for [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")s to perform network I/O using standard Rust `async` during execution. However, this design also makes it very easy to mix CPU intensive and latency sensitive I/O work on the same thread pool ([`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime")). Using the same (default) [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") is convenient, and often works well for initial development and processing local files, but it can lead to problems under load and/or when reading from network sources such as AWS S3.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#optimizing-latency-throttled-cpu--io-under-highly-concurrent-load" class="doc-anchor">§</a>Optimizing Latency: Throttled CPU / IO under Highly Concurrent Load

If your system does not fully utilize either the CPU or network bandwidth during execution, or you see significantly higher tail (e.g. p99) latencies responding to network requests, **it is likely you need to use a different [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") for DataFusion plans**. The [thread_pools example](https://github.com/apache/datafusion/tree/main/datafusion-examples/examples/thread_pools.rs) has an example of how to do so.

As shown below, using the same [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime") for both CPU intensive processing and network requests can introduce significant delays in responding to those network requests. Delays in processing network requests can and does lead network flow control to throttle the available bandwidth in response. This effect can be especially pronounced when running multiple queries concurrently.

``` text
                                                                         Legend

                                                                         ┏━━━━━━┓
                           Processing network request                    ┃      ┃  CPU bound work
                           is delayed due to processing                  ┗━━━━━━┛
                           CPU bound work                                ┌─┐
                                                                         │ │       Network request
                                        ││                               └─┘       processing

                                        ││
                               ─ ─ ─ ─ ─  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                              │                                            │

                              ▼                                            ▼
┌─────────────┐           ┌─┐┌─┐┏━━━━━━━━━━━━━━━━━━━┓┏━━━━━━━━━━━━━━━━━━━┓┌─┐
│             │thread 1   │ ││ │┃     Decoding      ┃┃     Filtering     ┃│ │
│             │           └─┘└─┘┗━━━━━━━━━━━━━━━━━━━┛┗━━━━━━━━━━━━━━━━━━━┛└─┘
│             │           ┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┓
│Tokio Runtime│thread 2   ┃   Decoding   ┃     Filtering     ┃   Decoding   ┃       ...
│(thread pool)│           ┗━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┛
│             │     ...                               ...
│             │           ┏━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┓┌─┐ ┏━━━━━━━━━━━━━━┓
│             │thread N   ┃     Decoding      ┃     Filtering     ┃│ │ ┃   Decoding   ┃
└─────────────┘           ┗━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┛└─┘ ┗━━━━━━━━━━━━━━┛
                          ─────────────────────────────────────────────────────────────▶
                                                                                          time
```

The bottleneck resulting from network throttling can be avoided by using separate [`Runtime`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/runtime/struct.Runtime.html "struct tokio::runtime::runtime::Runtime")s for the different types of work, as shown in the diagram below.

``` text
                   A separate thread pool processes network       Legend
                   requests, reducing the latency for
                   processing each request                        ┏━━━━━━┓
                                                                  ┃      ┃  CPU bound work
                                        │                         ┗━━━━━━┛
                                         │                        ┌─┐
                              ┌ ─ ─ ─ ─ ┘                         │ │       Network request
                                 ┌ ─ ─ ─ ┘                        └─┘       processing
                              │
                              ▼  ▼
┌─────────────┐           ┌─┐┌─┐┌─┐
│             │thread 1   │ ││ ││ │
│             │           └─┘└─┘└─┘
│Tokio Runtime│                                          ...
│(thread pool)│thread 2
│             │
│"IO Runtime" │     ...
│             │                                                   ┌─┐
│             │thread N                                           │ │
└─────────────┘                                                   └─┘
                          ─────────────────────────────────────────────────────────────▶
                                                                                          time

┌─────────────┐           ┏━━━━━━━━━━━━━━━━━━━┓┏━━━━━━━━━━━━━━━━━━━┓
│             │thread 1   ┃     Decoding      ┃┃     Filtering     ┃
│             │           ┗━━━━━━━━━━━━━━━━━━━┛┗━━━━━━━━━━━━━━━━━━━┛
│Tokio Runtime│           ┏━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┓
│(thread pool)│thread 2   ┃   Decoding   ┃     Filtering     ┃   Decoding   ┃       ...
│             │           ┗━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┛
│ CPU Runtime │     ...                               ...
│             │           ┏━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━┓
│             │thread N   ┃     Decoding      ┃     Filtering     ┃   Decoding   ┃
└─────────────┘           ┗━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┛
                         ─────────────────────────────────────────────────────────────▶
                                                                                          time
```

Note that DataFusion does not use [`tokio::task::spawn_blocking`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/task/blocking/fn.spawn_blocking.html "fn tokio::task::blocking::spawn_blocking") for CPU-bounded work, because `spawn_blocking` is designed for blocking **IO**, not designed CPU bound tasks. Among other challenges, spawned blocking tasks can’t yield waiting for input (can’t call `await`) so they can’t be used to limit the number of concurrent CPU bound tasks or keep the processing pipeline to the same core.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#state-management-and-configuration" class="doc-anchor">§</a>State Management and Configuration

[`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") contain options to control DataFusion’s execution.

The state required to execute queries is managed by the following structures:

1.  [`SessionContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html "struct datafusion::execution::context::SessionContext"): State needed to create [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s such as the table definitions and the function registries.

2.  [`TaskContext`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html "struct datafusion::execution::TaskContext"): State needed for execution such as the [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool"), [`DiskManager`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager"), and [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry").

3.  [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps"): Per-execution properties and data (such as starting timestamps, etc).

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#resource-management" class="doc-anchor">§</a>Resource Management

The amount of memory and temporary local disk space used by DataFusion when running a plan can be controlled using the [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") and [`DiskManager`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager"). Other runtime options can be found on [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv").

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#crate-organization" class="doc-anchor">§</a>Crate Organization

Most users interact with DataFusion via this crate (`datafusion`), which re-exports all functionality needed to build and execute queries.

There are three other crates that provide additional functionality that must be used directly:

- [`datafusion_proto`](https://crates.io/crates/datafusion-proto): Plan serialization and deserialization
- [`datafusion_substrait`](https://crates.io/crates/datafusion-substrait): Support for the substrait plan serialization format
- [`datafusion_sqllogictest`](https://crates.io/crates/datafusion-sqllogictest) : The DataFusion SQL logic test runner

DataFusion is internally split into multiple sub crates to enforce modularity and improve compilation times. See the [list of modules](https://docs.rs/datafusion/50.2.0/datafusion/index.html#modules) for all available sub-crates. Major ones are

- [datafusion_common](https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_common/index.html "mod datafusion_common"): Common traits and types
- [datafusion_catalog](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/datafusion_catalog/index.html "mod datafusion_catalog"): Catalog APIs such as [`SchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html "trait datafusion::catalog::SchemaProvider") and [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")
- [datafusion_datasource](https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource/index.html "mod datafusion_datasource"): File and Data IO such as [`FileSource`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/trait.FileSource.html "trait datafusion::datasource::physical_plan::FileSource") and [`DataSink`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/trait.DataSink.html "trait datafusion::datasource::sink::DataSink")
- [datafusion_session](https://docs.rs/datafusion-session/50.2.0/x86_64-unknown-linux-gnu/datafusion_session/index.html "mod datafusion_session"): [`Session`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html "trait datafusion::catalog::Session") and related structures
- [datafusion_execution](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/datafusion_execution/index.html "mod datafusion_execution"): State and structures needed for execution
- [datafusion_expr](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_expr/index.html "mod datafusion_expr"): [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan"), [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") and related logical planning structure
- [datafusion_functions](https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions/index.html "mod datafusion_functions"): Scalar function packages
- [datafusion_functions_aggregate](https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate/index.html "mod datafusion_functions_aggregate"): Aggregate functions such as `MIN`, `MAX`, `SUM`, etc
- [datafusion_functions_nested](https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_nested/index.html "mod datafusion_functions_nested"): Scalar function packages for `ARRAY`s, `MAP`s and `STRUCT`s
- [datafusion_functions_table](https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_table/index.html "mod datafusion_functions_table"): Table Functions such as `GENERATE_SERIES`
- [datafusion_functions_window](https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_window/index.html "mod datafusion_functions_window"): Window functions such as `ROW_NUMBER`, `RANK`, etc
- [datafusion_optimizer](https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/datafusion_optimizer/index.html "mod datafusion_optimizer"): [`OptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerRule.html "trait datafusion::optimizer::OptimizerRule")s and [`AnalyzerRule`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.AnalyzerRule.html "trait datafusion::optimizer::AnalyzerRule")s
- [datafusion_physical_expr](https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_expr/index.html "mod datafusion_physical_expr"): [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") and related expressions
- [datafusion_physical_plan](https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_plan/index.html "mod datafusion_physical_plan"): [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and related expressions
- [datafusion_physical_optimizer](https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_optimizer/index.html "mod datafusion_physical_optimizer"): [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") and related expressions
- [datafusion_sql](https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/datafusion_sql/index.html "mod datafusion_sql"): SQL planner ([`SqlToRel`](https://docs.rs/datafusion/50.2.0/datafusion/sql/planner/struct.SqlToRel.html "struct datafusion::sql::planner::SqlToRel"))

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#citing-datafusion-in-academic-papers" class="doc-anchor">§</a>Citing DataFusion in Academic Papers

You can use the following citation to reference DataFusion in academic papers:

``` text
@inproceedings{lamb2024apache
  title={Apache Arrow DataFusion: A Fast, Embeddable, Modular Analytic Query Engine},
  author={Lamb, Andrew and Shen, Yijie and Heres, Dani{\"e}l and Chakraborty, Jayjeet and Kabak, Mehmet Ozan and Hsieh, Liang-Chi and Sun, Chao},
  booktitle={Companion of the 2024 International Conference on Management of Data},
  pages={5--17},
  year={2024}
}
```

## Re-exports<a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#reexports" class="anchor">§</a>

`pub use `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html" class="mod" title="mod datafusion::common::arrow"><code>arrow</code></a>`;`

`pub use `<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/index.html" class="mod" title="mod object_store"><code>object_store</code></a>`;`

`pub use `<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/index.html" class="mod" title="mod parquet"><code>parquet</code></a>`;``parquet`

`pub use datafusion_datasource_avro::`<a href="https://docs.rs/apache-avro/0.20.0/x86_64-unknown-linux-gnu/apache_avro/index.html" class="mod" title="mod apache_avro"><code>apache_avro</code></a>`;``avro`

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/index.html" class="mod" title="mod datafusion::catalog">catalog</a>  
re-export of [`datafusion_catalog`](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/datafusion_catalog/index.html "mod datafusion_catalog") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/index.html" class="mod" title="mod datafusion::common">common</a>  
re-export of [`datafusion_common`](https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_common/index.html "mod datafusion_common") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/index.html" class="mod" title="mod datafusion::config">config</a>  
Runtime configuration, via [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/index.html" class="mod" title="mod datafusion::dataframe">dataframe</a>  
[`DataFrame`](https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html "struct datafusion::dataframe::DataFrame") API for building and executing query plans.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/index.html" class="mod" title="mod datafusion::datasource">datasource</a>  
DataFusion data sources: [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") and [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/index.html" class="mod" title="mod datafusion::error">error</a>  
DataFusion error type [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") and [`Result`](https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html "type datafusion::error::Result").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/index.html" class="mod" title="mod datafusion::execution">execution</a>  
Shared state for query planning and execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/index.html" class="mod" title="mod datafusion::functions">functions</a>  
re-export of [`datafusion_functions`](https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions/index.html "mod datafusion_functions") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_aggregate/index.html" class="mod" title="mod datafusion::functions_aggregate">functions_aggregate</a>  
re-export of [`datafusion_functions_aggregate`](https://docs.rs/datafusion-functions-aggregate/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate/index.html "mod datafusion_functions_aggregate") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/index.html" class="mod" title="mod datafusion::functions_nested">functions_nested</a>  
re-export of [`datafusion_functions_nested`](https://docs.rs/datafusion-functions-nested/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_nested/index.html "mod datafusion_functions_nested") crate, if “nested_expressions” feature is enabled

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/index.html" class="mod" title="mod datafusion::functions_table">functions_table</a>  
re-export of [`datafusion_functions_table`](https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_table/index.html "mod datafusion_functions_table") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/index.html" class="mod" title="mod datafusion::functions_window">functions_window</a>  
re-export of [`datafusion_functions_window`](https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_window/index.html "mod datafusion_functions_window") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/index.html" class="mod" title="mod datafusion::logical_expr">logical_expr</a>  
re-export of [`datafusion_expr`](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_expr/index.html "mod datafusion_expr") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/index.html" class="mod" title="mod datafusion::logical_expr_common">logical_expr_common</a>  
re-export of [`datafusion_expr_common`](https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_expr_common/index.html "mod datafusion_expr_common") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/index.html" class="mod" title="mod datafusion::optimizer">optimizer</a>  
re-export of [`datafusion_optimizer`](https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/datafusion_optimizer/index.html "mod datafusion_optimizer") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/index.html" class="mod" title="mod datafusion::physical_expr">physical_expr</a>  
re-export of [`datafusion_physical_expr`](https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_expr/index.html "mod datafusion_physical_expr") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/index.html" class="mod" title="mod datafusion::physical_expr_adapter">physical_expr_adapter</a>  
re-export of [`datafusion_physical_expr_adapter`](https://docs.rs/datafusion-physical-expr-adapter/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_expr_adapter/index.html "mod datafusion_physical_expr_adapter") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/index.html" class="mod" title="mod datafusion::physical_expr_common">physical_expr_common</a>  
re-export of [`datafusion_physical_expr`](https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_expr/index.html "mod datafusion_physical_expr") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/index.html" class="mod" title="mod datafusion::physical_optimizer">physical_optimizer</a>  
re-export of [`datafusion_physical_optimizer`](https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_optimizer/index.html "mod datafusion_physical_optimizer") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/index.html" class="mod" title="mod datafusion::physical_plan">physical_plan</a>  
re-export of [`datafusion_physical_plan`](https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_plan/index.html "mod datafusion_physical_plan") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_planner/index.html" class="mod" title="mod datafusion::physical_planner">physical_planner</a>  
Planner for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") to [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html" class="mod" title="mod datafusion::prelude">prelude</a>  
DataFusion “prelude” to simplify importing common types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/index.html" class="mod" title="mod datafusion::scalar">scalar</a>  
[`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") single value representation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/index.html" class="mod" title="mod datafusion::sql">sql</a>  
re-export of [`datafusion_sql`](https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/datafusion_sql/index.html "mod datafusion_sql") crate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/index.html" class="mod" title="mod datafusion::test">test</a>Non-WebAssembly  
Common unit test utility methods

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/index.html" class="mod" title="mod datafusion::test_util">test_util</a>  
Utility functions to make testing DataFusion based crates easier

<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/index.html" class="mod" title="mod datafusion::variable">variable</a>  
re-export of variable provider for `@name` and `@@name` style runtime values.

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/macro.assert_batches_eq.html" class="macro" title="macro datafusion::assert_batches_eq">assert_batches_eq</a>  
Compares formatted output of a record batch with an expected vector of strings, with the result of pretty formatting record batches. This is a macro so errors appear on the correct line

<a href="https://docs.rs/datafusion/50.2.0/datafusion/macro.assert_batches_sorted_eq.html" class="macro" title="macro datafusion::assert_batches_sorted_eq">assert_batches_sorted_eq</a>  
Compares formatted output of a record batch with an expected vector of strings in a way that order does not matter. This is a macro so errors appear on the correct line

<a href="https://docs.rs/datafusion/50.2.0/datafusion/macro.dataframe.html" class="macro" title="macro datafusion::dataframe">dataframe</a>  
Macro for creating DataFrame.

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/constant.DATAFUSION_VERSION.html" class="constant" title="constant datafusion::DATAFUSION_VERSION">DATAFUSION_VERSION</a>  
DataFusion crate version
