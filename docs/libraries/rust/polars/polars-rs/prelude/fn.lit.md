# Function lit Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/plans/lit.rs.html#617" class="src">Source</a>

``` rust
pub fn lit<L>(t: L) -> Exprwhere
    L: Literal,
```

Available on **crate feature `lazy`** only.

Expand description

Create a Literal Expression from `L`. A literal expression behaves like a column that contains a single distinct value.

The column is automatically of the “correct” length to make the operations work. Often this is determined by the length of the `LazyFrame` it is being used with. For instance, `lazy_df.with_column(lit(5).alias("five"))` creates a new column named “five” that is the length of the Dataframe (at the time `collect` is called), where every value in the column is `5`.
