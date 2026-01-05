# Struct PathCacher Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path_cache.rs.html#57-66" class="src">Source</a>

``` rust
pub struct PathCacher<Q: PathQuery> { /* private fields */ }
```

Available on **crate feature `internal-path-cache`** only.

Expand description

PathCacher is a cache for path query.

OpenDAL is designed for path based storage systems, such as S3, HDFS, etc. But there are many services that are not path based, such as OneDrive, Google Drive, etc. For these services, we look up files based on id. The lookup of id is very expensive, so we cache the path to id mapping in PathCacher.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#behavior" class="doc-anchor">Â§</a>Behavior

The `path` in the cache is always an absolute one. For example, if the service root is `/root/`, then the path of file `a/b` in cache will be `/root/a/b`.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#impl-PathCacher%3CQ%3E" class="anchor">Â§</a>

### impl\<Q: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html" class="trait" title="trait opendal::raw::PathQuery">PathQuery</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html" class="struct" title="struct opendal::raw::PathCacher">PathCacher</a>\<Q\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#method.new" class="fn">new</a>(query: Q) -\> Self

Create a new path cacher.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#method.with_lock" class="fn">with_lock</a>(self) -\> Self

Enable the lock for the path cacher.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#method.insert" class="fn">insert</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Insert a new cache entry.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#method.remove" class="fn">remove</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Remove a cache entry.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#method.get" class="fn">get</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>

Get the id for the given path.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#method.ensure_dir" class="fn">ensure_dir</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Ensure input dir exists.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html#blanket-implementations" class="anchor">Â§</a>
