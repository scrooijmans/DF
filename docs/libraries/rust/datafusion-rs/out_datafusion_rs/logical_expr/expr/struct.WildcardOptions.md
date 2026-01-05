# Struct WildcardOptions Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr.rs.html#1386" class="src">Source</a>

``` rust
pub struct WildcardOptions {
    pub ilike: Option<IlikeSelectItem>,
    pub exclude: Option<ExcludeSelectItem>,
    pub except: Option<ExceptSelectItem>,
    pub replace: Option<PlannedReplaceSelectItem>,
    pub rename: Option<RenameSelectItem>,
}
```

Expand description

Additional options for wildcards, e.g. Snowflake `EXCLUDE`/`RENAME` and Bigquery `EXCEPT`.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#structfield.ilike" class="anchor field">§</a>`ilike: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.IlikeSelectItem.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::IlikeSelectItem"><code>IlikeSelectItem</code></a>`>`

`[ILIKE...]`. Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#structfield.exclude" class="anchor field">§</a>`exclude: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ExcludeSelectItem.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ExcludeSelectItem"><code>ExcludeSelectItem</code></a>`>`

`[EXCLUDE...]`. Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#structfield.except" class="anchor field">§</a>`except: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExceptSelectItem.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExceptSelectItem"><code>ExceptSelectItem</code></a>`>`

`[EXCEPT...]`. BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_except> Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#except>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#structfield.replace" class="anchor field">§</a>`replace: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.PlannedReplaceSelectItem.html" class="struct" title="struct datafusion::logical_expr::expr::PlannedReplaceSelectItem"><code>PlannedReplaceSelectItem</code></a>`>`

`[REPLACE]` BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_replace> Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#replace> Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#structfield.rename" class="anchor field">§</a>`rename: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.RenameSelectItem.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::RenameSelectItem"><code>RenameSelectItem</code></a>`>`

`[RENAME ...]`. Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-WildcardOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.with_replace" class="fn">with_replace</a>(self, replace: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.PlannedReplaceSelectItem.html" class="struct" title="struct datafusion::logical_expr::expr::PlannedReplaceSelectItem">PlannedReplaceSelectItem</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-Clone-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-Debug-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-Default-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-Display-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-Hash-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-PartialEq-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-PartialOrd-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-Eq-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#impl-StructuralPartialEq-for-WildcardOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions">WildcardOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html#blanket-implementations" class="anchor">§</a>
