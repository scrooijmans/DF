# Function regexp_match Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/regexp.rs.html#423-427" class="src">Source</a>

``` rust
pub fn regexp_match(
    array: &dyn Array,
    regex_array: &dyn Datum,
    flags_array: Option<&dyn Datum>,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Extract all groups matched by a regular expression for a given String array.

Modelled after the Postgres [regexp_match](https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-POSIX-REGEXP).

Returns a ListArray of [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") with each element containing the leftmost-first match of the corresponding index in `regex_array` to string in `array`

If there is no match, the list element is NULL.

If a match is found, and the pattern contains no capturing parenthesized subexpressions, then the list element is a single-element [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") containing the substring matching the whole pattern.

If a match is found, and the pattern contains capturing parenthesized subexpressions, then the list element is a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") whose n’th element is the substring matching the n’th capturing parenthesized subexpression of the pattern.

The flags parameter is an optional text string containing zero or more single-letter flags that change the function’s behavior.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/regexp/fn.regexp_match.html#see-also" class="doc-anchor">§</a>See Also

- [`regexp_is_match`](https://docs.rs/arrow/latest/arrow/compute/fn.regexp_is_match.html "fn arrow::compute::regexp_is_match") for matching (rather than extracting) a regular expression against an array of strings
