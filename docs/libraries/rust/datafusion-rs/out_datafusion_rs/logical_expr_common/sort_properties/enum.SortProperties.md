# Enum SortProperties Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/sort_properties.rs.html#37" class="src">Source</a>

``` rust
pub enum SortProperties {
    Ordered(SortOptions),
    Unordered,
    Singleton,
}
```

Expand description

To propagate [`SortOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html "struct datafusion::common::arrow::compute::SortOptions") across the `PhysicalExpr`, it is insufficient to simply use `Option<SortOptions>`: There must be a differentiation between unordered columns and literal values, since literals may not break the ordering when they are used as a child of some binary expression when the other child has some ordering. On the other hand, unordered columns cannot maintain ordering when they take part in such operations.

Example: ((a_ordered + b_unordered) + c_ordered) expression cannot end up with sorted data; however the ((a_ordered + 999) + c_ordered) expression can. Therefore, we need two different variants for literals and unordered columns as literals are often more ordering-friendly under most mathematical operations.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#variant.Ordered" class="anchor">§</a>

### Ordered(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>)

Use the ordinary [`SortOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html "struct datafusion::common::arrow::compute::SortOptions") struct to represent ordered data:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#variant.Unordered" class="anchor">§</a>

### Unordered

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#variant.Singleton" class="anchor">§</a>

### Singleton

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-SortProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.add" class="fn">add</a>(&self, rhs: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.sub" class="fn">sub</a>(&self, rhs: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.gt_or_gteq" class="fn">gt_or_gteq</a>(&self, rhs: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.and_or" class="fn">and_or</a>(&self, rhs: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-Clone-for-SortProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-Debug-for-SortProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-Default-for-SortProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-Neg-for-SortProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.neg" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg" class="fn">neg</a>(self) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

Performs the unary `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-PartialEq-for-SortProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-Copy-for-SortProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#impl-StructuralPartialEq-for-SortProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/sort_properties/enum.SortProperties.html#blanket-implementations" class="anchor">§</a>
