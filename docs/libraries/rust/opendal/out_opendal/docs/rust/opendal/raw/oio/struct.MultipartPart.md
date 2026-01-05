# Struct MultipartPart Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/multipart_write.rs.html#114-121" class="src">Source</a>

``` rust
pub struct MultipartPart {
    pub part_number: usize,
    pub etag: String,
    pub checksum: Option<String>,
}
```

Expand description

The result of [`MultipartWrite::write_part`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.MultipartWrite.html#tymethod.write_part "method opendal::raw::oio::MultipartWrite::write_part").

services implement should convert MultipartPart to their own represents.

- `part_number` is the index of the part, starting from 0.
- `etag` is the `ETag` of the part.
- `checksum` is the optional checksum of the part.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#structfield.part_number" class="anchor field">Â§</a>`part_number: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The number of the part, starting from 0.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#structfield.etag" class="anchor field">Â§</a>`etag: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The etag of the part.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#structfield.checksum" class="anchor field">Â§</a>`checksum: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The checksum of the part.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#impl-Clone-for-MultipartPart" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html" class="struct" title="struct opendal::raw::oio::MultipartPart">MultipartPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html" class="struct" title="struct opendal::raw::oio::MultipartPart">MultipartPart</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html#blanket-implementations" class="anchor">Â§</a>
