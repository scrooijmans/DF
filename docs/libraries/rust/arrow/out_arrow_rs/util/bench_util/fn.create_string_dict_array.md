# Function create_string_dict_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#358-378" class="src">Source</a>

``` rust
pub fn create_string_dict_array<K: ArrowDictionaryKeyType>(
    size: usize,
    null_density: f32,
    str_len: usize,
) -> DictionaryArray<K>
```

Available on **crate feature `test_utils`** only.

Expand description

Creates an random (but fixed-seeded) array of a given size and null density consisting of random 4 character alphanumeric strings
