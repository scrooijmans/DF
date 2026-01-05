# Trait Configurator Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/builder.rs.html#123-145" class="src">Source</a>

``` rust
pub trait Configurator:
    Serialize
    + DeserializeOwned
    + Debug
    + 'static {
    type Builder: Builder;

    // Required method
    fn into_builder(self) -> Self::Builder;

    // Provided methods
    fn from_uri(_uri: &OperatorUri) -> Result<Self> { ... }
    fn from_iter(
        iter: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self> { ... }
}
```

Expand description

Configurator is used to configure the underlying service.

This trait allows the developer to define a configuration struct that can:

- deserialize from an iterator like hashmap or vector.
- convert into a service builder and finally build the underlying services.

Usually, users donâ€™t need to use or import this trait directly, they can use `Operator` API instead.

For example:

``` rust
use std::collections::HashMap;

use opendal::services::MemoryConfig;
use opendal::Operator;
async fn test() -> Result<()> {
    let mut cfg = MemoryConfig::default();
    cfg.root = Some("/".to_string());

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::from_config(cfg)?.finish();

    Ok(())
}
```

Some service builder might contain in memory options like `http_client` . Users can call `into_builder` to convert the configuration into a builder instead.

``` rust
use std::collections::HashMap;

use opendal::raw::HttpClient;
use opendal::services::S3Config;
use opendal::Configurator;
use opendal::Operator;

async fn test() -> Result<()> {
    let mut cfg = S3Config::default();
    cfg.root = Some("/".to_string());
    cfg.bucket = "test".to_string();

    let builder = cfg.into_builder();
    let builder = builder.http_client(HttpClient::new()?);

    // Build an `Operator` to start operating the storage.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Required Associated Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#required-associated-types" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a>: <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a>

Associated builder for this configuration.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#tymethod.into_builder" class="fn">into_builder</a>(self) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype" title="type opendal::Configurator::Builder">Builder</a>

Convert this configuration into a service builder.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_uri" class="fn">from_uri</a>(\_uri: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Build configuration from a parsed URI plus merged options.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.from_iter" class="fn">from_iter</a>(iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Deserialize from an iterator.

This API is provided by opendal, developer should not implement it.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#method.into_builder" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#tymethod.into_builder" class="fn">into_builder</a>(self) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype" title="type opendal::Configurator::Builder">Builder</a>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-AliyunDriveConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDriveConfig.html" class="struct" title="struct opendal::services::AliyunDriveConfig">AliyunDriveConfig</a>

Available on **crate feature `services-aliyun-drive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AliyunDrive.html" class="struct" title="struct opendal::services::AliyunDrive">AliyunDriveBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-AlluxioConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AlluxioConfig.html" class="struct" title="struct opendal::services::AlluxioConfig">AlluxioConfig</a>

Available on **crate feature `services-alluxio`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Alluxio.html" class="struct" title="struct opendal::services::Alluxio">AlluxioBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-AzblobConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzblobConfig.html" class="struct" title="struct opendal::services::AzblobConfig">AzblobConfig</a>

Available on **crate feature `services-azblob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azblob.html" class="struct" title="struct opendal::services::Azblob">AzblobBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-AzdlsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzdlsConfig.html" class="struct" title="struct opendal::services::AzdlsConfig">AzdlsConfig</a>

Available on **crate feature `services-azdls`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azdls.html" class="struct" title="struct opendal::services::Azdls">AzdlsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-AzfileConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.AzfileConfig.html" class="struct" title="struct opendal::services::AzfileConfig">AzfileConfig</a>

Available on **crate feature `services-azfile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Azfile.html" class="struct" title="struct opendal::services::Azfile">AzfileBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-B2Config" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2Config.html" class="struct" title="struct opendal::services::B2Config">B2Config</a>

Available on **crate feature `services-b2`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-7" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.B2.html" class="struct" title="struct opendal::services::B2">B2Builder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-CacacheConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CacacheConfig.html" class="struct" title="struct opendal::services::CacacheConfig">CacacheConfig</a>

Available on **crate feature `services-cacache`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-8" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cacache.html" class="struct" title="struct opendal::services::Cacache">CacacheBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-CloudflareKvConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKvConfig.html" class="struct" title="struct opendal::services::CloudflareKvConfig">CloudflareKvConfig</a>

Available on **crate feature `services-cloudflare-kv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-9" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CloudflareKv.html" class="struct" title="struct opendal::services::CloudflareKv">CloudflareKvBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-CompfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CompfsConfig.html" class="struct" title="struct opendal::services::CompfsConfig">CompfsConfig</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-10" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-CosConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CosConfig.html" class="struct" title="struct opendal::services::CosConfig">CosConfig</a>

Available on **crate feature `services-cos`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-11" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Cos.html" class="struct" title="struct opendal::services::Cos">CosBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-D1Config" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1Config.html" class="struct" title="struct opendal::services::D1Config">D1Config</a>

Available on **crate feature `services-d1`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-12" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1Builder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-DashmapConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DashmapConfig.html" class="struct" title="struct opendal::services::DashmapConfig">DashmapConfig</a>

Available on **crate feature `services-dashmap`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-13" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-DbfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DbfsConfig.html" class="struct" title="struct opendal::services::DbfsConfig">DbfsConfig</a>

Available on **crate feature `services-dbfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-14" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dbfs.html" class="struct" title="struct opendal::services::Dbfs">DbfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-DropboxConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DropboxConfig.html" class="struct" title="struct opendal::services::DropboxConfig">DropboxConfig</a>

Available on **crate feature `services-dropbox`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-15" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dropbox.html" class="struct" title="struct opendal::services::Dropbox">DropboxBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-EtcdConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.EtcdConfig.html" class="struct" title="struct opendal::services::EtcdConfig">EtcdConfig</a>

Available on **crate feature `services-etcd`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-16" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Etcd.html" class="struct" title="struct opendal::services::Etcd">EtcdBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-FoundationdbConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FoundationdbConfig.html" class="struct" title="struct opendal::services::FoundationdbConfig">FoundationdbConfig</a>

Available on **crate feature `services-foundationdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-17" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Foundationdb.html" class="struct" title="struct opendal::services::Foundationdb">FoundationdbBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-FsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FsConfig.html" class="struct" title="struct opendal::services::FsConfig">FsConfig</a>

Available on **crate feature `services-fs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-18" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-FtpConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FtpConfig.html" class="struct" title="struct opendal::services::FtpConfig">FtpConfig</a>

Available on **crate feature `services-ftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-19" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ftp.html" class="struct" title="struct opendal::services::Ftp">FtpBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-GcsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GcsConfig.html" class="struct" title="struct opendal::services::GcsConfig">GcsConfig</a>

Available on **crate feature `services-gcs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-20" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gcs.html" class="struct" title="struct opendal::services::Gcs">GcsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-GdriveConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GdriveConfig.html" class="struct" title="struct opendal::services::GdriveConfig">GdriveConfig</a>

Available on **crate feature `services-gdrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-21" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gdrive.html" class="struct" title="struct opendal::services::Gdrive">GdriveBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-GhacConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GhacConfig.html" class="struct" title="struct opendal::services::GhacConfig">GhacConfig</a>

Available on **crate feature `services-ghac`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-22" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ghac.html" class="struct" title="struct opendal::services::Ghac">GhacBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-GithubConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GithubConfig.html" class="struct" title="struct opendal::services::GithubConfig">GithubConfig</a>

Available on **crate feature `services-github`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-23" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Github.html" class="struct" title="struct opendal::services::Github">GithubBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-GridfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.GridfsConfig.html" class="struct" title="struct opendal::services::GridfsConfig">GridfsConfig</a>

Available on **crate feature `services-gridfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-24" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Gridfs.html" class="struct" title="struct opendal::services::Gridfs">GridfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-HdfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsConfig.html" class="struct" title="struct opendal::services::HdfsConfig">HdfsConfig</a>

Available on **crate feature `services-hdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-25" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Hdfs.html" class="struct" title="struct opendal::services::Hdfs">HdfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-HdfsNativeConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNativeConfig.html" class="struct" title="struct opendal::services::HdfsNativeConfig">HdfsNativeConfig</a>

Available on **crate feature `services-hdfs-native`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-26" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HdfsNative.html" class="struct" title="struct opendal::services::HdfsNative">HdfsNativeBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-HttpConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HttpConfig.html" class="struct" title="struct opendal::services::HttpConfig">HttpConfig</a>

Available on **crate feature `services-http`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-27" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html" class="struct" title="struct opendal::services::Http">HttpBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-HuggingfaceConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HuggingfaceConfig.html" class="struct" title="struct opendal::services::HuggingfaceConfig">HuggingfaceConfig</a>

Available on **crate feature `services-huggingface`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-28" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-IpfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpfsConfig.html" class="struct" title="struct opendal::services::IpfsConfig">IpfsConfig</a>

Available on **crate feature `services-ipfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-29" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipfs.html" class="struct" title="struct opendal::services::Ipfs">IpfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-IpmfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.IpmfsConfig.html" class="struct" title="struct opendal::services::IpmfsConfig">IpmfsConfig</a>

Available on **crate feature `services-ipmfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-30" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Ipmfs.html" class="struct" title="struct opendal::services::Ipmfs">IpmfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-KoofrConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.KoofrConfig.html" class="struct" title="struct opendal::services::KoofrConfig">KoofrConfig</a>

Available on **crate feature `services-koofr`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-31" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Koofr.html" class="struct" title="struct opendal::services::Koofr">KoofrBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-LakefsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.LakefsConfig.html" class="struct" title="struct opendal::services::LakefsConfig">LakefsConfig</a>

Available on **crate feature `services-lakefs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-32" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MemcachedConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemcachedConfig.html" class="struct" title="struct opendal::services::MemcachedConfig">MemcachedConfig</a>

Available on **crate feature `services-memcached`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-33" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memcached.html" class="struct" title="struct opendal::services::Memcached">MemcachedBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MemoryConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MemoryConfig.html" class="struct" title="struct opendal::services::MemoryConfig">MemoryConfig</a>

Available on **crate feature `services-memory`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-34" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Memory.html" class="struct" title="struct opendal::services::Memory">MemoryBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MiniMokaConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMokaConfig.html" class="struct" title="struct opendal::services::MiniMokaConfig">MiniMokaConfig</a>

Available on **crate feature `services-mini-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-35" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MiniMoka.html" class="struct" title="struct opendal::services::MiniMoka">MiniMokaBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MokaConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MokaConfig.html" class="struct" title="struct opendal::services::MokaConfig">MokaConfig</a>

Available on **crate feature `services-moka`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-36" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Moka.html" class="struct" title="struct opendal::services::Moka">MokaBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MongodbConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MongodbConfig.html" class="struct" title="struct opendal::services::MongodbConfig">MongodbConfig</a>

Available on **crate feature `services-mongodb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-37" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mongodb.html" class="struct" title="struct opendal::services::Mongodb">MongodbBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MonoiofsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MonoiofsConfig.html" class="struct" title="struct opendal::services::MonoiofsConfig">MonoiofsConfig</a>

Available on **crate feature `services-monoiofs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-38" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-MysqlConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MysqlConfig.html" class="struct" title="struct opendal::services::MysqlConfig">MysqlConfig</a>

Available on **crate feature `services-mysql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-39" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Mysql.html" class="struct" title="struct opendal::services::Mysql">MysqlBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-ObsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.ObsConfig.html" class="struct" title="struct opendal::services::ObsConfig">ObsConfig</a>

Available on **crate feature `services-obs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-40" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Obs.html" class="struct" title="struct opendal::services::Obs">ObsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-OnedriveConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OnedriveConfig.html" class="struct" title="struct opendal::services::OnedriveConfig">OnedriveConfig</a>

Available on **crate feature `services-onedrive`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-41" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Onedrive.html" class="struct" title="struct opendal::services::Onedrive">OnedriveBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-OssConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.OssConfig.html" class="struct" title="struct opendal::services::OssConfig">OssConfig</a>

Available on **crate feature `services-oss`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-42" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Oss.html" class="struct" title="struct opendal::services::Oss">OssBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-PcloudConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PcloudConfig.html" class="struct" title="struct opendal::services::PcloudConfig">PcloudConfig</a>

Available on **crate feature `services-pcloud`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-43" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Pcloud.html" class="struct" title="struct opendal::services::Pcloud">PcloudBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-PersyConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PersyConfig.html" class="struct" title="struct opendal::services::PersyConfig">PersyConfig</a>

Available on **crate feature `services-persy`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-44" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Persy.html" class="struct" title="struct opendal::services::Persy">PersyBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-PostgresqlConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.PostgresqlConfig.html" class="struct" title="struct opendal::services::PostgresqlConfig">PostgresqlConfig</a>

Available on **crate feature `services-postgresql`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-45" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Postgresql.html" class="struct" title="struct opendal::services::Postgresql">PostgresqlBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-RedbConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedbConfig.html" class="struct" title="struct opendal::services::RedbConfig">RedbConfig</a>

Available on **crate feature `services-redb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-46" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redb.html" class="struct" title="struct opendal::services::Redb">RedbBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-RedisConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RedisConfig.html" class="struct" title="struct opendal::services::RedisConfig">RedisConfig</a>

Available on **crate feature `services-redis`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-47" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Redis.html" class="struct" title="struct opendal::services::Redis">RedisBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-RocksdbConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.RocksdbConfig.html" class="struct" title="struct opendal::services::RocksdbConfig">RocksdbConfig</a>

Available on **crate feature `services-rocksdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-48" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Rocksdb.html" class="struct" title="struct opendal::services::Rocksdb">RocksdbBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-S3Config" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3Config.html" class="struct" title="struct opendal::services::S3Config">S3Config</a>

Available on **crate feature `services-s3`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-49" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.S3.html" class="struct" title="struct opendal::services::S3">S3Builder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-SeafileConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SeafileConfig.html" class="struct" title="struct opendal::services::SeafileConfig">SeafileConfig</a>

Available on **crate feature `services-seafile`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-50" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Seafile.html" class="struct" title="struct opendal::services::Seafile">SeafileBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-SftpConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SftpConfig.html" class="struct" title="struct opendal::services::SftpConfig">SftpConfig</a>

Available on **crate feature `services-sftp`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-51" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sftp.html" class="struct" title="struct opendal::services::Sftp">SftpBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-SledConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SledConfig.html" class="struct" title="struct opendal::services::SledConfig">SledConfig</a>

Available on **crate feature `services-sled`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-52" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-SqliteConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SqliteConfig.html" class="struct" title="struct opendal::services::SqliteConfig">SqliteConfig</a>

Available on **crate feature `services-sqlite`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-53" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sqlite.html" class="struct" title="struct opendal::services::Sqlite">SqliteBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-SurrealdbConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SurrealdbConfig.html" class="struct" title="struct opendal::services::SurrealdbConfig">SurrealdbConfig</a>

Available on **crate feature `services-surrealdb`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-54" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Surrealdb.html" class="struct" title="struct opendal::services::Surrealdb">SurrealdbBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-SwiftConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SwiftConfig.html" class="struct" title="struct opendal::services::SwiftConfig">SwiftConfig</a>

Available on **crate feature `services-swift`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-55" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-TikvConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.TikvConfig.html" class="struct" title="struct opendal::services::TikvConfig">TikvConfig</a>

Available on **crate feature `services-tikv`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-56" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Tikv.html" class="struct" title="struct opendal::services::Tikv">TikvBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-UpyunConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.UpyunConfig.html" class="struct" title="struct opendal::services::UpyunConfig">UpyunConfig</a>

Available on **crate feature `services-upyun`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-57" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Upyun.html" class="struct" title="struct opendal::services::Upyun">UpyunBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-VercelArtifactsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifactsConfig.html" class="struct" title="struct opendal::services::VercelArtifactsConfig">VercelArtifactsConfig</a>

Available on **crate feature `services-vercel-artifacts`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-58" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelArtifacts.html" class="struct" title="struct opendal::services::VercelArtifacts">VercelArtifactsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-VercelBlobConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlobConfig.html" class="struct" title="struct opendal::services::VercelBlobConfig">VercelBlobConfig</a>

Available on **crate feature `services-vercel-blob`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-59" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.VercelBlob.html" class="struct" title="struct opendal::services::VercelBlob">VercelBlobBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-WebdavConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebdavConfig.html" class="struct" title="struct opendal::services::WebdavConfig">WebdavConfig</a>

Available on **crate feature `services-webdav`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-60" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webdav.html" class="struct" title="struct opendal::services::Webdav">WebdavBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-WebhdfsConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.WebhdfsConfig.html" class="struct" title="struct opendal::services::WebhdfsConfig">WebhdfsConfig</a>

Available on **crate feature `services-webhdfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-61" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Webhdfs.html" class="struct" title="struct opendal::services::Webhdfs">WebhdfsBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#impl-Configurator-for-YandexDiskConfig" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html" class="trait" title="trait opendal::Configurator">Configurator</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDiskConfig.html" class="struct" title="struct opendal::services::YandexDiskConfig">YandexDiskConfig</a>

Available on **crate feature `services-yandex-disk`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder-62" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Configurator.html#associatedtype.Builder" class="associatedtype">Builder</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.YandexDisk.html" class="struct" title="struct opendal::services::YandexDisk">YandexDiskBuilder</a>
