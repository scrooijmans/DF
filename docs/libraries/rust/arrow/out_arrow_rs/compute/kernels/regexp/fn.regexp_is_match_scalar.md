# Function regexp_is_match_scalar Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/regexp.rs.html#175-181" class="src">Source</a>

``` rust
pub fn regexp_is_match_scalar<'a, S>(
    array: &'a S,
    regex: &str,
    flag: Option<&str>,
) -> Result<BooleanArray, ArrowError>where
    &'a S: StringArrayType<'a>,
```

Expand description

Return BooleanArray indicating which strings in an array match a single regular expression.

This is equivalent to the SQL `array ~ regex_array`, supporting [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") / [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray") / [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray") and a scalar.

See the documentation on [`regexp_is_match`](https://docs.rs/arrow/latest/arrow/compute/fn.regexp_is_match.html "fn arrow::compute::regexp_is_match") for more details on arguments

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/regexp/fn.regexp_is_match_scalar.html#see-also" class="doc-anchor">§</a>See Also

- [`regexp_is_match`](https://docs.rs/arrow/latest/arrow/compute/fn.regexp_is_match.html "fn arrow::compute::regexp_is_match") for matching an array of regular expression against an array of strings
- [`regexp_match`](https://docs.rs/arrow/latest/arrow/compute/fn.regexp_match.html "fn arrow::compute::regexp_match") for extracting groups from a string array based on a regular expression

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/regexp/fn.regexp_is_match_scalar.html#example" class="doc-anchor">§</a>Example

``` rust
// array of strings to match
let array = StringArray::from(vec!["Foo", "Bar", "FooBar", "Baz"]);
let regexp = "^Foo"; // regular expression to match against
let flags: Option<&str> = None;  // flags can control the matching behavior
// The result is a BooleanArray indicating when each string in `array`
// matches the regular expression `regexp`
let result = regexp_is_match_scalar(&array, regexp, None).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, true, false]));
```
