# Struct FileIOBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#198-207" class="src">Source</a>

``` rust
pub struct FileIOBuilder { /* private fields */ }
```

Expand description

Builder for [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#impl-FileIOBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.new" class="fn">new</a>(scheme_str: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>) -\> Self

Creates a new builder with scheme. See [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO") for supported schemes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.new_fs_io" class="fn">new_fs_io</a>() -\> Self

Creates a new builder for local file io.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.into_parts" class="fn">into_parts</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>)

Fetch the scheme string.

The scheme_str will be empty if it’s None.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.with_prop" class="fn">with_prop</a>(self, key: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>, value: impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>) -\> Self

Add argument for operator.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.with_props" class="fn">with_props</a>( self, args: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>, impl <a href="https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html" class="trait" title="trait alloc::string::ToString">ToString</a>)\>, ) -\> Self

Add argument for operator.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.with_extension" class="fn">with_extension</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>(self, ext: T) -\> Self

Add an extension to the file IO builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.with_extensions" class="fn">with_extensions</a>(self, extensions: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>) -\> Self

Adds multiple extensions to the file IO builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.extension" class="fn">extension</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>\>

where T: 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Fetch an extension from the file IO builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>\>

Builds [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#impl-Clone-for-FileIOBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#impl-Debug-for-FileIOBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html#blanket-implementations" class="anchor">§</a>
