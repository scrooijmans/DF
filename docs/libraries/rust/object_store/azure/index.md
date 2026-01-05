# Module azure Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/azure/mod.rs.html#18-409" class="src">Source</a>

Available on **crate feature `azure`** only.

Expand description

An object store implementation for Azure blob storage

### <a href="https://docs.rs/object_store/latest/object_store/azure/index.html#streaming-uploads" class="doc-anchor">§</a>Streaming uploads

[ObjectStore::put_multipart](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart") will upload data in blocks and write a blob from those blocks.

Unused blocks will automatically be dropped after 7 days.

## Modules<a href="https://docs.rs/object_store/latest/object_store/azure/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/authority_hosts/index.html" class="mod" title="mod object_store::azure::authority_hosts">authority_hosts</a>  
A list of known Azure authority hosts

## Structs<a href="https://docs.rs/object_store/latest/object_store/azure/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAccessKey.html" class="struct" title="struct object_store::azure::AzureAccessKey">AzureAccessKey</a>  
A shared Azure Storage Account Key

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html" class="struct" title="struct object_store::azure::AzureAuthorizer">AzureAuthorizer</a>  
Authorize a [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") with an [`AzureAuthorizer`](https://docs.rs/object_store/latest/object_store/azure/struct.AzureAuthorizer.html "struct object_store::azure::AzureAuthorizer")

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html" class="struct" title="struct object_store::azure::MicrosoftAzure">MicrosoftAzure</a>  
Interface for [Microsoft Azure Blob Storage](https://azure.microsoft.com/en-us/services/storage/blobs/).

<a href="https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html" class="struct" title="struct object_store::azure::MicrosoftAzureBuilder">MicrosoftAzureBuilder</a>  
Configure a connection to Microsoft Azure Blob Storage container using the specified credentials.

## Enums<a href="https://docs.rs/object_store/latest/object_store/azure/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureConfigKey.html" class="enum" title="enum object_store::azure::AzureConfigKey">AzureConfigKey</a>  
Configuration keys for [`MicrosoftAzureBuilder`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html "struct object_store::azure::MicrosoftAzureBuilder")

<a href="https://docs.rs/object_store/latest/object_store/azure/enum.AzureCredential.html" class="enum" title="enum object_store::azure::AzureCredential">AzureCredential</a>  
An Azure storage credential

## Type Aliases<a href="https://docs.rs/object_store/latest/object_store/azure/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/azure/type.AzureCredentialProvider.html" class="type" title="type object_store::azure::AzureCredentialProvider">AzureCredentialProvider</a>  
[`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for [`MicrosoftAzure`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html "struct object_store::azure::MicrosoftAzure")
