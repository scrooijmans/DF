# Constant S3_SSE_KEY Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/storage_s3.rs.html#55" class="src">Source</a>

``` rust
pub const S3_SSE_KEY: &str = "s3.sse.key";
```

Expand description

S3 Server Side Encryption Key. If S3 encryption type is kms, input is a KMS Key ID. In case this property is not set, default key “aws/s3” is used. If encryption type is custom, input is a custom base-64 AES256 symmetric key.
