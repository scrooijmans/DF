# Lazy API - Polars user guide
Polars supports two modes of operation: lazy and eager. The examples so far have used the eager API, in which the query is executed immediately. In the lazy API, the query is only evaluated once it is _collected_. Deferring the execution to the last minute can have significant performance advantages and is why the lazy API is preferred in most cases. Let us demonstrate this with an example:

Python Rust

[`read_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.read_csv.html)

```
df = pl.read_csv("docs/assets/data/iris.csv")
df_small = df.filter(pl.col("sepal_length") > 5)
df_agg = df_small.group_by("species").agg(pl.col("sepal_width").mean())
print(df_agg)

```


[`CsvReader`](https://docs.pola.rs/api/rust/dev/polars/prelude/struct.CsvReader.html) · [Available on feature csv](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag csv")

```
let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("docs/assets/data/iris.csv".into()))
    .unwrap()
    .finish()
    .unwrap();
let mask = df.column("sepal_length")?.f64()?.gt(5.0);
let df_small = df.filter(&mask)?;
#[allow(deprecated)]
let df_agg = df_small
    .group_by(["species"])?
    .select(["sepal_width"])
    .mean()?;
println!("{df_agg}");

```


In this example we use the eager API to:

1.  Read the iris [dataset](https://archive.ics.uci.edu/dataset/53/iris).
2.  Filter the dataset based on sepal length.
3.  Calculate the mean of the sepal width per species.

Every step is executed immediately returning the intermediate results. This can be very wasteful as we might do work or load extra data that is not being used. If we instead used the lazy API and waited on execution until all the steps are defined then the query planner could perform various optimizations. In this case:

*   Predicate pushdown: Apply filters as early as possible while reading the dataset, thus only reading rows with sepal length greater than 5.
*   Projection pushdown: Select only the columns that are needed while reading the dataset, thus removing the need to load additional columns (e.g., petal length and petal width).

Python Rust

[`scan_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.scan_csv.html)

```
q = (
    pl.scan_csv("docs/assets/data/iris.csv")
    .filter(pl.col("sepal_length") > 5)
    .group_by("species")
    .agg(pl.col("sepal_width").mean())
)

df = q.collect()

```


[`LazyCsvReader`](https://docs.pola.rs/api/rust/dev/polars/prelude/struct.LazyCsvReader.html) · [Available on feature csv](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag csv")

```
let q = LazyCsvReader::new(PlPath::new("docs/assets/data/iris.csv"))
    .with_has_header(true)
    .finish()?
    .filter(col("sepal_length").gt(lit(5)))
    .group_by(vec![col("species")])
    .agg([col("sepal_width").mean()]);
let df = q.collect()?;
println!("{df}");

```


These will significantly lower the load on memory & CPU thus allowing you to fit bigger datasets in memory and process them faster. Once the query is defined you call `collect` to inform Polars that you want to execute it. You can [learn more about the lazy API in its dedicated chapter](../../lazy/).

Eager API

In many cases the eager API is actually calling the lazy API under the hood and immediately collecting the result. This has the benefit that within the query itself optimization(s) made by the query planner can still take place.

When to use which
-----------------

In general, the lazy API should be preferred unless you are either interested in the intermediate results or are doing exploratory work and don't know yet what your query is going to look like.

Previewing the query plan
-------------------------

When using the lazy API you can use the function `explain` to ask Polars to create a description of the query plan that will be executed once you collect the results. This can be useful if you want to see what types of optimizations Polars performs on your queries. We can ask Polars to explain the query `q` we defined above:

Python Rust

[`explain`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.explain)

```
let q = LazyCsvReader::new(PlPath::new("docs/assets/data/iris.csv"))
    .with_has_header(true)
    .finish()?
    .filter(col("sepal_length").gt(lit(5)))
    .group_by(vec![col("species")])
    .agg([col("sepal_width").mean()]);
println!("{}", q.explain(true)?);

```


```
AGGREGATE[maintain_order: false]
  [col("sepal_width").mean()] BY [col("species")]
  FROM
  Csv SCAN [docs/assets/data/iris.csv]
  PROJECT 3/5 COLUMNS
  SELECTION: [(col("sepal_length")) > (5.0)]

```


Immediately, we can see in the explanation that Polars did apply predicate pushdown, as it is only reading rows where the sepal length is greater than 5, and it did apply projection pushdown, as it is only reading the columns that are needed by the query.

The function `explain` can also be used to see how expression expansion will unfold in the context of a given schema. Consider the example expression from the [section on expression expansion](about:blank/expressions-and-contexts/#expression-expansion):

```
(pl.col(pl.Float64) * 1.1).name.suffix("*1.1")

```


We can use `explain` to see how this expression would evaluate against an arbitrary schema:

[`explain`](https://docs.pola.rs/api/python/stable/reference/lazyframe/api/polars.LazyFrame.explain.html)

```
schema = pl.Schema(
    {
        "int_1": pl.Int16,
        "int_2": pl.Int32,
        "float_1": pl.Float64,
        "float_2": pl.Float64,
        "float_3": pl.Float64,
    }
)

print(
    pl.LazyFrame(schema=schema)
    .select((pl.col(pl.Float64) * 1.1).name.suffix("*1.1"))
    .explain()
)

```


```
SELECT [[(col("float_1")) * (1.1)].alias("float_1*1.1"), [(col("float_2")) * (1.1)].alias("float_2*1.1"), [(col("float_3")) * (1.1)].alias("float_3*1.1")]
  DF ["int_1", "int_2", "float_1", "float_2", ...]; PROJECT["float_1", "float_2", "float_3"] 3/5 COLUMNS

```
