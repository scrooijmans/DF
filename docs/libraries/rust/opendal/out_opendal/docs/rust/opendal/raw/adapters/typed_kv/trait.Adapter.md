# Trait Adapter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/typed_kv/api.rs.html#44-67" class="src">Source</a>

``` rust
pub trait Adapter:
    Send
    + Sync
    + Debug
    + Unpin
    + 'static {
    // Required methods
    fn info(&self) -> Info;
    fn get(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<Option<Value>>> + MaybeSend;
    fn set(
        &self,
        path: &str,
        value: Value,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
    fn delete(&self, path: &str) -> impl Future<Output = Result<()>> + MaybeSend;

    // Provided method
    fn scan(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<Vec<String>>> + MaybeSend { ... }
}
```

Expand description

Adapter is the typed adapter to underlying kv services.

By implement this trait, any kv service can work as an OpenDAL Service.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#notes" class="doc-anchor">Â§</a>Notes

`typed_kv::Adapter` is the typed version of `kv::Adapter`. Itâ€™s more efficient if the underlying kv service can store data with its type. For example, we can store `Bytes` along with its metadata so that we donâ€™t need to serialize/deserialize it when we get it from the service.

Ideally, we should use `typed_kv::Adapter` instead of `kv::Adapter` for in-memory rust libs like moka and dashmap.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Info.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Info">Info</a>

Return the info of this key value accessor.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#tymethod.get" class="fn">get</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Get a value from adapter.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#tymethod.set" class="fn">set</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Value.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Value">Value</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Set a value into adapter.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#tymethod.delete" class="fn">delete</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Delete a value from adapter.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#method.scan" class="fn">scan</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Scan a key prefix to get all keys that start with this key.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html#implementors" class="anchor">Â§</a>
