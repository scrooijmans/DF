# Rust Object Store Donation | InfluxData

### By [Raphael Taylor-Davies /](https://www.influxdata.com/blog/author/raphael-taylor-davies) [Use Cases](https://www.influxdata.com/blog/category/usecase), [Developer](https://www.influxdata.com/blog/category/tech)

Aug 22, 2022

Today we are happy to officially announce that InfluxData has donated a [generic object store implementation to the Apache Arrow project](https://github.com/apache/arrow-rs/issues/2030).

Using this crate, the same code can easily interact with AWS S3, Azure Blob Storage, Google Cloud Storage, local files, memory, and more by a simple runtime configuration change.

You can find the [latest release on crates.io](https://crates.io/crates/object_store).

We expect this will accelerate the pace of innovation within the Rust ecosystem. Whether you are building a cloud-agnostic service to handle user-uploaded videos, images, and documents, a high-performance analytics system, or something else that needs access to commodity object storage, this crate can help you and we can’t wait to see what people build with it.

## Why do you need an object store crate?

Aside from providing bulk data storage for many cloud-based services, we believe the future of analytic systems in particular involves querying data stored on object storage.

Object store is the generic term for what might be loosely described as an “infinite FTP server in the cloud”, that offers almost unlimited highly available and durable key-value storage on demand. Alongside virtual machines and block storage, object storage is one of the key commodity services provided by all modern cloud service providers. Examples include [S3](https://aws.amazon.com/s3/), [Microsoft Azure Blob Storage](https://azure.microsoft.com/en-gb/services/storage/blobs/#overview), [Google Cloud Storage](https://cloud.google.com/storage/), [MinIO](https://min.io/), [Ceph Object Gateway](https://docs.ceph.com/en/quincy/radosgw/index.html), [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_design.html#Introduction), and others.

To achieve this near-infinite scaling, object stores provide a subset of the functionality of traditional file systems such as [NTFS](https://docs.microsoft.com/en-us/windows-server/storage/file-server/ntfs-overview) or [ext4](https://docs.kernel.org/admin-guide/ext4.html). Specifically, they identify objects with a “key” and store arbitrary bytes as a value:

![Rust Object Store Donation - Figure 1](https://images.ctfassets.net/o7xu9whrs0u9/3a6on0Fu2MNaSu2Ps49H2p/3c190cbcf671e631ad24310295ec0e1d/rust-object-store-donation-figure-1.PNG) **Figure 1:** Object stores store arbitrary bytes identified by a string key.

Unlike filesystems, object stores typically lack an explicit notion of directories, and best practice uses a restricted subset of ASCII for keys. Instead, path-like traversal is achieved using LIST operations with a prefix, and illegal character sequencers are percent-encoded.

![Rust Object Store Donation - Figure 2](https://images.ctfassets.net/o7xu9whrs0u9/3SEfVhuyhjwhTuaTPXY3IU/a7135851515724c94b2532b6f82e0612/rust-object-store-donation-figure-2.PNG)

**Figure 2:** Object stores can LIST objects with a specified prefix, which can be used to group files together. In this example, asking for objects with prefix “`/pictures/`” results in all the `.jpg` objects, while asking for prefix “`/parquet/`” results in all the `.parquet` objects.

Consistently listing and traversing the quasi-directory structure encoded in the object keys across object store implementations and local file systems is one common source of frustration, as not only do filesystems behave very differently to object stores, but each of the object store implementations have their own quirks.

Having a focused, easy-to-use, high-performance, async object store library, written in idiomatic Rust, frees you from worrying about these details and lets you instead focus on your system’s logic. The underlying implementation is abstracted away from application code, and can easily be selected at runtime, allowing the same binary to run in multiple clouds.

This flexibility also facilitates local development as it allows testing against a local filesystem, or even an in-memory store, without requiring any additional binaries such as [MinIO](https://min.io/), and allowing the use of familiar tools such `ls, cat` or your choice of file browser.

## How to use it?

Here is a simplistic example that finds the number of zeros in files that are on remote object storage:

```
let object_store: Arc<dyn ObjectStore> = get_object_store();

    // list all objects in the "parquet" prefix (aka directory)
    let path: Path = "parquet".try_into().unwrap();
    let list_stream = object_store
        .list(Some(&path))
        .await
        .expect("Error listing files");

    // List all files in the store
    list_stream
        .map(|meta| async {
            let meta = meta.expect("Error listing");

            // fetch the bytes from object store
            let stream = object_store
                .get(&meta.location)
                .await
                .unwrap()
                .into_stream();

            // Count the zeros
            let num_zeros = stream
                .map(|bytes| {
                    let bytes = bytes.unwrap();
                    bytes.iter().filter(|b| **b == 0).count()
                })
                .collect::<Vec<usize>>()
                .await
                .into_iter()
                .sum::<usize>();

            (meta.location.to_string(), num_zeros)
        })
        .collect::<FuturesOrdered<_>>()
        .await
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .for_each(|i| println!("{} has {} zeros", i.0, i.1));
}
```

Which prints out something like:

```
test_fixtures/parquet/1.parquet has 174 zeros
test_fixtures/parquet/2.parquet has 53 zeros
```

As written the code lists the files (in a paginated way) and fetches their contents in parallel. This may not be great if there are thousands of files. However, we can easily take advantage of the Rust streams and change

```
.collect::<FuturesOrdered<_>>()
```

to

```
.buffered(10)
```

Which will now limit the program to 10 GET requests in parallel.

The coolest part of the object_store crate is that the same code works for all the different object stores, and the only thing that changes is the definition of `get_object_store`

To read from S3:

```
fn get_object_store() -> Arc<dyn ObjectStore> {
    let s3 = AmazonS3Builder::new()
        .with_access_key_id(ACCESS_KEY_ID)
        .with_secret_access_key(SECRET_KEY)
        .with_region(REGION)
        .with_bucket_name(BUCKET_NAME)
        .build()
        .expect("error creating s3");

    Arc::new(s3)
}
```

To read from Azure:

```
fn get_object_store() -> Arc<dyn ObjectStore> {
    let azure = MicrosoftAzureBuilder::new()
        .with_account(STORAGE_ACCOUNT)
        .with_access_key(ACCESS_KEY)
        .with_container_name(BUCKET_NAME)
        .build()
        .expect("error creating azure");

    Arc::new(azure)
}
```

To read from GCP:

```
fn get_object_store() -> Arc<dyn ObjectStore> {
    let gcs = GoogleCloudStorageBuilder::new()
        .with_service_account_path(PATH_TO_SERVICE_ACCOUNT_JSON)
        .with_bucket_name(BUCKET_NAME)
        .build()
        .expect("error creating gcs");
    Arc::new(gcs)
}
```

To read from the local filesystem:

```
fn get_object_store() -> Arc<dyn ObjectStore> {
    let local_fs =
        LocalFileSystem::new_with_prefix(PREFIX)
          .expect("Error creating local file system");
    Arc::new(local_fs)
}
```

To reiterate, the major benefit is that you do not have to integrate different abstractions for the different object stores – the client code is always the same and under the covers uses the appropriate optimized implementation.

The [object_store](https://crates.io/crates/object_store) crate is also extensible which allows plugging in other object storage systems, while still retaining the ability to read files from the local filesystem, to take advantage of optimized file access offered by some systems – see [GetFileResult](https://docs.rs/object_store/latest/object_store/enum.GetResult.html).

A more full-featured and working example can be found in the [rust_object_store_demo](https://github.com/alamb/rust_object_store_demo) repository.

## Why donate to Apache

The dream for Rust is the development productivity of Python or Ruby with the speed and memory efficiency of C/C++. Part of delivering this dream is ensuring that it integrates easily with the broader technology ecosystem, and in modern analytic systems this increasingly means data on object storage.

Thus, it is important to make it easy, and yet still efficient, for Rust programs to read and write data to object stores (AWS, S3, GCP). There are individual crates which implement cloud provider specific SDKs such as [rusoto_s3](https://crates.io/crates/rusoto_s3) or [Azure_storage](https://crates.io/crates/azure_storage); however, accessing the most common feature set via the same interface is often what is needed to accelerate the development of cross-cloud analytic systems. This crate is explicitly NOT meant to replace the full-blown cloud SDKs, but instead to provide a consistent object store abstraction that is portable across the many different underlying implementations.

We had exactly this requirement when we set out to develop [influxdb_iox](https://github.com/influxdata/influxdb). InfluxDB and InfluxData Cloud run on AWS, GCP, Azure, and on-prem, and we needed IOx to do so as well. We could not find an existing library that suited our needs, so the InfluxData IOx team developed one within our project.

This effort was originally implemented by Rust Ecosystem Legend Carol (Nichols II Goulding) @[carols10cents](https://github.com/carols10cents) (primary author of [the Rust Book](https://doc.rust-lang.org/stable/book/)) and heavily extended by [Marco Neumann](about:/cdn-cgi/l/email-protection#4b26252e3e262a25250b22252d273e332f2a3f2a65282426) and [Raphael Taylor-Davies](about:/cdn-cgi/l/email-protection#ccbeadbca4ada9a08ca5a2aaa0b9b4a8adb8ade2afa3a1) as we crafted its integration into DataFusion.

IOx uses the Rust, Apache Arrow, Apache Parquet and DataFusion projects, which we also contribute to heavily, and it was increasingly important that IOx’s object store interactions were efficient via DataFusion. As we investigated the alternatives, we hit the point where this required deeper integration with the object store.

We hope that this donation further accelerates the creation of high-quality analytic systems in Rust and can’t wait to see what the community builds with it! We especially hope that the alignment with Apache Arrow will permit an elegantly integrated experience with libraries that can easily and efficiently read arrow-compatible files, such as parquet, CSV and newline-delimited JSON, natively from local or remote object storage. For applications that desire SQL or other higher level query engine capabilities, check out [Apache Arrow DataFusion](https://github.com/apache/arrow-datafusion).

You can see more about the donation, and its rationale in [this GitHub issue](https://github.com/influxdata/object_store_rs/issues/41) and [this one as well](https://github.com/apache/arrow-rs/issues/2030).

## What’s next

In the near term, we plan better integration with the [parquet](https://docs.rs/parquet/latest/parquet/) crate. In particular the [async parquet reader](https://docs.rs/parquet/latest/parquet/arrow/async_reader/index.html) has been explicitly developed with a generic object_store crate in mind. It currently supports projection, and row-group level predicate pushdown to minimize the data fetched from object storage, and support for page and row-level predicate pushdown is likely to land in the next release slated for the 22nd August 2022.

We also expect to continue to improve the integration with [Apache Arrow DataFusion](https://github.com/apache/arrow-datafusion), ensuring it provides best in class support for querying data from object storage, efficiently decoupling IO from CPU-bound work, and making the most efficient use of modern multicore processors.

Finally there is an ongoing effort to move away from depending on large SDKs such as rusoto, and the Azure SDK for Rust. Whilst they have served us well, moving away from them will significantly reduce the dependency burden, simplify the implementation, and further improve consistency across the various implementations.

We think a thriving community drives everyone forward. We encourage you to check out the [crate](https://docs.rs/object_store/latest/object_store/), and lend us a hand! Try it out in your project and let us know how it goes, or find us on github [here](https://github.com/apache/arrow-rs/tree/master/object_store). There is a list of good open items for new comers [here](https://github.com/apache/arrow-rs/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22+label%3Aobject-store).

## Kudos

Thank you to [Raphael Taylor-Davies](about:/cdn-cgi/l/email-protection#a9dbc8d9c1c8ccc5e9c0c7cfc5dcd1cdc8ddc887cac6c4), [Paul Dix](about:/cdn-cgi/l/email-protection#b4c4d5c1d8f4dddad2d8c1ccd0d5c0d59ad7dbd9), [Nga Tran](about:/cdn-cgi/l/email-protection#9bf5efe9faf5dbf2f5fdf7eee3fffaeffab5f8f4f6), and [Marco Neumann](about:/cdn-cgi/l/email-protection#b1dcdfd4c4dcd0dfdff1d8dfd7ddc4c9d5d0c5d09fd2dedc) who reviewed early versions of this document and contributed many improvements.
