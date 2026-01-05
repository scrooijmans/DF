# Trait PathQuery Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/path_cache.rs.html#29-44" class="src">Source</a>

``` rust
pub trait PathQuery {
    // Required methods
    fn root(&self) -> impl Future<Output = Result<String>> + MaybeSend;
    fn query(
        &self,
        parent_id: &str,
        name: &str,
    ) -> impl Future<Output = Result<Option<String>>> + MaybeSend;
    fn create_dir(
        &self,
        parent_id: &str,
        name: &str,
    ) -> impl Future<Output = Result<String>> + MaybeSend;
}
```

Available on **crate feature `internal-path-cache`** only.

Expand description

The trait required for path cacher.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html#tymethod.root" class="fn">root</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Fetch the id for the root of the service.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html#tymethod.query" class="fn">query</a>( &self, parent_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Query the id by parent_id and name.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html#tymethod.create_dir" class="fn">create_dir</a>( &self, parent_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Create a dir by parent_id and name.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html#implementors" class="anchor">Â§</a>
