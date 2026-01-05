# Module rfc_0661_path_in_accessor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#122" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Path in accessor

- Proposal Name: `path_in_accessor`
- Start Date: 2022-09-12
- RFC PR: [apache/opendal#661](https://github.com/apache/opendal/pull/661)
- Tracking Issue: [apache/opendal#662](https://github.com/apache/opendal/issues/662)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#summary" class="doc-anchor">Â§</a>Summary

Move the path from `OpXxx` to `Accessor` directly.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#motivation" class="doc-anchor">Â§</a>Motivation

`Accessor` uses `OpXxx` to carry `path` input:

``` rust
impl Accessor {
    async fn read(&self, args: &OpRead) -> Result<BytesReader> {
        let _ = args;
        unimplemented!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct OpRead {
    path: String,
    offset: Option<u64>,
    size: Option<u64>,
}
```

However, nearly all operation requires a `path`. And the path is represented in `String`, which means we have to clone it:

``` rust
impl OpRead {
    pub fn new(path: &str, range: impl RangeBounds<u64>) -> Result<Self> {
        let br = BytesRange::from(range);

        Ok(Self {
            path: path.to_string(),
            offset: br.offset(),
            size: br.size(),
        })
    }
}
```

Besides, we canâ€™t expose low-level APIs like:

``` rust
impl Object {
    pub async fn read_with(&self, op: OpRead) -> Result<Vec<u8>> {
        ..
    }
}
```

Because users canâ€™t build the required `OpRead`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

With this RFC, users can use low-level APIs can control the `OpXxx` directly:

``` rust
impl Object {
    pub async fn read_with(&self, op: OpRead) -> Result<Vec<u8>> {
        ..
    }

    pub async fn write_with(&self, op: OpWrite, bs: impl Into<Vec<u8>>) -> Result<()> {
        ..
    }
}
```

So we can add more args in requests like:

``` rust
o.write_with(OpWrite::new().with_content_md5("xxxxx"), bs).await;
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

All `path` in `OpXxx` will be moved to `Accessor` directly:

``` rust
pub trait Accessor: Send + Sync + Debug {
    async fn create(&self, path: &str, args: OpCreate) -> Result<()> {}
    
    async fn read(&self, path: &str, args: OpRead) -> Result<BytesReader> {}
    
    ...
}
```

- All functions that accept `OpXxx` requires ownership instead of reference.
- All `OpXxx::new()` will introduce breaking changes:
  ``` diff
  - pub fn new(path: &str, range: impl RangeBounds<u64>) -> Result<Self>
  + pub fn new(range: impl RangeBounds<u64>) -> Self
  ```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#breaking-changes" class="doc-anchor">Â§</a>Breaking Changes

This RFC may break usersâ€™ code in the following ways:

- Code that depends on `Accessor`:
  - Self-implemented Services
  - Self-implemented Layers
- Code that depends on `OpXxx`

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

We can add more fields in `OpXxx`.
