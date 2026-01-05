# Module temporal Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/lib.rs.html#33" class="src">Source</a>

Expand description

Defines temporal kernels for time and date related functions.

## Enums<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/enum.DatePart.html" class="enum" title="enum arrow::compute::kernels::temporal::DatePart">DatePart</a>  
Valid parts to extract from date/time/timestamp arrays.

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/fn.date_part.html" class="fn" title="fn arrow::compute::kernels::temporal::date_part">date_part</a>  
Given an array, return a new array with the extracted [`DatePart`](https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html "enum arrow::compute::DatePart") as signed 32-bit integer values.
