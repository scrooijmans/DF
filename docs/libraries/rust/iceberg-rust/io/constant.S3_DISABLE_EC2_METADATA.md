# Constant S3_DISABLE_EC2_METADATA Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/storage_s3.rs.html#69" class="src">Source</a>

``` rust
pub const S3_DISABLE_EC2_METADATA: &str = "s3.disable-ec2-metadata";
```

Expand description

Option to skip loading the credential from EC2 metadata (typically used in conjunction with `S3_ALLOW_ANONYMOUS`).
