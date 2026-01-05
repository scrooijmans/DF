# Trait IntoScalar Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/into_scalar.rs.html#5" class="src">Source</a>

``` rust
pub trait IntoScalar {
    // Required method
    fn into_scalar(self, dtype: DataType) -> Result<Scalar, PolarsError>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-bool" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#impl-IntoScalar-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoScalar.html" class="trait" title="trait polars::prelude::IntoScalar">IntoScalar</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#method.into_scalar-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#tymethod.into_scalar" class="fn">into_scalar</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.IntoScalar.html#implementors" class="anchor">§</a>
