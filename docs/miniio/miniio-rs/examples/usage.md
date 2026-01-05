## 1. Conclusion

In this development, we successfully connected to Minio using the AWS SDK in Rust and performed S3-compatible bucket operations. By using a custom `resolver`, we specified the Minio endpoint and employed Tokio for asynchronous programming.

Here is the key code for connecting to Minio and creating a bucket:

```rust
use aws_sdk_s3::{Client, Credentials, Region};
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::endpoint::{Endpoint, ResolveEndpoint, EndpointFuture, Params};

#[derive(Debug)]
struct DefaultResolver {
    endpoint: String,
}

impl ResolveEndpoint for DefaultResolver {
    fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'static> {
        let endpoint = Endpoint::immutable(self.endpoint.parse().unwrap());
        EndpointFuture::ready(Ok(endpoint))
    }
}

#[tokio::main]
async fn main() {
    let endpoint = "http://minio:9000".to_string();
    let access_key = "XLKtI4YgqXJA11JlqyB7";
    let secret_key = "qSlpgFhm4uNX01dUgfRvuCmc36zpxPpHQgTKOBcO";

    let resolver = DefaultResolver { endpoint };

    let config = Builder::new()
        .endpoint_resolver(resolver)
        .region(Region::new("us-east-1"))
        .credentials_provider(Credentials::new(access_key, secret_key, None, None, "static"))
        .build();

    let client = Client::from_conf(config);

    let bucket_name = "test-bucket";
    let result = client.create_bucket().bucket(bucket_name).send().await;

    match result {
        Ok(_) => println!("Bucket created successfully"),
        Err(e) => eprintln!("Failed to create bucket: {:?}", e),
    }
}
```

This code demonstrates how to connect to Minio and create a bucket with the specified bucket name. By using the `resolver`, we can specify the custom Minio endpoint.

You can find the final code at the following link:

[Final code here](https://gist.github.com/u-na-gi/3e16ee4b80f0fc4370159692445cbf06)


## 2. Environment Setup and Project Configuration

In this project, we set up an environment to connect to Minio using Rust and perform S3-compatible object storage operations. Assuming that a Rust environment is already prepared, we will explain how to set up Minio and the project's dependencies.

### 2.1 Setting Up Minio with Docker Compose

To run Minio locally, we used Docker Compose. You can easily start Minio using the following settings:

```yaml
version: "3"
services:
  minio:
    image: minio/minio:RELEASE.2024-08-17T01-24-54Z
    environment:
      MINIO_ACCESS_KEY: XLKtI4YgqXJA11JlqyB7
      MINIO_SECRET_KEY: qSlpgFhm4uNX01dUgfRvuCmc36zpxPpHQgTKOBcO
    command: server /data --address :9000 --console-address :9001
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - minio-data:/data
volumes:
  minio-data:
```

This configuration exposes Minio on port `9000` and the management console on port `9001`.

Next, run the following command to start Minio:

```bash
docker-compose up -d
```

This will launch Minio locally, and you can test the S3-compatible object storage. The management console can be accessed at `http://localhost:9001`.

### 2.2 Cargo Project Setup

Next, let's configure the `Cargo.toml` to connect to S3 using Rust. We specified dependencies like `aws-sdk-s3` and `aws-config`, enabling us to perform S3 operations and connect to Minio.

```toml
[package]
name = "core"
version = "0.1.0"
edition = "2021"

[dependencies]
aws-sdk-s3 = { version = "1.52.0", features = ["behavior-version-latest"] }
aws-config = { version = "1.0.1", features = ["behavior-version-latest"] }
tokio = { version = "1.40.0", features = ["full"] }
aws-smithy-runtime = { version = "1.7.1" }
aws-smithy-runtime-api = { version = "1.7.2" }
aws-smithy-types = { version = "1.2.7" }

[dependencies.uuid]
version = "1.10.0"
features = [
    "v4",                # Lets you generate random UUIDs
    "fast-rng",          # Use a faster (but still sufficiently random) RNG
    "macro-diagnostics", # Enable better diagnostics for compile-time UUIDs
]
```

This setup also includes the `tokio` crate for asynchronous S3 operations and the `uuid` crate for generating unique identifiers. With this configuration, we are ready to perform S3-compatible bucket operations and connect to Minio.


## 3. Troubleshooting the Connection to Minio

When connecting to Minio with Rust, we encountered several errors but resolved them step by step, eventually achieving a working connection to S3-compatible storage. This section introduces the errors and solutions.

### 3.1 `Invalid client configuration: A behavior major version must be set` Error

When connecting to Minio, the following error occurred:

```
Invalid client configuration: A behavior major version must be set when sending a request or constructing a client. You must set it during client construction or by enabling the `behavior-version-latest` cargo feature.
```

This error occurred because the S3 client was not configured correctly. To resolve this, we enabled the `behavior-version-latest` feature in the `Cargo.toml` file.

```toml
aws-sdk-s3 = { version = "1.52.0", features = ["behavior-version-latest"] }
```

### 3.2 Name Resolution Error and Custom Endpoint Configuration

While trying to create a bucket in Minio, we encountered the following name resolution error:

```
dns error: failed to lookup address information: Name or service not known
```

The root cause was that we had not configured the `DefaultResolver`. To resolve Minio’s custom endpoint correctly, we needed to configure a custom endpoint resolver.

The `endpoint_resolver` documentation provided the following information:

```
Sets the endpoint resolver to use when making requests.

When unset, the client will use a generated endpoint resolver based on the endpoint resolution rules for aws_sdk_s3.

Note: setting an endpoint resolver will replace any endpoint URL that has been set. This method accepts an endpoint resolver specific to this service. If you want to provide a shared endpoint resolver, use Self::set_endpoint_resolver.
```

Using this, we implemented the `DefaultResolver` and set a custom endpoint to resolve this error:

```rust
#[derive(Debug)]
struct DefaultResolver {
    endpoint: String,
}

impl ResolveEndpoint for DefaultResolver {
    fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'static> {
        let endpoint = Endpoint::immutable(self.endpoint.parse().unwrap());
        EndpointFuture::ready(Ok(endpoint))
    }
}

let config = aws_sdk_s3::config::Builder::new()
    .endpoint_resolver(DefaultResolver { endpoint: "http://minio:9000".to_string() })
    .region(Region::new("us-east-1"))
    .credentials_provider(Credentials::new(access_key, secret_key, None, None, "static"))
    .build();
let client = Client::from_conf(config);
```

By using the `DefaultResolver`, we were able to configure Minio's custom endpoint and resolve the name resolution error.

### 3.3 `InvalidAccessKeyId` Error

When trying to create a bucket in Minio, the following error occurred:

```
InvalidAccessKeyId: The Access Key Id you provided does not exist in our records.
```

This error was caused by incorrect access keys being set in the Docker environment. By updating the `docker-compose.yml` file and providing the correct access keys, we resolved this error:

```yaml
environment:
  MINIO_ACCESS_KEY: XLKtI4YgqXJA11JlqyB7
  MINIO_SECRET_KEY: qSlpgFhm4uNX01dUgfRvuCmc36zpxPpHQgTKOBcO
```


## 4. Running Tests Against Minio

After successfully connecting to Minio and creating a bucket, the next step was to confirm the functionality through testing. We utilized `tokio` to support asynchronous testing in Rust. Here, we explain the process of testing the bucket creation with Minio.

### 4.1 Asynchronous Test Setup

Rust's standard testing framework doesn't support asynchronous functions. To perform asynchronous testing, we used the `#[tokio::test]` attribute, which allows us to test asynchronous functions.

Below is an example test for creating a bucket in Minio:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_minio_create_bucket() {
        let is_minio = true;
        let prefix: String = Uuid::new_v4()
            .as_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        let test_bucket_name = format!("test-bucket-{prefix}", prefix = prefix);


        let endpoint = format!(
            "http://minio:9000/{bucket_name}",
            bucket_name = test_bucket_name
        );
        let client = S3::new(is_minio, Some(endpoint)).await;

        let result = client.create_bucket(&test_bucket_name).await;

        match result {
            Ok(Some(output)) => {
                println!("Bucket created successfully: {:?}", output);
            }
            Ok(None) => {
                println!("Bucket already exists or no output was returned.");
            }
            Err(e) => {
                eprintln!("Failed to create bucket: {}", e);
                panic!("Test failed due to bucket creation error.");
            }
        };
    }
}
```

### 4.2 Testing Bucket Creation

In this test, we generate a unique bucket name using a UUID and try to create that bucket in Minio. If the test succeeds, we see the message "Bucket created successfully." If the bucket already exists, the test proceeds without errors.

The `test_minio_create_bucket` flow is as follows:

1. **Generate a UUID** to create a unique bucket name.
2. Set the Minio endpoint and initialize the `S3` client.
3. Use the `create_bucket` function to attempt bucket creation.
4. Print the result on success, and if the bucket already exists, continue without errors.

### 4.3 Confirming Test Results

Tests can be executed with `cargo test`. After running `cargo test`, the following output confirms whether the test was successful:

```bash
running 1 test
test s3::tests::test_minio_create_bucket ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

This result confirms that bucket creation with Minio is working as expected.


## 5. Summary and Reflection

In this project, we successfully used the AWS SDK in Rust to connect to Minio and perform S3-compatible bucket operations. Below are the key points we reviewed.

### 5.1 Connecting to Minio

We used Docker Compose to quickly set up an S3-compatible storage solution with Minio running locally. In Rust, we connected to Minio using the AWS SDK and introduced Tokio to enable asynchronous programming.

We faced challenges with endpoint configuration at first but overcame them by using the `DefaultResolver` to correctly resolve Minio’s custom endpoint, solving the name resolution issue.

### 5.2 Error Resolution Process

Several errors arose while connecting to Minio, but resolving them sequentially allowed us to achieve smooth operation. The key points were enabling the `behavior-version-latest` feature and properly configuring access keys to resolve the `InvalidAccessKeyId` error.

The biggest challenge was configuring the custom endpoint for Minio, which was resolved by implementing `DefaultResolver`.

### 5.3 Running Tests

We utilized Rust’s asynchronous test feature and ran tests to verify that bucket creation with Minio worked correctly. The `cargo test` confirmed that our S3-compatible operations were functioning as expected.

### 5.4 Future Outlook

Through this development, we gained a foundational understanding of how to handle S3-compatible storage like Minio in Rust. Moving forward, we plan to implement more complex S3 operations, such as object upload and deletion, and expand our testing. Additionally, we aim to optimize error handling further, with retry strategies and improved logging.
