# Trait IntoCredentialProvider Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/credential_provider.rs.html#142" class="src">Source</a>

``` rust
pub trait IntoCredentialProvider: Sized {
    // Required method
    fn storage_update_options(
        &self,
    ) -> Result<Vec<(PlSmallStr, PlSmallStr)>, PolarsError>;
}
```

Available on **crate feature `polars-io`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#tymethod.storage_update_options" class="fn">storage_update_options</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Note, technically shouldn’t be under the `IntoCredentialProvider` trait, but it’s here for convenience.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#impl-IntoCredentialProvider-for-PlCredentialProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html" class="trait" title="trait polars::prelude::cloud::credential_provider::IntoCredentialProvider">IntoCredentialProvider</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/enum.PlCredentialProvider.html" class="enum" title="enum polars::prelude::cloud::credential_provider::PlCredentialProvider">PlCredentialProvider</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#impl-IntoCredentialProvider-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html" class="trait" title="trait polars::prelude::cloud::credential_provider::IntoCredentialProvider">IntoCredentialProvider</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>
