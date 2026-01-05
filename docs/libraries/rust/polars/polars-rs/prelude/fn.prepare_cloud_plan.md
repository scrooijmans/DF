# Function prepare_cloud_plan Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/client/mod.rs.html#8" class="src">Source</a>

``` rust
pub fn prepare_cloud_plan(dsl: DslPlan) -> Result<Vec<u8>, PolarsError>
```

Available on **crate feature `lazy`** only.

Expand description

Prepare the given [`DslPlan`](https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html "enum polars::prelude::DslPlan") for execution on Polars Cloud.
