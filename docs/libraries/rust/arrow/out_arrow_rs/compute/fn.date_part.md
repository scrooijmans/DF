# Function date_part Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/temporal.rs.html#146" class="src">Source</a>

``` rust
pub fn date_part(
    array: &dyn Array,
    part: DatePart,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Given an array, return a new array with the extracted [`DatePart`](https://docs.rs/arrow/latest/arrow/compute/enum.DatePart.html "enum arrow::compute::DatePart") as signed 32-bit integer values.

Currently only supports temporal types:

- Date32/Date64
- Time32/Time64
- Timestamp
- Interval
- Duration

Returns an [`Int32Array`](https://docs.rs/arrow/latest/arrow/array/type.Int32Array.html "type arrow::array::Int32Array") unless input was a dictionary type, in which case returns the dictionary but with this function applied onto its values.

If array passed in is not of the above listed types (or is a dictionary array where the values array isn’t of the above listed types), then this function will return an error.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.date_part.html#examples" class="doc-anchor">§</a>Examples

``` rust
let input: TimestampMicrosecondArray =
    vec![Some(1612025847000000), None, Some(1722015847000000)].into();

let week = date_part(&input, DatePart::Week).unwrap();
let week_iso = date_part(&input, DatePart::WeekISO).unwrap();
let expected: Int32Array = vec![Some(4), None, Some(30)].into();
assert_eq!(week.as_ref(), &expected);
assert_eq!(week_iso.as_ref(), &expected);
let year_iso = date_part(&input, DatePart::YearISO).unwrap();
let expected: Int32Array = vec![Some(2021), None, Some(2024)].into();
assert_eq!(year_iso.as_ref(), &expected);
```
