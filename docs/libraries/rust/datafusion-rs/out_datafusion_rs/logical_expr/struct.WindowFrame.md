# Struct WindowFrame Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/window_frame.rs.html#40" class="src">Source</a>

``` rust
pub struct WindowFrame {
    pub units: WindowFrameUnits,
    pub start_bound: WindowFrameBound,
    pub end_bound: WindowFrameBound,
    /* private fields */
}
```

Expand description

The frame specification determines which output rows are read by an aggregate window function. The ending frame boundary can be omitted if the `BETWEEN` and `AND` keywords that surround the starting frame boundary are also omitted, in which case the ending frame boundary defaults to `CURRENT ROW`.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#structfield.units" class="anchor field">§</a>`units: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.WindowFrameUnits.html" class="enum" title="enum datafusion::logical_expr::WindowFrameUnits"><code>WindowFrameUnits</code></a>

Frame type - either `ROWS`, `RANGE` or `GROUPS`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#structfield.start_bound" class="anchor field">§</a>`start_bound: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::WindowFrameBound"><code>WindowFrameBound</code></a>

Starting frame boundary

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#structfield.end_bound" class="anchor field">§</a>`end_bound: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::WindowFrameBound"><code>WindowFrameBound</code></a>

Ending frame boundary

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-WindowFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.new" class="fn">new</a>(order_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

Creates a new, default window frame (with the meaning of default depending on whether the frame contains an `ORDER BY` clause and this ordering is strict (i.e. no ties).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.reverse" class="fn">reverse</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

Get reversed window frame. For example `3 ROWS PRECEDING AND 2 ROWS FOLLOWING` –\> `2 ROWS PRECEDING AND 3 ROWS FOLLOWING`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.is_causal" class="fn">is_causal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get whether window frame is causal

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.new_bounds" class="fn">new_bounds</a>( units: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.WindowFrameUnits.html" class="enum" title="enum datafusion::logical_expr::WindowFrameUnits">WindowFrameUnits</a>, start_bound: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::WindowFrameBound">WindowFrameBound</a>, end_bound: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::WindowFrameBound">WindowFrameBound</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

Initializes window frame from units (type), start bound and end bound.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.regularize_order_bys" class="fn">regularize_order_bys</a>( &self, order_by: &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Regularizes the ORDER BY clause of the window frame.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.can_accept_multi_orderby" class="fn">can_accept_multi_orderby</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the window frame can accept multiple ORDER BY expressions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.is_ever_expanding" class="fn">is_ever_expanding</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Is the window frame ever-expanding (it always grows in the superset sense). Useful when understanding if set-monotonicity properties of functions can be exploited.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-Clone-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-Debug-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-Display-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-Hash-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-PartialEq-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-PartialOrd-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-TryFrom%3CWindowFrame%3E-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::WindowFrame">WindowFrame</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::WindowFrame">WindowFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-Eq-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#impl-StructuralPartialEq-for-WindowFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html#blanket-implementations" class="anchor">§</a>
