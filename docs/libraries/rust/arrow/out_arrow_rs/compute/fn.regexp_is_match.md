# Function regexp_is_match Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/regexp.rs.html#69-77" class="src">Source</a>

``` rust
pub fn regexp_is_match<'a, S1, S2, S3>(
    array: &'a S1,
    regex_array: &'a S2,
    flags_array: Option<&'a S3>,
) -> Result<BooleanArray, ArrowError>where
    &'a S1: StringArrayType<'a>,
    &'a S2: StringArrayType<'a>,
    &'a S3: StringArrayType<'a>,
```

Expand description

Return BooleanArray indicating which strings in an array match an array of regular expressions.

This is equivalent to the SQL `array ~ regex_array`, supporting [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") / [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray") / [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray").

If `regex_array` element has an empty value, the corresponding result value is always true.

`flags_array` are optional [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") / [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray") / [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray") flag, which allow special search modes, such as case-insensitive and multi-line mode. See the documentation [here](https://docs.rs/regex/1.5.4/regex/#grouping-and-flags) for more information.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.regexp_is_match.html#see-also" class="doc-anchor">§</a>See Also

- [`regexp_is_match_scalar`](https://docs.rs/arrow/latest/arrow/compute/fn.regexp_is_match_scalar.html "fn arrow::compute::regexp_is_match_scalar") for matching a single regular expression against an array of strings
- [`regexp_match`](https://docs.rs/arrow/latest/arrow/compute/fn.regexp_match.html "fn arrow::compute::regexp_match") for extracting groups from a string array based on a regular expression

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.regexp_is_match.html#example" class="doc-anchor">§</a>Example

``` rust
// First array is the array of strings to match
let array = StringArray::from(vec!["Foo", "Bar", "FooBar", "Baz"]);
// Second array is the array of regular expressions to match against
let regex_array = StringArray::from(vec!["^Foo", "^Foo", "Bar$", "Baz"]);
// Third array is the array of flags to use for each regular expression, if desired
// (the type must be provided to satisfy type inference for the third parameter)
let flags_array: Option<&StringArray> = None;
// The result is a BooleanArray indicating when each string in `array`
// matches the corresponding regular expression in `regex_array`
let result = regexp_is_match(&array, &regex_array, flags_array).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, true, true]));
```
