# Trait SchemaNamesAndDtypes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/schema/mod.rs.html#108" class="src">Source</a>

``` rust
pub trait SchemaNamesAndDtypes {
    type DataType: Debug + Clone + Default + PartialEq;

    const IS_ARROW: bool;

    // Required method
    fn iter_names_and_dtypes(&self) -> impl ExactSizeIterator;
}
```

## Required Associated Constants<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedconstant.IS_ARROW" class="constant">IS_ARROW</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedtype.DataType" class="associatedtype">DataType</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#tymethod.iter_names_and_dtypes" class="fn">iter_names_and_dtypes</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#impl-SchemaNamesAndDtypes-for-Schema%3CDataType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html" class="trait" title="trait polars::prelude::SchemaNamesAndDtypes">SchemaNamesAndDtypes</a> for <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedconstant.IS_ARROW-1" class="anchor">§</a>

#### const <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedconstant.IS_ARROW" class="constant">IS_ARROW</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = false

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedtype.DataType-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedtype.DataType" class="associatedtype">DataType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#method.iter_names_and_dtypes" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#tymethod.iter_names_and_dtypes" class="fn">iter_names_and_dtypes</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#impl-SchemaNamesAndDtypes-for-Schema%3CArrowField%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html" class="trait" title="trait polars::prelude::SchemaNamesAndDtypes">SchemaNamesAndDtypes</a> for <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedconstant.IS_ARROW-2" class="anchor">§</a>

#### const <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedconstant.IS_ARROW" class="constant">IS_ARROW</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = true

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedtype.DataType-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#associatedtype.DataType" class="associatedtype">DataType</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#method.iter_names_and_dtypes-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#tymethod.iter_names_and_dtypes" class="fn">iter_names_and_dtypes</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.SchemaNamesAndDtypes.html#implementors" class="anchor">§</a>
