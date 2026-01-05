# Function partial_sort Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#979-981" class="src">Source</a>

``` rust
pub fn partial_sort<T, F>(v: &mut [T], limit: usize, is_less: F)where
    F: FnMut(&T, &T) -> Ordering,
```

Expand description

It’s unstable_sort, may not preserve the order of equal elements
