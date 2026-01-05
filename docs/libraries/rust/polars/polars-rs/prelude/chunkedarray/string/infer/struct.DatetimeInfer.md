# Struct DatetimeInfer Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/infer.rs.html#212" class="src">Source</a>

``` rust
pub struct DatetimeInfer<T>where
    T: PolarsNumericType,{
    pub pattern: Pattern,
    pub logical_type: DataType,
    /* private fields */
}
```

Available on **crate feature `temporal`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#structfield.pattern" class="anchor field">§</a>`pattern: `<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern"><code>Pattern</code></a><a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#structfield.logical_type" class="anchor field">§</a>`logical_type: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#impl-DatetimeInfer%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.parse" class="fn">parse</a>(&mut self, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#impl-Clone-for-DatetimeInfer%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#impl-StrpTimeParser%3Ci32%3E-for-DatetimeInfer%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::StrpTimeParser">StrpTimeParser</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.parse_bytes-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#tymethod.parse_bytes" class="fn">parse_bytes</a>( &mut self, val: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], \_time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#impl-StrpTimeParser%3Ci64%3E-for-DatetimeInfer%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::StrpTimeParser">StrpTimeParser</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.parse_bytes" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.StrpTimeParser.html#tymethod.parse_bytes" class="fn">parse_bytes</a>( &mut self, val: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#impl-TryFromWithUnit%3CPattern%3E-for-DatetimeInfer%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::TryFromWithUnit">TryFromWithUnit</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.try_from_with_unit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#tymethod.try_from_with_unit" class="fn">try_from_with_unit</a>( value: <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>, \_time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#impl-TryFromWithUnit%3CPattern%3E-for-DatetimeInfer%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::TryFromWithUnit">TryFromWithUnit</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#method.try_from_with_unit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#tymethod.try_from_with_unit" class="fn">try_from_with_unit</a>( value: <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>, time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html#blanket-implementations" class="anchor">§</a>
