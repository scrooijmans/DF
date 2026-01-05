# Trait Scan Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/kv/api.rs.html#30-35" class="src">Source</a>

``` rust
pub trait Scan:
    Send
    + Sync
    + Unpin {
    // Required method
    fn next(
        &mut self,
    ) -> impl Future<Output = Result<Option<String>>> + MaybeSend;
}
```

Expand description

Scan is the async iterator returned by `Adapter::scan`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#tymethod.next" class="fn">next</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Fetch the next key in the current key prefix

`Ok(None)` means no further key will be returned

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#impl-Scan-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html" class="trait" title="trait opendal::raw::adapters::kv::Scan">Scan</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

A noop implementation of Scan

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#method.next" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#impl-Scan-for-Box%3CT%3E" class="anchor">Â§</a>

### impl\<T: ScanDyn + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html" class="trait" title="trait opendal::raw::adapters::kv::Scan">Scan</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#method.next-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#tymethod.next" class="fn">next</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Scan.html#implementors" class="anchor">Â§</a>
