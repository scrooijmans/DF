# Struct AzureAuthorizer Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/credential.rs.html#225-228" class="src">Source</a>

``` rust
pub struct AzureAuthorizer<'a> { /* private fields */ }
```

Available on **crate feature `azure`** only.

Expand description

Authorize a [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") with an [`AzureAuthorizer`](https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html "struct object_store::azure::AzureAuthorizer")

## Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#impl-AzureAuthorizer%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html" class="struct" title="struct object_store::azure::AzureAuthorizer">AzureAuthorizer</a>\<'a\>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#method.new" class="fn">new</a>(credential: &'a <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>, account: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Create a new [`AzureAuthorizer`](https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html "struct object_store::azure::AzureAuthorizer")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#method.authorize" class="fn">authorize</a>(&self, request: &mut <a href="https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html" class="type" title="type object_store::client::HttpRequest">HttpRequest</a>)

Authorize `request`

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#impl-Debug-for-AzureAuthorizer%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html" class="struct" title="struct object_store::azure::AzureAuthorizer">AzureAuthorizer</a>\<'a\>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html#blanket-implementations" class="anchor">§</a>
