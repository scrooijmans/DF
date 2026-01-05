# Function apply_multiple Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/mod.rs.html#1655-1659" class="src">Source</a>

``` rust
pub fn apply_multiple<F, DT, E>(
    function: F,
    expr: E,
    output_type: DT,
    returns_scalar: bool,
) -> Exprwhere
    F: Fn(&mut [Column]) -> Result<Column, PolarsError> + 'static + Send + Sync,
    DT: Fn(&Schema<DataType>, &[Field]) -> Result<Field, PolarsError> + 'static + Send + Sync,
    E: AsRef<[Expr]>,
```

Available on **crate feature `lazy`** only.

Expand description

Apply a function/closure over the groups of multiple columns. This should only be used in a group_by aggregation.

It is the responsibility of the caller that the schema is correct by giving the correct output_type. If None given the output type of the input expr is used.

This difference with [`map_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.map_multiple.html "fn polars::prelude::map_multiple") is that [`apply_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.apply_multiple.html "fn polars::prelude::apply_multiple") will create a separate [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") per group.

- [`map_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.map_multiple.html "fn polars::prelude::map_multiple") should be used for operations that are independent of groups, e.g. `multiply * 2`, or `raise to the power`
- [`apply_multiple`](https://docs.rs/polars/latest/polars/prelude/fn.apply_multiple.html "fn polars::prelude::apply_multiple") should be used for operations that work on a group of data. e.g. `sum`, `count`, etc.
