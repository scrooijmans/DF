# Enum AcrossPartitions Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/equivalence/class.rs.html#42" class="src">Source</a>

``` rust
pub enum AcrossPartitions {
    Heterogeneous,
    Uniform(Option<ScalarValue>),
}
```

Expand description

Represents whether a constant expression’s value is uniform or varies across partitions. Has two variants:

- `Heterogeneous`: The constant expression may have different values for different partitions.
- `Uniform(Option<ScalarValue>)`: The constant expression has the same value across all partitions, or is `None` if the value is unknown.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#variant.Heterogeneous" class="anchor">§</a>

### Heterogeneous

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#variant.Uniform" class="anchor">§</a>

### Uniform(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>)

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-Clone-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-Debug-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-Default-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-Display-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-PartialEq-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-Eq-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#impl-StructuralPartialEq-for-AcrossPartitions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html" class="enum" title="enum datafusion::physical_expr::AcrossPartitions">AcrossPartitions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.AcrossPartitions.html#blanket-implementations" class="anchor">§</a>
