# Enum DataTypeSelector Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/selector.rs.html#113" class="src">Source</a>

``` rust
pub enum DataTypeSelector {
Show 23 variants    Union(Arc<DataTypeSelector>, Arc<DataTypeSelector>),
    Difference(Arc<DataTypeSelector>, Arc<DataTypeSelector>),
    ExclusiveOr(Arc<DataTypeSelector>, Arc<DataTypeSelector>),
    Intersect(Arc<DataTypeSelector>, Arc<DataTypeSelector>),
    Wildcard,
    Empty,
    AnyOf(Arc<[DataType]>),
    Integer,
    UnsignedInteger,
    SignedInteger,
    Float,
    Enum,
    Categorical,
    Nested,
    List(Option<Arc<DataTypeSelector>>),
    Array(Option<Arc<DataTypeSelector>>, Option<usize>),
    Struct,
    Decimal,
    Numeric,
    Temporal,
    Datetime(TimeUnitSet, TimeZoneSet),
    Duration(TimeUnitSet),
    Object,
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Union" class="anchor">§</a>

### Union(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Difference" class="anchor">§</a>

### Difference(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.ExclusiveOr" class="anchor">§</a>

### ExclusiveOr(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Intersect" class="anchor">§</a>

### Intersect(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Wildcard" class="anchor">§</a>

### Wildcard

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Empty" class="anchor">§</a>

### Empty

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.AnyOf" class="anchor">§</a>

### AnyOf(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\]\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Integer" class="anchor">§</a>

### Integer

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.UnsignedInteger" class="anchor">§</a>

### UnsignedInteger

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.SignedInteger" class="anchor">§</a>

### SignedInteger

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Float" class="anchor">§</a>

### Float

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Enum" class="anchor">§</a>

### Enum

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Categorical" class="anchor">§</a>

### Categorical

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Nested" class="anchor">§</a>

### Nested

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Array" class="anchor">§</a>

### Array(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>\>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Struct" class="anchor">§</a>

### Struct

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Decimal" class="anchor">§</a>

### Decimal

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Numeric" class="anchor">§</a>

### Numeric

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Temporal" class="anchor">§</a>

### Temporal

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Datetime" class="anchor">§</a>

### Datetime(<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeZoneSet.html" class="enum" title="enum polars::prelude::TimeZoneSet">TimeZoneSet</a>)

Selector for `DataType::Datetime` with optional matching on TimeUnit and TimeZone.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Duration" class="anchor">§</a>

### Duration(<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

Selector for `DataType::Duration` with optional matching on TimeUnit.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#variant.Object" class="anchor">§</a>

### Object

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.matches" class="fn">matches</a>(&self, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.as_selector" class="fn">as_selector</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-BitAnd-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-BitAndAssign-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html" class="trait" title="trait core::ops::bit::BitAndAssign">BitAndAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.bitand_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign" class="fn">bitand_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>)

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-BitOr-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-BitOrAssign-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html" class="trait" title="trait core::ops::bit::BitOrAssign">BitOrAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.bitor_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign" class="fn">bitor_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>)

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-BitXor-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-BitXorAssign-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html" class="trait" title="trait core::ops::bit::BitXorAssign">BitXorAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.bitxor_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign" class="fn">bitxor_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>)

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Clone-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Debug-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Deserialize%3C&#39;de%3E-for-DataTypeSelector" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Display-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Hash-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Not-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.not" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::Not::Output">Output</a>

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-PartialEq-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Serialize-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Sub-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-SubAssign-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html" class="trait" title="trait core::ops::arith::SubAssign">SubAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#method.sub_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign" class="fn">sub_assign</a>(&mut self, rhs: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>)

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-Eq-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#impl-StructuralPartialEq-for-DataTypeSelector" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html" class="enum" title="enum polars::prelude::DataTypeSelector">DataTypeSelector</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeSelector.html#blanket-implementations" class="anchor">§</a>
