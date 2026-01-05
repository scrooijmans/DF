# Module aws Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/mod.rs.html#18-903" class="src">Source</a>

Available on **crate feature `aws`** only.

Expand description

An object store implementation for S3

### <a href="https://docs.rs/object_store/latest/object_store/aws/index.html#multipart-uploads" class="doc-anchor">§</a>Multipart uploads

Multipart uploads can be initiated with the [ObjectStore::put_multipart](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart") method.

If the writer fails for any reason, you may have parts uploaded to AWS but not used that you will be charged for. [`MultipartUpload::abort`](https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html#tymethod.abort "method object_store::MultipartUpload::abort") may be invoked to drop these unneeded parts, however, it is recommended that you consider implementing [automatic cleanup](https://aws.amazon.com/blogs/aws/s3-lifecycle-management-update-support-for-multipart-uploads-and-delete-markers/) of unused parts that are older than some threshold.

## Structs<a href="https://docs.rs/object_store/latest/object_store/aws/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html" class="struct" title="struct object_store::aws::AmazonS3">AmazonS3</a>  
Interface for [Amazon S3](https://aws.amazon.com/s3/).

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html" class="struct" title="struct object_store::aws::AmazonS3Builder">AmazonS3Builder</a>  
Configure a connection to Amazon S3 using the specified credentials in the specified Amazon region and bucket.

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsAuthorizer.html" class="struct" title="struct object_store::aws::AwsAuthorizer">AwsAuthorizer</a>  
Authorize a [`HttpRequest`](https://docs.rs/object_store/latest/object_store/client/type.HttpRequest.html "type object_store::client::HttpRequest") with an [`AwsCredential`](https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html "struct object_store::aws::AwsCredential") using [AWS SigV4](https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html)

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.AwsCredential.html" class="struct" title="struct object_store::aws::AwsCredential">AwsCredential</a>  
A set of AWS security credentials

<a href="https://docs.rs/object_store/latest/object_store/aws/struct.DynamoCommit.html" class="struct" title="struct object_store::aws::DynamoCommit">DynamoCommit</a>  
A DynamoDB-based commit protocol, used to provide conditional write support for S3

## Enums<a href="https://docs.rs/object_store/latest/object_store/aws/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.AmazonS3ConfigKey.html" class="enum" title="enum object_store::aws::AmazonS3ConfigKey">AmazonS3ConfigKey</a>  
Configuration keys for [`AmazonS3Builder`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder")

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>  
Enum representing checksum algorithm supported by S3.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3ConditionalPut.html" class="enum" title="enum object_store::aws::S3ConditionalPut">S3ConditionalPut</a>  
Configure how to provide conditional put support for [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3").

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.S3CopyIfNotExists.html" class="enum" title="enum object_store::aws::S3CopyIfNotExists">S3CopyIfNotExists</a>  
Configure how to provide [`ObjectStore::copy_if_not_exists`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists "method object_store::ObjectStore::copy_if_not_exists") for [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3").

## Functions<a href="https://docs.rs/object_store/latest/object_store/aws/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/fn.resolve_bucket_region.html" class="fn" title="fn object_store::aws::resolve_bucket_region">resolve_bucket_region</a>Non-WebAssembly  
Get the bucket region using the [HeadBucket API](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadBucket.html). This will fail if the bucket does not exist.

## Type Aliases<a href="https://docs.rs/object_store/latest/object_store/aws/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/type.AwsCredentialProvider.html" class="type" title="type object_store::aws::AwsCredentialProvider">AwsCredentialProvider</a>  
[`CredentialProvider`](https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html "trait object_store::client::CredentialProvider") for [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3")
