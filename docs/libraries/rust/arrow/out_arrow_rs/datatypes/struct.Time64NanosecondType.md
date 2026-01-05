# Struct Time64NanosecondType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#218-223" class="src">Source</a>

``` rust
pub struct Time64NanosecondType {}
```

Expand description

64-bit time type: the elapsed time since midnight in nanoseconds.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#impl-ArrowPrimitiveType-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#associatedconstant.DATA_TYPE" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

the corresponding Arrow data type of this primitive type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#associatedtype.Native" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype">Native</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

Corresponding Rust native type for the primitive type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.default_value" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value" class="fn">default_value</a>() -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns a default value of this primitive type. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#method.default_value)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#impl-Debug-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#impl-Parser-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html" class="trait" title="trait arrow::compute::kernels::cast_utils::Parser">Parser</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.parse" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#tymethod.parse" class="fn">parse</a>( string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.parse_formatted" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted" class="fn">parse_formatted</a>( string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, format: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Parse a string to the native type with a format string [Read more](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html#method.parse_formatted)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#impl-RandomTemporalValue-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

Available on **crate feature `test_utils`** only.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.value_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#tymethod.value_range" class="fn">value_range</a>() -\> impl <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html" class="trait" title="trait rand::distr::uniform::SampleRange">SampleRange</a>\<Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Range of values representing the elapsed time since midnight in nanoseconds. The range is from 0 to 24 hours.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.gen_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.gen_range" class="fn">gen_range</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value within the range of the type

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#method.random" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html#method.random" class="fn">random</a>\<R: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html" class="trait" title="trait rand::rng::Rng">Rng</a>\>(rng: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>) -\> Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

where Self::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://docs.rs/rand/0.9.2/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html" class="trait" title="trait rand::distr::uniform::SampleUniform">SampleUniform</a>,

Generate a random value of the type

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#impl-ArrowTemporalType-for-Time64NanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html#blanket-implementations" class="anchor">§</a>
