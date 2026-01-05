# Function nlike Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/like.rs.html#99" class="src">Source</a>

``` rust
pub fn nlike(
    left: &dyn Datum,
    right: &dyn Datum,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Perform SQL `left NOT LIKE right`

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.nlike.html#notes" class="doc-anchor">§</a>Notes

- This is a negative of [`like`](https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.like.html "fn arrow::compute::kernels::comparison::like")
- See the documentation on [`like`](https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.like.html "fn arrow::compute::kernels::comparison::like") for more details
