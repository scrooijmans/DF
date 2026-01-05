# Function and_kleene Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/boolean.rs.html#60" class="src">Source</a>

``` rust
pub fn and_kleene(
    left: &BooleanArray,
    right: &BooleanArray,
) -> Result<BooleanArray, ArrowError>
```

Expand description

Logical ‘and’ boolean values with Kleene logic

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and_kleene.html#behavior" class="doc-anchor">§</a>Behavior

This function behaves as follows with nulls:

- `true` and `null` = `null`
- `null` and `true` = `null`
- `false` and `null` = `false`
- `null` and `false` = `false`
- `null` and `null` = `null`

In other words, in this context a null value really means "unknown", and an unknown value ‘and’ false is always false. For a different null behavior, see function "and".

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and_kleene.html#example" class="doc-anchor">§</a>Example

``` rust
let a = BooleanArray::from(vec![Some(true), Some(false), None]);
let b = BooleanArray::from(vec![None, None, None]);
let and_ab = and_kleene(&a, &b).unwrap();
assert_eq!(and_ab, BooleanArray::from(vec![None, Some(false), None]));
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/fn.and_kleene.html#fails" class="doc-anchor">§</a>Fails

If the operands have different lengths
