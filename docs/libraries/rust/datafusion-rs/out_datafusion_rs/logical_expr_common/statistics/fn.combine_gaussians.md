# Function combine_gaussians Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/statistics.rs.html#634-638" class="src">Source</a>

``` rust
pub fn combine_gaussians(
    op: &Operator,
    left: &GaussianDistribution,
    right: &GaussianDistribution,
) -> Result<Option<GaussianDistribution>, DataFusionError>
```

Expand description

Applies the given operation to the given Gaussian distributions. Currently, this function handles only addition and subtraction operations. If the result is not a Gaussian random variable, it returns `None`. For details, see:

<https://en.wikipedia.org/wiki/Sum_of_normally_distributed_random_variables>
