# Trait Adapter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/kv/api.rs.html#90-132" class="src">Source</a>

``` rust
pub trait Adapter:
    Send
    + Sync
    + Debug
    + Unpin
    + 'static {
    type Scanner: Scan;

    // Required methods
    fn info(&self) -> Info;
    fn get(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<Option<Buffer>>> + MaybeSend;
    fn set(
        &self,
        path: &str,
        value: Buffer,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
    fn delete(&self, path: &str) -> impl Future<Output = Result<()>> + MaybeSend;

    // Provided methods
    fn scan(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<Self::Scanner>> + MaybeSend { ... }
    fn append(
        &self,
        path: &str,
        value: &[u8],
    ) -> impl Future<Output = Result<()>> + MaybeSend { ... }
}
```

Expand description

KvAdapter is the adapter to underlying kv services.

By implement this trait, any kv service can work as an OpenDAL Service.

## Required Associated Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#required-associated-types" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#associatedtype.Scanner" class="associatedtype">Scanner</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html" class="trait" title="trait opendal::raw::adapters::kv::Scan">Scan</a>

TODO: use default associate type `= ()` after stabilized

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/struct.Info.html" class="struct" title="struct opendal::raw::adapters::kv::Info">Info</a>

Return the info of this key value accessor.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#tymethod.get" class="fn">get</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Get a key from service.

- return `Ok(None)` if this key is not exist.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#tymethod.set" class="fn">set</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Set a key into service.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#tymethod.delete" class="fn">delete</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Delete a key from service.

- return `Ok(())` even if this key is not exist.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#method.scan" class="fn">scan</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#associatedtype.Scanner" class="associatedtype" title="type opendal::raw::adapters::kv::Adapter::Scanner">Scanner</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Scan a key prefix to get all keys that start with this key.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#method.append" class="fn">append</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Append a key into service

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#implementors" class="anchor">Â§</a>
