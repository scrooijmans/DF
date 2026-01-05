# Trait PolarsCategoricalType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/mod.rs.html#125" class="src">Source</a>

``` rust
pub unsafe trait PolarsCategoricalType: PolarsDataType {
    type Native: NumericNative + CatNative + DictionaryKey + PartialEq + Eq + Hash;
    type PolarsPhysical: PolarsIntegerType<Native = Self::Native>;

    // Required method
    fn physical() -> CategoricalPhysical;
}
```

Expand description

## <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#safety" class="doc-anchor">§</a>Safety

The physical() return type must be correct for Native.

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype">Native</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.NumericNative.html" class="trait" title="trait polars::prelude::NumericNative">NumericNative</a> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.CatNative.html" class="trait" title="trait polars::prelude::CatNative">CatNative</a> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/dictionary/trait.DictionaryKey.html" class="trait" title="trait polars_arrow::array::dictionary::DictionaryKey">DictionaryKey</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype">PolarsPhysical</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsIntegerType.html" class="trait" title="trait polars::prelude::PolarsIntegerType">PolarsIntegerType</a>\<Native = Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::Native">Native</a>\>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#tymethod.physical" class="fn">physical</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::CategoricalPhysical">CategoricalPhysical</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#impl-PolarsCategoricalType-for-Categorical8Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical8Type.html" class="struct" title="struct polars::prelude::Categorical8Type">Categorical8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype">PolarsPhysical</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#impl-PolarsCategoricalType-for-Categorical16Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical16Type.html" class="struct" title="struct polars::prelude::Categorical16Type">Categorical16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype">PolarsPhysical</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#impl-PolarsCategoricalType-for-Categorical32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType">PolarsCategoricalType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Categorical32Type.html" class="struct" title="struct polars::prelude::Categorical32Type">Categorical32Type</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype">PolarsPhysical</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>
