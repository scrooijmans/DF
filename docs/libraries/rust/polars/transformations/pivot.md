# Pivots - Polars user guide
Pivot a column in a `DataFrame` and perform one of the following aggregations:

*   first
*   last
*   sum
*   min
*   max
*   mean
*   median
*   len

The pivot operation consists of a group by one, or multiple columns (these will be the new y-axis), the column that will be pivoted (this will be the new x-axis) and an aggregation.

Dataset
-------

Python Rust

[`DataFrame`](https://docs.pola.rs/api/python/stable/reference/dataframe/index.html)

```
df = pl.DataFrame(
    {
        "foo": ["A", "A", "B", "B", "C"],
        "N": [1, 2, 2, 4, 2],
        "bar": ["k", "l", "m", "n", "o"],
    }
)
print(df)

```


[`DataFrame`](https://docs.pola.rs/api/rust/dev/polars/frame/struct.DataFrame.html)

```
let df = df!(
        "foo"=> ["A", "A", "B", "B", "C"],
        "bar"=> ["k", "l", "m", "n", "o"],
        "N"=> [1, 2, 2, 4, 2],
)?;
println!("{}", &df);

```


```
shape: (5, 3)
┌─────┬─────┬─────┐
│ foo ┆ N   ┆ bar │
│ --- ┆ --- ┆ --- │
│ str ┆ i64 ┆ str │
╞═════╪═════╪═════╡
│ A   ┆ 1   ┆ k   │
│ A   ┆ 2   ┆ l   │
│ B   ┆ 2   ┆ m   │
│ B   ┆ 4   ┆ n   │
│ C   ┆ 2   ┆ o   │
└─────┴─────┴─────┘

```


Eager
-----

Python Rust

[`pivot`](https://docs.pola.rs/api/python/stable/reference/dataframe/api/polars.DataFrame.pivot.html)

```
out = df.pivot("bar", index="foo", values="N", aggregate_function="first")
print(out)

```


[`pivot`](https://docs.pola.rs/api/rust/dev/polars_lazy/frame/pivot/fn.pivot.html)

```
let out = pivot(&df, ["foo"], Some(["bar"]), Some(["N"]), false, None, None)?;
println!("{}", &out);

```


```
shape: (3, 6)
┌─────┬──────┬──────┬──────┬──────┬──────┐
│ foo ┆ k    ┆ l    ┆ m    ┆ n    ┆ o    │
│ --- ┆ ---  ┆ ---  ┆ ---  ┆ ---  ┆ ---  │
│ str ┆ i64  ┆ i64  ┆ i64  ┆ i64  ┆ i64  │
╞═════╪══════╪══════╪══════╪══════╪══════╡
│ A   ┆ 1    ┆ 2    ┆ null ┆ null ┆ null │
│ B   ┆ null ┆ null ┆ 2    ┆ 4    ┆ null │
│ C   ┆ null ┆ null ┆ null ┆ null ┆ 2    │
└─────┴──────┴──────┴──────┴──────┴──────┘

```


Lazy
----

A Polars `LazyFrame` always need to know the schema of a computation statically (before collecting the query). As a pivot's output schema depends on the data, and it is therefore impossible to determine the schema without running the query.

Polars could have abstracted this fact for you just like Spark does, but we don't want you to shoot yourself in the foot with a shotgun. The cost should be clear upfront.

Python Rust

[`pivot`](https://docs.pola.rs/api/python/stable/reference/dataframe/api/polars.DataFrame.pivot.html)

```
q = (
    df.lazy()
    .collect()
    .pivot(index="foo", on="bar", values="N", aggregate_function="first")
    .lazy()
)
out = q.collect()
print(out)

```


[`pivot`](https://docs.pola.rs/api/rust/dev/polars_lazy/frame/pivot/fn.pivot.html)

```
let q = df.lazy();
let q2 = pivot(
    &q.collect()?,
    ["foo"],
    Some(["bar"]),
    Some(["N"]),
    false,
    None,
    None,
)?
.lazy();
let out = q2.collect()?;
println!("{}", &out);

```


```
shape: (3, 6)
┌─────┬──────┬──────┬──────┬──────┬──────┐
│ foo ┆ k    ┆ l    ┆ m    ┆ n    ┆ o    │
│ --- ┆ ---  ┆ ---  ┆ ---  ┆ ---  ┆ ---  │
│ str ┆ i64  ┆ i64  ┆ i64  ┆ i64  ┆ i64  │
╞═════╪══════╪══════╪══════╪══════╪══════╡
│ A   ┆ 1    ┆ 2    ┆ null ┆ null ┆ null │
│ B   ┆ null ┆ null ┆ 2    ┆ 4    ┆ null │
│ C   ┆ null ┆ null ┆ null ┆ null ┆ 2    │
└─────┴──────┴──────┴──────┴──────┴──────┘

```
