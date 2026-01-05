# Struct NoNull Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/utils/mod.rs.html#47" class="src">Source</a>

``` rust
pub struct NoNull<T> { /* private fields */ }
```

Expand description

Just a wrapper structure which is useful for certain impl specializations.

This is for instance use to implement `impl<T> FromIterator<T::Native> for NoNull<ChunkedArray<T>>` as `Option<T::Native>` was already implemented: `impl<T> FromIterator<Option<T::Native>> for ChunkedArray<T>`

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-NoNull%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.new" class="fn">new</a>(inner: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<T\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.into_inner" class="fn">into_inner</a>(self) -\> T

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-Deref-for-NoNull%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = T

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-DerefMut-for-NoNull%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html" class="trait" title="trait core::ops::deref::DerefMut">DerefMut</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<T\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.deref_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut" class="fn">deref_mut</a>(&mut self) -\> &mut \<<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Mutably dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromIterator%3C%3CT+as+PolarsNumericType%3E::Native%3E-for-NoNull%3CChunkedArray%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromIterator%3Cbool%3E-for-NoNull%3CChunkedArray%3CBooleanType%3E%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromIteratorReversed%3C%3CT+as+PolarsNumericType%3E::Native%3E-for-NoNull%3CChunkedArray%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/trusted_len/rev/trait.FromIteratorReversed.html" class="trait" title="trait polars_arrow::legacy::trusted_len::rev::FromIteratorReversed">FromIteratorReversed</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_trusted_len_iter_rev" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/trusted_len/rev/trait.FromIteratorReversed.html#tymethod.from_trusted_len_iter_rev" class="fn">from_trusted_len_iter_rev</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where I: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>\<Item = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromIteratorReversed%3Cbool%3E-for-NoNull%3CChunkedArray%3CBooleanType%3E%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/trusted_len/rev/trait.FromIteratorReversed.html" class="trait" title="trait polars_arrow::legacy::trusted_len::rev::FromIteratorReversed">FromIteratorReversed</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_trusted_len_iter_rev-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/trusted_len/rev/trait.FromIteratorReversed.html#tymethod.from_trusted_len_iter_rev" class="fn">from_trusted_len_iter_rev</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

where I: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromParallelIterator%3C%3CT+as+PolarsNumericType%3E::Native%3E-for-NoNull%3CChunkedArray%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html" class="trait" title="trait rayon::iter::FromParallelIterator">FromParallelIterator</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_par_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html#tymethod.from_par_iter" class="fn">from_par_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where I: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

Creates an instance of the collection from the parallel iterator `par_iter`. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html#tymethod.from_par_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromTrustedLenIterator%3C%3CT+as+PolarsNumericType%3E::Native%3E-for-NoNull%3CChunkedArray%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/utils/trait.FromTrustedLenIterator.html" class="trait" title="trait polars_arrow::legacy::utils::FromTrustedLenIterator">FromTrustedLenIterator</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_iter_trusted_length" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/utils/trait.FromTrustedLenIterator.html#tymethod.from_iter_trusted_length" class="fn">from_iter_trusted_length</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#impl-FromTrustedLenIterator%3Cbool%3E-for-NoNull%3CChunkedArray%3CBooleanType%3E%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/utils/trait.FromTrustedLenIterator.html" class="trait" title="trait polars_arrow::legacy::utils::FromTrustedLenIterator">FromTrustedLenIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#method.from_iter_trusted_length-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/legacy/utils/trait.FromTrustedLenIterator.html#tymethod.from_iter_trusted_length" class="fn">from_iter_trusted_length</a>\<I\>(iter: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html" class="struct" title="struct polars::prelude::NoNull">NoNull</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>,

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.NoNull.html#blanket-implementations" class="anchor">§</a>
