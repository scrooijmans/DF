# Function like Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/like.rs.html#79" class="src">Source</a>

``` rust
pub fn like(
    left: &dyn Datum,
    right: &dyn Datum,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Perform SQL `left LIKE right`

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.like.html#supported-datatypes" class="doc-anchor">§</a>Supported DataTypes

`left` and `right` must be the same type, and one of

- Utf8
- LargeUtf8
- Utf8View

There are two wildcards supported with the LIKE operator:

1.  `%` - The percent sign represents zero, one, or multiple characters
2.  `_` - The underscore represents a single character

Example

``` rust
let strings = StringArray::from(vec!["Arrow", "Arrow", "Arrow", "Ar"]);
let patterns = StringArray::from(vec!["A%", "B%", "A.", "A_"]);

let result = like(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, false, true]));
```
