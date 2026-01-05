# Struct GroupsIdx Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/position.rs.html#16" class="src">Source</a>

``` rust
pub struct GroupsIdx { /* private fields */ }
```

Expand description

Indexes of the groups, the first index is stored separately. this make sorting fast.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-GroupsIdx" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.new" class="fn">new</a>(first: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, all: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>, sorted: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.sort" class="fn">sort</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.is_sorted_flag" class="fn">is_sorted_flag</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html" class="struct" title="struct core::iter::adapters::zip::Zip">Zip</a>\<<a href="https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html" class="struct" title="struct core::iter::adapters::copied::Copied">Copied</a>\<<a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>, <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'\_, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.all" class="fn">all</a>(&self) -\> &\[<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.first" class="fn">first</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.first_mut" class="fn">first_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.new_empty" class="fn">new_empty</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-Clone-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-Debug-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-Default-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-Drop-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-From%3CGroupsIdx%3E-for-GroupsType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(groups: <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsType.html" class="enum" title="enum polars::prelude::GroupsType">GroupsType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-From%3CVec%3C(u32,+UnitVec%3Cu32%3E)%3E%3E-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-From%3CVec%3CVec%3C(u32,+UnitVec%3Cu32%3E)%3E%3E%3E-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-FromIterator%3C(u32,+UnitVec%3Cu32%3E)%3E-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-FromParallelIterator%3C(u32,+UnitVec%3Cu32%3E)%3E-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html" class="trait" title="trait rayon::iter::FromParallelIterator">FromParallelIterator</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.from_par_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html#tymethod.from_par_iter" class="fn">from_par_iter</a>\<I\>(par_iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

where I: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)\>,

Creates an instance of the collection from the parallel iterator `par_iter`. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html#tymethod.from_par_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-IntoIterator-for-%26GroupsIdx" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, &'a <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html" class="struct" title="struct core::iter::adapters::zip::Zip">Zip</a>\<<a href="https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html" class="struct" title="struct core::iter::adapters::copied::Copied">Copied</a>\<<a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>, <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'a, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-IntoIterator-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

The type of the elements being iterated over.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.IntoIter-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html" class="struct" title="struct core::iter::adapters::zip::Zip">Zip</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/into_iter/struct.IntoIter.html" class="struct" title="struct alloc::vec::into_iter::IntoIter">IntoIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/into_iter/struct.IntoIter.html" class="struct" title="struct alloc::vec::into_iter::IntoIter">IntoIter</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.into_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-IntoParallelIterator-for-%26GroupsIdx" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a> for &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.Iter" class="anchor">§</a>

#### type <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype">Iter</a> = <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/zip/struct.Zip.html" class="struct" title="struct rayon::iter::zip::Zip">Zip</a>\<<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/copied/struct.Copied.html" class="struct" title="struct rayon::iter::copied::Copied">Copied</a>\<<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/slice/struct.Iter.html" class="struct" title="struct rayon::slice::Iter">Iter</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>, <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/slice/struct.Iter.html" class="struct" title="struct rayon::slice::Iter">Iter</a>\<'a, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>\>

The parallel iterator type that will be created.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, &'a <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

The type of item that the parallel iterator will produce.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.into_par_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#tymethod.into_par_iter" class="fn">into_par_iter</a>(self) -\> \<&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a> as <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>

Converts `self` into a parallel iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#tymethod.into_par_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-IntoParallelIterator-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.Iter-1" class="anchor">§</a>

#### type <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype">Iter</a> = <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/zip/struct.Zip.html" class="struct" title="struct rayon::iter::zip::Zip">Zip</a>\<<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/vec/struct.IntoIter.html" class="struct" title="struct rayon::vec::IntoIter">IntoIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/vec/struct.IntoIter.html" class="struct" title="struct rayon::vec::IntoIter">IntoIter</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\>\>

The parallel iterator type that will be created.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

The type of item that the parallel iterator will produce.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.into_par_iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#tymethod.into_par_iter" class="fn">into_par_iter</a>(self) -\> \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a> as <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>

Converts `self` into a parallel iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#tymethod.into_par_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-PartialEq-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-Eq-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#impl-StructuralPartialEq-for-GroupsIdx" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html" class="struct" title="struct polars::prelude::GroupsIdx">GroupsIdx</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsIdx.html#blanket-implementations" class="anchor">§</a>
