# Function contains Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/like.rs.html#186" class="src">Source</a>

``` rust
pub fn contains(
    left: &dyn Datum,
    right: &dyn Datum,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Perform SQL `CONTAINS(left, right)`

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.contains.html#supported-datatypes" class="doc-anchor">§</a>Supported DataTypes

`left` and `right` must be the same type, and one of

- Utf8
- LargeUtf8
- Utf8View
- Binary
- LargeBinary
- BinaryView

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.contains.html#example" class="doc-anchor">§</a>Example

``` rust
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs", "arrow-rs", "Parquet"]);
let patterns = StringArray::from(vec!["arr", "-rs", "arrow-cpp", "X"]);

let result = contains(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, false, false]));
```
