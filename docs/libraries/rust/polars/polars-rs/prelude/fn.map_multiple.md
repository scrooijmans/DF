# Function map_multiple Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/mod.rs.html#1627-1631" class="src">Source</a>

``` rust
pub fn map_multiple<F, DT, E>(function: F, expr: E, output_type: DT) -> Exprwhere
    F: Fn(&mut [Column]) -> Result<Column, PolarsError> + 'static + Send + Sync,
    DT: Fn(&Schema<DataType>, &[Field]) -> Result<Field, PolarsError> + 'static + Send + Sync,
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Apply a function/closure over multiple columns once the logical plan get executed.

This function is very similar to [`apply_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.apply_multiple.html "fn polars::prelude::apply_multiple"), but differs in how it handles aggregations.

- [`map_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.map_multiple.html "fn polars::prelude::map_multiple") should be used for operations that are independent of groups, e.g. `multiply * 2`, or `raise to the power`
- [`apply_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.apply_multiple.html "fn polars::prelude::apply_multiple") should be used for operations that work on a group of data. e.g. `sum`, `count`, etc.

It is the responsibility of the caller that the schema is correct by giving the correct output_type. If None given the output type of the input expr is used.
