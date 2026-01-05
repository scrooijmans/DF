# Constant S3_ASSUME_ROLE_ARN Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/storage_s3.rs.html#60" class="src">Source</a>

``` rust
pub const S3_ASSUME_ROLE_ARN: &str = "client.assume-role.arn";
```

Expand description

If set, all AWS clients will assume a role of the given ARN, instead of using the default credential chain.
