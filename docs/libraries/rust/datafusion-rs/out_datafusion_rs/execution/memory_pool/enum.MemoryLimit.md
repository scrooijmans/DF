# Enum MemoryLimit Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/memory_pool/mod.rs.html#219" class="src">Source</a>

``` rust
pub enum MemoryLimit {
    Infinite,
    Finite(usize),
    Unknown,
}
```

Expand description

Memory limit of `MemoryPool`

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html#variant.Infinite" class="anchor">§</a>

### Infinite

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html#variant.Finite" class="anchor">§</a>

### Finite(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Bounded memory limit in bytes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html#variant.Unknown" class="anchor">§</a>

### Unknown

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html#blanket-implementations" class="anchor">§</a>
