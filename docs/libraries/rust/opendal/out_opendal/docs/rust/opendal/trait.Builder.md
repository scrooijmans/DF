# Trait Builder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/builder.rs.html#52-58" class="src">Source</a>

``` rust
pub trait Builder: Default + 'static {
    type Config: Configurator;

    // Required method
    fn build(self) -> Result<impl Access>;
}
```

Expand description

Builder is used to set up underlying services.

This trait allows the developer to define a builder struct that can:

- build a service via builder style API.
- configure in-memory options like `http_client` or `customized_credential_load`.

Usually, users donâ€™t need to use or import this trait directly, they can use `Operator` API instead.

For example:

``` rust
use opendal::services::Fs;
use opendal::Operator;
async fn test() -> Result<()> {
    // Create fs backend builder.
    let mut builder = Fs::default().root("/tmp");

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Required Associated Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#required-associated-types" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a>: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a>

Associated configuration for this builder.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

Dummy implementation of builder

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-AliyunDriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

Available on **crate feature `services-aliyun-drive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDriveConfig.html" class="struct" title="struct opendal::services::AliyunDriveConfig">AliyunDriveConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-AlluxioBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Alluxio.html" class="struct" title="struct opendal::services::Alluxio">AlluxioBuilder</a>

Available on **crate feature `services-alluxio`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AlluxioConfig.html" class="struct" title="struct opendal::services::AlluxioConfig">AlluxioConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-AzblobBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

Available on **crate feature `services-azblob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzblobConfig.html" class="struct" title="struct opendal::services::AzblobConfig">AzblobConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-AzdlsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-AzfileBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

Available on **crate feature `services-azfile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzfileConfig.html" class="struct" title="struct opendal::services::AzfileConfig">AzfileConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-B2Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

Available on **crate feature `services-b2`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-7" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2Config.html" class="struct" title="struct opendal::services::B2Config">B2Config</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-CacacheBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">CacacheBuilder</a>

Available on **crate feature `services-cacache`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-8" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CacacheConfig.html" class="struct" title="struct opendal::services::CacacheConfig">CacacheConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-CloudflareKvBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

Available on **crate feature `services-cloudflare-kv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-9" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKvConfig.html" class="struct" title="struct opendal::services::CloudflareKvConfig">CloudflareKvConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-CompfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-10" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CompfsConfig.html" class="struct" title="struct opendal::services::CompfsConfig">CompfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-CosBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cos.html" class="struct" title="struct opendal::services::Cos">CosBuilder</a>

Available on **crate feature `services-cos`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-11" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CosConfig.html" class="struct" title="struct opendal::services::CosConfig">CosConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-D1Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1Builder</a>

Available on **crate feature `services-d1`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-12" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1Config.html" class="struct" title="struct opendal::services::D1Config">D1Config</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-DashmapBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

Available on **crate feature `services-dashmap`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-13" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DashmapConfig.html" class="struct" title="struct opendal::services::DashmapConfig">DashmapConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-DbfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

Available on **crate feature `services-dbfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-14" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DbfsConfig.html" class="struct" title="struct opendal::services::DbfsConfig">DbfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-DropboxBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

Available on **crate feature `services-dropbox`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-15" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DropboxConfig.html" class="struct" title="struct opendal::services::DropboxConfig">DropboxConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-EtcdBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

Available on **crate feature `services-etcd`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-16" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.EtcdConfig.html" class="struct" title="struct opendal::services::EtcdConfig">EtcdConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-FoundationdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">FoundationdbBuilder</a>

Available on **crate feature `services-foundationdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-17" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FoundationdbConfig.html" class="struct" title="struct opendal::services::FoundationdbConfig">FoundationdbConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-FsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

Available on **crate feature `services-fs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-18" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FsConfig.html" class="struct" title="struct opendal::services::FsConfig">FsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-FtpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

Available on **crate feature `services-ftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-19" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FtpConfig.html" class="struct" title="struct opendal::services::FtpConfig">FtpConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-GcsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

Available on **crate feature `services-gcs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-20" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GcsConfig.html" class="struct" title="struct opendal::services::GcsConfig">GcsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-GdriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

Available on **crate feature `services-gdrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-21" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GdriveConfig.html" class="struct" title="struct opendal::services::GdriveConfig">GdriveConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-GhacBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

Available on **crate feature `services-ghac`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-22" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GhacConfig.html" class="struct" title="struct opendal::services::GhacConfig">GhacConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-GithubBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Github.html" class="struct" title="struct opendal::services::Github">GithubBuilder</a>

Available on **crate feature `services-github`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-23" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GithubConfig.html" class="struct" title="struct opendal::services::GithubConfig">GithubConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-GridfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">GridfsBuilder</a>

Available on **crate feature `services-gridfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-24" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GridfsConfig.html" class="struct" title="struct opendal::services::GridfsConfig">GridfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-HdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

Available on **crate feature `services-hdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-25" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsConfig.html" class="struct" title="struct opendal::services::HdfsConfig">HdfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-HdfsNativeBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

Available on **crate feature `services-hdfs-native`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-26" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNativeConfig.html" class="struct" title="struct opendal::services::HdfsNativeConfig">HdfsNativeConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-HttpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

Available on **crate feature `services-http`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-27" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HttpConfig.html" class="struct" title="struct opendal::services::HttpConfig">HttpConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-HuggingfaceBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Available on **crate feature `services-huggingface`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-28" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HuggingfaceConfig.html" class="struct" title="struct opendal::services::HuggingfaceConfig">HuggingfaceConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-IpfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipfs.html" class="struct" title="struct opendal::services::Ipfs">IpfsBuilder</a>

Available on **crate feature `services-ipfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-29" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpfsConfig.html" class="struct" title="struct opendal::services::IpfsConfig">IpfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-IpmfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

Available on **crate feature `services-ipmfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-30" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpmfsConfig.html" class="struct" title="struct opendal::services::IpmfsConfig">IpmfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-KoofrBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

Available on **crate feature `services-koofr`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-31" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.KoofrConfig.html" class="struct" title="struct opendal::services::KoofrConfig">KoofrConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-LakefsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Available on **crate feature `services-lakefs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-32" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.LakefsConfig.html" class="struct" title="struct opendal::services::LakefsConfig">LakefsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MemcachedBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

Available on **crate feature `services-memcached`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-33" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemcachedConfig.html" class="struct" title="struct opendal::services::MemcachedConfig">MemcachedConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MemoryBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">MemoryBuilder</a>

Available on **crate feature `services-memory`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-34" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemoryConfig.html" class="struct" title="struct opendal::services::MemoryConfig">MemoryConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MiniMokaBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

Available on **crate feature `services-mini-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-35" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMokaConfig.html" class="struct" title="struct opendal::services::MiniMokaConfig">MiniMokaConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MokaBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

Available on **crate feature `services-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-36" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaConfig.html" class="struct" title="struct opendal::services::MokaConfig">MokaConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MongodbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">MongodbBuilder</a>

Available on **crate feature `services-mongodb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-37" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MongodbConfig.html" class="struct" title="struct opendal::services::MongodbConfig">MongodbConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MonoiofsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

Available on **crate feature `services-monoiofs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-38" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MonoiofsConfig.html" class="struct" title="struct opendal::services::MonoiofsConfig">MonoiofsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-MysqlBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

Available on **crate feature `services-mysql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-39" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MysqlConfig.html" class="struct" title="struct opendal::services::MysqlConfig">MysqlConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-ObsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

Available on **crate feature `services-obs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-40" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.ObsConfig.html" class="struct" title="struct opendal::services::ObsConfig">ObsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-OnedriveBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

Available on **crate feature `services-onedrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-41" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OnedriveConfig.html" class="struct" title="struct opendal::services::OnedriveConfig">OnedriveConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-OssBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-42" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-PcloudBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

Available on **crate feature `services-pcloud`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-43" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PcloudConfig.html" class="struct" title="struct opendal::services::PcloudConfig">PcloudConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-PersyBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Persy.html" class="struct" title="struct opendal::services::Persy">PersyBuilder</a>

Available on **crate feature `services-persy`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-44" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PersyConfig.html" class="struct" title="struct opendal::services::PersyConfig">PersyConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-PostgresqlBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

Available on **crate feature `services-postgresql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-45" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PostgresqlConfig.html" class="struct" title="struct opendal::services::PostgresqlConfig">PostgresqlConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-RedbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

Available on **crate feature `services-redb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-46" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedbConfig.html" class="struct" title="struct opendal::services::RedbConfig">RedbConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-RedisBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

Available on **crate feature `services-redis`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-47" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedisConfig.html" class="struct" title="struct opendal::services::RedisConfig">RedisConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-RocksdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

Available on **crate feature `services-rocksdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-48" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RocksdbConfig.html" class="struct" title="struct opendal::services::RocksdbConfig">RocksdbConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-S3Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-49" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-SeafileBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

Available on **crate feature `services-seafile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-50" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SeafileConfig.html" class="struct" title="struct opendal::services::SeafileConfig">SeafileConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-SftpBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

Available on **crate feature `services-sftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-51" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SftpConfig.html" class="struct" title="struct opendal::services::SftpConfig">SftpConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-SledBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

Available on **crate feature `services-sled`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-52" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SledConfig.html" class="struct" title="struct opendal::services::SledConfig">SledConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-SqliteBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

Available on **crate feature `services-sqlite`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-53" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SqliteConfig.html" class="struct" title="struct opendal::services::SqliteConfig">SqliteConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-SurrealdbBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

Available on **crate feature `services-surrealdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-54" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SurrealdbConfig.html" class="struct" title="struct opendal::services::SurrealdbConfig">SurrealdbConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-SwiftBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Available on **crate feature `services-swift`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-55" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SwiftConfig.html" class="struct" title="struct opendal::services::SwiftConfig">SwiftConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-TikvBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

Available on **crate feature `services-tikv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-56" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.TikvConfig.html" class="struct" title="struct opendal::services::TikvConfig">TikvConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-UpyunBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Upyun.html" class="struct" title="struct opendal::services::Upyun">UpyunBuilder</a>

Available on **crate feature `services-upyun`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-57" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.UpyunConfig.html" class="struct" title="struct opendal::services::UpyunConfig">UpyunConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-VercelArtifactsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

Available on **crate feature `services-vercel-artifacts`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-58" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifactsConfig.html" class="struct" title="struct opendal::services::VercelArtifactsConfig">VercelArtifactsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-VercelBlobBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

Available on **crate feature `services-vercel-blob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-59" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlobConfig.html" class="struct" title="struct opendal::services::VercelBlobConfig">VercelBlobConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-WebdavBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

Available on **crate feature `services-webdav`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-60" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebdavConfig.html" class="struct" title="struct opendal::services::WebdavConfig">WebdavConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-WebhdfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

Available on **crate feature `services-webhdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-61" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebhdfsConfig.html" class="struct" title="struct opendal::services::WebhdfsConfig">WebhdfsConfig</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#impl-Builder-for-YandexDiskBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>

Available on **crate feature `services-yandex-disk`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config-62" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDiskConfig.html" class="struct" title="struct opendal::services::YandexDiskConfig">YandexDiskConfig</a>
