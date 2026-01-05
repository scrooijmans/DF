# Function union_extract Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/union_extract.rs.html#79" class="src">Source</a>

``` rust
pub fn union_extract(
    union_array: &UnionArray,
    target: &str,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Returns the value of the target field when selected, or NULL otherwise.

``` text
┌─────────────────┐                                   ┌─────────────────┐
│       A=1       │                                   │        1        │
├─────────────────┤                                   ├─────────────────┤
│      A=NULL     │                                   │       NULL      │
├─────────────────┤    union_extract(values, 'A')     ├─────────────────┤
│      B='t'      │  ────────────────────────────▶    │       NULL      │
├─────────────────┤                                   ├─────────────────┤
│       A=3       │                                   │        3        │
├─────────────────┤                                   ├─────────────────┤
│      B=NULL     │                                   │       NULL      │
└─────────────────┘                                   └─────────────────┘
   union array                                              result
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/union_extract/fn.union_extract.html#errors" class="doc-anchor">§</a>Errors

Returns error if target field is not found

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/union_extract/fn.union_extract.html#examples" class="doc-anchor">§</a>Examples

``` rust
let fields = UnionFields::new(
    [1, 3],
    [
        Field::new("A", DataType::Int32, true),
        Field::new("B", DataType::Utf8, true)
    ]
);

let union = UnionArray::try_new(
    fields,
    vec![1, 1, 3, 1, 3].into(),
    None,
    vec![
        Arc::new(Int32Array::from(vec![Some(1), None, None, Some(3), Some(0)])),
        Arc::new(StringArray::from(vec![None, None, Some("t"), Some("."), None]))
    ]
).unwrap();

// Extract field A
let extracted = union_extract(&union, "A").unwrap();

assert_eq!(*extracted, Int32Array::from(vec![Some(1), None, None, Some(3), None]));
```
