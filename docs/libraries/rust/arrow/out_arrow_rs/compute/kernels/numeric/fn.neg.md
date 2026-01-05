# Function neg Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#101" class="src">Source</a>

``` rust
pub fn neg(array: &dyn Array) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Negates each element of `array`, returning an error on overflow

Note: negation of unsigned arrays is not supported and will return in an error, for wrapping unsigned negation consider using [`neg_wrapping`](https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.neg_wrapping.html "fn arrow::compute::kernels::numeric::neg_wrapping")
