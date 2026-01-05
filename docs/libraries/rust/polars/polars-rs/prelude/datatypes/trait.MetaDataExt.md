# Trait MetaDataExt Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/dtype.rs.html#19" class="src">Source</a>

``` rust
pub trait MetaDataExt: IntoMetadata {
    // Provided methods
    fn pl_enum_metadata(&self) -> Option<&str> { ... }
    fn pl_categorical_metadata(&self) -> Option<&str> { ... }
    fn maintain_type(&self) -> bool { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#method.pl_enum_metadata" class="fn">pl_enum_metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#method.pl_categorical_metadata" class="fn">pl_categorical_metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#method.maintain_type" class="fn">maintain_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#impl-MetaDataExt-for-BTreeMap%3CPlSmallStr,+PlSmallStr%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.MetaDataExt.html" class="trait" title="trait polars::prelude::MetaDataExt">MetaDataExt</a> for <a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.MetaDataExt.html#implementors" class="anchor">§</a>
