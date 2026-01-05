# Struct Scalar Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/scalar.rs.html#128" class="src">Source</a>

``` rust
pub struct Scalar<T>(/* private fields */)
where
    T: Array;
```

Expand description

A wrapper around a single value [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") that implements [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") and indicates [compute](https://docs.rs/arrow/latest/arrow/compute/index.html) kernels should treat this array as a scalar value (a single value).

Using a [`Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html "struct datafusion::common::arrow::array::Scalar") is often much more efficient than creating an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") with the same (repeated) value.

See [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") for more information.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#example" class="doc-anchor">§</a>Example

``` rust
// Create a (typed) scalar for Int32Array for the value 42
let scalar = Scalar::new(Int32Array::from(vec![42]));

// Create a scalar using PrimtiveArray::scalar
let scalar = Int32Array::new_scalar(42);

// create a scalar from an ArrayRef (for dynamic typed Arrays)
let array: ArrayRef = get_array();
let scalar = Scalar::new(array);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#impl-Scalar%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#method.new" class="fn">new</a>(array: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

Create a new [`Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html "struct datafusion::common::arrow::array::Scalar") from an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array")

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#panics" class="doc-anchor">§</a>Panics

Panics if `array.len() != 1`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#method.into_inner" class="fn">into_inner</a>(self) -\> T

Returns the inner array

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#impl-Clone-for-Scalar%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#impl-Datum-for-Scalar%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html" class="trait" title="trait datafusion::common::arrow::array::Datum">Datum</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html#tymethod.get" class="fn">get</a>(&self) -\> (&dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Returns the value for this [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") and a boolean indicating if the value is scalar

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#impl-Debug-for-Scalar%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#impl-Copy-for-Scalar%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>,

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html#blanket-implementations" class="anchor">§</a>
