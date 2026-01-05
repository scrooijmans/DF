# Module rfc_0793_generic_kv_services Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#126" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Generic KV services

- Proposal Name: `generic-kv-services`
- Start Date: 2022-10-03
- RFC PR: [apache/opendal#793](https://github.com/apache/opendal/pull/793)
- Tracking Issue: [apache/opendal#794](https://github.com/apache/opendal/issues/794)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#summary" class="doc-anchor">Â§</a>Summary

Add generic kv services support OpenDAL.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL now has some kv services support:

- memory
- redis

However, maintaining them is complex and very easy to be wrong. We donâ€™t want to implement similar logic for every kv service. This RFC intends to introduce a generic kv service so that we can:

- Implement OpenDAL Accessor on this generic kv service
- Add new kv service support via generic kv API.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

No user-side changes.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

OpenDAL will introduce a generic kv service:

``` rust
trait KeyValueAccessor {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: &[u8], value: &[u8]) -> Result<()>;
}
```

We will implement the OpenDAL service on `KeyValueAccessor`. To add new kv service support, users only need to implement it against `KeyValueAccessor`.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#spec" class="doc-anchor">Â§</a>Spec

This RFC is mainly inspired by [TiFS: FUSE based on TiKV](https://github.com/Hexilee/tifs/blob/main/contribution/design.md). We will use the same `ScopedKey` idea in `TiFS`.

``` rust
pub enum ScopedKey {
    Meta,
    Inode(u64),
    Block {
        ino: u64,
        block: u64,
    },
    Entry {
        parent: u64,
        name: String,
    },
}
```

We can encode a scoped key into a byte array as a key. Following is the common layout of an encoded key.

``` text
+ 1byte +<----------------------------+ dynamic size +------------------------------------>+
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       v                                                                                  v
+------------------------------------------------------------------------------------------+
|       |                                                                                  |
| scope |                                       body                                       |
|       |                                                                                  |
+-------+----------------------------------------------------------------------------------+
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#meta" class="doc-anchor">Â§</a>Meta

There is only one key in the meta scope. The meta key is designed to store metadata of our filesystem. Following is the layout of an encoded meta key.

``` text
+ 1byte +
|       |
|       |
|       |
|       |
|       |
|       |
|       v
+-------+
|       |
|   0   |
|       |
+-------+
```

This key will store data:

``` rust
pub struct Meta {
    inode_next: u64,
}
```

The meta-structure contains only an auto-increasing counter `inode_next`, designed to generate an inode number.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#inode" class="doc-anchor">Â§</a>Inode

Keys in the inode scope are designed to store attributes of files. Following is the layout of an encoded inode key.

``` text
+ 1byte +<-------------------------------+ 8bytes +--------------------------------------->+
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       |                                                                                  |
|       v                                                                                  v
+------------------------------------------------------------------------------------------+
|       |                                                                                  |
|   1   |                               inode number                                       |
|       |                                                                                  |
+-------+----------------------------------------------------------------------------------+
```

This key will store data:

``` rust
pub struct Inode {
    meta: Metadata,
    blocks: HashMap<u64, u32>,
}
```

blocks is the map from `block_id` -\> `size`. We will use this map to calculate the correct blocks to read.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#block" class="doc-anchor">Â§</a>Block

Keys in the block scope are designed to store blocks of a file. Following is the layout of an encoded block key.

``` text
+ 1byte +<----------------- 8bytes ---------------->+<------------------- 8bytes ----------------->+
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       v                                           v                                              v
+--------------------------------------------------------------------------------------------------+
|       |                                           |                                              |
|   2   |              inode number                 |                  block index                 |
|       |                                           |                                              |
+-------+-------------------------------------------+----------------------------------------------+
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#entry" class="doc-anchor">Â§</a>Entry

Keys in the file index scope are designed to store the entry of the file. Following is the layout of an encoded file entry key.

``` text
+ 1byte +<----------------- 8bytes ---------------->+<-------------- dynamic size ---------------->+
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       |                                           |                                              |
|       v                                           v                                              v
+--------------------------------------------------------------------------------------------------+
|       |                                           |                                              |
|   3   |     inode number of parent directory      |         file name in utf-8 encoding          |
|       |                                           |                                              |
+-------+-------------------------------------------+----------------------------------------------+
```

Store the correct inode number for this file.

``` rust
pub struct Index {
    pub ino: u64,
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None.
