# Trait IntoMetadata Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/dtype.rs.html#47" class="src">Source</a>

``` rust
pub trait IntoMetadata {
    // Required method
    fn into_metadata_ref(&self) -> &BTreeMap<PlSmallStr, PlSmallStr>;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#tymethod.into_metadata_ref" class="fn">into_metadata_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#impl-IntoMetadata-for-BTreeMap%3CPlSmallStr,+PlSmallStr%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html" class="trait" title="trait polars::prelude::IntoMetadata">IntoMetadata</a> for <a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#method.into_metadata_ref" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#tymethod.into_metadata_ref" class="fn">into_metadata_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoMetadata.html#implementors" class="anchor">§</a>
