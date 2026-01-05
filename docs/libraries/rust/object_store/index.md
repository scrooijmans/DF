# Crate object_store Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#18-1673" class="src">Source</a>

Expand description

## <a href="https://docs.rs/object_store/latest/object_store/index.html#object_store" class="doc-anchor">§</a>object_store

This crate provides a uniform API for interacting with object storage services and local files via the [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") trait.

Using this crate, the same binary and code can run in multiple clouds and local test environments, via a simple runtime configuration change.

## <a href="https://docs.rs/object_store/latest/object_store/index.html#highlights" class="doc-anchor">§</a>Highlights

1.  A high-performance async API focused on providing a consistent interface mirroring that of object stores such as [S3](https://aws.amazon.com/s3/)

2.  Production quality, leading this crate to be used in large scale production systems, such as [crates.io](https://github.com/rust-lang/crates.io) and \[InfluxDB IOx\]

3.  Support for advanced functionality, including atomic, conditional reads and writes, vectored IO, bulk deletion, and more…

4.  Stable and predictable governance via the [Apache Arrow](https://arrow.apache.org/) project

5.  Small dependency footprint, depending on only a small number of common crates

Originally developed by [InfluxData](https://www.influxdata.com/) and subsequently donated to [Apache Arrow](https://arrow.apache.org/).

## <a href="https://docs.rs/object_store/latest/object_store/index.html#available-objectstore-implementations" class="doc-anchor">§</a>Available [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") Implementations

By default, this crate provides the following implementations:

- Memory: [`InMemory`](https://docs.rs/object_store/latest/object_store/memory/struct.InMemory.html "struct object_store::memory::InMemory")

Feature flags are used to enable support for other implementations:

- Local filesystem: [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem")
- [`gcp`](https://docs.rs/object_store/latest/object_store/gcp/index.html "mod object_store::gcp"): [Google Cloud Storage](https://cloud.google.com/storage/) support. See [`GoogleCloudStorageBuilder`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorageBuilder.html "struct object_store::gcp::GoogleCloudStorageBuilder")
- [`aws`](https://docs.rs/object_store/latest/object_store/aws/index.html "mod object_store::aws"): [Amazon S3](https://aws.amazon.com/s3/). See [`AmazonS3Builder`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3Builder.html "struct object_store::aws::AmazonS3Builder")
- [`azure`](https://docs.rs/object_store/latest/object_store/azure/index.html "mod object_store::azure"): [Azure Blob Storage](https://azure.microsoft.com/en-gb/services/storage/blobs/). See [`MicrosoftAzureBuilder`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzureBuilder.html "struct object_store::azure::MicrosoftAzureBuilder")
- [`http`](https://docs.rs/object_store/latest/object_store/http/index.html "mod object_store::http"): [HTTP/WebDAV Storage](https://datatracker.ietf.org/doc/html/rfc2518). See [`HttpBuilder`](https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html "struct object_store::http::HttpBuilder")

## <a href="https://docs.rs/object_store/latest/object_store/index.html#why-not-a-filesystem-interface" class="doc-anchor">§</a>Why not a Filesystem Interface?

The [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") interface is designed to mirror the APIs of object stores and *not* filesystems, and thus has stateless APIs instead of cursor based interfaces such as [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") or [`Seek`](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek") available in filesystems.

This design provides the following advantages:

- All operations are atomic, and readers cannot observe partial and/or failed writes
- Methods map directly to object store APIs, providing both efficiency and predictability
- Abstracts away filesystem and operating system specific quirks, ensuring portability
- Allows for functionality not native to filesystems, such as operation preconditions and atomic multipart uploads

This crate does provide [`BufReader`](https://docs.rs/object_store/latest/object_store/buffered/struct.BufReader.html "struct object_store::buffered::BufReader") and [`BufWriter`](https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html "struct object_store::buffered::BufWriter") adapters which provide a more filesystem-like API for working with the [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") trait, however, they should be used with care

## <a href="https://docs.rs/object_store/latest/object_store/index.html#adapters" class="doc-anchor">§</a>Adapters

[`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instances can be composed with various adapters which add additional functionality:

- Rate Throttling: [`ThrottleConfig`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html "struct object_store::throttle::ThrottleConfig")
- Concurrent Request Limit: [`LimitStore`](https://docs.rs/object_store/latest/object_store/limit/struct.LimitStore.html "struct object_store::limit::LimitStore")

## <a href="https://docs.rs/object_store/latest/object_store/index.html#configuration-system" class="doc-anchor">§</a>Configuration System

This crate provides a configuration system inspired by the APIs exposed by [fsspec](https://filesystem-spec.readthedocs.io/en/latest/api.html#fsspec.filesystem), [PyArrow FileSystem](https://arrow.apache.org/docs/python/generated/pyarrow.fs.FileSystem.html#pyarrow.fs.FileSystem.from_uri), and [Hadoop FileSystem](https://hadoop.apache.org/docs/r3.0.0/api/org/apache/hadoop/fs/FileSystem.html#get-java.net.URI-org.apache.hadoop.conf.Configuration-), allowing creating a [`DynObjectStore`](https://docs.rs/object_store/latest/object_store/type.DynObjectStore.html "type object_store::DynObjectStore") from a URL and an optional list of key value pairs. This provides a flexible interface to support a wide variety of user-defined store configurations, with minimal additional application complexity.

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
// Can manually create a specific store variant using the appropriate builder
let store: AmazonS3 = AmazonS3Builder::from_env()
    .with_bucket_name("my-bucket").build().unwrap();

// Alternatively can create an ObjectStore from an S3 URL
let url = Url::parse("s3://bucket/path").unwrap();
let (store, path) = parse_url(&url).unwrap();
assert_eq!(path.as_ref(), "path");

// Potentially with additional options
let (store, path) = parse_url_opts(&url, vec![("aws_access_key_id", "...")]).unwrap();

// Or with URLs that encode the bucket name in the URL path
let url = Url::parse("https://ACCOUNT_ID.r2.cloudflarestorage.com/bucket/path").unwrap();
let (store, path) = parse_url(&url).unwrap();
assert_eq!(path.as_ref(), "path");
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#list-objects" class="doc-anchor">§</a>List objects

Use the [`ObjectStore::list`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list "method object_store::ObjectStore::list") method to iterate over objects in remote storage or files in the local filesystem:

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
// create an ObjectStore
let object_store: Arc<dyn ObjectStore> = get_object_store();

// Recursively list all files below the 'data' path.
// 1. On AWS S3 this would be the 'data/' prefix
// 2. On a local filesystem, this would be the 'data' directory
let prefix = Path::from("data");

// Get an `async` stream of Metadata objects:
let mut list_stream = object_store.list(Some(&prefix));

// Print a line about each object
while let Some(meta) = list_stream.next().await.transpose().unwrap() {
    println!("Name: {}, size: {}", meta.location, meta.size);
}
```

Which will print out something like the following:

``` text
Name: data/file01.parquet, size: 112832
Name: data/file02.parquet, size: 143119
Name: data/child/file03.parquet, size: 100
...
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#fetch-objects" class="doc-anchor">§</a>Fetch objects

Use the [`ObjectStore::get`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get "method object_store::ObjectStore::get") method to fetch the data bytes from remote storage or files in the local filesystem as a stream.

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
// Create an ObjectStore
let object_store: Arc<dyn ObjectStore> = get_object_store();

// Retrieve a specific file
let path = Path::from("data/file01.parquet");

// Fetch just the file metadata
let meta = object_store.head(&path).await.unwrap();
println!("{meta:?}");

// Fetch the object including metadata
let result: GetResult = object_store.get(&path).await.unwrap();
assert_eq!(result.meta, meta);

// Buffer the entire object in memory
let object: Bytes = result.bytes().await.unwrap();
assert_eq!(object.len() as u64, meta.size);

// Alternatively stream the bytes from object storage
let stream = object_store.get(&path).await.unwrap().into_stream();

// Count the '0's using `try_fold` from `TryStreamExt` trait
let num_zeros = stream
    .try_fold(0, |acc, bytes| async move {
        Ok(acc + bytes.iter().filter(|b| **b == 0).count())
    }).await.unwrap();

println!("Num zeros in {} is {}", path, num_zeros);
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#put-object" class="doc-anchor">§</a>Put Object

Use the [`ObjectStore::put`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put "method object_store::ObjectStore::put") method to atomically write data.

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/file1");
let payload = PutPayload::from_static(b"hello");
object_store.put(&path, payload).await.unwrap();
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#multipart-upload" class="doc-anchor">§</a>Multipart Upload

Use the [`ObjectStore::put_multipart`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart "method object_store::ObjectStore::put_multipart") method to atomically write a large amount of data

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let upload =  object_store.put_multipart(&path).await.unwrap();
let mut write = WriteMultipart::new(upload);
write.write(b"hello");
write.finish().await.unwrap();
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#vectored-read" class="doc-anchor">§</a>Vectored Read

A common pattern, especially when reading structured datasets, is to need to fetch multiple, potentially non-contiguous, ranges of a particular object.

[`ObjectStore::get_ranges`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_ranges "method object_store::ObjectStore::get_ranges") provides an efficient way to perform such vectored IO, and will automatically coalesce adjacent ranges into an appropriate number of parallel requests.

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let ranges = object_store.get_ranges(&path, &[90..100, 400..600, 0..10]).await.unwrap();
assert_eq!(ranges.len(), 3);
assert_eq!(ranges[0].len(), 10);
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#vectored-write" class="doc-anchor">§</a>Vectored Write

When writing data it is often the case that the size of the output is not known ahead of time.

A common approach to handling this is to bump-allocate a `Vec`, whereby the underlying allocation is repeatedly reallocated, each time doubling the capacity. The performance of this is suboptimal as reallocating memory will often involve copying it to a new location.

Fortunately, as [`PutPayload`](https://docs.rs/object_store/latest/object_store/struct.PutPayload.html "struct object_store::PutPayload") does not require memory regions to be contiguous, it is possible to instead allocate memory in chunks and avoid bump allocating. [`PutPayloadMut`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html "struct object_store::PutPayloadMut") encapsulates this approach

<a href="https://docs.rs/object_store/latest/object_store/index.html#" class="tooltip" title="This example is not tested on wasm32">ⓘ</a>

``` rust
let object_store: Arc<dyn ObjectStore> = get_object_store();
let path = Path::from("data/large_file");
let mut buffer = PutPayloadMut::new().with_block_size(8192);
for _ in 0..22 {
    buffer.extend_from_slice(&[0; 1024]);
}
let payload = buffer.freeze();

// Payload consists of 3 separate 8KB allocations
assert_eq!(payload.as_ref().len(), 3);
assert_eq!(payload.as_ref()[0].len(), 8192);
assert_eq!(payload.as_ref()[1].len(), 8192);
assert_eq!(payload.as_ref()[2].len(), 6144);

object_store.put(&path, payload).await.unwrap();
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#conditional-fetch" class="doc-anchor">§</a>Conditional Fetch

More complex object retrieval can be supported by [`ObjectStore::get_opts`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.get_opts "method object_store::ObjectStore::get_opts").

For example, efficiently refreshing a cache without re-fetching the entire object data if the object hasn’t been modified.

``` rust
struct CacheEntry {
    /// Data returned by last request
    data: Bytes,
    /// ETag identifying the object returned by the server
    e_tag: String,
    /// Instant of last refresh
    refreshed_at: Instant,
}

/// Example cache that checks entries after 10 seconds for a new version
struct Cache {
    entries: HashMap<Path, CacheEntry>,
    store: Arc<dyn ObjectStore>,
}

impl Cache {
    pub async fn get(&mut self, path: &Path) -> Result<Bytes> {
        Ok(match self.entries.get_mut(path) {
            Some(e) => match e.refreshed_at.elapsed() < Duration::from_secs(10) {
                true => e.data.clone(), // Return cached data
                false => { // Check if remote version has changed
                    let opts = GetOptions {
                        if_none_match: Some(e.e_tag.clone()),
                        ..GetOptions::default()
                    };
                    match self.store.get_opts(&path, opts).await {
                        Ok(d) => e.data = d.bytes().await?,
                        Err(Error::NotModified { .. }) => {} // Data has not changed
                        Err(e) => return Err(e),
                    };
                    e.refreshed_at = Instant::now();
                    e.data.clone()
                }
            },
            None => { // Not cached, fetch data
                let get = self.store.get(&path).await?;
                let e_tag = get.meta.e_tag.clone();
                let data = get.bytes().await?;
                if let Some(e_tag) = e_tag {
                    let entry = CacheEntry {
                        e_tag,
                        data: data.clone(),
                        refreshed_at: Instant::now(),
                    };
                    self.entries.insert(path.clone(), entry);
                }
                data
            }
        })
    }
}
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#conditional-put" class="doc-anchor">§</a>Conditional Put

The default behaviour when writing data is to upsert any existing object at the given path, overwriting any previous value. More complex behaviours can be achieved using [`PutMode`](https://docs.rs/object_store/latest/object_store/enum.PutMode.html "enum object_store::PutMode"), and can be used to build [Optimistic Concurrency Control](https://en.wikipedia.org/wiki/Optimistic_concurrency_control) based transactions. This facilitates building metadata catalogs, such as [Apache Iceberg](https://iceberg.apache.org/) or [Delta Lake](https://delta.io/), directly on top of object storage, without relying on a separate DBMS.

``` rust
let store = get_object_store();
let path = Path::from("test");

// Perform a conditional update on path
loop {
    // Perform get request
    let r = store.get(&path).await.unwrap();

    // Save version information fetched
    let version = UpdateVersion {
        e_tag: r.meta.e_tag.clone(),
        version: r.meta.version.clone(),
    };

    // Compute new version of object contents
    let new = do_update(r.bytes().await.unwrap());

    // Attempt to commit transaction
    match store.put_opts(&path, new.into(), PutMode::Update(version).into()).await {
        Ok(_) => break, // Successfully committed
        Err(Error::Precondition { .. }) => continue, // Object has changed, try again
        Err(e) => panic!("{e}")
    }
}
```

## <a href="https://docs.rs/object_store/latest/object_store/index.html#tls-certificates" class="doc-anchor">§</a>TLS Certificates

Stores that use HTTPS/TLS (this is true for most cloud stores) can choose the source of their [CA](https://en.wikipedia.org/wiki/Certificate_authority) certificates. By default the system-bundled certificates are used (see [`rustls-native-certs`](https://crates.io/crates/rustls-native-certs/)). The `tls-webpki-roots` feature switch can be used to also bundle Mozilla’s root certificates with the library/application (see [`webpki-roots`](https://crates.io/crates/webpki-roots)).

## <a href="https://docs.rs/object_store/latest/object_store/index.html#customizing-http-clients" class="doc-anchor">§</a>Customizing HTTP Clients

Many [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementations permit customization of the HTTP client via the [`HttpConnector`](https://docs.rs/object_store/latest/object_store/client/trait.HttpConnector.html "trait object_store::client::HttpConnector") trait and utilities in the [`client`](https://docs.rs/object_store/latest/object_store/client/index.html "mod object_store::client") module. Examples include injecting custom HTTP headers or using an alternate tokio Runtime I/O requests.

## Re-exports<a href="https://docs.rs/object_store/latest/object_store/index.html#reexports" class="anchor">§</a>

`pub use client::`<a href="https://docs.rs/object_store/latest/object_store/client/enum.ClientConfigKey.html" class="enum" title="enum object_store::client::ClientConfigKey"><code>ClientConfigKey</code></a>`;``cloud`

`pub use client::`<a href="https://docs.rs/object_store/latest/object_store/client/struct.ClientOptions.html" class="struct" title="struct object_store::client::ClientOptions"><code>ClientOptions</code></a>`;``cloud`

`pub use client::`<a href="https://docs.rs/object_store/latest/object_store/client/trait.CredentialProvider.html" class="trait" title="trait object_store::client::CredentialProvider"><code>CredentialProvider</code></a>`;``cloud`

`pub use client::`<a href="https://docs.rs/object_store/latest/object_store/client/struct.StaticCredentialProvider.html" class="struct" title="struct object_store::client::StaticCredentialProvider"><code>StaticCredentialProvider</code></a>`;``cloud`

`pub use client::`<a href="https://docs.rs/object_store/latest/object_store/client/struct.Certificate.html" class="struct" title="struct object_store::client::Certificate"><code>Certificate</code></a>`;``cloud` and non-WebAssembly

## Modules<a href="https://docs.rs/object_store/latest/object_store/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/index.html" class="mod" title="mod object_store::aws">aws</a>`aws`  
An object store implementation for S3

<a href="https://docs.rs/object_store/latest/object_store/azure/index.html" class="mod" title="mod object_store::azure">azure</a>`azure`  
An object store implementation for Azure blob storage

<a href="https://docs.rs/object_store/latest/object_store/buffered/index.html" class="mod" title="mod object_store::buffered">buffered</a>  
Utilities for performing tokio-style buffered IO

<a href="https://docs.rs/object_store/latest/object_store/chunked/index.html" class="mod" title="mod object_store::chunked">chunked</a>Non-WebAssembly  
A [`ChunkedStore`](https://docs.rs/object_store/latest/object_store/chunked/struct.ChunkedStore.html "struct object_store::chunked::ChunkedStore") that can be used to test streaming behaviour

<a href="https://docs.rs/object_store/latest/object_store/client/index.html" class="mod" title="mod object_store::client">client</a>`cloud`  
Generic utilities for [`reqwest`](https://docs.rs/reqwest/0.12.23/x86_64-unknown-linux-gnu/reqwest/index.html "mod reqwest") based [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementations

<a href="https://docs.rs/object_store/latest/object_store/delimited/index.html" class="mod" title="mod object_store::delimited">delimited</a>  
Utility for streaming newline delimited files from object storage

<a href="https://docs.rs/object_store/latest/object_store/gcp/index.html" class="mod" title="mod object_store::gcp">gcp</a>`gcp`  
An object store implementation for Google Cloud Storage

<a href="https://docs.rs/object_store/latest/object_store/http/index.html" class="mod" title="mod object_store::http">http</a>`http`  
An object store implementation for generic HTTP servers

<a href="https://docs.rs/object_store/latest/object_store/integration/index.html" class="mod" title="mod object_store::integration">integration</a>`integration`  
Integration tests for custom object store implementations

<a href="https://docs.rs/object_store/latest/object_store/limit/index.html" class="mod" title="mod object_store::limit">limit</a>  
An object store that limits the maximum concurrency of the wrapped implementation

<a href="https://docs.rs/object_store/latest/object_store/list/index.html" class="mod" title="mod object_store::list">list</a>  
Paginated Listing

<a href="https://docs.rs/object_store/latest/object_store/local/index.html" class="mod" title="mod object_store::local">local</a>`fs` and non-WebAssembly  
An object store implementation for a local filesystem

<a href="https://docs.rs/object_store/latest/object_store/memory/index.html" class="mod" title="mod object_store::memory">memory</a>  
An in-memory object store implementation

<a href="https://docs.rs/object_store/latest/object_store/multipart/index.html" class="mod" title="mod object_store::multipart">multipart</a>  
Cloud Multipart Upload

<a href="https://docs.rs/object_store/latest/object_store/path/index.html" class="mod" title="mod object_store::path">path</a>  
Path abstraction for Object Storage

<a href="https://docs.rs/object_store/latest/object_store/prefix/index.html" class="mod" title="mod object_store::prefix">prefix</a>  
An object store wrapper handling a constant path prefix

<a href="https://docs.rs/object_store/latest/object_store/registry/index.html" class="mod" title="mod object_store::registry">registry</a>  
Map object URLs to [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore")

<a href="https://docs.rs/object_store/latest/object_store/signer/index.html" class="mod" title="mod object_store::signer">signer</a>`cloud`  
Abstraction of signed URL generation for those object store implementations that support it

<a href="https://docs.rs/object_store/latest/object_store/throttle/index.html" class="mod" title="mod object_store::throttle">throttle</a>  
A throttling object store wrapper

## Structs<a href="https://docs.rs/object_store/latest/object_store/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>  
The value of an [`Attribute`](https://docs.rs/object_store/latest/object_store/enum.Attribute.html "enum object_store::Attribute")

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>  
Additional attributes of an object

<a href="https://docs.rs/object_store/latest/object_store/struct.AttributesIter.html" class="struct" title="struct object_store::AttributesIter">AttributesIter</a>  
Iterator over [`Attributes`](https://docs.rs/object_store/latest/object_store/struct.Attributes.html "struct object_store::Attributes")

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html" class="struct" title="struct object_store::BackoffConfig">BackoffConfig</a>`cloud`  
Exponential backoff with decorrelated jitter algorithm

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>  
A type map of protocol extensions.

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>  
Options for a get request, such as range

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>  
Result for a get request

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>  
A set of HTTP headers

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>  
Represents an HTTP header field value.

<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html" class="struct" title="struct object_store::ListResult">ListResult</a>  
Result of a list call that includes objects, prefixes (directories) and a token for the next set of results. Individual result sets may be limited to 1,000 objects based on the underlying object storage’s limitations.

<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>  
The metadata that describes an object.

<a href="https://docs.rs/object_store/latest/object_store/struct.PutMultipartOptions.html" class="struct" title="struct object_store::PutMultipartOptions">PutMultipartOptions</a>  
Options for [`ObjectStore::put_multipart_opts`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts "method object_store::ObjectStore::put_multipart_opts")

<a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>  
Options for a put request

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>  
A cheaply cloneable, ordered collection of [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadIntoIter.html" class="struct" title="struct object_store::PutPayloadIntoIter">PutPayloadIntoIter</a>  
An owning iterator of [`PutPayload`](https://docs.rs/object_store/latest/object_store/struct.PutPayload.html "struct object_store::PutPayload")

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadIter.html" class="struct" title="struct object_store::PutPayloadIter">PutPayloadIter</a>  
An iterator over [`PutPayload`](https://docs.rs/object_store/latest/object_store/struct.PutPayload.html "struct object_store::PutPayload")

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html" class="struct" title="struct object_store::PutPayloadMut">PutPayloadMut</a>  
A builder for [`PutPayload`](https://docs.rs/object_store/latest/object_store/struct.PutPayload.html "struct object_store::PutPayload") that avoids reallocating memory

<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>  
Result for a put request

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>`cloud`  
The configuration for how to respond to request errors

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>  
A collection of key value pairs used to annotate objects

<a href="https://docs.rs/object_store/latest/object_store/struct.UpdateVersion.html" class="struct" title="struct object_store::UpdateVersion">UpdateVersion</a>  
Uniquely identifies a version of an object to update

<a href="https://docs.rs/object_store/latest/object_store/struct.WriteMultipart.html" class="struct" title="struct object_store::WriteMultipart">WriteMultipart</a>  
A synchronous write API for uploading data in parallel in fixed size chunks

## Enums<a href="https://docs.rs/object_store/latest/object_store/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>  
Additional object attribute types

<a href="https://docs.rs/object_store/latest/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>  
A specialized `Error` for object store-related errors

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>  
Request only a portion of an object’s bytes

<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html" class="enum" title="enum object_store::GetResultPayload">GetResultPayload</a>  
The kind of a [`GetResult`](https://docs.rs/object_store/latest/object_store/struct.GetResult.html "struct object_store::GetResult")

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>  
Recognizes various URL formats, identifying the relevant [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore")

<a href="https://docs.rs/object_store/latest/object_store/enum.PutMode.html" class="enum" title="enum object_store::PutMode">PutMode</a>  
Configure preconditions for the put operation

## Constants<a href="https://docs.rs/object_store/latest/object_store/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/constant.OBJECT_STORE_COALESCE_DEFAULT.html" class="constant" title="constant object_store::OBJECT_STORE_COALESCE_DEFAULT">OBJECT_STORE_COALESCE_DEFAULT</a>  
Range requests with a gap less than or equal to this, will be coalesced into a single request by [`coalesce_ranges`](https://docs.rs/object_store/latest/object_store/fn.coalesce_ranges.html "fn object_store::coalesce_ranges")

## Traits<a href="https://docs.rs/object_store/latest/object_store/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>  
A trait allowing writing an object in fixed size chunks

<a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>  
Universal API to multiple object store services.

## Functions<a href="https://docs.rs/object_store/latest/object_store/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/fn.coalesce_ranges.html" class="fn" title="fn object_store::coalesce_ranges">coalesce_ranges</a>  
Takes a function `fetch` that can fetch a range of bytes and uses this to fetch the provided byte `ranges`

<a href="https://docs.rs/object_store/latest/object_store/fn.collect_bytes.html" class="fn" title="fn object_store::collect_bytes">collect_bytes</a>  
Collect a stream into [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") avoiding copying in the event of a single chunk

<a href="https://docs.rs/object_store/latest/object_store/fn.parse_url.html" class="fn" title="fn object_store::parse_url">parse_url</a>  
Create an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") based on the provided `url`

<a href="https://docs.rs/object_store/latest/object_store/fn.parse_url_opts.html" class="fn" title="fn object_store::parse_url_opts">parse_url_opts</a>  
Create an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") based on the provided `url` and options

## Type Aliases<a href="https://docs.rs/object_store/latest/object_store/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/type.DynObjectStore.html" class="type" title="type object_store::DynObjectStore">DynObjectStore</a>  
An alias for a dynamically dispatched object store implementation.

<a href="https://docs.rs/object_store/latest/object_store/type.MultipartId.html" class="type" title="type object_store::MultipartId">MultipartId</a>  
Id type for multipart uploads.

<a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>  
A specialized `Result` for object store-related errors

<a href="https://docs.rs/object_store/latest/object_store/type.UploadPart.html" class="type" title="type object_store::UploadPart">UploadPart</a>  
An upload part request
