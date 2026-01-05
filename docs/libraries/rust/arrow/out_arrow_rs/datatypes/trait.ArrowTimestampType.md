# Trait ArrowTimestampType Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#320" class="src">Source</a>

``` rust
pub trait ArrowTimestampType: ArrowTemporalType<Native = i64> {
    const UNIT: TimeUnit;

    // Required method
    fn make_value(naive: NaiveDateTime) -> Option<i64>;
}
```

Expand description

A timestamp type allows us to create array builders that take a timestamp.

## Required Associated Constants<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a>

The [`TimeUnit`](https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html "enum arrow::datatypes::TimeUnit") of this timestamp.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#tymethod.make_value" class="fn">make_value</a>(naive: <a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Creates a ArrowTimestampType::Native from the provided [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")

See [`DataType::Timestamp`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Timestamp "variant arrow::datatypes::DataType::Timestamp") for more information on timezone handling

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#impl-ArrowTimestampType-for-TimestampMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a> = TimeUnit::Microsecond

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#impl-ArrowTimestampType-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a> = TimeUnit::Millisecond

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#impl-ArrowTimestampType-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT-3" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a> = TimeUnit::Nanosecond

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#impl-ArrowTimestampType-for-TimestampSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT-4" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html#associatedconstant.UNIT" class="constant">UNIT</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum arrow::datatypes::TimeUnit">TimeUnit</a> = TimeUnit::Second
