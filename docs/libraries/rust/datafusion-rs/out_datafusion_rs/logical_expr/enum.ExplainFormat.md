# Enum ExplainFormat Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/plan.rs.html#2977" class="src">Source</a>

``` rust
pub enum ExplainFormat {
    Indent,
    Tree,
    PostgresJSON,
    Graphviz,
}
```

Expand description

Output formats for controlling for Explain plans

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variant.Indent" class="anchor">§</a>

### Indent

Indent mode

Example:

``` text
> explain format indent select x from values (1) t(x);
+---------------+-----------------------------------------------------+
| plan_type     | plan                                                |
+---------------+-----------------------------------------------------+
| logical_plan  | SubqueryAlias: t                                    |
|               |   Projection: column1 AS x                          |
|               |     Values: (Int64(1))                              |
| physical_plan | ProjectionExec: expr=[column1@0 as x]               |
|               |   DataSourceExec: partitions=1, partition_sizes=[1] |
|               |                                                     |
+---------------+-----------------------------------------------------+
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variant.Tree" class="anchor">§</a>

### Tree

Tree mode

Example:

``` text
> explain format tree select x from values (1) t(x);
+---------------+-------------------------------+
| plan_type     | plan                          |
+---------------+-------------------------------+
| physical_plan | ┌───────────────────────────┐ |
|               | │       ProjectionExec      │ |
|               | │    --------------------   │ |
|               | │        x: column1@0       │ |
|               | └─────────────┬─────────────┘ |
|               | ┌─────────────┴─────────────┐ |
|               | │       DataSourceExec      │ |
|               | │    --------------------   │ |
|               | │         bytes: 128        │ |
|               | │       format: memory      │ |
|               | │          rows: 1          │ |
|               | └───────────────────────────┘ |
|               |                               |
+---------------+-------------------------------+
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variant.PostgresJSON" class="anchor">§</a>

### PostgresJSON

Postgres Json mode

A displayable structure that produces plan in postgresql JSON format.

Users can use this format to visualize the plan in existing plan visualization tools, for example [dalibo](https://explain.dalibo.com/)

Example:

``` text
> explain format pgjson select x from values (1) t(x);
+--------------+--------------------------------------+
| plan_type    | plan                                 |
+--------------+--------------------------------------+
| logical_plan | [                                    |
|              |   {                                  |
|              |     "Plan": {                        |
|              |       "Alias": "t",                  |
|              |       "Node Type": "Subquery",       |
|              |       "Output": [                    |
|              |         "x"                          |
|              |       ],                             |
|              |       "Plans": [                     |
|              |         {                            |
|              |           "Expressions": [           |
|              |             "column1 AS x"           |
|              |           ],                         |
|              |           "Node Type": "Projection", |
|              |           "Output": [                |
|              |             "x"                      |
|              |           ],                         |
|              |           "Plans": [                 |
|              |             {                        |
|              |               "Node Type": "Values", |
|              |               "Output": [            |
|              |                 "column1"            |
|              |               ],                     |
|              |               "Plans": [],           |
|              |               "Values": "(Int64(1))" |
|              |             }                        |
|              |           ]                          |
|              |         }                            |
|              |       ]                              |
|              |     }                                |
|              |   }                                  |
|              | ]                                    |
+--------------+--------------------------------------+
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variant.Graphviz" class="anchor">§</a>

### Graphviz

Graphviz mode

Example:

``` text
> explain format graphviz select x from values (1) t(x);
+--------------+------------------------------------------------------------------------+
| plan_type    | plan                                                                   |
+--------------+------------------------------------------------------------------------+
| logical_plan |                                                                        |
|              | // Begin DataFusion GraphViz Plan,                                     |
|              | // display it online here: https://dreampuf.github.io/GraphvizOnline   |
|              |                                                                        |
|              | digraph {                                                              |
|              |   subgraph cluster_1                                                   |
|              |   {                                                                    |
|              |     graph[label="LogicalPlan"]                                         |
|              |     2[shape=box label="SubqueryAlias: t"]                              |
|              |     3[shape=box label="Projection: column1 AS x"]                      |
|              |     2 -> 3 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |     4[shape=box label="Values: (Int64(1))"]                            |
|              |     3 -> 4 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |   }                                                                    |
|              |   subgraph cluster_5                                                   |
|              |   {                                                                    |
|              |     graph[label="Detailed LogicalPlan"]                                |
|              |     6[shape=box label="SubqueryAlias: t\nSchema: [x:Int64;N]"]         |
|              |     7[shape=box label="Projection: column1 AS x\nSchema: [x:Int64;N]"] |
|              |     6 -> 7 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |     8[shape=box label="Values: (Int64(1))\nSchema: [column1:Int64;N]"] |
|              |     7 -> 8 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |   }                                                                    |
|              | }                                                                      |
|              | // End DataFusion GraphViz Plan                                        |
|              |                                                                        |
+--------------+------------------------------------------------------------------------+
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-Clone-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-Debug-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-FromStr-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

Implement parsing strings to `ExplainFormat`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>( format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-Hash-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-PartialEq-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-Eq-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#impl-StructuralPartialEq-for-ExplainFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html" class="enum" title="enum datafusion::logical_expr::ExplainFormat">ExplainFormat</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#blanket-implementations" class="anchor">§</a>
