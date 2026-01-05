# Trait Datum Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/scalar.rs.html#78" class="src">Source</a>

``` rust
pub trait Datum {
    // Required method
    fn get(&self) -> (&dyn Array, bool);
}
```

Expand description

A possibly [`Scalar`](https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html "struct arrow::array::Scalar") [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

This allows optimised binary kernels where one or more arguments are constant

``` rust
fn eq_impl<T: ArrowPrimitiveType>(
    a: &PrimitiveArray<T>,
    a_scalar: bool,
    b: &PrimitiveArray<T>,
    b_scalar: bool,
) -> BooleanArray {
    let (array, scalar) = match (a_scalar, b_scalar) {
        (true, true) | (false, false) => {
            let len = a.len().min(b.len());
            let nulls = NullBuffer::union(a.nulls(), b.nulls());
            let buffer = BooleanBuffer::collect_bool(len, |idx| a.value(idx) == b.value(idx));
            return BooleanArray::new(buffer, nulls);
        }
        (true, false) => (b, (a.null_count() == 0).then(|| a.value(0))),
        (false, true) => (a, (b.null_count() == 0).then(|| b.value(0))),
    };
    match scalar {
        Some(v) => {
            let len = array.len();
            let nulls = array.nulls().cloned();
            let buffer = BooleanBuffer::collect_bool(len, |idx| array.value(idx) == v);
            BooleanArray::new(buffer, nulls)
        }
        None => BooleanArray::new_null(array.len()),
    }
}

pub fn eq(l: &dyn Datum, r: &dyn Datum) -> Result<BooleanArray, ArrowError> {
    let (l_array, l_scalar) = l.get();
    let (r_array, r_scalar) = r.get();
    downcast_primitive_array!(
        (l_array, r_array) => Ok(eq_impl(l_array, l_scalar, r_array, r_scalar)),
        (a, b) => Err(ArrowError::NotYetImplemented(format!("{a} == {b}"))),
    )
}

// Comparison of two arrays
let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
let b = Int32Array::from(vec![1, 2, 4, 7, 3]);
let r = eq(&a, &b).unwrap();
let values: Vec<_> = r.values().iter().collect();
assert_eq!(values, &[true, true, false, false, false]);

// Comparison of an array and a scalar
let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
let b = Int32Array::new_scalar(1);
let r = eq(&a, &b).unwrap();
let values: Vec<_> = r.values().iter().collect();
assert_eq!(values, &[true, false, false, false, false]);
```

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#tymethod.get" class="fn">get</a>(&self) -\> (&dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Returns the value for this [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum") and a boolean indicating if the value is scalar

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#impl-Datum-for-%26dyn+Array" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a> for &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#impl-Datum-for-dyn+Array" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a> for dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#impl-Datum-for-Scalar%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html" class="struct" title="struct arrow::array::Scalar">Scalar</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html#impl-Datum-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a> for T

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>,
