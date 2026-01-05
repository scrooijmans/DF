# Trait Allocation Copy item path

<a href="https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/src/arrow_buffer/alloc/mod.rs.html#31" class="src">Source</a>

``` rust
pub trait Allocation:
    RefUnwindSafe
    + Send
    + Sync { }
```

Expand description

The owner of an allocation. The trait implementation is responsible for dropping the allocations once no more references exist.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/trait.Allocation.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/trait.Allocation.html#impl-Allocation-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/trait.Allocation.html" class="trait" title="trait datafusion::common::arrow::alloc::Allocation">Allocation</a> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html" class="trait" title="trait core::panic::unwind_safe::RefUnwindSafe">RefUnwindSafe</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,
