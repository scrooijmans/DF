# Function create_sparse_dict_from_values Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#510-544" class="src">Source</a>

``` rust
pub fn create_sparse_dict_from_values<K>(
    size: usize,
    null_density: f32,
    values: &dyn Array,
    key_range: Range<K::Native>,
) -> DictionaryArray<K>where
    K: ArrowDictionaryKeyType,
    StandardUniform: Distribution<K::Native>,
    K::Native: SampleUniform,
```

Available on **crate feature `test_utils`** only.

Expand description

Creates a random (but fixed-seeded) dictionary array of a given size and null density with the provided values array and key range
