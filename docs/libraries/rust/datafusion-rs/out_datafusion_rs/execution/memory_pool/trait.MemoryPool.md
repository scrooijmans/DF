# Trait MemoryPool Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/memory_pool/mod.rs.html#179" class="src">Source</a>

``` rust
pub trait MemoryPool:
    Send
    + Sync
    + Debug {
    // Required methods
    fn grow(&self, reservation: &MemoryReservation, additional: usize);
    fn shrink(&self, reservation: &MemoryReservation, shrink: usize);
    fn try_grow(
        &self,
        reservation: &MemoryReservation,
        additional: usize,
    ) -> Result<(), DataFusionError>;
    fn reserved(&self) -> usize;

    // Provided methods
    fn register(&self, _consumer: &MemoryConsumer) { ... }
    fn unregister(&self, _consumer: &MemoryConsumer) { ... }
    fn memory_limit(&self) -> MemoryLimit { ... }
}
```

Expand description

Tracks and potentially limits memory use across operators during execution.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#memory-management-overview" class="doc-anchor">§</a>Memory Management Overview

DataFusion is a streaming query engine, processing most queries without buffering the entire input. Most operators require a fixed amount of memory based on the schema and target batch size. However, certain operations such as sorting and grouping/joining, require buffering intermediate results, which can require memory proportional to the number of input rows.

Rather than tracking all allocations, DataFusion takes a pragmatic approach: Intermediate memory used as data streams through the system is not accounted (it assumed to be “small”) but the large consumers of memory must register and constrain their use. This design trades off the additional code complexity of memory tracking with limiting resource usage.

When limiting memory with a `MemoryPool` you should typically reserve some overhead (e.g. 10%) for the “small” memory allocations that are not tracked.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#memory-management-design" class="doc-anchor">§</a>Memory Management Design

As explained above, DataFusion’s design ONLY limits operators that require “large” amounts of memory (proportional to number of input rows), such as `GroupByHashExec`. It does NOT track and limit memory used internally by other operators such as `DataSourceExec` or the `RecordBatch`es that flow between operators. Furthermore, operators should not reserve memory for the batches they produce. Instead, if a consumer operator needs to hold batches from its producers in memory for an extended period, it is the consumer operator’s responsibility to reserve the necessary memory for those batches.

In order to avoid allocating memory until the OS or the container system kills the process, DataFusion `ExecutionPlan`s (operators) that consume large amounts of memory must first request their desired allocation from a [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") before allocating more. The request is typically managed via a [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") and [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer").

If the allocation is successful, the operator should proceed and allocate the desired memory. If the allocation fails, the operator must either first free memory (e.g. by spilling to local disk) and try again, or error.

Note that a `MemoryPool` can be shared by concurrently executing plans, which can be used to control memory usage in a multi-tenant system.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#how-memorypool-works-by-example" class="doc-anchor">§</a>How MemoryPool works by example

Scenario 1: For `Filter` operator, `RecordBatch`es will stream through it, so it don’t have to keep track of memory usage through [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool").

Scenario 2: For `CrossJoin` operator, if the input size gets larger, the intermediate state will also grow. So `CrossJoin` operator will use [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") to limit the memory usage. 2.1 `CrossJoin` operator has read a new batch, asked memory pool for additional memory. Memory pool updates the usage and returns success. 2.2 `CrossJoin` has read another batch, and tries to reserve more memory again, memory pool does not have enough memory. Since `CrossJoin` operator has not implemented spilling, it will stop execution and return an error.

Scenario 3: For `Aggregate` operator, its intermediate states will also accumulate as the input size gets larger, but with spilling capability. When it tries to reserve more memory from the memory pool, and the memory pool has already reached the memory limit, it will return an error. Then, `Aggregate` operator will spill the intermediate buffers to disk, and release memory from the memory pool, and continue to retry memory reservation.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#related-structs" class="doc-anchor">§</a>Related Structs

To better understand memory management in DataFusion, here are the key structs and their relationships:

- [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer"): A named allocation traced by a particular operator. If an execution is parallelized, and there are multiple partitions of the same operator, each partition will have a separate `MemoryConsumer`.
- `SharedRegistration`: A registration of a `MemoryConsumer` with a `MemoryPool`. `SharedRegistration` and `MemoryPool` have a many-to-one relationship. `MemoryPool` implementation can decide how to allocate memory based on the registered consumers. (e.g. `FairSpillPool` will try to share available memory evenly among all registered consumers)
- [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation"): Each `MemoryConsumer`/operator can have multiple `MemoryReservation`s for different internal data structures. The relationship between `MemoryConsumer` and `MemoryReservation` is one-to-many. This design enables cleaner operator implementations:
  - Different `MemoryReservation`s can be used for different purposes
  - `MemoryReservation` follows RAII principles - to release a reservation, simply drop the `MemoryReservation` object. When all `MemoryReservation`s for a `SharedRegistration` are dropped, the `SharedRegistration` is dropped when its reference count reaches zero, automatically unregistering the `MemoryConsumer` from the `MemoryPool`.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#relationship-diagram" class="doc-anchor">§</a>Relationship Diagram

``` text
┌──────────────────┐     ┌──────────────────┐
│MemoryReservation │     │MemoryReservation │
└───┬──────────────┘     └──────────────────┘ ......
    │belongs to                    │
    │      ┌───────────────────────┘           │  │
    │      │                                   │  │
    ▼      ▼                                   ▼  ▼
┌────────────────────────┐       ┌────────────────────────┐
│   SharedRegistration   │       │   SharedRegistration   │
│   ┌────────────────┐   │       │   ┌────────────────┐   │
│   │                │   │       │   │                │   │
│   │ MemoryConsumer │   │       │   │ MemoryConsumer │   │
│   │                │   │       │   │                │   │
│   └────────────────┘   │       │   └────────────────┘   │
└────────────┬───────────┘       └────────────┬───────────┘
             │                                │
             │                        register│into
             │                                │
             └─────────────┐   ┌──────────────┘
                           │   │
                           ▼   ▼
   ╔═══════════════════════════════════════════════════╗
   ║                                                   ║
   ║                    MemoryPool                     ║
   ║                                                   ║
   ╚═══════════════════════════════════════════════════╝
```

For example, there are two parallel partitions of an operator X: each partition corresponds to a `MemoryConsumer` in the above diagram. Inside each partition of operator X, there are typically several `MemoryReservation`s - one for each internal data structure that needs memory tracking (e.g., 1 reservation for the hash table, and 1 reservation for buffered input, etc.).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#implementing-memorypool" class="doc-anchor">§</a>Implementing `MemoryPool`

You can implement a custom allocation policy by implementing the [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") trait and configuring a `SessionContext` appropriately. However, DataFusion comes with the following simple memory pool implementations that handle many common cases:

- [`UnboundedMemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html "struct datafusion::execution::memory_pool::UnboundedMemoryPool"): no memory limits (the default)

- [`GreedyMemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.GreedyMemoryPool.html "struct datafusion::execution::memory_pool::GreedyMemoryPool"): Limits memory usage to a fixed size using a “first come first served” policy

- [`FairSpillPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.FairSpillPool.html "struct datafusion::execution::memory_pool::FairSpillPool"): Limits memory usage to a fixed size, allocating memory to all spilling operators fairly

- [`TrackConsumersPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html "struct datafusion::execution::memory_pool::TrackConsumersPool"): Wraps another [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") and tracks consumers, providing better error messages on the largest memory users.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.grow" class="fn">grow</a>(&self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Infallibly grow the provided `reservation` by `additional` bytes

This must always succeed

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.shrink" class="fn">shrink</a>(&self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, shrink: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Infallibly shrink the provided `reservation` by `shrink` bytes

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.try_grow" class="fn">try_grow</a>( &self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempt to grow the provided `reservation` by `additional` bytes

On error the `allocation` will not be increased in size

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.reserved" class="fn">reserved</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the total amount of memory reserved

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.register" class="fn">register</a>(&self, \_consumer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>)

Registers a new [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer")

Note: Subsequent calls to [`Self::grow`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.grow "method datafusion_execution::memory_pool::MemoryPool::grow::grow") must be made to reserve memory

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.unregister" class="fn">unregister</a>(&self, \_consumer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>)

Records the destruction of a [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") with [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer")

Note: Prior calls to [`Self::shrink`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.shrink "method datafusion_execution::memory_pool::MemoryPool::shrink::shrink") must be made to free any reserved memory

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.memory_limit" class="fn">memory_limit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html" class="enum" title="enum datafusion::execution::memory_pool::MemoryLimit">MemoryLimit</a>

Return the memory limit of the pool

The default implementation of `MemoryPool::memory_limit` will return `MemoryLimit::Unknown`. If you are using your custom memory pool, but have the requirement to know the memory usage limit of the pool, please implement this method to return it(`Memory::Finite(limit)`).

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#impl-MemoryPool-for-FairSpillPool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.FairSpillPool.html" class="struct" title="struct datafusion::execution::memory_pool::FairSpillPool">FairSpillPool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#impl-MemoryPool-for-GreedyMemoryPool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.GreedyMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::GreedyMemoryPool">GreedyMemoryPool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#impl-MemoryPool-for-UnboundedMemoryPool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::UnboundedMemoryPool">UnboundedMemoryPool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#impl-MemoryPool-for-TrackConsumersPool%3CI%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html" class="struct" title="struct datafusion::execution::memory_pool::TrackConsumersPool">TrackConsumersPool</a>\<I\>

where I: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a>,
