# Function substring Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/substring.rs.html#72-76" class="src">Source</a>

``` rust
pub fn substring(
    array: &dyn Array,
    start: i64,
    length: Option<u64>,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") with substrings of all the elements in `array`.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html#arguments" class="doc-anchor">§</a>Arguments

- `start` - The start index of all substrings. If `start >= 0`, then count from the start of the string, otherwise count from the end of the string.

- `length`(option) - The length of all substrings. If `length` is [None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), then the substring is from `start` to the end of the string.

Attention: Both `start` and `length` are counted by byte, not by char.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html#basic-usage" class="doc-anchor">§</a>Basic usage

``` rust
let array = StringArray::from(vec![Some("arrow"), None, Some("rust")]);
let result = substring(&array, 1, Some(4)).unwrap();
let result = result.as_any().downcast_ref::<StringArray>().unwrap();
assert_eq!(result, &StringArray::from(vec![Some("rrow"), None, Some("ust")]));
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html#error" class="doc-anchor">§</a>Error

- The function errors when the passed array is not a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray"), [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray"), [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") or [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") with supported array type as its value type.
- The function errors if the offset of a substring in the input array is at invalid char boundary (only for \[Large\]String array). It is recommended to use [`substring_by_char`](https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring_by_char.html "fn arrow::compute::kernels::substring::substring_by_char") if the input array may contain non-ASCII chars.

### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html#example-of-trying-to-get-an-invalid-utf-8-format-substring" class="doc-anchor">§</a>Example of trying to get an invalid utf-8 format substring

``` rust
let array = StringArray::from(vec![Some("E=mc²")]);
let error = substring(&array, 0, Some(5)).unwrap_err().to_string();
assert!(error.contains("invalid utf-8 boundary"));
```
