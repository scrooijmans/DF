# Function or_kleene Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#155" class="src">Source</a>

``` rust
pub fn or_kleene(
    left: &BooleanArray,
    right: &BooleanArray,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Logical ‘or’ boolean values with Kleene logic

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.or_kleene.html#behavior" class="doc-anchor">§</a>Behavior

This function behaves as follows with nulls:

- `true` or `null` = `true`
- `null` or `true` = `true`
- `false` or `null` = `null`
- `null` or `false` = `null`
- `null` or `null` = `null`

In other words, in this context a null value really means "unknown", and an unknown value ‘or’ true is always true. For a different null behavior, see function "or".

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.or_kleene.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(true), Some(false), None]);
let b = BooleanArray::from(vec![None, None, None]);
let or_ab = or_kleene(&a, &b).unwrap();
assert_eq!(or_ab, BooleanArray::from(vec![Some(true), None, None]));
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.or_kleene.html#fails" class="doc-anchor">§</a>Fails

If the operands have different lengths
