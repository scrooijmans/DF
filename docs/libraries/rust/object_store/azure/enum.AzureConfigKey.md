# Enum AzureConfigKey Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/builder.rs.html#198-385" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum AzureConfigKey {
Show 24 variants    AccountName,
    AccessKey,
    ClientId,
    ClientSecret,
    AuthorityId,
    AuthorityHost,
    SasKey,
    Token,
    UseEmulator,
    Endpoint,
    UseFabricEndpoint,
    MsiEndpoint,
    ObjectId,
    MsiResourceId,
    FederatedTokenFile,
    UseAzureCli,
    SkipSignature,
    ContainerName,
    DisableTagging,
    FabricTokenServiceUrl,
    FabricWorkloadHost,
    FabricSessionToken,
    FabricClusterIdentifier,
    Client(ClientConfigKey),
}
```

Available on **crate feature `azure`** only.

Expand description

Configuration keys for [`MicrosoftAzureBuilder`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html "struct object_store::azure::MicrosoftAzureBuilder")

Configuration via keys can be done via [`MicrosoftAzureBuilder::with_config`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html#method.with_config "method object_store::azure::MicrosoftAzureBuilder::with_config")

## <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#example" class="doc-anchor">§</a>Example

``` rust
let builder = MicrosoftAzureBuilder::new()
    .with_config("azure_client_id".parse().unwrap(), "my-client-id")
    .with_config(AzureConfigKey::AuthorityId, "my-tenant-id");
```

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.AccountName" class="anchor">§</a>

### AccountName

The name of the azure storage account

Supported keys:

- `azure_storage_account_name`
- `account_name`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.AccessKey" class="anchor">§</a>

### AccessKey

Master key for accessing storage account

Supported keys:

- `azure_storage_account_key`
- `azure_storage_access_key`
- `azure_storage_master_key`
- `access_key`
- `account_key`
- `master_key`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.ClientId" class="anchor">§</a>

### ClientId

Service principal client id for authorizing requests

Supported keys:

- `azure_storage_client_id`
- `azure_client_id`
- `client_id`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.ClientSecret" class="anchor">§</a>

### ClientSecret

Service principal client secret for authorizing requests

Supported keys:

- `azure_storage_client_secret`
- `azure_client_secret`
- `client_secret`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.AuthorityId" class="anchor">§</a>

### AuthorityId

Tenant id used in oauth flows

Supported keys:

- `azure_storage_tenant_id`
- `azure_storage_authority_id`
- `azure_tenant_id`
- `azure_authority_id`
- `tenant_id`
- `authority_id`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.AuthorityHost" class="anchor">§</a>

### AuthorityHost

Authority host used in oauth flows

Supported keys:

- `azure_storage_authority_host`
- `azure_authority_host`
- `authority_host`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.SasKey" class="anchor">§</a>

### SasKey

Shared access signature.

The signature is expected to be percent-encoded, much like they are provided in the azure storage explorer or azure portal.

Supported keys:

- `azure_storage_sas_key`
- `azure_storage_sas_token`
- `sas_key`
- `sas_token`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.Token" class="anchor">§</a>

### Token

Bearer token

Supported keys:

- `azure_storage_token`
- `bearer_token`
- `token`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.UseEmulator" class="anchor">§</a>

### UseEmulator

Use object store with azurite storage emulator

Supported keys:

- `azure_storage_use_emulator`
- `object_store_use_emulator`
- `use_emulator`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.Endpoint" class="anchor">§</a>

### Endpoint

Override the endpoint used to communicate with blob storage

Supported keys:

- `azure_storage_endpoint`
- `azure_endpoint`
- `endpoint`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.UseFabricEndpoint" class="anchor">§</a>

### UseFabricEndpoint

Use object store with url scheme account.dfs.fabric.microsoft.com

Supported keys:

- `azure_use_fabric_endpoint`
- `use_fabric_endpoint`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.MsiEndpoint" class="anchor">§</a>

### MsiEndpoint

Endpoint to request a imds managed identity token

Supported keys:

- `azure_msi_endpoint`
- `azure_identity_endpoint`
- `identity_endpoint`
- `msi_endpoint`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.ObjectId" class="anchor">§</a>

### ObjectId

Object id for use with managed identity authentication

Supported keys:

- `azure_object_id`
- `object_id`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.MsiResourceId" class="anchor">§</a>

### MsiResourceId

Msi resource id for use with managed identity authentication

Supported keys:

- `azure_msi_resource_id`
- `msi_resource_id`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.FederatedTokenFile" class="anchor">§</a>

### FederatedTokenFile

File containing token for Azure AD workload identity federation

Supported keys:

- `azure_federated_token_file`
- `federated_token_file`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.UseAzureCli" class="anchor">§</a>

### UseAzureCli

Use azure cli for acquiring access token

Supported keys:

- `azure_use_azure_cli`
- `use_azure_cli`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.SkipSignature" class="anchor">§</a>

### SkipSignature

Skip signing requests

Supported keys:

- `azure_skip_signature`
- `skip_signature`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.ContainerName" class="anchor">§</a>

### ContainerName

Container name

Supported keys:

- `azure_container_name`
- `container_name`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.DisableTagging" class="anchor">§</a>

### DisableTagging

Disables tagging objects

This can be desirable if not supported by the backing store

Supported keys:

- `azure_disable_tagging`
- `disable_tagging`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.FabricTokenServiceUrl" class="anchor">§</a>

### FabricTokenServiceUrl

Fabric token service url

Supported keys:

- `azure_fabric_token_service_url`
- `fabric_token_service_url`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.FabricWorkloadHost" class="anchor">§</a>

### FabricWorkloadHost

Fabric workload host

Supported keys:

- `azure_fabric_workload_host`
- `fabric_workload_host`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.FabricSessionToken" class="anchor">§</a>

### FabricSessionToken

Fabric session token

Supported keys:

- `azure_fabric_session_token`
- `fabric_session_token`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.FabricClusterIdentifier" class="anchor">§</a>

### FabricClusterIdentifier

Fabric cluster identifier

Supported keys:

- `azure_fabric_cluster_identifier`
- `fabric_cluster_identifier`

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#variant.Client" class="anchor">§</a>

### Client(<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey">ClientConfigKey</a>)

Client options

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-AsRef%3Cstr%3E-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Clone-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Debug-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Deserialize%3C&#39;de%3E-for-AzureConfigKey" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-FromStr-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Hash-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-PartialEq-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Serialize-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.226/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Copy-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-Eq-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#impl-StructuralPartialEq-for-AzureConfigKey" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html#blanket-implementations" class="anchor">§</a>
