# Module cache Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/lib.rs.html#29" class="src">Source</a>

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/index.html" class="mod" title="mod datafusion::execution::cache::cache_manager">cache_manager</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/index.html" class="mod" title="mod datafusion::execution::cache::cache_unit">cache_unit</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/lru_queue/index.html" class="mod" title="mod datafusion::execution::cache::lru_queue">lru_queue</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html" class="trait" title="trait datafusion::execution::cache::CacheAccessor">CacheAccessor</a>  
The cache accessor, users usually working on this interface while manipulating caches. This interface does not get `mut` references and thus has to handle its own locking via internal mutability. It can be accessed via multiple concurrent queries during planning and execution.
