# Struct CredentialProviderFunction Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/credential_provider.rs.html#209" class="src">Source</a>

``` rust
pub struct CredentialProviderFunction(/* private fields */);
```

Available on **crate feature `polars-io`** only.

Expand description

Wrapper that implements [`IntoCredentialProvider`](https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html "trait polars::prelude::cloud::credential_provider::IntoCredentialProvider"), [`Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"), [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"), [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") etc.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#impl-Clone-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#impl-Debug-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#impl-Hash-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#impl-IntoCredentialProvider-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html" class="trait" title="trait polars::prelude::cloud::credential_provider::IntoCredentialProvider">IntoCredentialProvider</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.storage_update_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/trait.IntoCredentialProvider.html#tymethod.storage_update_options" class="fn">storage_update_options</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Note, technically shouldn’t be under the `IntoCredentialProvider` trait, but it’s here for convenience.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#impl-PartialEq-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#impl-Eq-for-CredentialProviderFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html" class="struct" title="struct polars::prelude::cloud::credential_provider::CredentialProviderFunction">CredentialProviderFunction</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/struct.CredentialProviderFunction.html#blanket-implementations" class="anchor">§</a>
