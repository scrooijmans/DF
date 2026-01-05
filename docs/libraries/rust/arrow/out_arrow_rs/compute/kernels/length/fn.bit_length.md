# Function bit_length Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/length.rs.html#117" class="src">Source</a>

``` rust
pub fn bit_length(array: &dyn Array) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns an array of Int32/Int64 denoting the number of bits in each value in the array.

- this only accepts StringArray/Utf8, LargeString/LargeUtf8, BinaryArray and LargeBinaryArray, or DictionaryArray with above Arrays as values
- bit_length of null is null.
- bit_length is in number of bits
