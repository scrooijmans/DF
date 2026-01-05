# Function dtype_cols Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/functions/selectors.rs.html#64" class="src">Source</a>

``` rust
pub fn dtype_cols<DT>(dtype: DT) -> DataTypeSelectorwhere
    DT: AsRef<[DataType]>,
```

Available on **crate feature `lazy`** only.

Expand description

Select multiple columns by dtype.
