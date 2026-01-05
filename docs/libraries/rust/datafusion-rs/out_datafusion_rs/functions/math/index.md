# Module math Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/lib.rs.html#123" class="src">Source</a>

Expand description

Mathematical functions. Enabled via feature flag `math_expressions` “math” DataFusion functions

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/abs/index.html" class="mod" title="mod datafusion::functions::math::abs">abs</a>

math expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/bounds/index.html" class="mod" title="mod datafusion::functions::math::bounds">bounds</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/cot/index.html" class="mod" title="mod datafusion::functions::math::cot">cot</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/expr_fn/index.html" class="mod" title="mod datafusion::functions::math::expr_fn">expr_fn</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/factorial/index.html" class="mod" title="mod datafusion::functions::math::factorial">factorial</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/gcd/index.html" class="mod" title="mod datafusion::functions::math::gcd">gcd</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/iszero/index.html" class="mod" title="mod datafusion::functions::math::iszero">iszero</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/lcm/index.html" class="mod" title="mod datafusion::functions::math::lcm">lcm</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/log/index.html" class="mod" title="mod datafusion::functions::math::log">log</a>

Math function: `log()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/monotonicity/index.html" class="mod" title="mod datafusion::functions::math::monotonicity">monotonicity</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/nans/index.html" class="mod" title="mod datafusion::functions::math::nans">nans</a>

Math function: `isnan()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/nanvl/index.html" class="mod" title="mod datafusion::functions::math::nanvl">nanvl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/pi/index.html" class="mod" title="mod datafusion::functions::math::pi">pi</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/power/index.html" class="mod" title="mod datafusion::functions::math::power">power</a>

Math function: `power()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/random/index.html" class="mod" title="mod datafusion::functions::math::random">random</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/round/index.html" class="mod" title="mod datafusion::functions::math::round">round</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/signum/index.html" class="mod" title="mod datafusion::functions::math::signum">signum</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/trunc/index.html" class="mod" title="mod datafusion::functions::math::trunc">trunc</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.abs.html" class="fn" title="fn datafusion::functions::math::abs">abs</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of abs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.acos.html" class="fn" title="fn datafusion::functions::math::acos">acos</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of acos

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.acosh.html" class="fn" title="fn datafusion::functions::math::acosh">acosh</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of acosh

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.asin.html" class="fn" title="fn datafusion::functions::math::asin">asin</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of asin

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.asinh.html" class="fn" title="fn datafusion::functions::math::asinh">asinh</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of asinh

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.atan.html" class="fn" title="fn datafusion::functions::math::atan">atan</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of atan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.atan2.html" class="fn" title="fn datafusion::functions::math::atan2">atan2</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of atan2

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.atanh.html" class="fn" title="fn datafusion::functions::math::atanh">atanh</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of atanh

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.cbrt.html" class="fn" title="fn datafusion::functions::math::cbrt">cbrt</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of cbrt

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.ceil.html" class="fn" title="fn datafusion::functions::math::ceil">ceil</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of ceil

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.cos.html" class="fn" title="fn datafusion::functions::math::cos">cos</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of cos

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.cosh.html" class="fn" title="fn datafusion::functions::math::cosh">cosh</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of cosh

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.cot.html" class="fn" title="fn datafusion::functions::math::cot">cot</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of cot

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.degrees.html" class="fn" title="fn datafusion::functions::math::degrees">degrees</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of degrees

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.exp.html" class="fn" title="fn datafusion::functions::math::exp">exp</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of exp

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.factorial.html" class="fn" title="fn datafusion::functions::math::factorial">factorial</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of factorial

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.floor.html" class="fn" title="fn datafusion::functions::math::floor">floor</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of floor

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.functions.html" class="fn" title="fn datafusion::functions::math::functions">functions</a>  
Returns all DataFusion functions defined in this package

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.gcd.html" class="fn" title="fn datafusion::functions::math::gcd">gcd</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of gcd

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.isnan.html" class="fn" title="fn datafusion::functions::math::isnan">isnan</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of isnan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.iszero.html" class="fn" title="fn datafusion::functions::math::iszero">iszero</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of iszero

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.lcm.html" class="fn" title="fn datafusion::functions::math::lcm">lcm</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of lcm

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.ln.html" class="fn" title="fn datafusion::functions::math::ln">ln</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of ln

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.log.html" class="fn" title="fn datafusion::functions::math::log">log</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of log

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.log2.html" class="fn" title="fn datafusion::functions::math::log2">log2</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of log2

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.log10.html" class="fn" title="fn datafusion::functions::math::log10">log10</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of log10

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.nanvl.html" class="fn" title="fn datafusion::functions::math::nanvl">nanvl</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of nanvl

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.pi.html" class="fn" title="fn datafusion::functions::math::pi">pi</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of pi

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.power.html" class="fn" title="fn datafusion::functions::math::power">power</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of power

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.radians.html" class="fn" title="fn datafusion::functions::math::radians">radians</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of radians

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.random.html" class="fn" title="fn datafusion::functions::math::random">random</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of random

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.round.html" class="fn" title="fn datafusion::functions::math::round">round</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of round

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.signum.html" class="fn" title="fn datafusion::functions::math::signum">signum</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of signum

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.sin.html" class="fn" title="fn datafusion::functions::math::sin">sin</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of sin

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.sinh.html" class="fn" title="fn datafusion::functions::math::sinh">sinh</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of sinh

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.sqrt.html" class="fn" title="fn datafusion::functions::math::sqrt">sqrt</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of sqrt

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.tan.html" class="fn" title="fn datafusion::functions::math::tan">tan</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of tan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.tanh.html" class="fn" title="fn datafusion::functions::math::tanh">tanh</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of tanh

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/fn.trunc.html" class="fn" title="fn datafusion::functions::math::trunc">trunc</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of trunc
