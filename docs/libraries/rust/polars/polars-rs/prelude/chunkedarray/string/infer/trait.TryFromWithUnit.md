# Trait TryFromWithUnit Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/infer.rs.html#222" class="src">Source</a>

``` rust
pub trait TryFromWithUnit<T>: Sized {
    type Error;

    // Required method
    fn try_from_with_unit(
        pattern: T,
        unit: Option<TimeUnit>,
    ) -> Result<Self, PolarsError>;
}
```

Available on **crate feature `temporal`** only.

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#tymethod.try_from_with_unit" class="fn">try_from_with_unit</a>( pattern: T, unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#impl-TryFromWithUnit%3CPattern%3E-for-DatetimeInfer%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::TryFromWithUnit">TryFromWithUnit</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#impl-TryFromWithUnit%3CPattern%3E-for-DatetimeInfer%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::TryFromWithUnit">TryFromWithUnit</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>
