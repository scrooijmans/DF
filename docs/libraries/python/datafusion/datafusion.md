# Apache DataFusion — Apache DataFusion documentation

![DataFusion Logo](_images/2x_bgwhite_original.png)

DataFusion is an extensible query engine written in [Rust](http://rustlang.org/) that uses [Apache Arrow](https://arrow.apache.org/) as its in-memory format.

The documentation on this site is for the [core DataFusion project](https://github.com/apache/datafusion), which contains libraries and binaries for developers building fast and feature rich database and analytic systems, customized to particular workloads. See [use cases](https://datafusion.apache.org/user-guide/introduction.html#use-cases) for examples.

The following related subprojects target end users and have separate documentation.

- [DataFusion Python](https://datafusion.apache.org/python/) offers a Python interface for SQL and DataFrame queries.
- [DataFusion Comet](https://datafusion.apache.org/comet/) is an accelerator for Apache Spark based on DataFusion.
- [DataFusion Ballista](https://datafusion.apache.org/ballista/) is distributed processing extension for DataFusion.

“Out of the box,” DataFusion offers [SQL](https://datafusion.apache.org/user-guide/sql/index.html) and [Dataframe](https://docs.rs/datafusion/latest/datafusion/dataframe/struct.DataFrame.html) APIs, excellent [performance](https://benchmark.clickhouse.com/), built-in support for CSV, Parquet, JSON, and Avro, extensive customization, and a great community. [Python Bindings](https://github.com/apache/datafusion-python) are also available. [Ballista](https://datafusion.apache.org/ballista/) is Apache DataFusion extension enabling the parallelized execution of workloads across multiple nodes in a distributed environment.

DataFusion features a full query planner, a columnar, streaming, multi-threaded, vectorized execution engine, and partitioned data sources. You can customize DataFusion at almost all points including additional data sources, query languages, functions, custom operators and more. See the [Architecture](https://datafusion.apache.org/contributor-guide/architecture.html) section for more details.

To get started, see

- The [example usage](user-guide/example-usage.html) section of the user guide and the [datafusion-examples](https://github.com/apache/datafusion/tree/main/datafusion-examples) directory.
- The [library user guide](library-user-guide/index.html) for examples of using DataFusion’s extension APIs
- The [developer’s guide](about:blank/contributor-guide/index.html#developer-s-guide) for contributing and [communication](contributor-guide/communication.html) for getting in touch with us.

Links

- [GitHub and Issue Tracker](https://github.com/apache/datafusion)
- [crates.io](https://crates.io/crates/datafusion)
- [API Docs](https://docs.rs/datafusion/latest/datafusion/)
- [Blog](https://datafusion.apache.org/blog/)
- [Code of conduct](https://github.com/apache/datafusion/blob/main/CODE_OF_CONDUCT.md)
- [Download](download.html)

User Guide

- [Introduction](user-guide/introduction.html)
- [Example Usage](user-guide/example-usage.html)
- [Features](user-guide/features.html)
- [Concepts, Readings, Events](user-guide/concepts-readings-events.html)
- [🌎 Community Events](about:blank/user-guide/concepts-readings-events.html#community-events)
- [Crate Configuration](user-guide/crate-configuration.html)
- [DataFusion CLI](user-guide/cli/index.html)
- [DataFrame API](user-guide/dataframe.html)
- [Expression API](user-guide/expressions.html)
- [SQL Reference](user-guide/sql/index.html)
- [Configuration Settings](user-guide/configs.html)
- [Runtime Configuration Settings](about:blank/user-guide/configs.html#runtime-configuration-settings)
- [Tuning Guide](about:blank/user-guide/configs.html#tuning-guide)
- [Reading Explain Plans](user-guide/explain-usage.html)
- [Frequently Asked Questions](user-guide/faq.html)
- [How does DataFusion Compare with `XYZ`?](about:blank/user-guide/faq.html#how-does-datafusion-compare-with-xyz)

Library User Guide

- [Introduction](library-user-guide/index.html)
- [Upgrade Guides](library-user-guide/upgrading.html)
- [Extensions List](library-user-guide/extensions.html)
- [Using the SQL API](library-user-guide/using-the-sql-api.html)
- [Working with `Expr`s](library-user-guide/working-with-exprs.html)
- [Using the DataFrame API](library-user-guide/using-the-dataframe-api.html)
- [Write DataFrame to Files](about:blank/library-user-guide/using-the-dataframe-api.html#write-dataframe-to-files)
- [Building Logical Plans](library-user-guide/building-logical-plans.html)
- [Catalogs, Schemas, and Tables](library-user-guide/catalogs.html)
- [Functions](library-user-guide/functions/index.html)
- [Custom Table Provider](library-user-guide/custom-table-providers.html)
- [Table Constraint Enforcement](library-user-guide/table-constraints.html)
- [Extending DataFusion’s operators: custom LogicalPlan and Execution Plans](library-user-guide/extending-operators.html)
- [Profiling Cookbook](library-user-guide/profiling.html)
- [DataFusion Query Optimizer](library-user-guide/query-optimizer.html)

Contributor Guide

- [Introduction](contributor-guide/index.html)
- [Developer’s guide](about:blank/contributor-guide/index.html#developer-s-guide)
- [Reviewing Pull Requests](about:blank/contributor-guide/index.html#reviewing-pull-requests)
- [Communication](contributor-guide/communication.html)
- [Development Environment](contributor-guide/development_environment.html)
- [Architecture](contributor-guide/architecture.html)
- [Testing](contributor-guide/testing.html)
- [API health policy](contributor-guide/api-health.html)
- [HOWTOs](contributor-guide/howtos.html)
- [Roadmap and Improvement Proposals](contributor-guide/roadmap.html)
- [Governance](contributor-guide/governance.html)
- [Inviting New Committers and PMC Members](contributor-guide/inviting.html)
- [Specifications](contributor-guide/specification/index.html)
- [Google Summer of Code (GSOC)](contributor-guide/gsoc/index.html)
