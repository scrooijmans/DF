# Multiple - Polars user guide
Dealing with multiple files.
----------------------------

Polars can deal with multiple files differently depending on your needs and memory strain.

Let's create some files to give us some context:

Python

[`write_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.DataFrame.write_csv.html)

```
import polars as pl

df = pl.DataFrame({"foo": [1, 2, 3], "bar": [None, "ham", "spam"]})

for i in range(5):
    df.write_csv(f"docs/assets/data/my_many_files_{i}.csv")

```


Reading into a single `DataFrame`
---------------------------------

To read multiple files into a single `DataFrame`, we can use globbing patterns:

Python

[`read_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.read_csv.html)

```
df = pl.read_csv("docs/assets/data/my_many_files_*.csv")
print(df)

```


```
shape: (15, 2)
┌─────┬──────┐
│ foo ┆ bar  │
│ --- ┆ ---  │
│ i64 ┆ str  │
╞═════╪══════╡
│ 1   ┆ null │
│ 2   ┆ ham  │
│ 3   ┆ spam │
│ 1   ┆ null │
│ 2   ┆ ham  │
│ …   ┆ …    │
│ 2   ┆ ham  │
│ 3   ┆ spam │
│ 1   ┆ null │
│ 2   ┆ ham  │
│ 3   ┆ spam │
└─────┴──────┘

```


To see how this works we can take a look at the query plan. Below we see that all files are read separately and concatenated into a single `DataFrame`. Polars will try to parallelize the reading.

Python

[`show_graph`](https://docs.pola.rs/api/python/stable/reference/lazyframe/api/polars.LazyFrame.show_graph.html)

```
pl.scan_csv("docs/assets/data/my_many_files_*.csv").show_graph()

```


![](data:image/png;base64, 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)

Reading and processing in parallel
----------------------------------

If your files don't have to be in a single table you can also build a query plan for each file and execute them in parallel on the Polars thread pool.

All query plan execution is embarrassingly parallel and doesn't require any communication.

Python

[`scan_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.scan_csv.html)

```
import glob

import polars as pl

queries = []
for file in glob.glob("docs/assets/data/my_many_files_*.csv"):
    q = pl.scan_csv(file).group_by("bar").agg(pl.len(), pl.sum("foo"))
    queries.append(q)

dataframes = pl.collect_all(queries)
print(dataframes)

```


```
[shape: (3, 3)
┌──────┬─────┬─────┐
│ bar  ┆ len ┆ foo │
│ ---  ┆ --- ┆ --- │
│ str  ┆ u64 ┆ i64 │
╞══════╪═════╪═════╡
│ ham  ┆ 1   ┆ 2   │
│ null ┆ 1   ┆ 1   │
│ spam ┆ 1   ┆ 3   │
└──────┴─────┴─────┘, shape: (3, 3)
┌──────┬─────┬─────┐
│ bar  ┆ len ┆ foo │
│ ---  ┆ --- ┆ --- │
│ str  ┆ u64 ┆ i64 │
╞══════╪═════╪═════╡
│ spam ┆ 1   ┆ 3   │
│ null ┆ 1   ┆ 1   │
│ ham  ┆ 1   ┆ 2   │
└──────┴─────┴─────┘, shape: (3, 3)
┌──────┬─────┬─────┐
│ bar  ┆ len ┆ foo │
│ ---  ┆ --- ┆ --- │
│ str  ┆ u64 ┆ i64 │
╞══════╪═════╪═════╡
│ ham  ┆ 1   ┆ 2   │
│ null ┆ 1   ┆ 1   │
│ spam ┆ 1   ┆ 3   │
└──────┴─────┴─────┘, shape: (3, 3)
┌──────┬─────┬─────┐
│ bar  ┆ len ┆ foo │
│ ---  ┆ --- ┆ --- │
│ str  ┆ u64 ┆ i64 │
╞══════╪═════╪═════╡
│ null ┆ 1   ┆ 1   │
│ ham  ┆ 1   ┆ 2   │
│ spam ┆ 1   ┆ 3   │
└──────┴─────┴─────┘, shape: (3, 3)
┌──────┬─────┬─────┐
│ bar  ┆ len ┆ foo │
│ ---  ┆ --- ┆ --- │
│ str  ┆ u64 ┆ i64 │
╞══════╪═════╪═════╡
│ null ┆ 1   ┆ 1   │
│ spam ┆ 1   ┆ 3   │
│ ham  ┆ 1   ┆ 2   │
└──────┴─────┴─────┘]

```
