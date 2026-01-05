# Struct TimeUnitSet Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/selector.rs.html#52-64" class="src">Source</a>

``` rust
pub struct TimeUnitSet(/* private fields */);
```

Available on **crate feature `lazy`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedconstant.NANO_SECONDS" class="constant">NANO_SECONDS</a>: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedconstant.MICRO_SECONDS" class="constant">MICRO_SECONDS</a>: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedconstant.MILLI_SECONDS" class="constant">MILLI_SECONDS</a>: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-TimeUnitSet-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Get a flags value with all bits unset.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.all" class="fn">all</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Get a flags value with all known bits set.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bits" class="fn">bits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the underlying bits value.

The returned value is exactly the bits set in this flags value.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_bits" class="fn">from_bits</a>(bits: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>

Convert from a bits value.

This method will return `None` if any unknown bits are set.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_bits_truncate" class="fn">from_bits_truncate</a>(bits: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Convert from a bits value, unsetting any unknown bits.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_bits_retain" class="fn">from_bits_retain</a>(bits: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Convert from a bits value exactly.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_name" class="fn">from_name</a>(name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>

Get a flags value with the bits of a flag with the given name set.

This method will return `None` if `name` is empty or doesn’t correspond to any named flag.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether all bits in this flags value are unset.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.is_all" class="fn">is_all</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether all known bits in this flags value are set.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.intersects" class="fn">intersects</a>(&self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether any set bits in a source flags value are also set in a target flags value.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.contains" class="fn">contains</a>(&self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether all set bits in a source flags value are also set in a target flags value.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.insert" class="fn">insert</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The bitwise or (`|`) of the bits in two flags values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.remove" class="fn">remove</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The intersection of a source flags value with the complement of a target flags value (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `remove` won’t truncate `other`, but the `!` operator will.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.toggle" class="fn">toggle</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The bitwise exclusive-or (`^`) of the bits in two flags values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.set" class="fn">set</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Call `insert` when `value` is `true` or `remove` when `value` is `false`.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.intersection" class="fn">intersection</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise and (`&`) of the bits in two flags values.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.union" class="fn">union</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise or (`|`) of the bits in two flags values.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.difference" class="fn">difference</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The intersection of a source flags value with the complement of a target flags value (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.symmetric_difference" class="fn">symmetric_difference</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise exclusive-or (`^`) of the bits in two flags values.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.complement" class="fn">complement</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise negation (`!`) of the bits in a flags value, truncating the result.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-TimeUnitSet-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html" class="struct" title="struct bitflags::iter::Iter">Iter</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>

Yield a set of contained flags values.

Each yielded flags value will correspond to a defined named flag. Any unknown bits will be yielded together as a final flags value.

#### pub const fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.iter_names" class="fn">iter_names</a>(&self) -\> <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html" class="struct" title="struct bitflags::iter::IterNames">IterNames</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>

Yield a set of contained named flags values.

This method is like [`iter`](https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.iter), except only yields bits in contained named flags. Any unknown bits, or bits not corresponding to a contained flag will not be yielded.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Binary-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html" class="trait" title="trait core::fmt::Binary">Binary</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.fmt-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-BitAnd-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise and (`&`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-BitAndAssign-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html" class="trait" title="trait core::ops::bit::BitAndAssign">BitAndAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bitand_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign" class="fn">bitand_assign</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The bitwise and (`&`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-BitOr-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise or (`|`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-BitOrAssign-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html" class="trait" title="trait core::ops::bit::BitOrAssign">BitOrAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bitor_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign" class="fn">bitor_assign</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The bitwise or (`|`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-BitXor-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise exclusive-or (`^`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-BitXorAssign-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html" class="trait" title="trait core::ops::bit::BitXorAssign">BitXorAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bitxor_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign" class="fn">bitxor_assign</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The bitwise exclusive-or (`^`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Clone-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Debug-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Deserialize%3C&#39;de%3E-for-TimeUnitSet" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Display-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Extend%3CTimeUnitSet%3E-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iterator: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>,

The bitwise or (`|`) of the bits in each flags value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Flags-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html" class="trait" title="trait bitflags::traits::Flags">Flags</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedconstant.FLAGS" class="anchor">§</a>

#### const <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedconstant.FLAGS" class="constant">FLAGS</a>: &'static \[<a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/struct.Flag.html" class="struct" title="struct bitflags::traits::Flag">Flag</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>\]

The set of defined flags.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Bits" class="anchor">§</a>

#### type <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits" class="associatedtype">Bits</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

The underlying bits type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.bits-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits" class="fn">bits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Get the underlying bits value. [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_bits_retain-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.from_bits_retain" class="fn">from_bits_retain</a>(bits: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Convert from a bits value exactly.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.empty" class="fn">empty</a>() -\> Self

Get a flags value with all bits unset.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.all-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all" class="fn">all</a>() -\> Self

Get a flags value with all known bits set.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.contains_unknown_bits" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.contains_unknown_bits" class="fn">contains_unknown_bits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

This method will return `true` if any unknown bits are set.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_bits-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits" class="fn">from_bits</a>(bits: Self::<a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits" class="associatedtype" title="type bitflags::traits::Flags::Bits">Bits</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self\>

Convert from a bits value. [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_bits_truncate-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits_truncate" class="fn">from_bits_truncate</a>(bits: Self::<a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits" class="associatedtype" title="type bitflags::traits::Flags::Bits">Bits</a>) -\> Self

Convert from a bits value, unsetting any unknown bits.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_name-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name" class="fn">from_name</a>(name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self\>

Get a flags value with the bits of a flag with the given name set. [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html" class="struct" title="struct bitflags::iter::Iter">Iter</a>\<Self\>

Yield a set of contained flags values. [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.iter_names-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names" class="fn">iter_names</a>(&self) -\> <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html" class="struct" title="struct bitflags::iter::IterNames">IterNames</a>\<Self\>

Yield a set of contained named flags values. [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.is_empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether all bits in this flags value are unset.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.is_all-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.is_all" class="fn">is_all</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether all known bits in this flags value are set.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.intersects-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.intersects" class="fn">intersects</a>(&self, other: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Whether any set bits in a source flags value are also set in a target flags value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.contains-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.contains" class="fn">contains</a>(&self, other: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Whether all set bits in a source flags value are also set in a target flags value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.truncate" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.truncate" class="fn">truncate</a>(&mut self)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Remove any unknown bits from the flags.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.insert-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert" class="fn">insert</a>(&mut self, other: Self)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

The bitwise or (`|`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.remove-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove" class="fn">remove</a>(&mut self, other: Self)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

The intersection of a source flags value with the complement of a target flags value (`&!`). [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.toggle-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.toggle" class="fn">toggle</a>(&mut self, other: Self)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

The bitwise exclusive-or (`^`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.set-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.set" class="fn">set</a>(&mut self, other: Self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Call [`Flags::insert`](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert "method bitflags::traits::Flags::insert") when `value` is `true` or [`Flags::remove`](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove "method bitflags::traits::Flags::remove") when `value` is `false`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.clear" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.clear" class="fn">clear</a>(&mut self)

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Unsets all bits in the flags.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.intersection-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.intersection" class="fn">intersection</a>(self, other: Self) -\> Self

The bitwise and (`&`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.union-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.union" class="fn">union</a>(self, other: Self) -\> Self

The bitwise or (`|`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.difference-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference" class="fn">difference</a>(self, other: Self) -\> Self

The intersection of a source flags value with the complement of a target flags value (`&!`). [Read more](https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.symmetric_difference-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.symmetric_difference" class="fn">symmetric_difference</a>(self, other: Self) -\> Self

The bitwise exclusive-or (`^`) of the bits in two flags values.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.complement-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.complement" class="fn">complement</a>(self) -\> Self

The bitwise negation (`!`) of the bits in a flags value, truncating the result.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-From%3CTimeUnit%3E-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-FromIterator%3CTimeUnitSet%3E-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iterator: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>,

The bitwise or (`|`) of the bits in each flags value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Hash-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-IntoIterator-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/bitflags/2.9.1/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html" class="struct" title="struct bitflags::iter::Iter">Iter</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-LowerHex-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html" class="trait" title="trait core::fmt::LowerHex">LowerHex</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.fmt-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Not-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html" class="trait" title="trait core::ops::bit::Not">Not</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.not" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The bitwise negation (`!`) of the bits in a flags value, truncating the result.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Octal-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html" class="trait" title="trait core::fmt::Octal">Octal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.fmt-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-PartialEq-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Serialize-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Sub-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The intersection of a source flags value with the complement of a target flags value (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-SubAssign-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html" class="trait" title="trait core::ops::arith::SubAssign">SubAssign</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.sub_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign" class="fn">sub_assign</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>)

The intersection of a source flags value with the complement of a target flags value (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-UpperHex-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html" class="trait" title="trait core::fmt::UpperHex">UpperHex</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#method.fmt-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Copy-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-Eq-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#impl-StructuralPartialEq-for-TimeUnitSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html" class="struct" title="struct polars::prelude::TimeUnitSet">TimeUnitSet</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeUnitSet.html#blanket-implementations" class="anchor">§</a>
