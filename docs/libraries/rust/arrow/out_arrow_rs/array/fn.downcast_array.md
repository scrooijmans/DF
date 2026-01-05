# Function downcast_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#805-807" class="src">Source</a>

``` rust
pub fn downcast_array<T>(array: &dyn Array) -> Twhere
    T: From<ArrayData>,
```

Expand description

Downcasts a `dyn Array` to a concrete type

``` rust
struct ConcreteBatch {
    col1: Int32Array,
    col2: BooleanArray,
    col3: StringArray,
}

impl ConcreteBatch {
    fn new(batch: &RecordBatch) -> Self {
        Self {
            col1: downcast_array(batch.column(0).as_ref()),
            col2: downcast_array(batch.column(1).as_ref()),
            col3: downcast_array(batch.column(2).as_ref()),
        }
    }
}
```

## <a href="https://docs.rs/arrow/latest/arrow/array/fn.downcast_array.html#panics" class="doc-anchor">§</a>Panics

Panics if array is not of the correct data type
