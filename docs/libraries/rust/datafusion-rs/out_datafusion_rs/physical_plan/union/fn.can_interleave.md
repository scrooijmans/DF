# Function can_interleave Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/union.rs.html#527-529" class="src">Source</a>

``` rust
pub fn can_interleave<T>(inputs: impl Iterator<Item = T>) -> boolwhere
    T: Borrow<Arc<dyn ExecutionPlan>>,
```

Expand description

If all the input partitions have the same Hash partition spec with the first_input_partition The InterleaveExec is partition aware.

It might be too strict here in the case that the input partition specs are compatible but not exactly the same. For example one input partition has the partition spec Hash(‘a’,‘b’,‘c’) and other has the partition spec Hash(‘a’), It is safe to derive the out partition with the spec Hash(‘a’,‘b’,‘c’).
