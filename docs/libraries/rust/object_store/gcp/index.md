# Module gcp Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/gcp/mod.rs.html#18-435" class="src">Source</a>

Available on **crate feature `gcp`** only.

Expand description

An object store implementation for Google Cloud Storage

### <a href="https://docs.rs/object_store/latest/object_store/gcp/index.html#multipart-uploads" class="doc-anchor">§</a>Multipart uploads

[Multipart uploads](https://cloud.google.com/storage/docs/multipart-uploads) can be initiated with the [ObjectStore::put_multipart](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart") method. If neither [`MultipartUpload::complete`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.complete "method object_store::MultipartUpload::complete") nor [`MultipartUpload::abort`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort "method object_store::MultipartUpload::abort") is invoked, you may have parts uploaded to GCS but not used, that you will be charged for. It is recommended you configure a [lifecycle rule](https://cloud.google.com/storage/docs/lifecycle#abort-mpu) to abort incomplete multipart uploads after a certain period of time to avoid being charged for storing partial uploads.

### <a href="https://docs.rs/object_store/latest/object_store/gcp/index.html#using-http2" class="doc-anchor">§</a>Using HTTP/2

Google Cloud Storage supports both HTTP/2 and HTTP/1. HTTP/1 is used by default because it allows much higher throughput in our benchmarks (see [\#5194](https://github.com/apache/arrow-rs/issues/5194)). HTTP/2 can be enabled by setting [crate::ClientConfigKey::Http1Only](https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html#variant.Http1Only "variant object_store::client::ClientConfigKey::Http1Only") to false.

## Structs<a href="https://docs.rs/object_store/latest/object_store/gcp/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpCredential.html" class="struct" title="struct object_store::gcp::GcpCredential">GcpCredential</a>  
A Google Cloud Storage Credential

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html" class="struct" title="struct object_store::gcp::GcpSigningCredential">GcpSigningCredential</a>  
A Google Cloud Storage Credential for signing

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html" class="struct" title="struct object_store::gcp::GoogleCloudStorage">GoogleCloudStorage</a>  
Interface for [Google Cloud Storage](https://cloud.google.com/storage/).

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html" class="struct" title="struct object_store::gcp::GoogleCloudStorageBuilder">GoogleCloudStorageBuilder</a>  
Configure a connection to Google Cloud Storage.

<a href="https://docs.rs/object_store/latest/object_store/gcp/struct.ServiceAccountKey.html" class="struct" title="struct object_store::gcp::ServiceAccountKey">ServiceAccountKey</a>  
A private RSA key for a service account

## Enums<a href="https://docs.rs/object_store/latest/object_store/gcp/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/enum.GoogleConfigKey.html" class="enum" title="enum object_store::gcp::GoogleConfigKey">GoogleConfigKey</a>  
Configuration keys for [`GoogleCloudStorageBuilder`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html "struct object_store::gcp::GoogleCloudStorageBuilder")

## Type Aliases<a href="https://docs.rs/object_store/latest/object_store/gcp/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/gcp/type.GcpCredentialProvider.html" class="type" title="type object_store::gcp::GcpCredentialProvider">GcpCredentialProvider</a>  
[`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage")

<a href="https://docs.rs/object_store/latest/object_store/gcp/type.GcpSigningCredentialProvider.html" class="type" title="type object_store::gcp::GcpSigningCredentialProvider">GcpSigningCredentialProvider</a>  
[`GcpSigningCredential`](https://docs.rs/object_store/latest/object_store/gcp/struct.GcpSigningCredential.html "struct object_store::gcp::GcpSigningCredential") for [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage")
