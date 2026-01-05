# Module internals Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/internals/mod.rs.html#18-62" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

The internal implement details of OpenDAL.

OpenDAL has provides unified abstraction via two-level API sets:

- Public API like [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") provides user level API.
- Raw API like [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"), [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer") provides developer level API.

OpenDAL tries itâ€™s best to keep the public API stable. But raw APIs may change between minor releases from time to time. So most users should only use the public API. And only developers need to implement with raw API to implement a new service [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") or their own [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer").

In this section, we will talk about the following components:

- [`Access`](https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html "mod opendal::docs::internals::accessor"): to connect underlying storage services.
- [`Layer`](https://opendal.apache.org/docs/rust/opendal/docs/internals/layer/index.html "mod opendal::docs::internals::layer"): middleware/interceptor between storage services.

The relation between [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"), [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer") and [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") looks like the following:

``` text
âââââââââââââââââââââââââââââââââââââââââââââââââââ¬âââââââââââ
â                                                 â          â
â              ââââââââââââ  ââââââââââ           â          â
â              â          â  â        â¼           â          â
â      s3âââ   â          â  â Tracing Layer      â          â
â          â   â          â  â        â           â          â
â     gcsâââ¤   â          â  â        â¼           â          â
â          ââââºâ Accessor ââââ Metrics Layer âââââºâ Operator â
â  azblobâââ¤   â          â           â      â    â          â
â          â   â          â           â¼      â    â          â
â    hdfsâââ   â          â    Logging Layer â    â          â
â              â          â           â      â    â          â
â              ââââââââââââ           ââââââââ    â          â
â                                                 â          â
âââââââââââââââââââââââââââââââââââââââââââââââââââ´âââââââââââ
```

## Modules<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/index.html#modules" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html" class="mod" title="mod opendal::docs::internals::accessor">accessor</a>  
The internal implementation details of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access").

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/layer/index.html" class="mod" title="mod opendal::docs::internals::layer">layer</a>  
The internal implementation details of [`Layer`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html "trait opendal::raw::Layer").
