# Features — Apache DataFusion documentation

## General

- SQL Parser
- SQL Query Planner
- DataFrame API
- Parallel query execution
- Streaming Execution

## Optimizations

- Query Optimizer
- Constant folding
- Join Reordering
- Limit Pushdown
- Projection push down
- Predicate push down

## SQL Support

- Type coercion
- Projection (`SELECT`)
- Filter (`WHERE`)
- Filter post-aggregate (`HAVING`)
- Sorting (`ORDER BY`)
- Limit (`LIMIT`)
- Aggregate (`GROUP BY`)
- cast /try_cast
- [`VALUES` lists](https://www.postgresql.org/docs/current/queries-values.html)
- [String Functions](about:blank/sql/scalar_functions.html#string-functions)
- [Conditional Functions](about:blank/sql/scalar_functions.html#conditional-functions)
- [Time and Date Functions](about:blank/sql/scalar_functions.html#time-and-date-functions)
- [Math Functions](about:blank/sql/scalar_functions.html#math-functions)
- [Aggregate Functions](sql/aggregate_functions.html) (`SUM`, `MEDIAN`, and many more)
- Schema Queries
  - `SHOW TABLES`
  - `SHOW COLUMNS FROM <table/view>`
  - `SHOW CREATE TABLE <view>`
  - Basic SQL [Information Schema](sql/information_schema.html) (`TABLES`, `VIEWS`, `COLUMNS`)
  - Full SQL [Information Schema](sql/information_schema.html) support

- Support for nested types (`ARRAY`/`LIST` and `STRUCT`.
  - Read support
  - Write support
  - Field access (`col['field']` and \[`col[1]`\])
  - [Array Functions](about:blank/sql/scalar_functions.html#array-functions)
  - [Struct Functions](about:blank/sql/scalar_functions.html#struct-functions)
    - `struct`
    - [Postgres JSON operators](https://github.com/apache/datafusion/issues/6631) (`->`, `->>`, etc.)

- Subqueries
- Common Table Expressions (CTE)
- Set Operations (`UNION [ALL]`, `INTERSECT [ALL]`, `EXCEPT[ALL]`)
- Joins (`INNER`, `LEFT`, `RIGHT`, `FULL`, `CROSS`)
- Window Functions
  - Empty (`OVER()`)
  - Partitioning and ordering: (`OVER(PARTITION BY <..> ORDER BY <..>)`)
  - Custom Window (`ORDER BY time ROWS BETWEEN 2 PRECEDING AND 0 FOLLOWING)`)
  - User Defined Window and Aggregate Functions

- Catalogs
  - Schemas (`CREATE / DROP SCHEMA`)
  - Tables (`CREATE / DROP TABLE`, `CREATE TABLE AS SELECT`)

- Data Insert
  - `INSERT INTO`
  - `COPY .. INTO ..`
  - CSV
  - JSON
  - Parquet
  - Avro

## Runtime

- Streaming Grouping
- Streaming Window Evaluation
- Memory limits enforced
- Spilling (to disk) Sort
- Spilling (to disk) Grouping
- Spilling (to disk) Sort Merge Join
- Spilling (to disk) Hash Join

## Data Sources

In addition to allowing arbitrary datasources via the [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html) trait, DataFusion includes built in support for the following formats:

- CSV
- Parquet
  - Primitive and Nested Types
  - Row Group and Data Page pruning on min/max statistics
  - Row Group pruning on Bloom Filters
  - Predicate push down (late materialization) [not by default](https://github.com/apache/datafusion/issues/3463)

- JSON
- Avro
- Arrow
