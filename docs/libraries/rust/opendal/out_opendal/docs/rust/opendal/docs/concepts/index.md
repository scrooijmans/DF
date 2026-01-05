# Module concepts Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/concepts.rs.html#18-135" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

The core concepts of OpenDALâ€™s public API.

OpenDAL provides a unified abstraction that helps developers access all storage services.

There are two core concepts in OpenDAL:

- [`Builder`](https://opendal.apache.org/docs/rust/opendal/trait.Builder.html "trait opendal::Builder"): Builder accepts a series of parameters to set up an instance of underlying services. You can adjust the behaviour of underlying services with these parameters.
- [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator"): Developer can access underlying storage services with manipulating one Operator. The Operator is a delegate for underlying implementation detail, and provides one unified access interface, including `read`, `write`, `list` and so on.

If you are interested in internal implementation details, please have a look at [`internals`](https://opendal.apache.org/docs/rust/opendal/docs/internals/index.html "mod opendal::docs::internals").

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/concepts/index.html#builder" class="doc-anchor">Â§</a>Builder

Letâ€™s start with [`Builder`](https://opendal.apache.org/docs/rust/opendal/trait.Builder.html "trait opendal::Builder").

A `Builder` is a trait that is implemented by the underlying services. We can use a `Builder` to configure and create a service. Developer can only create one service via Builder, in other words, Builder is the only public API provided by services. And other detailed implementation will be hidden.

``` text
âââââââââââââ                 âââââââââââââ
â           â     build()     â           â
â  Builder  ââââââââââââââââââºâ  Service  â
â           â                 â           â
âââââââââââââ                 âââââââââââââ
```

All [`Builder`](https://opendal.apache.org/docs/rust/opendal/trait.Builder.html "trait opendal::Builder") provided by OpenDAL is under [`services`](https://opendal.apache.org/docs/rust/opendal/services/index.html "mod opendal::services"), we can refer to them like `opendal::services::S3`. By right the builder will be named like `OneServiceBuilder`, but usually we will export it to public with renaming it as one general name. For example, we will rename `S3Builder` to `S3` and developer will use `S3` finally.

For example:

``` rust
use opendal::services::S3;

let mut builder = S3::default();
builder.bucket("example");
builder.root("/path/to/file");
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/concepts/index.html#operator" class="doc-anchor">Â§</a>Operator

The [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") is a delegate for Service, the underlying implementation detail that implements [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"), and it also provides one unified access interface. It will hold one reference of Service with its all generic types erased by OpenDAL, which is the reason why we say the Operator is the delegate of one Service.

``` text
                  ââââââââââââââââââââââ
                  â      Operator      â
                  â         âdelegate  â
âââââââââââ build â         â¼          â rely on âââââââââââââââââââââââ
â Builder âââââââââ¼âââºââââââââââââââ   âââââââââââ¤ business logic code â
âââââââââââ       â   â  Service   â   â         âââââââââââââââââââââââ
                  âââââ´âââââââââââââ´ââââ
```

`Operator` can be built from `Builder`:

``` rust
use opendal::services::S3;
use opendal::Operator;

let mut builder = S3::default();
builder.bucket("example");
builder.root("/path/to/file");

let op = Operator::new(builder)?.finish();
```

- `Operator` has itâ€™s internal `Arc`, so itâ€™s **cheap** to clone it.
- `Operator` doesnâ€™t have generic parameters or lifetimes, so itâ€™s **easy** to use it everywhere.
- `Operator` implements `Send` and `Sync`, so itâ€™s **safe** to send it between threads.

After get an `Operator`, we can do operations on different paths.

``` text
                           ââââââââââââââââ
                 ââââââââââºâ read("abc")  â
                 â         ââââââââââââââââ
âââââââââââââ    â
â Operator  â    â         ââââââââââââââââ
â âââââââââ ââââââ¼âââââââââºâ write("def") â
â âServiceâ â    â         ââââââââââââââââ
âââ´ââââââââ´ââ    â
                 â         ââââââââââââââââ
                 ââââââââââºâ list("ghi/") â
                           ââââââââââââââââ
```

We can read data with given path in this way:

``` rust
use opendal::services::S3;
use opendal::Operator;

let mut builder = S3::default();
builder.bucket("example");
builder.root("/path/to/file");

let op = Operator::new(builder)?.finish();
let bs: Vec<u8> = op.read("abc").await?;
```
