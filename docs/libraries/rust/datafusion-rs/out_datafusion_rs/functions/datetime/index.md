# Module datetime Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/lib.rs.html#110" class="src">Source</a>

Expand description

Date and time expressions. Contains functions such as to_timestamp Enabled via feature flag `datetime_expressions` date & time DataFusion functions

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/common/index.html" class="mod" title="mod datafusion::functions::datetime::common">common</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/current_date/index.html" class="mod" title="mod datafusion::functions::datetime::current_date">current_date</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/current_time/index.html" class="mod" title="mod datafusion::functions::datetime::current_time">current_time</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/date_bin/index.html" class="mod" title="mod datafusion::functions::datetime::date_bin">date_bin</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/date_part/index.html" class="mod" title="mod datafusion::functions::datetime::date_part">date_part</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/date_trunc/index.html" class="mod" title="mod datafusion::functions::datetime::date_trunc">date_trunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/expr_fn/index.html" class="mod" title="mod datafusion::functions::datetime::expr_fn">expr_fn</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/from_unixtime/index.html" class="mod" title="mod datafusion::functions::datetime::from_unixtime">from_unixtime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/make_date/index.html" class="mod" title="mod datafusion::functions::datetime::make_date">make_date</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/now/index.html" class="mod" title="mod datafusion::functions::datetime::now">now</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/planner/index.html" class="mod" title="mod datafusion::functions::datetime::planner">planner</a>

SQL planning extensions like [`DatetimeFunctionPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/planner/struct.DatetimeFunctionPlanner.html "struct datafusion::functions::datetime::planner::DatetimeFunctionPlanner")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_char/index.html" class="mod" title="mod datafusion::functions::datetime::to_char">to_char</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_date/index.html" class="mod" title="mod datafusion::functions::datetime::to_date">to_date</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_local_time/index.html" class="mod" title="mod datafusion::functions::datetime::to_local_time">to_local_time</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_timestamp/index.html" class="mod" title="mod datafusion::functions::datetime::to_timestamp">to_timestamp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/to_unixtime/index.html" class="mod" title="mod datafusion::functions::datetime::to_unixtime">to_unixtime</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.current_date.html" class="fn" title="fn datafusion::functions::datetime::current_date">current_date</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of current_date

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.current_time.html" class="fn" title="fn datafusion::functions::datetime::current_time">current_time</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of current_time

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.date_bin.html" class="fn" title="fn datafusion::functions::datetime::date_bin">date_bin</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of date_bin

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.date_part.html" class="fn" title="fn datafusion::functions::datetime::date_part">date_part</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of date_part

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.date_trunc.html" class="fn" title="fn datafusion::functions::datetime::date_trunc">date_trunc</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of date_trunc

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.from_unixtime.html" class="fn" title="fn datafusion::functions::datetime::from_unixtime">from_unixtime</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of from_unixtime

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.functions.html" class="fn" title="fn datafusion::functions::datetime::functions">functions</a>  
Returns all DataFusion functions defined in this package

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.make_date.html" class="fn" title="fn datafusion::functions::datetime::make_date">make_date</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of make_date

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.now.html" class="fn" title="fn datafusion::functions::datetime::now">now</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of now

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_char.html" class="fn" title="fn datafusion::functions::datetime::to_char">to_char</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_char

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_date.html" class="fn" title="fn datafusion::functions::datetime::to_date">to_date</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_date

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_local_time.html" class="fn" title="fn datafusion::functions::datetime::to_local_time">to_local_time</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_local_time

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_timestamp.html" class="fn" title="fn datafusion::functions::datetime::to_timestamp">to_timestamp</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_timestamp

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_timestamp_micros.html" class="fn" title="fn datafusion::functions::datetime::to_timestamp_micros">to_timestamp_micros</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_timestamp_micros

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_timestamp_millis.html" class="fn" title="fn datafusion::functions::datetime::to_timestamp_millis">to_timestamp_millis</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_timestamp_millis

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_timestamp_nanos.html" class="fn" title="fn datafusion::functions::datetime::to_timestamp_nanos">to_timestamp_nanos</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_timestamp_nanos

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_timestamp_seconds.html" class="fn" title="fn datafusion::functions::datetime::to_timestamp_seconds">to_timestamp_seconds</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_timestamp_seconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/fn.to_unixtime.html" class="fn" title="fn datafusion::functions::datetime::to_unixtime">to_unixtime</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of to_unixtime
