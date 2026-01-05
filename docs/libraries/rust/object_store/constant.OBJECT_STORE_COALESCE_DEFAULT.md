# Constant OBJECT_STORE_COALESCE_DEFAULT Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/util.rs.html#92" class="src">Source</a>

``` rust
pub const OBJECT_STORE_COALESCE_DEFAULT: u64 = _; // 1_048_576u64
```

Expand description

Range requests with a gap less than or equal to this, will be coalesced into a single request by [`coalesce_ranges`](https://docs.rs/object_store/latest/object_store/fn.coalesce_ranges.html "fn object_store::coalesce_ranges")
