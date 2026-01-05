# Struct OpWrite Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/ops.rs.html#682-693" class="src">Source</a>

``` rust
pub struct OpWrite { /* private fields */ }
```

Expand description

Args for `write` operation.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#impl-OpWrite" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.new" class="fn">new</a>() -\> Self

Create a new `OpWrite`.

If input path is not a file path, an error will be returned.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.append" class="fn">append</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get the append from op.

The append is the flag to indicate that this write operation is an append operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_append" class="fn">with_append</a>(self, append: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set the append mode of op.

If the append mode is set, the data will be appended to the end of the file.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#notes" class="doc-anchor">Â§</a>Notes

Service could return `Unsupported` if the underlying storage does not support append.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.content_type" class="fn">content_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the content type from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_content_type" class="fn">with_content_type</a>(self, content_type: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the content type of option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.content_disposition" class="fn">content_disposition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the content disposition from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_content_disposition" class="fn">with_content_disposition</a>(self, content_disposition: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the content disposition of option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.content_encoding" class="fn">content_encoding</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the content encoding from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_content_encoding" class="fn">with_content_encoding</a>(self, content_encoding: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the content encoding of option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.cache_control" class="fn">cache_control</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get the cache control from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_cache_control" class="fn">with_cache_control</a>(self, cache_control: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the content type of option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.concurrent" class="fn">concurrent</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the concurrent.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_concurrent" class="fn">with_concurrent</a>(self, concurrent: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the maximum concurrent write task amount.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_if_match" class="fn">with_if_match</a>(self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the If-Match of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.if_match" class="fn">if_match</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get If-Match from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_if_none_match" class="fn">with_if_none_match</a>(self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Set the If-None-Match of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.if_none_match" class="fn">if_none_match</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Get If-None-Match from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_if_not_exists" class="fn">with_if_not_exists</a>(self, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Set the If-Not-Exist of the option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.if_not_exists" class="fn">if_not_exists</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get If-Not-Exist from option

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.with_user_metadata" class="fn">with_user_metadata</a>(self, metadata: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Set the user defined metadata of the op

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.user_metadata" class="fn">user_metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>

Get the user defined metadata from the op

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#impl-Clone-for-OpWrite" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#impl-Debug-for-OpWrite" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#impl-Default-for-OpWrite" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#impl-From%3COpWrite%3E-for-PresignOperation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html#blanket-implementations" class="anchor">Â§</a>
