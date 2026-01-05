# Struct DtraceLayer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/dtrace.rs.html#126" class="src">Source</a>

``` rust
pub struct DtraceLayer {}
```

Available on **Linux and crate feature `layers-dtrace`** only.

Expand description

Support User Statically-Defined Tracing(aka USDT) on Linux

This layer is an experimental feature, it will be enabled by `features = ["layers-dtrace"]` in Cargo.toml.

For now we have following probes:

#### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#for-accessor" class="doc-anchor">Â§</a>For Accessor

1.  \${operation}\_start, arguments: path

    1.  create_dir
    2.  read
    3.  write
    4.  stat
    5.  delete
    6.  list
    7.  presign

2.  \${operation}\_end, arguments: path

    1.  create_dir
    2.  read
    3.  write
    4.  stat
    5.  delete
    6.  list
    7.  presign

#### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#for-reader" class="doc-anchor">Â§</a>For Reader

1.  reader_read_start, arguments: path
2.  reader_read_ok, arguments: path, length
3.  reader_read_error, arguments: path

#### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#for-writer" class="doc-anchor">Â§</a>For Writer

1.  writer_write_start, arguments: path
2.  writer_write_ok, arguments: path, length
3.  writer_write_error, arguments: path
4.  writer_abort_start, arguments: path
5.  writer_abort_ok, arguments: path
6.  writer_abort_error, arguments: path
7.  writer_close_start, arguments: path
8.  writer_close_ok, arguments: path
9.  writer_close_error, arguments: path

Example:

``` rust

// `Accessor` provides the low level APIs, we will use `Operator` normally.
let op: Operator = Operator::new(services::Fs::default().root("/tmp"))?
    .layer(DtraceLayer::default())
    .finish();

let path = "/tmp/test.txt";
for _ in 1..100000 {
    let bs = vec![0; 64 * 1024 * 1024];
    op.write(path, bs).await?;
    op.read(path).await?;
}
Ok(())
```

Then you can use `readelf -n target/debug/examples/dtrace` to see the probes:

``` text
Displaying notes found in: .note.stapsdt
  Owner                Data size        Description
  stapsdt              0x00000039       NT_STAPSDT (SystemTap probe descriptors)
    Provider: opendal
    Name: create_dir_start
    Location: 0x00000000000f8f05, Base: 0x0000000000000000, Semaphore: 0x00000000003649f8
    Arguments: -8@%rax
  stapsdt              0x00000037       NT_STAPSDT (SystemTap probe descriptors)
    Provider: opendal
    Name: create_dir_end
    Location: 0x00000000000f9284, Base: 0x0000000000000000, Semaphore: 0x00000000003649fa
    Arguments: -8@%rax
  stapsdt              0x0000003c       NT_STAPSDT (SystemTap probe descriptors)
    Provider: opendal
    Name: blocking_list_start
    Location: 0x00000000000f9487, Base: 0x0000000000000000, Semaphore: 0x0000000000364a28
    Arguments: -8@%rax
  stapsdt              0x0000003a       NT_STAPSDT (SystemTap probe descriptors)
    Provider: opendal
    Name: blocking_list_end
    Location: 0x00000000000f9546, Base: 0x0000000000000000, Semaphore: 0x0000000000364a2a
    Arguments: -8@%rax
  stapsdt              0x0000003c       NT_STAPSDT (SystemTap probe descriptors)
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#impl-Clone-for-DtraceLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#impl-Debug-for-DtraceLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#impl-Default-for-DtraceLayer" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#impl-Layer%3CA%3E-for-DtraceLayer" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html" class="struct" title="struct opendal::layers::DtraceLayer">DtraceLayer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#associatedtype.LayeredAccess" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype">LayeredAccess</a> = DTraceAccessor\<A\>

The layered accessor that returned by this layer.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#method.layer" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#tymethod.layer" class="fn">layer</a>(&self, inner: A) -\> Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>

Intercept the operations on the underlying storage.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.DtraceLayer.html#blanket-implementations" class="anchor">Â§</a>
