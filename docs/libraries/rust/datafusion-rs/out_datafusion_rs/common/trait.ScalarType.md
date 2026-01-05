# Trait ScalarType Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/scalar/mod.rs.html#4574" class="src">Source</a>

``` rust
pub trait ScalarType<T>where
    T: ArrowNativeType,{
    // Required method
    fn scalar(r: Option<T>) -> ScalarValue;
}
```

Expand description

Trait used to map a NativeType to a ScalarValue

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#tymethod.scalar" class="fn">scalar</a>(r: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

returns a scalar from an optional T

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#impl-ScalarType%3Cf32%3E-for-Float32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Float32Type">Float32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#impl-ScalarType%3Ci32%3E-for-Date32Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct datafusion::common::arrow::datatypes::Date32Type">Date32Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#impl-ScalarType%3Ci64%3E-for-TimestampMicrosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#impl-ScalarType%3Ci64%3E-for-TimestampMillisecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#impl-ScalarType%3Ci64%3E-for-TimestampNanosecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ScalarType.html#impl-ScalarType%3Ci64%3E-for-TimestampSecondType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/trait.ScalarType.html" class="trait" title="trait datafusion::scalar::ScalarType">ScalarType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct datafusion::common::arrow::datatypes::TimestampSecondType">TimestampSecondType</a>
