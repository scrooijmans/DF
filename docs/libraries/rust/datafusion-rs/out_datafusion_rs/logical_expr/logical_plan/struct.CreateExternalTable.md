# Struct CreateExternalTable Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/ddl.rs.html#203" class="src">Source</a>

``` rust
pub struct CreateExternalTable {Show 13 fields
    pub schema: Arc<DFSchema>,
    pub name: TableReference,
    pub location: String,
    pub file_type: String,
    pub table_partition_cols: Vec<String>,
    pub if_not_exists: bool,
    pub temporary: bool,
    pub definition: Option<String>,
    pub order_exprs: Vec<Vec<Sort>>,
    pub unbounded: bool,
    pub options: HashMap<String, String>,
    pub constraints: Constraints,
    pub column_defaults: HashMap<String, Expr>,
}
```

Expand description

Creates an external table.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema"><code>DFSchema</code></a>`>`

The table schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference"><code>TableReference</code></a>

The table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The physical location

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.file_type" class="anchor field">§</a>`file_type: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The file type of physical file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Partition Columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Option to not error if table already exists

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the table is a temporary table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.definition" class="anchor field">§</a>`definition: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

SQL used to create the table, if available

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.order_exprs" class="anchor field">§</a>`order_exprs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr"><code>Sort</code></a>`>>`

Order expressions supplied by user

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.unbounded" class="anchor field">§</a>`unbounded: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the table is an infinite streams

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Table(provider) specific options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.constraints" class="anchor field">§</a>`constraints: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints"><code>Constraints</code></a>

The list of constraints in the schema, such as primary key, unique, etc.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#structfield.column_defaults" class="anchor field">§</a>`column_defaults: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr"><code>Expr</code></a>`>`

Default values for columns

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-Clone-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-Debug-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-Hash-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-PartialEq-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-PartialOrd-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-Eq-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#impl-StructuralPartialEq-for-CreateExternalTable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateExternalTable.html" class="struct" title="struct datafusion::logical_expr::CreateExternalTable">CreateExternalTable</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/struct.CreateExternalTable.html#blanket-implementations" class="anchor">§</a>
