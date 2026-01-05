# Module rfc_0221_create_dir Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#58" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Create dir

- Proposal Name: `create-dir`
- Start Date: 2022-04-06
- RFC PR: [apache/opendal#221](https://github.com/apache/opendal/pull/221)
- Tracking Issue: [apache/opendal#222](https://github.com/apache/opendal/issues/222)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#summary" class="doc-anchor">Â§</a>Summary

Add creating dir support for OpenDAL.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Interoperability between OpenDAL services requires dir support. The object storage system will simulate dir operations with `/` via object ends. But we canâ€™t share the same behavior with `fs`, as `mkdir` is a separate syscall.

So we need to unify the behavior about dir across different services.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

After this proposal got merged, we will treat all paths that end with `/` as a dir.

For example:

- `read("abc/")` will return an `IsDir` error.
- `write("abc/")` will return an `IsDir` error.
- `stat("abc/")` will be guaranteed to return a dir or a `NotDir` error.
- `delete("abc/")` will be guaranteed to delete a dir or `NotDir` / `NotEmpty` error.
- `list("abc/")` will be guaranteed to list a dir or a `NotDir` error.

And we will support create an empty object:

``` rust
// create a dir object "abc/"
let _ = op.object("abc/").create().await?;
// create a file object "abc"
let _ = op.object("abc").create().await?;
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

And we will add a new API called `create` to create an empty object.

``` rust
struct OpCreate {
    path: String,
    mode: ObjectMode,
}

pub trait Accessor: Send + Sync + Debug {
    async fn create(&self, args: &OpCreate) -> Result<Metadata>;
}
```

`Object` will expose API like `create` which will call `Accessor::create()` internally.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

When writing this proposal, [io_error_more](https://github.com/rust-lang/rust/issues/86442) is not stabilized yet. We canâ€™t use `NotADirectory` nor `IsADirectory` directly.

Using `from_raw_os_error` is unacceptable because we canâ€™t carry our error context.

``` rust
use std::io;

let error = io::Error::from_raw_os_error(22);
assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
```

So we will use `ErrorKind::Other` for now, which means our users canâ€™t check the following errors:

- `IsADirectory`
- `DirectoryNotEmpty`
- `NotADirectory`

Until they get stabilized.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
