# Struct ColumnStatistics Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/stats.rs.html#621" class="src">Source</a>

``` rust
pub struct ColumnStatistics {
    pub null_count: Precision<usize>,
    pub max_value: Precision<ScalarValue>,
    pub min_value: Precision<ScalarValue>,
    pub sum_value: Precision<ScalarValue>,
    pub distinct_count: Precision<usize>,
}
```

Expand description

Statistics for a column within a relation

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#structfield.null_count" class="anchor field">§</a>`null_count: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Number of null values on column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#structfield.max_value" class="anchor field">§</a>`max_value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue"><code>ScalarValue</code></a>`>`

Maximum value of column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#structfield.min_value" class="anchor field">§</a>`min_value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue"><code>ScalarValue</code></a>`>`

Minimum value of column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#structfield.sum_value" class="anchor field">§</a>`sum_value: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue"><code>ScalarValue</code></a>`>`

Sum value of a column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#structfield.distinct_count" class="anchor field">§</a>`distinct_count: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Number of distinct values

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.is_singleton" class="fn">is_singleton</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Column contains a single non null value (e.g constant).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.new_unknown" class="fn">new_unknown</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Returns a [`ColumnStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html "struct datafusion::common::ColumnStatistics") instance having all [`Precision::Absent`](https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html#variant.Absent "variant datafusion::common::stats::Precision::Absent") parameters.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.with_null_count" class="fn">with_null_count</a>(self, null_count: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Set the null count

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.with_max_value" class="fn">with_max_value</a>( self, max_value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Set the max value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.with_min_value" class="fn">with_min_value</a>( self, min_value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Set the min value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.with_sum_value" class="fn">with_sum_value</a>( self, sum_value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Set the sum value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.with_distinct_count" class="fn">with_distinct_count</a>( self, distinct_count: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Set the distinct count

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.to_inexact" class="fn">to_inexact</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

If the exactness of a [`ColumnStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html "struct datafusion::common::ColumnStatistics") instance is lost, this function relaxes the exactness of all information by converting them [`Precision::Inexact`](https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html#variant.Inexact "variant datafusion::common::stats::Precision::Inexact").

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-Clone-for-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-Debug-for-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-Default-for-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-PartialEq-for-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-Eq-for-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#impl-StructuralPartialEq-for-ColumnStatistics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html#blanket-implementations" class="anchor">§</a>
