# Function interleave Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/interleave.rs.html#70-73" class="src">Source</a>

``` rust
pub fn interleave(
    values: &[&dyn Array],
    indices: &[(usize, usize)],
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Takes elements by index from a list of [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), creating a new [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") from those values.

Each element in `indices` is a pair of `usize` with the first identifying the index of the [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") in `values`, and the second the index of the value within that [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

``` text
┌─────────────────┐      ┌─────────┐                                  ┌─────────────────┐
│        A        │      │ (0, 0)  │        interleave(               │        A        │
├─────────────────┤      ├─────────┤          [values0, values1],     ├─────────────────┤
│        D        │      │ (1, 0)  │          indices                 │        B        │
└─────────────────┘      ├─────────┤        )                         ├─────────────────┤
  values array 0         │ (1, 1)  │      ─────────────────────────▶  │        C        │
                         ├─────────┤                                  ├─────────────────┤
                         │ (0, 1)  │                                  │        D        │
                         └─────────┘                                  └─────────────────┘
┌─────────────────┐       indices
│        B        │        array
├─────────────────┤                                                    result
│        C        │
├─────────────────┤
│        E        │
└─────────────────┘
  values array 1
```

For selecting values by index from a single array see [`crate::take`](https://docs.rs/arrow/latest/arrow/compute/kernels/take/index.html "mod arrow::compute::kernels::take")
