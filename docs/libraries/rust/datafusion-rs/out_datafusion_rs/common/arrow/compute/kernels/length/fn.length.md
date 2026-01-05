# Function length Copy item path

<a href="https://docs.rs/arrow-string/56.0.0/x86_64-unknown-linux-gnu/src/arrow_string/length.rs.html#55" class="src">Source</a>

``` rust
pub fn length(array: &dyn Array) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns an array of Int32/Int64 denoting the length of each value in the array.

For list array, length is the number of elements in each list. For string array and binary array, length is the number of bytes of each value.

- this only accepts ListArray/LargeListArray, StringArray/LargeStringArray/StringViewArray, BinaryArray/LargeBinaryArray, and FixedSizeListArray, or DictionaryArray with above Arrays as values
- length of null is null.
