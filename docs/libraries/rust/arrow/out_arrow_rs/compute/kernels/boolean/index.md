# Module boolean Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/lib.rs.html#31" class="src">Source</a>

Expand description

Defines boolean kernels on Arrow `BooleanArray`’s, e.g. `AND`, `OR` and `NOT`.

These kernels can leverage SIMD if available on your system. Currently no runtime detection is provided, you should enable the specific SIMD intrinsics using `RUSTFLAGS="-C target-feature=+avx2"` for example. See the documentation [here](https://doc.rust-lang.org/stable/core/arch/) for more information.

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and.html" class="fn" title="fn arrow::compute::kernels::boolean::and">and</a>  
Performs `AND` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and_kleene.html" class="fn" title="fn arrow::compute::kernels::boolean::and_kleene">and_kleene</a>  
Logical ‘and’ boolean values with Kleene logic

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and_not.html" class="fn" title="fn arrow::compute::kernels::boolean::and_not">and_not</a>  
Performs `AND_NOT` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.is_not_null.html" class="fn" title="fn arrow::compute::kernels::boolean::is_not_null">is_not_null</a>  
Returns a non-null [BooleanArray](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") with whether each value of the array is not null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.is_null.html" class="fn" title="fn arrow::compute::kernels::boolean::is_null">is_null</a>  
Returns a non-null [BooleanArray](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") with whether each value of the array is null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.not.html" class="fn" title="fn arrow::compute::kernels::boolean::not">not</a>  
Performs unary `NOT` operation on an arrays. If value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.or.html" class="fn" title="fn arrow::compute::kernels::boolean::or">or</a>  
Performs `OR` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.or_kleene.html" class="fn" title="fn arrow::compute::kernels::boolean::or_kleene">or_kleene</a>  
Logical ‘or’ boolean values with Kleene logic
