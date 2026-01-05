# Module io Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/mod.rs.html#18-103" class="src">Source</a>

Expand description

File io implementation.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/index.html#how-to-build-fileio" class="doc-anchor">§</a>How to build `FileIO`

We provided a `FileIOBuilder` to build `FileIO` from scratch. For example:

``` rust
use iceberg::Result;
use iceberg::io::{FileIOBuilder, S3_REGION};

// Build a memory file io.
let file_io = FileIOBuilder::new("memory").build()?;
// Build an fs file io.
let file_io = FileIOBuilder::new("fs").build()?;
// Build an s3 file io.
let file_io = FileIOBuilder::new("s3")
    .with_prop(S3_REGION, "us-east-1")
    .build()?;
```

Or you can pass a path to ask `FileIO` to infer schema for you:

``` rust
use iceberg::Result;
use iceberg::io::{FileIO, S3_REGION};

// Build a memory file io.
let file_io = FileIO::from_path("memory:///")?.build()?;
// Build an fs file io.
let file_io = FileIO::from_path("fs:///tmp")?.build()?;
// Build an s3 file io.
let file_io = FileIO::from_path("s3://bucket/a")?
    .with_prop(S3_REGION, "us-east-1")
    .build()?;
```

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/index.html#how-to-use-fileio" class="doc-anchor">§</a>How to use `FileIO`

Currently `FileIO` provides simple methods for file operations:

- `delete`: Delete file.
- `exists`: Check if file exists.
- `new_input`: Create input file for reading.
- `new_output`: Create output file for writing.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">AwsCredential</a>  
Credential that holds the `access_key` and `secret_key`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.CustomAwsCredentialLoader.html" class="struct" title="struct iceberg::io::CustomAwsCredentialLoader">CustomAwsCredentialLoader</a>  
Custom AWS credential loader. This can be used to load credentials from a custom source, such as the AWS SDK.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>  
Container for storing type-safe extensions used to configure underlying FileIO behavior.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>  
FileIO implementation, used to manipulate files in underlying storage.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>  
Builder for [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileMetadata.html" class="struct" title="struct iceberg::io::FileMetadata">FileMetadata</a>  
The struct the represents the metadata of a file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html" class="struct" title="struct iceberg::io::InputFile">InputFile</a>  
Input file is used for reading from files.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>  
Output file is used for writing to files..

## Constants<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.CLIENT_REGION.html" class="constant" title="constant iceberg::io::CLIENT_REGION">CLIENT_REGION</a>  
Region to use for the S3 client.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_ACCESS_KEY_ID.html" class="constant" title="constant iceberg::io::S3_ACCESS_KEY_ID">S3_ACCESS_KEY_ID</a>  
S3 access key id.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_ALLOW_ANONYMOUS.html" class="constant" title="constant iceberg::io::S3_ALLOW_ANONYMOUS">S3_ALLOW_ANONYMOUS</a>  
Option to skip signing requests (e.g. for public buckets/folders).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_ASSUME_ROLE_ARN.html" class="constant" title="constant iceberg::io::S3_ASSUME_ROLE_ARN">S3_ASSUME_ROLE_ARN</a>  
If set, all AWS clients will assume a role of the given ARN, instead of using the default credential chain.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_ASSUME_ROLE_EXTERNAL_ID.html" class="constant" title="constant iceberg::io::S3_ASSUME_ROLE_EXTERNAL_ID">S3_ASSUME_ROLE_EXTERNAL_ID</a>  
Optional external ID used to assume an IAM role.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_ASSUME_ROLE_SESSION_NAME.html" class="constant" title="constant iceberg::io::S3_ASSUME_ROLE_SESSION_NAME">S3_ASSUME_ROLE_SESSION_NAME</a>  
Optional session name used to assume an IAM role.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_DISABLE_CONFIG_LOAD.html" class="constant" title="constant iceberg::io::S3_DISABLE_CONFIG_LOAD">S3_DISABLE_CONFIG_LOAD</a>  
Option to skip loading configuration from config file and the env.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_DISABLE_EC2_METADATA.html" class="constant" title="constant iceberg::io::S3_DISABLE_EC2_METADATA">S3_DISABLE_EC2_METADATA</a>  
Option to skip loading the credential from EC2 metadata (typically used in conjunction with `S3_ALLOW_ANONYMOUS`).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_ENDPOINT.html" class="constant" title="constant iceberg::io::S3_ENDPOINT">S3_ENDPOINT</a>  
Following are arguments for [s3 file io](https://py.iceberg.apache.org/configuration/#s3). S3 endpoint.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_PATH_STYLE_ACCESS.html" class="constant" title="constant iceberg::io::S3_PATH_STYLE_ACCESS">S3_PATH_STYLE_ACCESS</a>  
S3 Path Style Access.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_REGION.html" class="constant" title="constant iceberg::io::S3_REGION">S3_REGION</a>  
S3 region.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_SECRET_ACCESS_KEY.html" class="constant" title="constant iceberg::io::S3_SECRET_ACCESS_KEY">S3_SECRET_ACCESS_KEY</a>  
S3 secret access key.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_SESSION_TOKEN.html" class="constant" title="constant iceberg::io::S3_SESSION_TOKEN">S3_SESSION_TOKEN</a>  
S3 session token. This is required when using temporary credentials.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_SSE_KEY.html" class="constant" title="constant iceberg::io::S3_SSE_KEY">S3_SSE_KEY</a>  
S3 Server Side Encryption Key. If S3 encryption type is kms, input is a KMS Key ID. In case this property is not set, default key “aws/s3” is used. If encryption type is custom, input is a custom base-64 AES256 symmetric key.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_SSE_MD5.html" class="constant" title="constant iceberg::io::S3_SSE_MD5">S3_SSE_MD5</a>  
S3 Server Side Encryption MD5.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/constant.S3_SSE_TYPE.html" class="constant" title="constant iceberg::io::S3_SSE_TYPE">S3_SSE_TYPE</a>  
S3 Server Side Encryption Type.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.AwsCredentialLoad.html" class="trait" title="trait iceberg::io::AwsCredentialLoad">AwsCredentialLoad</a>  
Loader trait will try to load credential from different sources.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html" class="trait" title="trait iceberg::io::FileRead">FileRead</a>  
Trait for reading file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a>  
Trait for writing file.
