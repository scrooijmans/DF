# Module numeric Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/lib.rs.html#32" class="src">Source</a>

Expand description

Defines numeric arithmetic kernels on [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray"), such as [`add`](https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.add.html "fn arrow::compute::kernels::numeric::add")

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.add.html" class="fn" title="fn arrow::compute::kernels::numeric::add">add</a>  
Perform `lhs + rhs`, returning an error on overflow

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.add_wrapping.html" class="fn" title="fn arrow::compute::kernels::numeric::add_wrapping">add_wrapping</a>  
Perform `lhs + rhs`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_integer "method arrow::datatypes::DataType::is_integer")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.div.html" class="fn" title="fn arrow::compute::kernels::numeric::div">div</a>  
Perform `lhs / rhs`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.mul.html" class="fn" title="fn arrow::compute::kernels::numeric::mul">mul</a>  
Perform `lhs * rhs`, returning an error on overflow

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.mul_wrapping.html" class="fn" title="fn arrow::compute::kernels::numeric::mul_wrapping">mul_wrapping</a>  
Perform `lhs * rhs`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_integer "method arrow::datatypes::DataType::is_integer")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.neg.html" class="fn" title="fn arrow::compute::kernels::numeric::neg">neg</a>  
Negates each element of `array`, returning an error on overflow

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.neg_wrapping.html" class="fn" title="fn arrow::compute::kernels::numeric::neg_wrapping">neg_wrapping</a>  
Negates each element of `array`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_integer "method arrow::datatypes::DataType::is_integer")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.rem.html" class="fn" title="fn arrow::compute::kernels::numeric::rem">rem</a>  
Perform `lhs % rhs`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.sub.html" class="fn" title="fn arrow::compute::kernels::numeric::sub">sub</a>  
Perform `lhs - rhs`, returning an error on overflow

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.sub_wrapping.html" class="fn" title="fn arrow::compute::kernels::numeric::sub_wrapping">sub_wrapping</a>  
Perform `lhs - rhs`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_integer "method arrow::datatypes::DataType::is_integer")
