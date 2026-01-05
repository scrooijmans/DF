# Enum Statement Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/statement.rs.html#33" class="src">Source</a>

``` rust
pub enum Statement {
    TransactionStart(TransactionStart),
    TransactionEnd(TransactionEnd),
    SetVariable(SetVariable),
    Prepare(Prepare),
    Execute(Execute),
    Deallocate(Deallocate),
}
```

Expand description

Various types of Statements.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#transactions" class="doc-anchor">§</a>Transactions:

While DataFusion does not offer support transactions, it provides [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") support to assist building database systems using DataFusion

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variant.TransactionStart" class="anchor">§</a>

### TransactionStart(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.TransactionStart.html" class="struct" title="struct datafusion::logical_expr::TransactionStart">TransactionStart</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variant.TransactionEnd" class="anchor">§</a>

### TransactionEnd(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.TransactionEnd.html" class="struct" title="struct datafusion::logical_expr::TransactionEnd">TransactionEnd</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variant.SetVariable" class="anchor">§</a>

### SetVariable(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SetVariable.html" class="struct" title="struct datafusion::logical_expr::SetVariable">SetVariable</a>)

Set a Variable

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variant.Prepare" class="anchor">§</a>

### Prepare(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Prepare.html" class="struct" title="struct datafusion::logical_expr::Prepare">Prepare</a>)

Prepare a statement and find any bind parameters (e.g. `?`). This is used to implement SQL-prepared statements.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variant.Execute" class="anchor">§</a>

### Execute(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Execute.html" class="struct" title="struct datafusion::logical_expr::Execute">Execute</a>)

Execute a prepared statement. This is used to implement SQL ‘EXECUTE’.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#variant.Deallocate" class="anchor">§</a>

### Deallocate(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Deallocate.html" class="struct" title="struct datafusion::logical_expr::Deallocate">Deallocate</a>)

Deallocate a prepared statement. This is used to implement SQL ‘DEALLOCATE’.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-Statement" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>

Get a reference to the logical plan’s schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return a descriptive string describing the type of this [`Statement`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html "enum datafusion::logical_expr::Statement")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.display" class="fn">display</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>

Return a `format`able structure with the a human readable description of this LogicalPlan node per node, not including children.

See [crate::LogicalPlan::display](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.display "method datafusion::logical_expr::LogicalPlan::display") for an example

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-Clone-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-Debug-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-Hash-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-PartialEq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-PartialOrd-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-Eq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#impl-StructuralPartialEq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::Statement">Statement</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Statement.html#blanket-implementations" class="anchor">§</a>
