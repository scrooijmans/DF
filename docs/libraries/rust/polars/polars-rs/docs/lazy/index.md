# Module lazy Copy item path

<a href="https://docs.rs/polars/latest/src/polars/docs/lazy.rs.html#1-287" class="src">Source</a>

Expand description

## <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#polars-lazy-cookbook" class="doc-anchor">§</a>Polars Lazy cookbook

This page should serve as a cookbook to quickly get you started with Polars’ query engine. The lazy API allows you to create complex well performing queries on top of Polars eager.

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#tree-of-contents" class="doc-anchor">§</a>Tree Of Contents

- [Start a lazy computation](https://docs.rs/polars/latest/polars/docs/lazy/index.html#start-a-lazy-computation)
- [Filter](https://docs.rs/polars/latest/polars/docs/lazy/index.html#filter)
- [Sort](https://docs.rs/polars/latest/polars/docs/lazy/index.html#sort)
- [GroupBy](https://docs.rs/polars/latest/polars/docs/lazy/index.html#group_by)
- [Joins](https://docs.rs/polars/latest/polars/docs/lazy/index.html#joins)
- [Conditionally apply](https://docs.rs/polars/latest/polars/docs/lazy/index.html#conditionally-apply)
- [Black box function](https://docs.rs/polars/latest/polars/docs/lazy/index.html#black-box-function)

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#start-a-lazy-computation" class="doc-anchor">§</a>Start a lazy computation

``` rust
use polars::prelude::*;
use polars::df;

let df = df![
    "a" => [1, 2, 3],
    "b" => [None, Some("a"), Some("b")]
]?;
// from an eager DataFrame
let lf: LazyFrame = df.lazy();

// scan a csv file lazily
let lf: LazyFrame = LazyCsvReader::new("some_path")
    .with_has_header(true)
    .finish()?;

// scan a parquet file lazily
let lf: LazyFrame = LazyFrame::scan_parquet("some_path", Default::default())?;
```

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#filter" class="doc-anchor">§</a>Filter

``` rust
use polars::prelude::*;
use polars::df;

let df = df![
    "a" => [1, 2, 3],
    "b" => [None, Some("a"), Some("b")]
]?;

let filtered = df.lazy()
    .filter(col("a").gt(lit(2)))
    .collect()?;

// filtered:

// ╭─────┬─────╮
// │ a   ┆ b   │
// │ --- ┆ --- │
// │ i64 ┆ str │
// ╞═════╪═════╡
// │ 3   ┆ "c" │
// ╰─────┴─────╯
```

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#sort" class="doc-anchor">§</a>Sort

``` rust
use polars::prelude::*;
use polars::df;

let df = df![
    "a" => [1, 2, 3],
    "b" => ["a", "a", "b"]
]?;
// sort this DataFrame by multiple columns

let sorted = df.lazy()
    .sort_by_exprs(vec![col("b"), col("a")], SortMultipleOptions::default())
    .collect()?;

// sorted:

// ╭─────┬─────╮
// │ a   ┆ b   │
// │ --- ┆ --- │
// │ i64 ┆ str │
// ╞═════╪═════╡
// │ 1   ┆ "a" │
// │ 2   ┆ "a" │
// │ 3   ┆ "b" │
// ╰─────┴─────╯
```

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#groupby" class="doc-anchor">§</a>Groupby

This example is from the polars [user guide](https://docs.pola.rs/user-guide/concepts/expressions-and-contexts/#group_by-and-aggregations).

``` rust
use polars::prelude::*;

 let df = LazyCsvReader::new("reddit.csv")
    .with_has_header(true)
    .with_separator(b',')
    .finish()?
    .group_by([col("comment_karma")])
    .agg([col("name").n_unique().alias("unique_names"), col("link_karma").max()])
    // take only 100 rows.
    .fetch(100)?;
```

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#joins" class="doc-anchor">§</a>Joins

``` rust
use polars::prelude::*;
use polars::df;
let df_a = df![
    "a" => [1, 2, 1, 1],
    "b" => ["a", "b", "c", "c"],
    "c" => [0, 1, 2, 3]
]?;

let df_b = df![
    "foo" => [1, 1, 1],
    "bar" => ["a", "c", "c"],
    "ham" => ["let", "var", "const"]
]?;

let lf_a = df_a.clone().lazy();
let lf_b = df_b.clone().lazy();

let joined = lf_a.join(lf_b, vec![col("a")], vec![col("foo")], JoinArgs::new(JoinType::Full)).collect()?;
// joined:

// ╭─────┬─────┬─────┬──────┬─────────╮
// │ b   ┆ c   ┆ a   ┆ bar  ┆ ham     │
// │ --- ┆ --- ┆ --- ┆ ---  ┆ ---     │
// │ str ┆ i64 ┆ i64 ┆ str  ┆ str     │
// ╞═════╪═════╪═════╪══════╪═════════╡
// │ "a" ┆ 0   ┆ 1   ┆ "a"  ┆ "let"   │
// │ "a" ┆ 0   ┆ 1   ┆ "c"  ┆ "var"   │
// │ "a" ┆ 0   ┆ 1   ┆ "c"  ┆ "const" │
// │ "b" ┆ 1   ┆ 2   ┆ null ┆ null    │
// │ "c" ┆ 2   ┆ 1   ┆ null ┆ null    │
// │ "c" ┆ 3   ┆ 1   ┆ null ┆ null    │
// ╰─────┴─────┴─────┴──────┴─────────╯

// other join syntax options
let inner = lf_a.inner_join(lf_b, col("a"), col("foo")).collect()?;

let left = lf_a.left_join(lf_b, col("a"), col("foo")).collect()?;

let outer = lf_a.full_join(lf_b, col("a"), col("foo")).collect()?;

let joined_with_builder = lf_a.join_builder()
    .with(lf_b)
    .left_on(vec![col("a")])
    .right_on(vec![col("foo")])
    .how(JoinType::Inner)
    .force_parallel(true)
    .finish()
    .collect()?;
```

### <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#conditionally-apply" class="doc-anchor">§</a>Conditionally apply

If we want to create a new column based on some condition, we can use the [`when`](https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.when "method polars::prelude::Then::when")/[`then`](https://docs.rs/polars/latest/polars/prelude/struct.When.html#method.then "method polars::prelude::When::then")/[`otherwise`](https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.otherwise "method polars::prelude::Then::otherwise") expressions.

- [`when`](https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.when "method polars::prelude::Then::when") - accepts a predicate expression
- [`then`](https://docs.rs/polars/latest/polars/prelude/struct.When.html#method.then "method polars::prelude::When::then") - expression to use when `predicate == true`
- [`otherwise`](https://docs.rs/polars/latest/polars/prelude/struct.Then.html#method.otherwise "method polars::prelude::Then::otherwise") - expression to use when `predicate == false`

``` rust
use polars::prelude::*;
use polars::df;
let df = df![
    "range" => [1, 2, 3, 4, 5, 6, 8, 9, 10],
    "left" => (0..10).map(|_| Some("foo")).collect::<Vec<_>>(),
    "right" => (0..10).map(|_| Some("bar")).collect::<Vec<_>>()
]?;

let new = df.lazy()
    .with_column(when(col("range").gt_eq(lit(5)))
        .then(col("left"))
        .otherwise(col("right")).alias("foo_or_bar")
    ).collect()?;

// new:

// ╭───────┬───────┬───────┬────────────╮
// │ range ┆ left  ┆ right ┆ foo_or_bar │
// │ ---   ┆ ---   ┆ ---   ┆ ---        │
// │ i64   ┆ str   ┆ str   ┆ str        │
// ╞═══════╪═══════╪═══════╪════════════╡
// │ 0     ┆ "foo" ┆ "bar" ┆ "bar"      │
// │ 1     ┆ "foo" ┆ "bar" ┆ "bar"      │
// │ 2     ┆ "foo" ┆ "bar" ┆ "bar"      │
// │ 3     ┆ "foo" ┆ "bar" ┆ "bar"      │
// │ …     ┆ …     ┆ …     ┆ …          │
// │ 5     ┆ "foo" ┆ "bar" ┆ "foo"      │
// │ 6     ┆ "foo" ┆ "bar" ┆ "foo"      │
// │ 7     ┆ "foo" ┆ "bar" ┆ "foo"      │
// │ 8     ┆ "foo" ┆ "bar" ┆ "foo"      │
// │ 9     ┆ "foo" ┆ "bar" ┆ "foo"      │
// ╰───────┴───────┴───────┴────────────╯
```

## <a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#black-box-function" class="doc-anchor">§</a>Black box function

The expression API should be expressive enough for most of what you want to achieve, but it can happen that you need to pass the values to an external function you do not control. The snippet below shows how we use the [`Struct`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Struct "variant polars::prelude::DataType::Struct") datatype to be able to apply a function over multiple inputs.

<a href="https://docs.rs/polars/latest/polars/docs/lazy/index.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
use polars::prelude::*;
fn my_black_box_function(a: f32, b: f32) -> f32 {
    // do something
    a
}

fn apply_multiples() -> PolarsResult<DataFrame> {
    df![
        "a" => [1.0f32, 2.0, 3.0],
        "b" => [3.0f32, 5.1, 0.3]
    ]?
    .lazy()
    .select([as_struct(vec![col("a"), col("b")]).map(
        |s| {
            let ca = s.struct_()?;

            let series_a = ca.field_by_name("a")?;
            let series_b = ca.field_by_name("b")?;
            let chunked_a = series_a.f32()?;
            let chunked_b = series_b.f32()?;

            let out: Float32Chunked = chunked_a
                .into_iter()
                .zip(chunked_b.into_iter())
                .map(|(opt_a, opt_b)| match (opt_a, opt_b) {
                    (Some(a), Some(b)) => Some(my_black_box_function(a, b)),
                    _ => None,
                })
                .collect();

            Ok(Some(out.into_series()))
        },
        |_, f| Ok(Field::new(f.name().clone(), DataType::Float32))
    )])
    .collect()
}
```
