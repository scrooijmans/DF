# Function ilike Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/like.rs.html#90" class="src">Source</a>

``` rust
pub fn ilike(
    left: &dyn Datum,
    right: &dyn Datum,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Perform SQL `left ILIKE right`

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.ilike.html#notes" class="doc-anchor">§</a>Notes

- This is a case-insensitive version of [`like`](https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.like.html "fn arrow::compute::kernels::comparison::like")
- See the documentation on [`like`](https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.like.html "fn arrow::compute::kernels::comparison::like") for more details
- Implements loose matching as defined by the Unicode standard. For example, the `ﬀ` ligature is not equivalent to `FF` and `ß` is not equivalent to `SS`
