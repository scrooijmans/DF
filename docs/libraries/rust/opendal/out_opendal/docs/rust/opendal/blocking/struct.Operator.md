# Struct Operator Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/blocking/operator.rs.html#124-127" class="src">Source</a>

``` rust
pub struct Operator { /* private fields */ }
```

Available on **crate feature `blocking`** only.

Expand description

Use OpenDAL in blocking context.

## <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes" class="doc-anchor">Â§</a>Notes

blocking::Operator is a wrapper around [`AsyncOperator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator"). It calls async runtimesâ€™ `block_on` API to spawn blocking tasks. Please avoid using blocking::Operator in async context.

## <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples" class="doc-anchor">Â§</a>Examples

### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#init-in-async-context" class="doc-anchor">Â§</a>Init in async context

blocking::Operator will use current async contextâ€™s runtime to handle the async calls.

This is just for initialization. You must use `blocking::Operator` in blocking context.

``` rust

#[tokio::main]
async fn main() -> Result<()> {
    // Create fs backend builder.
    let mut builder = services::S3::default().bucket("test").region("us-east-1");
    let op = Operator::new(builder)?.finish();

    // Build an `blocking::Operator` with blocking layer to start operating the storage.
    let _: blocking::Operator = blocking::Operator::new(op)?;

    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#in-async-context-with-blocking-functions" class="doc-anchor">Â§</a>In async context with blocking functions

If `blocking::Operator` is called in blocking function, please fetch a \[`tokio::runtime::EnterGuard`\] first. You can use \[`Handle::try_current`\] first to get the handle and then call \[`Handle::enter`\]. This often happens in the case that async function calls blocking function.

``` rust

#[tokio::main]
async fn main() -> Result<()> {
    let _ = blocking_fn()?;
    Ok(())
}

fn blocking_fn() -> Result<blocking::Operator> {
    // Create fs backend builder.
    let mut builder = services::S3::default().bucket("test").region("us-east-1");
    let op = Operator::new(builder)?.finish();

    let handle = tokio::runtime::Handle::try_current().unwrap();
    let _guard = handle.enter();
    // Build an `blocking::Operator` to start operating the storage.
    let op: blocking::Operator = blocking::Operator::new(op)?;
    Ok(op)
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#in-blocking-context" class="doc-anchor">Â§</a>In blocking context

In a pure blocking context, we can create a runtime and use it to create the `blocking::Operator`.

> The following code uses a global statically created runtime as an example, please manage the runtime on demand.

``` rust

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

fn main() -> Result<()> {
    // Create fs backend builder.
    let mut builder = services::S3::default().bucket("test").region("us-east-1");
    let op = Operator::new(builder)?.finish();

    // Fetch the `EnterGuard` from global runtime.
    let _guard = RUNTIME.enter();
    // Build an `blocking::Operator` with blocking layer to start operating the storage.
    let _: blocking::Operator = blocking::Operator::new(op)?;

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#impl-Operator" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.new" class="fn">new</a>(op: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">AsyncOperator</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Create a new `BlockingLayer` with the current runtimeâ€™s handle

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.from_uri" class="fn">from_uri</a>(uri: impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Create a blocking operator from URI based configuration.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.info" class="fn">info</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

Get information of underlying accessor.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-1" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
use opendal::blocking::Operator;

let info = op.info();
```

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#impl-Operator-1" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>

#### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#operator-blocking-api" class="doc-anchor">Â§</a>Operator blocking API.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.stat" class="fn">stat</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Get given pathâ€™s metadata.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#behavior" class="doc-anchor">Â§</a>Behavior

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#services-that-support-create_dir" class="doc-anchor">Â§</a>Services that support `create_dir`

`test` and `test/` may vary in some services such as S3. However, on a local file system, theyâ€™re identical. Therefore, the behavior of `stat("test")` and `stat("test/")` might differ in certain edge cases. Always use `stat("test/")` when you need to access a directory if possible.

Here are the behavior list:

| Case | Path | Result |
|----|----|----|
| stat existing dir | `abc/` | Metadata with dir mode |
| stat existing file | `abc/def_file` | Metadata with file mode |
| stat dir without `/` | `abc/def_dir` | Error `NotFound` or metadata with dir mode |
| stat file with `/` | `abc/def_file/` | Error `NotFound` |
| stat not existing path | `xyz` | Error `NotFound` |

Refer to [RFC: List Prefix](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html "mod opendal::docs::rfcs::rfc_3243_list_prefix") for more details.

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#services-that-not-support-create_dir" class="doc-anchor">Â§</a>Services that not support `create_dir`

For services that not support `create_dir`, `stat("test/")` will return `NotFound` even when `test/abc` exists since the service wonâ€™t have the concept of dir. There is nothing we can do about this.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-2" class="doc-anchor">Â§</a>Examples

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#check-if-file-exists" class="doc-anchor">Â§</a>Check if file exists

``` rust
use opendal::blocking;
use opendal::ErrorKind;
if let Err(e) = op.stat("test") {
    if e.kind() == ErrorKind::NotFound {
        println!("file not exist")
    }
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.stat_options" class="fn">stat_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.StatOptions.html" class="struct" title="struct opendal::options::StatOptions">StatOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Get given pathâ€™s metadata with extra options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#behavior-1" class="doc-anchor">Â§</a>Behavior

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#services-that-support-create_dir-1" class="doc-anchor">Â§</a>Services that support `create_dir`

`test` and `test/` may vary in some services such as S3. However, on a local file system, theyâ€™re identical. Therefore, the behavior of `stat("test")` and `stat("test/")` might differ in certain edge cases. Always use `stat("test/")` when you need to access a directory if possible.

Here are the behavior list:

| Case | Path | Result |
|----|----|----|
| stat existing dir | `abc/` | Metadata with dir mode |
| stat existing file | `abc/def_file` | Metadata with file mode |
| stat dir without `/` | `abc/def_dir` | Error `NotFound` or metadata with dir mode |
| stat file with `/` | `abc/def_file/` | Error `NotFound` |
| stat not existing path | `xyz` | Error `NotFound` |

Refer to [RFC: List Prefix](https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html "mod opendal::docs::rfcs::rfc_3243_list_prefix") for more details.

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#services-that-not-support-create_dir-1" class="doc-anchor">Â§</a>Services that not support `create_dir`

For services that not support `create_dir`, `stat("test/")` will return `NotFound` even when `test/abc` exists since the service wonâ€™t have the concept of dir. There is nothing we can do about this.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.exists" class="fn">exists</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Check if this path exists or not.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#example" class="doc-anchor">Â§</a>Example

``` rust
use anyhow::Result;
use opendal::blocking;
use opendal::blocking::Operator;
fn test(op: blocking::Operator) -> Result<()> {
    let _ = op.exists("test")?;

    Ok(())
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.create_dir" class="fn">create_dir</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Create a dir at given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-1" class="doc-anchor">Â§</a>Notes

To indicate that a path is a directory, it is compulsory to include a trailing / in the path. Failure to do so may result in `NotADirectory` error being returned by OpenDAL.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#behavior-2" class="doc-anchor">Â§</a>Behavior

- Create on existing dir will succeed.
- Create dir is always recursive, works like `mkdir -p`

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-3" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
op.create_dir("path/to/dir/")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.read" class="fn">read</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read the whole path into a bytes.

This function will allocate a new bytes internally. For more precise memory control or reading data lazily, please use [`blocking::Operator::reader`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.reader "method opendal::blocking::Operator::reader")

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-4" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
let bs = op.read("path/to/file")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.read_options" class="fn">read_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReadOptions.html" class="struct" title="struct opendal::options::ReadOptions">ReadOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

Read the whole path into a bytes with extra options.

This function will allocate a new bytes internally. For more precise memory control or reading data lazily, please use [`blocking::Operator::reader`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.reader "method opendal::blocking::Operator::reader")

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.reader" class="fn">reader</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>\>

Create a new reader which can read the whole path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-5" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
let r = op.reader("path/to/file")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.reader_options" class="fn">reader_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ReaderOptions.html" class="struct" title="struct opendal::options::ReaderOptions">ReaderOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Reader.html" class="struct" title="struct opendal::blocking::Reader">Reader</a>\>

Create a new reader with extra options

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.write" class="fn">write</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Write bytes into given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-2" class="doc-anchor">Â§</a>Notes

- Write will make sure all bytes has been written, or an error will be returned.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-6" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;
use opendal::blocking;

op.write("path/to/file", vec![0; 4096])?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.write_options" class="fn">write_options</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, bs: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Write data with options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-3" class="doc-anchor">Â§</a>Notes

- Write will make sure all bytes has been written, or an error will be returned.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.writer" class="fn">writer</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html" class="struct" title="struct opendal::blocking::Writer">Writer</a>\>

Write multiple bytes into given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-4" class="doc-anchor">Â§</a>Notes

- Write will make sure all bytes has been written, or an error will be returned.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-7" class="doc-anchor">Â§</a>Examples

``` rust
use bytes::Bytes;

let mut w = op.writer("path/to/file")?;
w.write(vec![0; 4096])?;
w.write(vec![1; 4096])?;
w.close()?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.writer_options" class="fn">writer_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.WriteOptions.html" class="struct" title="struct opendal::options::WriteOptions">WriteOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Writer.html" class="struct" title="struct opendal::blocking::Writer">Writer</a>\>

Create a new writer with extra options

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.copy" class="fn">copy</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Copy a file from `from` to `to`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-5" class="doc-anchor">Â§</a>Notes

- `from` and `to` must be a file.
- `to` will be overwritten if it exists.
- If `from` and `to` are the same, nothing will happen.
- `copy` is idempotent. For same `from` and `to` input, the result will be the same.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-8" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;

op.copy("path/to/file", "path/to/file2")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.rename" class="fn">rename</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Rename a file from `from` to `to`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-6" class="doc-anchor">Â§</a>Notes

- `from` and `to` must be a file.
- `to` will be overwritten if it exists.
- If `from` and `to` are the same, a `IsSameFile` error will occur.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-9" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;

op.rename("path/to/file", "path/to/file2")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.delete" class="fn">delete</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Delete given path.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-7" class="doc-anchor">Â§</a>Notes

- Delete not existing error wonâ€™t return errors.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-10" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
op.delete("path/to/file")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.delete_options" class="fn">delete_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.DeleteOptions.html" class="struct" title="struct opendal::options::DeleteOptions">DeleteOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Delete given path with options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-8" class="doc-anchor">Â§</a>Notes

- Delete not existing error wonâ€™t return errors.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.delete_iter" class="fn">delete_iter</a>\<I, D\>(&self, iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = D\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete an infallible iterator of paths.

Also see:

- [`blocking::Operator::delete_try_iter`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.delete_try_iter "method opendal::blocking::Operator::delete_try_iter"): delete an fallible iterator of paths.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.delete_try_iter" class="fn">delete_try_iter</a>\<I, D\>(&self, try_iter: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<D\>\>, D: <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a>,

Delete a fallible iterator of paths.

Also see:

- [`blocking::Operator::delete_iter`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.delete_iter "method opendal::blocking::Operator::delete_iter"): delete an infallible iterator of paths.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.deleter" class="fn">deleter</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Deleter.html" class="struct" title="struct opendal::blocking::Deleter">Deleter</a>\>

Create a \[`BlockingDeleter`\] to continuously remove content from storage.

It leverages batch deletion capabilities provided by storage services for efficient removal.

Users can have more control over the deletion process by using \[`BlockingDeleter`\] directly.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.remove_all" class="fn">remove_all</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Remove the path and all nested dirs and files recursively.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-9" class="doc-anchor">Â§</a>Notes

We donâ€™t support batch delete now.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-11" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
op.remove_all("path/to/dir")?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.list" class="fn">list</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>\>\>

List entries that starts with given `path` in parent dir.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-10" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#recursively-list" class="doc-anchor">Â§</a>Recursively List

This function only read the children of the given directory. To read all entries recursively, use `blocking::Operator::list_options("path", opts)` instead.

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#streaming-list" class="doc-anchor">Â§</a>Streaming List

This function will read all entries in the given directory. It could take very long time and consume a lot of memory if the directory contains a lot of entries.

In order to avoid this, you can use [`blocking::Operator::lister`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.lister "method opendal::blocking::Operator::lister") to list entries in a streaming way.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-12" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::blocking;
use opendal::blocking::Operator;
use opendal::EntryMode;
let mut entries = op.list("path/to/dir/")?;
for entry in entries {
    match entry.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file")
        }
        EntryMode::DIR => {
            println!("Handling dir {}", entry.path())
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.list_options" class="fn">list_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Entry.html" class="struct" title="struct opendal::Entry">Entry</a>\>\>

List entries that starts with given `path` in parent dir. with options.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-11" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#streaming-list-1" class="doc-anchor">Â§</a>Streaming List

This function will read all entries in the given directory. It could take very long time and consume a lot of memory if the directory contains a lot of entries.

In order to avoid this, you can use [`blocking::Operator::lister`](https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.lister "method opendal::blocking::Operator::lister") to list entries in a streaming way.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.lister" class="fn">lister</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Lister.html" class="struct" title="struct opendal::blocking::Lister">Lister</a>\>

List entries that starts with given `path` in parent dir.

This function will create a new \[`BlockingLister`\] to list entries. Users can stop listing via dropping this [`Lister`](https://opendal.apache.org/docs/rust/opendal/struct.Lister.html "struct opendal::Lister").

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#notes-12" class="doc-anchor">Â§</a>Notes

###### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#recursively-list-1" class="doc-anchor">Â§</a>Recursively List

This function only read the children of the given directory. To read all entries recursively, use \[`blocking::Operator::lister_with`\] and `delimiter("")` instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#examples-13" class="doc-anchor">Â§</a>Examples

``` rust
use futures::TryStreamExt;
use opendal::blocking;
use opendal::blocking::Operator;
use opendal::EntryMode;
let mut ds = op.lister("path/to/dir/")?;
for de in ds {
    let de = de?;
    match de.metadata().mode() {
        EntryMode::FILE => {
            println!("Handling file")
        }
        EntryMode::DIR => {
            println!("Handling dir like start a new list via meta.path()")
        }
        EntryMode::Unknown => continue,
    }
}
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.lister_options" class="fn">lister_options</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, opts: <a href="https://opendal.apache.org/docs/rust/opendal/options/struct.ListOptions.html" class="struct" title="struct opendal::options::ListOptions">ListOptions</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Lister.html" class="struct" title="struct opendal::blocking::Lister">Lister</a>\>

List entries within a given directory as an iterator with options.

This function will create a new handle to list entries.

An error will be returned if given path doesnâ€™t end with `/`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.check" class="fn">check</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Check if this operator can work correctly.

We will send a `list` request to path and return any errors we met.

``` rust
use opendal::blocking;
use opendal::blocking::Operator;
use opendal::ErrorKind;

op.check()?;
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#impl-Clone-for-Operator" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#impl-Debug-for-Operator" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#impl-From%3COperator%3E-for-Operator" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(val: <a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html" class="struct" title="struct opendal::blocking::Operator">Operator</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/blocking/struct.Operator.html#blanket-implementations" class="anchor">Â§</a>
