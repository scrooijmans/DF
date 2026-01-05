# Module alloc Copy item path

<a href="https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/src/arrow_buffer/lib.rs.html#29" class="src">Source</a>

Expand description

Defines the low-level [`Allocation`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/trait.Allocation.html "trait datafusion::common::arrow::alloc::Allocation") API for shared memory regions

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/constant.ALIGNMENT.html" class="constant" title="constant datafusion::common::arrow::alloc::ALIGNMENT">ALIGNMENT</a>  
Cache and allocation multiple alignment size

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/trait.Allocation.html" class="trait" title="trait datafusion::common::arrow::alloc::Allocation">Allocation</a>  
The owner of an allocation. The trait implementation is responsible for dropping the allocations once no more references exist.
