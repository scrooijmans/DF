# Enum Scheme Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/scheme.rs.html#34-171" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum Scheme {
Show 65 variants    AliyunDrive,
    Atomicserver,
    Azblob,
    Azdls,
    B2,
    Compfs,
    Seafile,
    Upyun,
    VercelBlob,
    YandexDisk,
    Pcloud,
    Koofr,
    Cacache,
    CloudflareKv,
    Cos,
    D1,
    Dashmap,
    Etcd,
    Foundationdb,
    Dbfs,
    Fs,
    Ftp,
    Gcs,
    Ghac,
    Hdfs,
    Http,
    Huggingface,
    Alluxio,
    Ipfs,
    Ipmfs,
    Icloud,
    Memcached,
    Memory,
    MiniMoka,
    Moka,
    Monoiofs,
    Obs,
    Onedrive,
    Gdrive,
    Dropbox,
    Oss,
    Persy,
    Redis,
    Postgresql,
    Mysql,
    Sqlite,
    Rocksdb,
    S3,
    Sftp,
    Sled,
    Swift,
    VercelArtifacts,
    Webdav,
    Webhdfs,
    Redb,
    Tikv,
    Azfile,
    Mongodb,
    Gridfs,
    Github,
    HdfsNative,
    Surrealdb,
    Lakefs,
    NebulaGraph,
    Custom(&'static str),
}
```

Expand description

Services that OpenDAL supports

## <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#notes" class="doc-anchor">Â§</a>Notes

- Scheme is `non_exhaustive`, new variant COULD be added at any time.
- New variant SHOULD be added in alphabet orders,
- Users MUST NOT relay on its order.

## Variants (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variants" class="anchor">Â§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.AliyunDrive" class="anchor">Â§</a>

### AliyunDrive

[aliyun-drive](https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html "struct opendal::services::AliyunDrive"): Aliyun Drive services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Atomicserver" class="anchor">Â§</a>

### Atomicserver

\[atomicserver\]\[crate::services::Atomicserver\]: Atomicserver services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Azblob" class="anchor">Â§</a>

### Azblob

[azblob](https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html "struct opendal::services::Azblob"): Azure Storage Blob services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Azdls" class="anchor">Â§</a>

### Azdls

[Azdls](https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html "struct opendal::services::Azdls"): Azure Data Lake Storage Gen2.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.B2" class="anchor">Â§</a>

### B2

[B2](https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html "struct opendal::services::B2"): Backblaze B2 Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Compfs" class="anchor">Â§</a>

### Compfs

[Compfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html "struct opendal::services::Compfs"): Compio fs Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Seafile" class="anchor">Â§</a>

### Seafile

[Seafile](https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html "struct opendal::services::Seafile"): Seafile Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Upyun" class="anchor">Â§</a>

### Upyun

[Upyun](https://opendal.apache.org/docs/rust/opendal/services/struct.Upyun.html "struct opendal::services::Upyun"): Upyun Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.VercelBlob" class="anchor">Â§</a>

### VercelBlob

[VercelBlob](https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html "struct opendal::services::VercelBlob"): VercelBlob Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.YandexDisk" class="anchor">Â§</a>

### YandexDisk

[YandexDisk](https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html "struct opendal::services::YandexDisk"): YandexDisk Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Pcloud" class="anchor">Â§</a>

### Pcloud

[Pcloud](https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html "struct opendal::services::Pcloud"): Pcloud Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Koofr" class="anchor">Â§</a>

### Koofr

[Koofr](https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html "struct opendal::services::Koofr"): Koofr Services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Cacache" class="anchor">Â§</a>

### Cacache

[cacache](https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html "struct opendal::services::Cacache"): cacache backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.CloudflareKv" class="anchor">Â§</a>

### CloudflareKv

[cloudflare-kv](https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html "struct opendal::services::CloudflareKv"): Cloudflare KV services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Cos" class="anchor">Â§</a>

### Cos

[cos](https://opendal.apache.org/docs/rust/opendal/services/struct.Cos.html "struct opendal::services::Cos"): Tencent Cloud Object Storage services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.D1" class="anchor">Â§</a>

### D1

[d1](https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html "struct opendal::services::D1"): D1 services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Dashmap" class="anchor">Â§</a>

### Dashmap

[dashmap](https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html "struct opendal::services::Dashmap"): dashmap backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Etcd" class="anchor">Â§</a>

### Etcd

[etcd](https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html "struct opendal::services::Etcd"): Etcd Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Foundationdb" class="anchor">Â§</a>

### Foundationdb

[foundationdb](https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html "struct opendal::services::Foundationdb"): Foundationdb services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Dbfs" class="anchor">Â§</a>

### Dbfs

[dbfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html "struct opendal::services::Dbfs"): DBFS backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Fs" class="anchor">Â§</a>

### Fs

[fs](https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html "struct opendal::services::Fs"): POSIX-like file system.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Ftp" class="anchor">Â§</a>

### Ftp

[ftp](https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html "struct opendal::services::Ftp"): FTP backend.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Gcs" class="anchor">Â§</a>

### Gcs

[gcs](https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html "struct opendal::services::Gcs"): Google Cloud Storage backend.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Ghac" class="anchor">Â§</a>

### Ghac

[ghac](https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html "struct opendal::services::Ghac"): GitHub Action Cache services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Hdfs" class="anchor">Â§</a>

### Hdfs

[hdfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html "struct opendal::services::Hdfs"): Hadoop Distributed File System.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Http" class="anchor">Â§</a>

### Http

[http](https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html "struct opendal::services::Http"): HTTP backend.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Huggingface" class="anchor">Â§</a>

### Huggingface

[huggingface](https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html "struct opendal::services::Huggingface"): Huggingface services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Alluxio" class="anchor">Â§</a>

### Alluxio

[alluxio](https://opendal.apache.org/docs/rust/opendal/services/struct.Alluxio.html "struct opendal::services::Alluxio"): Alluxio services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Ipfs" class="anchor">Â§</a>

### Ipfs

[ipmfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Ipfs.html "struct opendal::services::Ipfs"): IPFS HTTP Gateway

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Ipmfs" class="anchor">Â§</a>

### Ipmfs

[ipmfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html "struct opendal::services::Ipmfs"): IPFS mutable file system

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Icloud" class="anchor">Â§</a>

### Icloud

\[icloud\]\[crate::services::Icloud\]: APPLE icloud services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Memcached" class="anchor">Â§</a>

### Memcached

[memcached](https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html "struct opendal::services::Memcached"): Memcached service support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Memory" class="anchor">Â§</a>

### Memory

[memory](https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html "struct opendal::services::Memory"): In memory backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.MiniMoka" class="anchor">Â§</a>

### MiniMoka

[mini-moka](https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html "struct opendal::services::MiniMoka"): Mini Moka backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Moka" class="anchor">Â§</a>

### Moka

[moka](https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html "struct opendal::services::Moka"): moka backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Monoiofs" class="anchor">Â§</a>

### Monoiofs

[monoiofs](https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html "struct opendal::services::Monoiofs"): monoio fs services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Obs" class="anchor">Â§</a>

### Obs

[obs](https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html "struct opendal::services::Obs"): Huawei Cloud OBS services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Onedrive" class="anchor">Â§</a>

### Onedrive

[onedrive](https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html "struct opendal::services::Onedrive"): Microsoft OneDrive services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Gdrive" class="anchor">Â§</a>

### Gdrive

[gdrive](https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html "struct opendal::services::Gdrive"): GoogleDrive services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Dropbox" class="anchor">Â§</a>

### Dropbox

[dropbox](https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html "struct opendal::services::Dropbox"): Dropbox services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Oss" class="anchor">Â§</a>

### Oss

[oss](https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html "struct opendal::services::Oss"): Aliyun Object Storage Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Persy" class="anchor">Â§</a>

### Persy

[persy](https://opendal.apache.org/docs/rust/opendal/services/struct.Persy.html "struct opendal::services::Persy"): persy backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Redis" class="anchor">Â§</a>

### Redis

[redis](https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html "struct opendal::services::Redis"): Redis services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Postgresql" class="anchor">Â§</a>

### Postgresql

[postgresql](https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html "struct opendal::services::Postgresql"): Postgresql services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Mysql" class="anchor">Â§</a>

### Mysql

[mysql](https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html "struct opendal::services::Mysql"): Mysql services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Sqlite" class="anchor">Â§</a>

### Sqlite

[sqlite](https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html "struct opendal::services::Sqlite"): Sqlite services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Rocksdb" class="anchor">Â§</a>

### Rocksdb

[rocksdb](https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html "struct opendal::services::Rocksdb"): RocksDB services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.S3" class="anchor">Â§</a>

### S3

[s3](https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html "struct opendal::services::S3"): AWS S3 alike services.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Sftp" class="anchor">Â§</a>

### Sftp

[sftp](https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html "struct opendal::services::Sftp"): SFTP services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Sled" class="anchor">Â§</a>

### Sled

[sled](https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html "struct opendal::services::Sled"): Sled services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Swift" class="anchor">Â§</a>

### Swift

[swift](https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html "struct opendal::services::Swift"): Swift backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.VercelArtifacts" class="anchor">Â§</a>

### VercelArtifacts

[Vercel Artifacts](https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html "struct opendal::services::VercelArtifacts"): Vercel Artifacts service, as known as Vercel Remote Caching.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Webdav" class="anchor">Â§</a>

### Webdav

[webdav](https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html "struct opendal::services::Webdav"): WebDAV support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Webhdfs" class="anchor">Â§</a>

### Webhdfs

[webhdfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html "struct opendal::services::Webhdfs"): WebHDFS RESTful API Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Redb" class="anchor">Â§</a>

### Redb

[redb](https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html "struct opendal::services::Redb"): Redb Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Tikv" class="anchor">Â§</a>

### Tikv

[tikv](https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html "struct opendal::services::Tikv"): Tikv Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Azfile" class="anchor">Â§</a>

### Azfile

[azfile](https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html "struct opendal::services::Azfile"): Azfile Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Mongodb" class="anchor">Â§</a>

### Mongodb

[mongodb](https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html "struct opendal::services::Mongodb"): MongoDB Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Gridfs" class="anchor">Â§</a>

### Gridfs

[gridfs](https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html "struct opendal::services::Gridfs"): MongoDB Gridfs Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Github" class="anchor">Â§</a>

### Github

[Github Contents](https://opendal.apache.org/docs/rust/opendal/services/struct.Github.html "struct opendal::services::Github"): Github contents support.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.HdfsNative" class="anchor">Â§</a>

### HdfsNative

[Native HDFS](https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html "struct opendal::services::HdfsNative"): Hdfs Native service, using rust hdfs-native client for hdfs

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Surrealdb" class="anchor">Â§</a>

### Surrealdb

[surrealdb](https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html "struct opendal::services::Surrealdb"): Surrealdb Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Lakefs" class="anchor">Â§</a>

### Lakefs

[lakefs](https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html "struct opendal::services::Lakefs"): LakeFS Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.NebulaGraph" class="anchor">Â§</a>

### NebulaGraph

[NebulaGraph](crate::services::NebulaGraph): NebulaGraph Services

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#variant.Custom" class="anchor">Â§</a>

### Custom(&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Custom that allow users to implement services outside OpenDAL.

#### <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#note" class="doc-anchor">Â§</a>NOTE

- Custom must not overwrite any existing services name.
- Custom must be in lower case.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Scheme" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.into_static" class="fn">into_static</a>(self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Convert self into static str.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.enabled" class="fn">enabled</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>\>

Get all enabled schemes.

OpenDAL could be compiled with different features, which will enable different schemes. This function returns all enabled schemes so users can make decisions based on it.

##### <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::Scheme;

let enabled_schemes = Scheme::enabled();
if !enabled_schemes.contains(&Scheme::Memory) {
    panic!("s3 support is not enabled")
}
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Clone-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Debug-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Default-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Display-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-From%3CScheme%3E-for-%26str" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-From%3CScheme%3E-for-String" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.from-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-FromStr-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#associatedtype.Err" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.from_str" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Hash-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.hash" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.hash_slice" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-PartialEq-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Copy-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-Eq-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#impl-StructuralPartialEq-for-Scheme" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html#blanket-implementations" class="anchor">Â§</a>
