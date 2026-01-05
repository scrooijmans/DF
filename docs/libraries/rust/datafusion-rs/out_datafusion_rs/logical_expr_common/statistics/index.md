# Module statistics Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#44" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/struct.BernoulliDistribution.html" class="struct" title="struct datafusion::logical_expr_common::statistics::BernoulliDistribution">BernoulliDistribution</a>  
Bernoulli distribution with success probability `p`. If `p` has a null value, the success probability is unknown. For a more in-depth discussion, see:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/struct.ExponentialDistribution.html" class="struct" title="struct datafusion::logical_expr_common::statistics::ExponentialDistribution">ExponentialDistribution</a>  
Exponential distribution with an optional shift. The probability density function (PDF) is defined as follows:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/struct.GaussianDistribution.html" class="struct" title="struct datafusion::logical_expr_common::statistics::GaussianDistribution">GaussianDistribution</a>  
Gaussian (normal) distribution, represented by its mean and variance. For a more in-depth discussion, see:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/struct.GenericDistribution.html" class="struct" title="struct datafusion::logical_expr_common::statistics::GenericDistribution">GenericDistribution</a>  
A generic distribution whose functional form is not available, which is approximated via some summary statistics. For a more in-depth discussion, see:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/struct.UniformDistribution.html" class="struct" title="struct datafusion::logical_expr_common::statistics::UniformDistribution">UniformDistribution</a>  
Uniform distribution, represented by its range. If the given range extends towards infinity, the distribution will be improper – which is OK. For a more in-depth discussion, see:

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr_common::statistics::Distribution">Distribution</a>  
This object defines probabilistic distributions that encode uncertain information about a single, scalar value. Currently, we support five core statistical distributions. New variants will be added over time.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.combine_bernoullis.html" class="fn" title="fn datafusion::logical_expr_common::statistics::combine_bernoullis">combine_bernoullis</a>  
This function takes a logical operator and two Bernoulli distributions, and it returns a new Bernoulli distribution that represents the result of the operation. Currently, only `AND` and `OR` operations are supported.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.combine_gaussians.html" class="fn" title="fn datafusion::logical_expr_common::statistics::combine_gaussians">combine_gaussians</a>  
Applies the given operation to the given Gaussian distributions. Currently, this function handles only addition and subtraction operations. If the result is not a Gaussian random variable, it returns `None`. For details, see:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.compute_mean.html" class="fn" title="fn datafusion::logical_expr_common::statistics::compute_mean">compute_mean</a>  
Computes the mean value for the result of the given binary operation on two unknown quantities represented by their [`Distribution`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html "enum datafusion::logical_expr::statistics::Distribution") objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.compute_median.html" class="fn" title="fn datafusion::logical_expr_common::statistics::compute_median">compute_median</a>  
Computes the median value for the result of the given binary operation on two unknown quantities represented by its [`Distribution`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html "enum datafusion::logical_expr::statistics::Distribution") objects. Currently, the median is calculable only for addition and subtraction operations on:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.compute_variance.html" class="fn" title="fn datafusion::logical_expr_common::statistics::compute_variance">compute_variance</a>  
Computes the variance value for the result of the given binary operation on two unknown quantities represented by their [`Distribution`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html "enum datafusion::logical_expr::statistics::Distribution") objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.create_bernoulli_from_comparison.html" class="fn" title="fn datafusion::logical_expr_common::statistics::create_bernoulli_from_comparison">create_bernoulli_from_comparison</a>  
Creates a new `Bernoulli` distribution by computing the resulting probability. Expects `op` to be a comparison operator, with `left` and `right` having numeric distributions. The resulting distribution has the `Float64` data type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/statistics/fn.new_generic_from_binary_op.html" class="fn" title="fn datafusion::logical_expr_common::statistics::new_generic_from_binary_op">new_generic_from_binary_op</a>  
Creates a new [`Generic`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html#variant.Generic "variant datafusion::logical_expr::statistics::Distribution::Generic") distribution that represents the result of the given binary operation on two unknown quantities represented by their [`Distribution`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html "enum datafusion::logical_expr::statistics::Distribution") objects. The function computes the mean, median and variance if possible.
