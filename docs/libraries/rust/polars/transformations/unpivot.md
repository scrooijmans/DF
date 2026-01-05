# Unpivots - Polars user guide
Unpivot unpivots a DataFrame from wide format to long format

Dataset
-------

Python Rust

[`DataFrame`](https://docs.pola.rs/api/python/stable/reference/dataframe/index.html)

```
import polars as pl

df = pl.DataFrame(
    {
        "A": ["a", "b", "a"],
        "B": [1, 3, 5],
        "C": [10, 11, 12],
        "D": [2, 4, 6],
    }
)
print(df)

```


[`DataFrame`](https://docs.pola.rs/api/rust/dev/polars/frame/struct.DataFrame.html)

```
let df = df!(
        "A"=> &["a", "b", "a"],
        "B"=> &[1, 3, 5],
        "C"=> &[10, 11, 12],
        "D"=> &[2, 4, 6],
)?;
println!("{}", &df);

```


```
shape: (3, 4)
┌─────┬─────┬─────┬─────┐
│ A   ┆ B   ┆ C   ┆ D   │
│ --- ┆ --- ┆ --- ┆ --- │
│ str ┆ i64 ┆ i64 ┆ i64 │
╞═════╪═════╪═════╪═════╡
│ a   ┆ 1   ┆ 10  ┆ 2   │
│ b   ┆ 3   ┆ 11  ┆ 4   │
│ a   ┆ 5   ┆ 12  ┆ 6   │
└─────┴─────┴─────┴─────┘

```


Eager + lazy
------------

`Eager` and `lazy` have the same API.

Python Rust

[`unpivot`](https://docs.pola.rs/api/python/stable/reference/dataframe/api/polars.DataFrame.unpivot.html)

```
out = df.unpivot(["C", "D"], index=["A", "B"])
print(out)

```


[`unpivot`](https://docs.pola.rs/api/rust/dev/polars/frame/struct.DataFrame.html#method.unpivot)

```
let out = df.unpivot(["A", "B"], ["C", "D"])?;
println!("{}", &out);

```


```
shape: (6, 4)
┌─────┬─────┬──────────┬───────┐
│ A   ┆ B   ┆ variable ┆ value │
│ --- ┆ --- ┆ ---      ┆ ---   │
│ str ┆ i64 ┆ str      ┆ i64   │
╞═════╪═════╪══════════╪═══════╡
│ a   ┆ 1   ┆ C        ┆ 10    │
│ b   ┆ 3   ┆ C        ┆ 11    │
│ a   ┆ 5   ┆ C        ┆ 12    │
│ a   ┆ 1   ┆ D        ┆ 2     │
│ b   ┆ 3   ┆ D        ┆ 4     │
│ a   ┆ 5   ┆ D        ┆ 6     │
└─────┴─────┴──────────┴───────┘

```
