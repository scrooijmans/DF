# Trait RandomTemporalValue Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/data_gen.rs.html#457-476" class="src">Source</a>

``` rust
pub trait RandomTemporalValue: ArrowTemporalType {
    // Required method
    fn value_range() -> impl SampleRange<Self::Native>;

    // Provided methods
    fn gen_range<R: Rng>(rng: &mut R) -> Self::Native
       where Self::Native: SampleUniform { ... }
    fn random<R: Rng>(rng: &mut R) -> Self::Native
       where Self::Native: SampleUniform { ... }
}
```

Available on **crate feature `test_utils`** only.

Expand description

Useful for testing. The range of values are not likely to be representative of the actual bounds.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#tymethod.value_range" class="fn">value_range</a>() -\> impl <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html" class="trait" title="trait rand::distr::uniform::SampleRange">SampleRange</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Returns the range of values for `impl`’d type

## Provided Methods<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.gen_range" class="fn">gen_range</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value within the range of the type

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.random" class="fn">random</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value of the type

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-Date32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-Date64Type" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-Time32MillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-Time32SecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-Time64MicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-TimestampMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#impl-RandomTemporalValue-for-TimestampSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>
