# Module services Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/mod.rs.html#18-328" class="src">Source</a>

Expand description

Services will provide builders to build underlying backends.

More ongoing services support is tracked at [opendal#5](https://github.com/apache/opendal/issues/5). Please feel free to submit issues if there are services not covered.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/services/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDrive</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDriveConfig.html" class="struct" title="struct opendal::services::AliyunDriveConfig">AliyunDriveConfig</a>  
Config for Aliyun Drive services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Alluxio.html" class="struct" title="struct opendal::services::Alluxio">Alluxio</a>  
[Alluxio](https://www.alluxio.io/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AlluxioConfig.html" class="struct" title="struct opendal::services::AlluxioConfig">AlluxioConfig</a>  
Config for alluxio services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">Azblob</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzblobConfig.html" class="struct" title="struct opendal::services::AzblobConfig">AzblobConfig</a>  
Azure Storage Blob services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">Azdls</a>  
Azure Data Lake Storage Gen2 Support. As known as `abfs`, `azdls` or `azdls`.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>  
Azure Data Lake Storage Gen2 Support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">Azfile</a>  
Azure File services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzfileConfig.html" class="struct" title="struct opendal::services::AzfileConfig">AzfileConfig</a>  
Azure File services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2</a>  
[b2](https://www.backblaze.com/cloud-storage) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2Config.html" class="struct" title="struct opendal::services::B2Config">B2Config</a>  
Config for backblaze b2 services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">Cacache</a>  
cacache service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CacacheConfig.html" class="struct" title="struct opendal::services::CacacheConfig">CacacheConfig</a>  
cacache service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKv</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKvConfig.html" class="struct" title="struct opendal::services::CloudflareKvConfig">CloudflareKvConfig</a>  
Cloudflare KV Service Support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">Compfs</a>  
\[`compio`\]-based file system support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CompfsConfig.html" class="struct" title="struct opendal::services::CompfsConfig">CompfsConfig</a>  
compio-based file system support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cos.html" class="struct" title="struct opendal::services::Cos">Cos</a>  
Tencent-Cloud COS services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CosConfig.html" class="struct" title="struct opendal::services::CosConfig">CosConfig</a>  
Tencent-Cloud COS services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1Config.html" class="struct" title="struct opendal::services::D1Config">D1Config</a>  
Config for [Cloudflare D1](https://developers.cloudflare.com/d1) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">Dashmap</a>  
[dashmap](https://github.com/xacrimon/dashmap) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DashmapConfig.html" class="struct" title="struct opendal::services::DashmapConfig">DashmapConfig</a>  
Config for Dashmap services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">Dbfs</a>  
[Dbfs](https://docs.databricks.com/api/azure/workspace/dbfs)â€™s REST API support. This service will visit the [DBFS API](https://docs.databricks.com/api/azure/workspace/dbfs) supported by [Databricks File System](https://docs.databricks.com/en/dbfs/index.html).

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DbfsConfig.html" class="struct" title="struct opendal::services::DbfsConfig">DbfsConfig</a>  
[Dbfs](https://docs.databricks.com/api/azure/workspace/dbfs)â€™s REST API support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">Dropbox</a>  
[Dropbox](https://www.dropbox.com/) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DropboxConfig.html" class="struct" title="struct opendal::services::DropboxConfig">DropboxConfig</a>  
Config for [Dropbox](https://www.dropbox.com/) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">Etcd</a>  
[Etcd](https://etcd.io/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.EtcdConfig.html" class="struct" title="struct opendal::services::EtcdConfig">EtcdConfig</a>  
Config for Etcd services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">Foundationdb</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FoundationdbConfig.html" class="struct" title="struct opendal::services::FoundationdbConfig">FoundationdbConfig</a>  
[foundationdb](https://www.foundationdb.org/) service support. Config for FoundationDB.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">Fs</a>  
POSIX file system support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FsConfig.html" class="struct" title="struct opendal::services::FsConfig">FsConfig</a>  
config for file system

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">Ftp</a>  
FTP and FTPS services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FtpConfig.html" class="struct" title="struct opendal::services::FtpConfig">FtpConfig</a>  
Config for Ftp services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">Gcs</a>  
[Google Cloud Storage](https://cloud.google.com/storage) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GcsConfig.html" class="struct" title="struct opendal::services::GcsConfig">GcsConfig</a>  
[Google Cloud Storage](https://cloud.google.com/storage) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">Gdrive</a>  
[GoogleDrive](https://drive.google.com/) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GdriveConfig.html" class="struct" title="struct opendal::services::GdriveConfig">GdriveConfig</a>  
[GoogleDrive](https://drive.google.com/) configuration.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">Ghac</a>  
GitHub Action Cache Services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GhacConfig.html" class="struct" title="struct opendal::services::GhacConfig">GhacConfig</a>  
Config for GitHub Action Cache Services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Github.html" class="struct" title="struct opendal::services::Github">Github</a>  
[github contents](https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#create-or-update-file-contents) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GithubConfig.html" class="struct" title="struct opendal::services::GithubConfig">GithubConfig</a>  
Config for GitHub services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">Gridfs</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GridfsConfig.html" class="struct" title="struct opendal::services::GridfsConfig">GridfsConfig</a>  
Config for Grid file system support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">Hdfs</a>  
A distributed file system that provides high-throughput access to application data.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsConfig.html" class="struct" title="struct opendal::services::HdfsConfig">HdfsConfig</a>  
[Hadoop Distributed File System (HDFSâ„¢)](https://hadoop.apache.org/) support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNative</a>  
[Hadoop Distributed File System (HDFSâ„¢)](https://hadoop.apache.org/) support. Using [Native Rust HDFS client](https://github.com/Kimahriman/hdfs-native). A distributed file system that provides high-throughput access to application data. Using [Native Rust HDFS client](https://github.com/Kimahriman/hdfs-native).

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNativeConfig.html" class="struct" title="struct opendal::services::HdfsNativeConfig">HdfsNativeConfig</a>  
Config for HdfsNative services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">Http</a>  
HTTP Read-only service support like [Nginx](https://www.nginx.com/) and [Caddy](https://caddyserver.com/).

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HttpConfig.html" class="struct" title="struct opendal::services::HttpConfig">HttpConfig</a>  
Config for Http service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">Huggingface</a>  
[Huggingface](https://huggingface.co/docs/huggingface_hub/package_reference/hf_api)â€™s API support. This service will visit the [Huggingface API](https://huggingface.co/docs/huggingface_hub/package_reference/hf_api) to access the Huggingface File System. Currently, we only support the `model` and `dataset` types of repositories, and operations are limited to reading and listing/stating.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HuggingfaceConfig.html" class="struct" title="struct opendal::services::HuggingfaceConfig">HuggingfaceConfig</a>  
Configuration for Huggingface service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipfs.html" class="struct" title="struct opendal::services::Ipfs">Ipfs</a>  
IPFS file system support based on [IPFS HTTP Gateway](https://docs.ipfs.tech/concepts/ipfs-gateway/).

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpfsConfig.html" class="struct" title="struct opendal::services::IpfsConfig">IpfsConfig</a>  
Config for IPFS file system support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">Ipmfs</a>  
IPFS file system support based on [IPFS MFS](https://docs.ipfs.tech/concepts/file-systems/) API.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpmfsConfig.html" class="struct" title="struct opendal::services::IpmfsConfig">IpmfsConfig</a>  
Config for IPFS MFS support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">Koofr</a>  
[Koofr](https://app.koofr.net/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.KoofrConfig.html" class="struct" title="struct opendal::services::KoofrConfig">KoofrConfig</a>  
Config for Koofr services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">Lakefs</a>  
[Lakefs](https://docs.lakefs.io/reference/api.html#/)â€™s API support. This service will visit the [Lakefs API](https://Lakefs.co/docs/Lakefs_hub/package_reference/hf_api) to access the Lakefs File System. Currently, we only support the `model` and `dataset` types of repositories, and operations are limited to reading and listing/stating.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.LakefsConfig.html" class="struct" title="struct opendal::services::LakefsConfig">LakefsConfig</a>  
Configuration for Lakefs service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">Memcached</a>  
[Memcached](https://memcached.org/) service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemcachedConfig.html" class="struct" title="struct opendal::services::MemcachedConfig">MemcachedConfig</a>  
Config for MemCached services support

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">Memory</a>  
In memory service support. (BTreeMap Based)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemoryConfig.html" class="struct" title="struct opendal::services::MemoryConfig">MemoryConfig</a>  
Config for memory.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMoka</a>  
[mini-moka](https://github.com/moka-rs/mini-moka) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMokaConfig.html" class="struct" title="struct opendal::services::MiniMokaConfig">MiniMokaConfig</a>  
Config for mini-moka support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">Moka</a>  
[moka](https://github.com/moka-rs/moka) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaConfig.html" class="struct" title="struct opendal::services::MokaConfig">MokaConfig</a>  
Config for Moka services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaValue.html" class="struct" title="struct opendal::services::MokaValue">MokaValue</a>  
Value stored in moka cache containing both metadata and content

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">Mongodb</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MongodbConfig.html" class="struct" title="struct opendal::services::MongodbConfig">MongodbConfig</a>  
Config for Mongodb service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">Monoiofs</a>  
File system support via \[`monoio`\].

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MonoiofsConfig.html" class="struct" title="struct opendal::services::MonoiofsConfig">MonoiofsConfig</a>  
Config for monoiofs services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">Mysql</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MysqlConfig.html" class="struct" title="struct opendal::services::MysqlConfig">MysqlConfig</a>  
Config for Mysql services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">Obs</a>  
Huawei-Cloud Object Storage Service (OBS) support

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.ObsConfig.html" class="struct" title="struct opendal::services::ObsConfig">ObsConfig</a>  
Config for Huawei-Cloud Object Storage Service (OBS) support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">Onedrive</a>  
Microsoft [OneDrive](https://onedrive.com) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OnedriveConfig.html" class="struct" title="struct opendal::services::OnedriveConfig">OnedriveConfig</a>  
Config for [OneDrive](https://onedrive.com) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">Oss</a>  
Aliyun Object Storage Service (OSS) support

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>  
Config for Aliyun Object Storage Service (OSS) support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">Pcloud</a>  
[pCloud](https://www.pcloud.com/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PcloudConfig.html" class="struct" title="struct opendal::services::PcloudConfig">PcloudConfig</a>  
Config for Pcloud services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Persy.html" class="struct" title="struct opendal::services::Persy">Persy</a>  
persy service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PersyConfig.html" class="struct" title="struct opendal::services::PersyConfig">PersyConfig</a>  
Config for persy service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">Postgresql</a>  
[PostgreSQL](https://www.postgresql.org/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PostgresqlConfig.html" class="struct" title="struct opendal::services::PostgresqlConfig">PostgresqlConfig</a>  
Config for PostgreSQL services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">Redb</a>  
Redb service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedbConfig.html" class="struct" title="struct opendal::services::RedbConfig">RedbConfig</a>  
Config for redb service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">Redis</a>  
[Redis](https://redis.io/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedisConfig.html" class="struct" title="struct opendal::services::RedisConfig">RedisConfig</a>  
Config for Redis services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">Rocksdb</a>  
RocksDB service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RocksdbConfig.html" class="struct" title="struct opendal::services::RocksdbConfig">RocksdbConfig</a>  
Config for Rocksdb Service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3</a>  
Aws S3 and compatible services (including minio, digitalocean space, Tencent Cloud Object Storage(COS) and so on) support. For more information about s3-compatible services, refer to [Compatible Services](https://opendal.apache.org/docs/rust/opendal/services/index.html#compatible-services).

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>  
Config for Aws S3 and compatible services (including minio, digitalocean space, Tencent Cloud Object Storage(COS) and so on) support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">Seafile</a>  
[seafile](https://www.seafile.com) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SeafileConfig.html" class="struct" title="struct opendal::services::SeafileConfig">SeafileConfig</a>  
Config for seafile services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">Sftp</a>  
SFTP services support. (only works on unix)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SftpConfig.html" class="struct" title="struct opendal::services::SftpConfig">SftpConfig</a>  
Config for Sftp Service support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">Sled</a>  
Sled services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SledConfig.html" class="struct" title="struct opendal::services::SledConfig">SledConfig</a>  
Config for Sled services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">Sqlite</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SqliteConfig.html" class="struct" title="struct opendal::services::SqliteConfig">SqliteConfig</a>  
Config for Sqlite support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">Surrealdb</a>  
Capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SurrealdbConfig.html" class="struct" title="struct opendal::services::SurrealdbConfig">SurrealdbConfig</a>  
Config for Surrealdb services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">Swift</a>  
[OpenStack Swift](https://docs.openstack.org/api-ref/object-store/#)â€™s REST API support. For more information about swift-compatible services, refer to [Compatible Services](https://opendal.apache.org/docs/rust/opendal/services/index.html#compatible-services).

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SwiftConfig.html" class="struct" title="struct opendal::services::SwiftConfig">SwiftConfig</a>  
Config for OpenStack Swift support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">Tikv</a>  
TiKV backend builder

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.TikvConfig.html" class="struct" title="struct opendal::services::TikvConfig">TikvConfig</a>  
Config for Tikv services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Upyun.html" class="struct" title="struct opendal::services::Upyun">Upyun</a>  
[upyun](https://www.upyun.com/products/file-storage) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.UpyunConfig.html" class="struct" title="struct opendal::services::UpyunConfig">UpyunConfig</a>  
Config for upyun services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifacts</a>  
[Vercel Cache](https://vercel.com/docs/concepts/monorepos/remote-caching) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifactsConfig.html" class="struct" title="struct opendal::services::VercelArtifactsConfig">VercelArtifactsConfig</a>  
Config for Vercel Cache support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlob</a>  
[VercelBlob](https://vercel.com/docs/storage/vercel-blob) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlobConfig.html" class="struct" title="struct opendal::services::VercelBlobConfig">VercelBlobConfig</a>  
Config for VercelBlob services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">Webdav</a>  
[WebDAV](https://datatracker.ietf.org/doc/html/rfc4918) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebdavConfig.html" class="struct" title="struct opendal::services::WebdavConfig">WebdavConfig</a>  
Config for [WebDAV](https://datatracker.ietf.org/doc/html/rfc4918) backend support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">Webhdfs</a>  
[WebHDFS](https://hadoop.apache.org/docs/stable/hadoop-project-dist/hadoop-hdfs/WebHDFS.html)â€™s REST API support. There two implementations of WebHDFS REST API:

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebhdfsConfig.html" class="struct" title="struct opendal::services::WebhdfsConfig">WebhdfsConfig</a>  
Config for WebHDFS support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDisk</a>  
[YandexDisk](https://360.yandex.com/disk/) services support.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDiskConfig.html" class="struct" title="struct opendal::services::YandexDiskConfig">YandexDiskConfig</a>  
Config for YandexDisk services support.

## Constants<a href="https://opendal.apache.org/docs/rust/opendal/services/index.html#constants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.AZBLOB_SCHEME.html" class="constant" title="constant opendal::services::AZBLOB_SCHEME">AZBLOB_SCHEME</a>`services-azblob`  
Default scheme for azblob service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.B2_SCHEME.html" class="constant" title="constant opendal::services::B2_SCHEME">B2_SCHEME</a>`services-b2`  
Default scheme for b2 service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.COS_SCHEME.html" class="constant" title="constant opendal::services::COS_SCHEME">COS_SCHEME</a>`services-cos`  
Default scheme for cos service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.FS_SCHEME.html" class="constant" title="constant opendal::services::FS_SCHEME">FS_SCHEME</a>`services-fs`  
Default scheme for fs service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.GCS_SCHEME.html" class="constant" title="constant opendal::services::GCS_SCHEME">GCS_SCHEME</a>`services-gcs`  
Default scheme for gcs service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.MEMORY_SCHEME.html" class="constant" title="constant opendal::services::MEMORY_SCHEME">MEMORY_SCHEME</a>`services-memory`  
Default scheme for memory service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.OBS_SCHEME.html" class="constant" title="constant opendal::services::OBS_SCHEME">OBS_SCHEME</a>`services-obs`  
Default scheme for obs service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.OSS_SCHEME.html" class="constant" title="constant opendal::services::OSS_SCHEME">OSS_SCHEME</a>`services-oss`  
Default scheme for oss service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.S3_SCHEME.html" class="constant" title="constant opendal::services::S3_SCHEME">S3_SCHEME</a>`services-s3`  
Default scheme for s3 service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/constant.UPYUN_SCHEME.html" class="constant" title="constant opendal::services::UPYUN_SCHEME">UPYUN_SCHEME</a>`services-upyun`  
Default scheme for upyun service.

## Type Aliases<a href="https://opendal.apache.org/docs/rust/opendal/services/index.html#types" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/type.MokaCacheBuilder.html" class="type" title="type opendal::services::MokaCacheBuilder">MokaCacheBuilder</a>  
Type alias of [`moka::future::CacheBuilder`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html)
