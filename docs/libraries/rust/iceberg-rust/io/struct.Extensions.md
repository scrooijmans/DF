# Struct Extensions Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#173" class="src">Source</a>

``` rust
pub struct Extensions(/* private fields */);
```

Expand description

Container for storing type-safe extensions used to configure underlying FileIO behavior.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#impl-Extensions" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.add" class="fn">add</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>\>(&mut self, ext: T)

Add an extension.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.extend" class="fn">extend</a>(&mut self, extensions: <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>)

Extends the current set of extensions with another set of extensions.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.get" class="fn">get</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>\>

where T: 'static + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Fetch an extension.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#impl-Clone-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#impl-Debug-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#impl-Default-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html" class="struct" title="struct iceberg::io::Extensions">Extensions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.Extensions.html#blanket-implementations" class="anchor">§</a>
