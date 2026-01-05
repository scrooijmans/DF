# Struct ExprProperties Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/sort_properties.rs.html#135" class="src">Source</a>

``` rust
pub struct ExprProperties {
    pub sort_properties: SortProperties,
    pub range: Interval,
    pub preserves_lex_ordering: bool,
}
```

Expand description

Represents the properties of a `PhysicalExpr`, including its sorting, range, and whether it preserves lexicographical ordering.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#structfield.sort_properties" class="anchor field">§</a>`sort_properties: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties"><code>SortProperties</code></a>

Properties that describe the sorting behavior of the expression, such as whether it is ordered, unordered, or a singleton value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#structfield.range" class="anchor field">§</a>`range: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval"><code>Interval</code></a>

A closed interval representing the range of possible values for the expression. Used to compute reliable bounds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#structfield.preserves_lex_ordering" class="anchor field">§</a>`preserves_lex_ordering: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Indicates whether the expression preserves lexicographical ordering of its inputs. For example, string concatenation preserves ordering, while addition does not.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#impl-ExprProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.new_unknown" class="fn">new_unknown</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

Creates a new `ExprProperties` instance with unknown sort properties, unknown range, and unknown lexicographical ordering preservation.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.with_order" class="fn">with_order</a>(self, order: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

Sets the sorting properties of the expression and returns the modified instance.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.with_range" class="fn">with_range</a>(self, range: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

Sets the range of the expression and returns the modified instance.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.with_preserves_lex_ordering" class="fn">with_preserves_lex_ordering</a>( self, preserves_lex_ordering: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

Sets whether the expression maintains lexicographical ordering and returns the modified instance.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#impl-Clone-for-ExprProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#impl-Debug-for-ExprProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html#blanket-implementations" class="anchor">§</a>
