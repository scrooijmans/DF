# Module memory_pool Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/lib.rs.html#32" class="src">Source</a>

Expand description

[`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") for memory management during query execution, [`proxy`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/index.html "mod datafusion::execution::memory_pool::proxy") for help with allocation accounting.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/index.html" class="mod" title="mod datafusion::execution::memory_pool::proxy">proxy</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/units/index.html" class="mod" title="mod datafusion::execution::memory_pool::units">units</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.FairSpillPool.html" class="struct" title="struct datafusion::execution::memory_pool::FairSpillPool">FairSpillPool</a>  
A [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that prevents spillable reservations from using more than an even fraction of the available memory sans any unspillable reservations (i.e. `(pool_size - unspillable_memory) / num_spillable_reservations`)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.GreedyMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::GreedyMemoryPool">GreedyMemoryPool</a>  
A [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that implements a greedy first-come first-serve limit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>  
A memory consumer is a named allocation traced by a particular [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") in a [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool"). All allocations are registered to a particular `MemoryConsumer`;

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>  
A [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") tracks an individual reservation of a number of bytes of memory in a [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that is freed back to the pool on drop.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html" class="struct" title="struct datafusion::execution::memory_pool::TrackConsumersPool">TrackConsumersPool</a>  
A [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that tracks the consumers that have reserved memory within the inner memory pool.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::UnboundedMemoryPool">UnboundedMemoryPool</a>  
A [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that enforces no limit

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html" class="enum" title="enum datafusion::execution::memory_pool::MemoryLimit">MemoryLimit</a>  
Memory limit of `MemoryPool`

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a>  
Tracks and potentially limits memory use across operators during execution.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/fn.human_readable_size.html" class="fn" title="fn datafusion::execution::memory_pool::human_readable_size">human_readable_size</a>  
Present size in human-readable form
