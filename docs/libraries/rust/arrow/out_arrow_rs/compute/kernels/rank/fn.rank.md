# Function rank Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/rank.rs.html#54" class="src">Source</a>

``` rust
pub fn rank(
    array: &dyn Array,
    options: Option<SortOptions>,
) -> Result<Vec<u32>, ArrowError>
```

Expand description

Assigns a rank to each value in `array` based on its position in the sorted order

Where values are equal, they will be assigned the highest of their ranks, leaving gaps in the overall rank assignment

``` rust
let array = StringArray::from(vec![Some("foo"), None, Some("foo"), None, Some("bar")]);
let ranks = rank(&array, None).unwrap();
assert_eq!(ranks, &[5, 2, 5, 2, 3]);
```
