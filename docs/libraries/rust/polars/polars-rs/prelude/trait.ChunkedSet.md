# Trait ChunkedSet Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/scatter.rs.html#9" class="src">Source</a>

``` rust
pub trait ChunkedSet<T>where
    T: Copy,{
    // Required method
    fn scatter<V>(self, idx: &[u32], values: V) -> Result<Series, PolarsError>
       where V: IntoIterator<Item = Option<T>>;
}
```

Available on **crate feature `polars-ops`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#tymethod.scatter" class="fn">scatter</a>\<V\>(self, idx: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], values: V) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where V: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\>,

Invariant for implementations: if the scatter() fails, typically because of bad indexes, then self should remain unmodified.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#impl-ChunkedSet%3Cbool%3E-for-%26ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html" class="trait" title="trait polars::prelude::ChunkedSet">ChunkedSet</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#impl-ChunkedSet%3C%26str%3E-for-%26ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html" class="trait" title="trait polars::prelude::ChunkedSet">ChunkedSet</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html#impl-ChunkedSet%3C%3CT+as+PolarsNumericType%3E::Native%3E-for-%26mut+ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedSet.html" class="trait" title="trait polars::prelude::ChunkedSet">ChunkedSet</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: PolarsOpsNumericType,
