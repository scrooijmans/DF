# Struct Transformed Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/tree_node.rs.html#657" class="src">Source</a>

``` rust
pub struct Transformed<T> {
    pub data: T,
    pub transformed: bool,
    pub tnr: TreeNodeRecursion,
}
```

Expand description

Result of tree walk / transformation APIs

`Transformed` is a wrapper around the tree node data (e.g. `Expr` or `LogicalPlan`). It is used to indicate whether the node was transformed and how the recursion should proceed.

[`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") API users control the transformation by returning:

- The resulting (possibly transformed) node,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") specifying how to proceed with the recursion.

At the end of the transformation, the return value will contain:

- The final (possibly transformed) tree,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") specifying how the recursion ended.

See also

- [`Transformed::update_data`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.update_data "method datafusion::common::tree_node::Transformed::update_data") to modify the node without changing the `transformed` flag
- [`Transformed::map_data`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.map_data "method datafusion::common::tree_node::Transformed::map_data") for fallable operation that return the same type
- [`Transformed::transform_data`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.transform_data "method datafusion::common::tree_node::Transformed::transform_data") to chain fallable transformations
- [`TransformedResult`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TransformedResult.html "trait datafusion::common::tree_node::TransformedResult") for working with `Result<Transformed<U>>`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#examples" class="doc-anchor">§</a>Examples

Use [`Transformed::yes`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.yes "associated function datafusion::common::tree_node::Transformed::yes") and [`Transformed::no`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.no "associated function datafusion::common::tree_node::Transformed::no") to signal that a node was rewritten and the recursion should continue:

``` rust
let expr = orig_expr();

// Create a new `Transformed` object signaling the node was not rewritten
let ret = Transformed::no(expr.clone());
assert!(!ret.transformed);

// Create a new `Transformed` object signaling the node was rewritten
let ret = Transformed::yes(expr);
assert!(ret.transformed)
```

Access the node within the `Transformed` object:

``` rust
let expr = orig_expr();

// `Transformed` object signaling the node was not rewritten
let ret = Transformed::no(expr.clone());
// Access the inner object using .data
assert_eq!(expr, ret.data);
```

Transform the node within the `Transformed` object.

``` rust
let expr = orig_expr();
let ret = Transformed::no(expr.clone())
  .transform_data(|expr| {
   // closure returns a result and potentially transforms the node
   // in this example, it does transform the node
   let new_expr = make_new_expr(expr);
   Ok(Transformed::yes(new_expr))
 }).unwrap();
// transformed flag is the union of the original ans closure's  transformed flag
assert!(ret.transformed);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#example-apis-that-use-treenode" class="doc-anchor">§</a>Example APIs that use `TreeNode`

- [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode"),
- [`TreeNode::rewrite`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite "method datafusion::common::tree_node::TreeNode::rewrite"),
- [`TreeNode::transform_down`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down "method datafusion::common::tree_node::TreeNode::transform_down"),
- [`TreeNode::transform_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up "method datafusion::common::tree_node::TreeNode::transform_up"),
- [`TreeNode::transform_down_up`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up "method datafusion::common::tree_node::TreeNode::transform_down_up")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#structfield.data" class="anchor field">§</a>`data: T`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#structfield.transformed" class="anchor field">§</a>`transformed: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#structfield.tnr" class="anchor field">§</a>`tnr: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion"><code>TreeNodeRecursion</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#impl-Transformed%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.new" class="fn">new</a>(data: T, transformed: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, tnr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

Create a new `Transformed` object with the given information.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.new_transformed" class="fn">new_transformed</a>(data: T, transformed: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

Create a `Transformed` with `transformed` and [`TreeNodeRecursion::Continue`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Continue "variant datafusion::common::tree_node::TreeNodeRecursion::Continue").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.yes" class="fn">yes</a>(data: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

Wrapper for transformed data with [`TreeNodeRecursion::Continue`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Continue "variant datafusion::common::tree_node::TreeNodeRecursion::Continue") statement.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.complete" class="fn">complete</a>(data: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

Wrapper for transformed data with [`TreeNodeRecursion::Stop`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Stop "variant datafusion::common::tree_node::TreeNodeRecursion::Stop") statement.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.no" class="fn">no</a>(data: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

Wrapper for unchanged data with [`TreeNodeRecursion::Continue`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html#variant.Continue "variant datafusion::common::tree_node::TreeNodeRecursion::Continue") statement.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.update_data" class="fn">update_data</a>\<U, F\>(self, f: F) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<U\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(T) -\> U,

Applies an infallible `f` to the data of this [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") object, without modifying the `transformed` flag.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.map_data" class="fn">map_data</a>\<U, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<U\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies a fallible `f` (returns `Result`) to the data of this [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") object, without modifying the `transformed` flag.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.transform_data" class="fn">transform_data</a>\<U, F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<U\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<U\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies a fallible transforming `f` to the data of this [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") object.

The returned `Transformed` object has the `transformed` flag set if either `self` or the return value of `f` have the `transformed` flag set.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.transform_children" class="fn">transform_children</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps the [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") object to the result of the given `f` depending on the current [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") value and the fact that `f` is changing the current node’s children.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.transform_sibling" class="fn">transform_sibling</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps the [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") object to the result of the given `f` depending on the current [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") value and the fact that `f` is changing the current node’s sibling.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.transform_parent" class="fn">transform_parent</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps the [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") object to the result of the given `f` depending on the current [`TreeNodeRecursion`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html "enum datafusion::common::tree_node::TreeNodeRecursion") value and the fact that `f` is changing the current node’s parent.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#impl-Debug-for-Transformed%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#impl-PartialEq-for-Transformed%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#impl-StructuralPartialEq-for-Transformed%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<T\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html#blanket-implementations" class="anchor">§</a>
