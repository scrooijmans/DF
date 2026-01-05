# Module cloud Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/lib.rs.html#14" class="src">Source</a>

Available on **crate feature `polars-io`** only.

Expand description

Interface with cloud storage through the object_store crate.

## Modules<a href="https://docs.rs/polars/latest/polars/prelude/cloud/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/credential_provider/index.html" class="mod" title="mod polars::prelude::cloud::credential_provider">credential_provider</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/index.html" class="mod" title="mod polars::prelude::cloud::options">options</a>

## Structs<a href="https://docs.rs/polars/latest/polars/prelude/cloud/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>  
Adaptor which wraps the interface of [ObjectStore::BufWriter](https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html) exposing a synchronous interface which implements `std::io::Write`.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>  
A location on cloud storage, may have wildcards.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>  
Options to connect to various cloud providers.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.PolarsObjectStore.html" class="struct" title="struct polars::prelude::cloud::PolarsObjectStore">PolarsObjectStore</a>  
Polars wrapper around [`ObjectStore`](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") functionality. This struct is cheaply cloneable.

## Enums<a href="https://docs.rs/polars/latest/polars/prelude/cloud/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

## Functions<a href="https://docs.rs/polars/latest/polars/prelude/cloud/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/fn.build_object_store.html" class="fn" title="fn polars::prelude::cloud::build_object_store">build_object_store</a>  
Build an [`ObjectStore`](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") based on the URL and passed in url. Return the cloud location and an implementation of the object store.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/fn.glob.html" class="fn" title="fn polars::prelude::cloud::glob">glob</a>  
List files with a prefix derived from the pattern.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/fn.object_path_from_str.html" class="fn" title="fn polars::prelude::cloud::object_path_from_str">object_path_from_str</a>  
Construct an object_store `Path` from a string without any encoding/decoding.

## Type Aliases<a href="https://docs.rs/polars/latest/polars/prelude/cloud/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/type.ObjectStorePath.html" class="type" title="type polars::prelude::cloud::ObjectStorePath">ObjectStorePath</a>
