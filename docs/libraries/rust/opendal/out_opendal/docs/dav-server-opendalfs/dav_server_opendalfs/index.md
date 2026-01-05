# Crate dav_server_opendalfs Copy item path

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/src/dav_server_opendalfs/lib.rs.html#18-52" class="src">Source</a>

Expand description

dav-server-opendalfs is an dav-server implementation using opendal.

This crate can help you to access ANY storage services with the same webdav API.

``` rust
use anyhow::Result;
use dav_server::davpath::DavPath;
use dav_server::fs::DavFileSystem;
use dav_server_opendalfs::OpendalFs;
use opendal::services::Memory;
use opendal::Operator;

#[tokio::test]
async fn test() -> Result<()> {
    let op = Operator::new(Memory::default())?.finish();

    let webdavfs = OpendalFs::new(op);

    let metadata = webdavfs
        .metadata(&DavPath::new("/").unwrap())
        .await
        .unwrap();
    println!("{}", metadata.is_dir());

    Ok(())
}
```

## Structs<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html" class="struct" title="struct dav_server_opendalfs::OpendalFs">OpendalFs</a>  
OpendalFs is a `DavFileSystem` implementation for opendal.
