# Function substring_by_char Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/substring.rs.html#189-193" class="src">Source</a>

``` rust
pub fn substring_by_char<OffsetSize>(
    array: &GenericByteArray<GenericStringType<OffsetSize>>,
    start: i64,
    length: Option<u64>,
) -> Result<GenericByteArray<GenericStringType<OffsetSize>>, ArrowError>where
    OffsetSize: OffsetSizeTrait,
```

Expand description

Substrings based on character index

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring_by_char.html#arguments" class="doc-anchor">§</a>Arguments

- `array` - The input string array

- `start` - The start index of all substrings. If `start >= 0`, then count from the start of the string, otherwise count from the end of the string.

- `length`(option) - The length of all substrings. If `length` is `None`, then the substring is from `start` to the end of the string.

Attention: Both `start` and `length` are counted by char.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring_by_char.html#performance" class="doc-anchor">§</a>Performance

This function is slower than [substring](https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html "fn arrow::compute::kernels::substring::substring"). Theoretically, the time complexity is `O(n)` where `n` is the length of the value buffer. It is recommended to use [substring](https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html "fn arrow::compute::kernels::substring::substring") if the input array only contains ASCII chars.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring_by_char.html#basic-usage" class="doc-anchor">§</a>Basic usage

``` rust
let array = StringArray::from(vec![Some("arrow"), None, Some("Γ ⊢x:T")]);
let result = substring_by_char(&array, 1, Some(4)).unwrap();
assert_eq!(result, StringArray::from(vec![Some("rrow"), None, Some(" ⊢x:")]));
```
