# Enum Selector Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/selector.rs.html#150" class="src">Source</a>

``` rust
pub enum Selector {
    Union(Arc<Selector>, Arc<Selector>),
    Difference(Arc<Selector>, Arc<Selector>),
    ExclusiveOr(Arc<Selector>, Arc<Selector>),
    Intersect(Arc<Selector>, Arc<Selector>),
    ByName {
        names: Arc<[PlSmallStr]>,
        strict: bool,
    },
    ByIndex {
        indices: Arc<[i64]>,
        strict: bool,
    },
    Matches(PlSmallStr),
    ByDType(DataTypeSelector),
    Wildcard,
    Empty,
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.Union" class="anchor">§</a>

### Union(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.Difference" class="anchor">§</a>

### Difference(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ExclusiveOr" class="anchor">§</a>

### ExclusiveOr(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.Intersect" class="anchor">§</a>

### Intersect(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByName" class="anchor">§</a>

### ByName

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByName.field.names" class="anchor field">§</a>`names: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<[`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`]>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByName.field.strict" class="anchor field">§</a>`strict: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByIndex" class="anchor">§</a>

### ByIndex

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByIndex.field.indices" class="anchor field">§</a>`indices: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<[`<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>`]>`

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByIndex.field.strict" class="anchor field">§</a>`strict: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.Matches" class="anchor">§</a>

### Matches(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.ByDType" class="anchor">§</a>

### ByDType(<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.Wildcard" class="anchor">§</a>

### Wildcard

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#variant.Empty" class="anchor">§</a>

### Empty

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Selector" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.into_columns" class="fn">into_columns</a>( &self, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ignored_columns: &<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Turns the selector into an ordered set of selected columns from the schema.

- The order of the columns corresponds to the order in the schema.
- Column names in `ignored_columns` are only used if they are explicitly mentioned by a `ByName` or `Nth`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.as_expr" class="fn">as_expr</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.to_dtype_selector" class="fn">to_dtype_selector</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.exclude_cols" class="fn">exclude_cols</a>(self, columns: impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

Exclude a column from a wildcard/regex selection.

You may also use regexes in the exclude as long as they start with `^` and end with `$`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.exclude_dtype" class="fn">exclude_dtype</a>\<D\>(self, dtypes: D) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

where D: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\]\>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-BitAnd-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-BitAndAssign-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html" class="trait" title="trait core::ops::bit::BitAndAssign">BitAndAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.bitand_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign" class="fn">bitand_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>)

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-BitOr-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-BitOrAssign-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html" class="trait" title="trait core::ops::bit::BitOrAssign">BitOrAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.bitor_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign" class="fn">bitor_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>)

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-BitXor-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-BitXorAssign-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html" class="trait" title="trait core::ops::bit::BitXorAssign">BitXorAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.bitxor_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign" class="fn">bitxor_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>)

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Clone-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Debug-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Deserialize%3C&#39;de%3E-for-Selector" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Display-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-From%3CSelector%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Hash-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Not-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.not" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::Not::Output">Output</a>

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-PartialEq-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Serialize-for-Selector" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Sub-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-SubAssign-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html" class="trait" title="trait core::ops::arith::SubAssign">SubAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#method.sub_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign" class="fn">sub_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>)

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-Eq-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#impl-StructuralPartialEq-for-Selector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html#blanket-implementations" class="anchor">§</a>
