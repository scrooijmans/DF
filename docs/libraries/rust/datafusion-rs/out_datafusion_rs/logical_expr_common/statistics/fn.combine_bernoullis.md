# Function combine_bernoullis Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/statistics.rs.html#579-583" class="src">Source</a>

``` rust
pub fn combine_bernoullis(
    op: &Operator,
    left: &BernoulliDistribution,
    right: &BernoulliDistribution,
) -> Result<BernoulliDistribution, DataFusionError>
```

Expand description

This function takes a logical operator and two Bernoulli distributions, and it returns a new Bernoulli distribution that represents the result of the operation. Currently, only `AND` and `OR` operations are supported.
