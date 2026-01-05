# Concepts — Apache Arrow DataFusion documentation

In this section, we will cover a basic example to introduce a few key concepts. We will use the 2021 Yellow Taxi Trip Records ([download](https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2021-01.parquet)), from the [TLC Trip Record Data](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page).

```
In [1]: from datafusion import SessionContext, col, lit, functions as f

In [2]: ctx = SessionContext()

In [3]: df = ctx.read_parquet("yellow_tripdata_2021-01.parquet")

In [4]: df = df.select(
   ...:     "trip_distance",
   ...:     col("total_amount").alias("total"),
   ...:     (f.round(lit(100.0) * col("tip_amount") / col("total_amount"), lit(1))).alias("tip_percent"),
   ...: )
   ...:

In [5]: df.show()
DataFrame()
+---------------+-------+-------------+
| trip_distance | total | tip_percent |
+---------------+-------+-------------+
| 2.1           | 11.8  | 0.0         |
| 0.2           | 4.3   | 0.0         |
| 14.7          | 51.95 | 16.7        |
| 10.6          | 36.35 | 16.6        |
| 4.94          | 24.36 | 16.7        |
| 1.6           | 14.15 | 16.6        |
| 4.1           | 17.3  | 0.0         |
| 5.7           | 21.8  | 0.0         |
| 9.1           | 28.8  | 0.0         |
| 2.7           | 18.95 | 16.6        |
| 6.11          | 24.3  | 0.0         |
| 1.21          | 10.79 | 23.1        |
| 7.4           | 33.92 | 0.0         |
| 1.7           | 14.16 | 16.7        |
| 0.81          | 8.3   | 0.0         |
| 1.01          | 10.3  | 9.7         |
| 0.73          | 12.09 | 23.1        |
| 1.17          | 12.36 | 16.7        |
| 0.78          | 9.96  | 16.7        |
| 1.66          | 12.3  | 0.0         |
+---------------+-------+-------------+

```

## Session Context

The first statement group creates a [`SessionContext`](about:blank/autoapi/datafusion/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext").

```
# create a context
ctx = datafusion.SessionContext()

```

A Session Context is the main interface for executing queries with DataFusion. It maintains the state of the connection between a user and an instance of the DataFusion engine. Additionally it provides the following functionality:

- Create a DataFrame from a data source.
- Register a data source as a table that can be referenced from a SQL query.
- Execute a SQL query

## DataFrame

The second statement group creates a `DataFrame`,

```
# Create a DataFrame from a file
df = ctx.read_parquet("yellow_tripdata_2021-01.parquet")

```

A DataFrame refers to a (logical) set of rows that share the same column names, similar to a [Pandas DataFrame](https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html). DataFrames are typically created by calling a method on [`SessionContext`](about:blank/autoapi/datafusion/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext"), such as `read_csv`, and can then be modified by calling the transformation methods, such as [`filter()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.filter "datafusion.dataframe.DataFrame.filter"), [`select()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.select "datafusion.dataframe.DataFrame.select"), [`aggregate()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.aggregate "datafusion.dataframe.DataFrame.aggregate"), and [`limit()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.limit "datafusion.dataframe.DataFrame.limit") to build up a query definition.

For more details on working with DataFrames, including visualization options and conversion to other formats, see [DataFrames](dataframe/index.html).

## Expressions

The third statement uses `Expressions` to build up a query definition. You can find explanations for what the functions below do in the user documentation for [`col()`](about:blank/autoapi/datafusion/index.html#datafusion.col "datafusion.col"), [`lit()`](about:blank/autoapi/datafusion/index.html#datafusion.lit "datafusion.lit"), [`round()`](about:blank/autoapi/datafusion/functions/index.html#datafusion.functions.round "datafusion.functions.round"), and [`alias()`](about:blank/autoapi/datafusion/expr/index.html#datafusion.expr.Expr.alias "datafusion.expr.Expr.alias").

```
df = df.select(
    "trip_distance",
    col("total_amount").alias("total"),
    (f.round(lit(100.0) * col("tip_amount") / col("total_amount"), lit(1))).alias("tip_percent"),
)

```

Finally the [`show()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.show "datafusion.dataframe.DataFrame.show") method converts the logical plan represented by the DataFrame into a physical plan and execute it, collecting all results and displaying them to the user. It is important to note that DataFusion performs lazy evaluation of the DataFrame. Until you call a method such as [`show()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.show "datafusion.dataframe.DataFrame.show") or [`collect()`](about:blank/autoapi/datafusion/dataframe/index.html#datafusion.dataframe.DataFrame.collect "datafusion.dataframe.DataFrame.collect"), DataFusion will not perform the query.
