# Function resolve_bucket_region Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/resolve.rs.html#49-75" class="src">Source</a>

``` rust
pub async fn resolve_bucket_region(
    bucket: &str,
    client_options: &ClientOptions,
) -> Result<String>
```

Available on **non-WebAssembly and crate feature `aws`** only.

Expand description

Get the bucket region using the [HeadBucket API](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadBucket.html). This will fail if the bucket does not exist.
