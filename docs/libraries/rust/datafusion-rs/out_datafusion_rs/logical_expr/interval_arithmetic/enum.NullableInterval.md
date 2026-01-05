# Enum NullableInterval Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/interval_arithmetic.rs.html#1694" class="src">Source</a>

``` rust
pub enum NullableInterval {
    Null {
        datatype: DataType,
    },
    MaybeNull {
        values: Interval,
    },
    NotNull {
        values: Interval,
    },
}
```

Expand description

An [Interval](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html "struct datafusion::logical_expr::interval_arithmetic::Interval") that also tracks null status using a boolean interval.

This represents values that may be in a particular range or be null.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#examples" class="doc-anchor">§</a>Examples

``` rust
use arrow::datatypes::DataType;
use datafusion_common::ScalarValue;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;

// [1, 2) U {NULL}
let maybe_null = NullableInterval::MaybeNull {
   values: Interval::try_new(
           ScalarValue::Int32(Some(1)),
           ScalarValue::Int32(Some(2)),
       ).unwrap(),
};

// (0, ∞)
let not_null = NullableInterval::NotNull {
  values: Interval::try_new(
           ScalarValue::Int32(Some(0)),
           ScalarValue::Int32(None),
       ).unwrap(),
};

// {NULL}
let null_interval = NullableInterval::Null { datatype: DataType::Int32 };

// {4}
let single_value = NullableInterval::from(ScalarValue::Int32(Some(4)));
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variant.Null" class="anchor">§</a>

### Null

The value is always null. This is typed so it can be used in physical expressions, which don’t do type coercion.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variant.Null.field.datatype" class="anchor field">§</a>`datatype: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variant.MaybeNull" class="anchor">§</a>

### MaybeNull

The value may or may not be null. If it is non-null, its is within the specified range.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variant.MaybeNull.field.values" class="anchor field">§</a>`values: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval"><code>Interval</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variant.NotNull" class="anchor">§</a>

### NotNull

The value is definitely not null, and is within the specified range.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#variant.NotNull.field.values" class="anchor field">§</a>`values: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval"><code>Interval</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-NullableInterval" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.values" class="fn">values</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\>

Get the values interval, or None if this interval is definitely null.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.data_type" class="fn">data_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Get the data type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.is_certainly_true" class="fn">is_certainly_true</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the value is definitely true (and not null).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.is_certainly_false" class="fn">is_certainly_false</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the value is definitely false (and not null).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.apply_operator" class="fn">apply_operator</a>( &self, op: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Operator.html" class="enum" title="enum datafusion::logical_expr::Operator">Operator</a>, rhs: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Apply the given operator to this interval and the given interval.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
use datafusion_common::ScalarValue;
use datafusion_expr_common::operator::Operator;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;

// 4 > 3 -> true
let lhs = NullableInterval::from(ScalarValue::Int32(Some(4)));
let rhs = NullableInterval::from(ScalarValue::Int32(Some(3)));
let result = lhs.apply_operator(&Operator::Gt, &rhs).unwrap();
assert_eq!(result, NullableInterval::from(ScalarValue::Boolean(Some(true))));

// [1, 3) > NULL -> NULL
let lhs = NullableInterval::NotNull {
    values: Interval::try_new(
           ScalarValue::Int32(Some(1)),
           ScalarValue::Int32(Some(3)),
       ).unwrap(),
};
let rhs = NullableInterval::from(ScalarValue::Int32(None));
let result = lhs.apply_operator(&Operator::Gt, &rhs).unwrap();
assert_eq!(result.single_value(), Some(ScalarValue::Boolean(None)));

// [1, 3] > [2, 4] -> [false, true]
let lhs = NullableInterval::NotNull {
    values: Interval::try_new(
           ScalarValue::Int32(Some(1)),
           ScalarValue::Int32(Some(3)),
       ).unwrap(),
};
let rhs = NullableInterval::NotNull {
   values: Interval::try_new(
           ScalarValue::Int32(Some(2)),
           ScalarValue::Int32(Some(4)),
       ).unwrap(),
};
let result = lhs.apply_operator(&Operator::Gt, &rhs).unwrap();
// Both inputs are valid (non-null), so result must be non-null
assert_eq!(result, NullableInterval::NotNull {
// Uncertain whether inequality is true or false
   values: Interval::UNCERTAIN,
});
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.contains" class="fn">contains</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>\>,

Decide if this interval is a superset of, overlaps with, or disjoint with `other` by returning `[true, true]`, `[false, true]` or `[false, false]` respectively.

NOTE: This function only works with intervals of the same data type. Attempting to compare intervals of different data types will lead to an error.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.single_value" class="fn">single_value</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

If the interval has collapsed to a single value, return that value. Otherwise, returns `None`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
use datafusion_common::ScalarValue;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;

let interval = NullableInterval::from(ScalarValue::Int32(Some(4)));
assert_eq!(interval.single_value(), Some(ScalarValue::Int32(Some(4))));

let interval = NullableInterval::from(ScalarValue::Int32(None));
assert_eq!(interval.single_value(), Some(ScalarValue::Int32(None)));

let interval = NullableInterval::MaybeNull {
    values: Interval::try_new(
        ScalarValue::Int32(Some(1)),
        ScalarValue::Int32(Some(4)),
    ).unwrap(),
};
assert_eq!(interval.single_value(), None);
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-Clone-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-Debug-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-Display-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-From%3CScalarValue%3E-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

Create an interval that represents a single value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-PartialEq-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-Eq-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#impl-StructuralPartialEq-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html#blanket-implementations" class="anchor">§</a>
