# Module accessor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/internals/accessor.rs.html#18-306" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

The internal implementation details of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access").

[`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") is the core trait of OpenDALâ€™s raw API. We operate underlying storage services via APIs provided by [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access").

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#introduction" class="doc-anchor">Â§</a>Introduction

[`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") can be split in the following parts:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
//                  <----------Trait Bound-------------->
pub trait Access: Send + Sync + Debug + Unpin + 'static {
    type Reader: oio::Read;                    // --+
    type Writer: oio::Write;                   //   +--> Associated Type
    type Lister: oio::List;                    //   +
    type Deleter: oio::Delete;                 // --+

    // APIs
    fn info(&self) -> Arc<AccessorInfo>;
    fn create_dir(
        &self,
        path: &str,
        args: OpCreateDir,
    ) -> impl core::future::Future<Output = Result<RpCreateDir>> + MaybeSend;
}
```

Letâ€™s go deep into [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") line by line.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#trait-bound" class="doc-anchor">Â§</a>Trait Bound

First we will read the declare of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") trait:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
pub trait Access: Send + Sync + Debug + Unpin + 'static {}
```

There are many trait boundings here. For now, [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") requires the following bound:

- [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"): Allow user to send between threads without extra wrapper.
- [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"): Allow user to sync between threads without extra lock.
- [`Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"): Allow users to print underlying debug information of accessor.
- [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"): Make sure `Access` can be safely moved after being pinned, so users donâ€™t need to `Pin<Box<A>>`.
- `'static`: Make sure `Access` is not a short-time reference, allow users to use `Access` in closures and futures without playing with lifetime.

Implementer of `Access` should take care of the following things:

- Implement `Debug` for backend, but donâ€™t leak credentials.
- Make sure the backend is `Send` and `Sync`, wrap the internal struct with `Arc<Mutex<T>>` if necessary.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#associated-type" class="doc-anchor">Â§</a>Associated Type

The first block of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") trait is our associated types. We require implementers to specify the type to be returned, thus avoiding the additional overhead of dynamic dispatch.

[`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") has four associated types so far:

- `Reader`: reader returned by `read` operation.
- `Writer`: writer returned by `write` operation.
- `Lister`: lister returned by `list` operation.
- `Deleter`: deleter returned by `delete` operation.

Implementer of `Access` should take care the following things:

- OpenDAL will erase those type at the final stage of Operator building. Please donâ€™t return dynamic trait object like `oio::Reader`.
- Use `()` as type if the operation is not supported.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#api-style" class="doc-anchor">Â§</a>API Style

Every API of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") follows the same style:

- All APIs have a unique [`Operation`](https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html "enum opendal::raw::Operation") and [`Capability`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html "struct opendal::Capability")
- All APIs are orthogonal and do not overlap with each other
- Most APIs accept `path` and `OpXxx`, and returns `RpXxx`.
- Most APIs have `async` and `blocking` variants, they share the same semantics but may have different underlying implementations.

[`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") can declare their capabilities via [`AccessorInfo`](https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html "struct opendal::raw::AccessorInfo")â€™s `set_native_capability`:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
impl Access for MyBackend {
    fn info(&self) -> Arc<AccessorInfo> {
        let am = AccessorInfo::default();
        am.set_native_capability(
            Capability {
                read: true,
                write: true,
                ..Default::default()
        });

        am.into()
    }
}
```

Now that you have mastered [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"), letâ€™s go and implement our own backend!

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#tutorial" class="doc-anchor">Â§</a>Tutorial

This tutorial implements a `duck` storage service that sends API requests to a super-powered duck. Gagaga!

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#scheme" class="doc-anchor">Â§</a>Scheme

First of all, letâ€™s pick a good [`Scheme`](https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html "enum opendal::Scheme") for our duck service. The scheme should be unique and easy to understand. Normally we should use its formal name.

For example, we will use `s3` for AWS S3 Compatible Storage Service instead of `aws` or `awss3`. This is because there are many storage vendors that provide s3-like RESTful APIs, and our s3 service is implemented to support all of them, not just AWS S3.

Obviously, we can use `duck` as scheme, letâ€™s add a new variant in [`Scheme`](https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html "enum opendal::Scheme"), and implement all required functions like `Scheme::from_str` and `Scheme::into_static`:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
pub enum Scheme {
    Duck,
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#builder" class="doc-anchor">Â§</a>Builder

Then we can implement a builder for the duck service. The [`Builder`](https://opendal.apache.org/docs/rust/opendal/trait.Builder.html "trait opendal::Builder") will provide APIs for users to configure, and they will create an instance of a particular service.

Letâ€™s create a `backend` mod under `services/duck` directory, and adding the following code.

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
use crate::raw::*;
use crate::*;

/// Duck Storage Service support. Gagaga!
///
/// # Capabilities
///
/// This service can be used to:
///
/// - [x] read
/// - [ ] write
/// - [ ] list
/// - [ ] presign
///
/// # Configuration
///
/// - `root`: Set the work dir for backend.
///
/// ## Via Builder
///
/// ```no_run
/// use std::sync::Arc;
///
/// use anyhow::Result;
/// use opendal::services::Duck;
/// use opendal::Operator;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // Create Duck backend builder.
///     let mut builder = DuckBuilder::default();
///     // Set the root for duck, all operations will happen under this root.
///     //
///     // NOTE: the root must be absolute path.
///     builder.root("/path/to/dir");
///
///     let op: Operator = Operator::new(builder)?.finish();
///
///     Ok(())
/// }
/// ```
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(default)]
#[non_exhaustive]
pub struct DuckConfig {
    pub root: Option<String>,
}

#[derive(Default, Clone)]
pub struct DuckBuilder {
    config: DuckConfig,
}
```

Now letâ€™s implement the required APIs for `DuckConfig`:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
impl Configurator for DuckConfig {
    type Builder = DuckBuilder;

    fn into_builder(self) -> Self::Builder {
        DuckBuilder { config: self }
    }
}
```

Note that `DuckBuilder` is part of our public API, so it needs to be documented. And any changes you make will directly affect users, so please take it seriously. Otherwise, you will be hunted down by many angry ducks.

Then, we can implement required APIs for `DuckBuilder`:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
impl DuckBuilder {
    /// Set root of this backend.
    ///
    /// All operations will happen under this root.
    pub fn root(&mut self, root: &str) -> &mut Self {
        self.config.root = if root.is_empty() {
            None
        } else {
            Some(root.to_string())
        };

        self
    }
}

impl Builder for DuckBuilder {
    type Config = DuckConfig;

    fn build(self) -> Result<impl Access>  {
        debug!("backend build started: {:?}", &self);

        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
        debug!("backend use root {}", &root);

        Ok(DuckBackend { root })
    }
}
```

`DuckBuilder` is ready now, letâ€™s try to play with real ducks!

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#backend" class="doc-anchor">Â§</a>Backend

Iâ€™m sure you can see it already: `DuckBuilder` will build a `DuckBackend` that implements [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access"). The backend is what we used to communicate with the super-powered ducks!

Letâ€™s keep adding more code under `backend.rs`:

<a href="https://opendal.apache.org/docs/rust/opendal/docs/internals/accessor/index.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
/// Duck storage service backend
#[derive(Clone, Debug)]
pub struct DuckBackend {
    root: String,
}

impl Access for DuckBackend {
    type Reader = DuckReader;
    type Writer = ();
    type Lister = ();
    type Deleter = ();

    fn info(&self) -> Arc<AccessorInfo> {
        let am = AccessorInfo::default();
        am.set_scheme("duck")
            .set_root(&self.root)
            .set_native_capability(
                Capability {
                    read: true,
                    ..Default::default()
            });

        am.into()
    }

    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
        gagaga!()
    }
}
```

Congratulations, we have implemented an [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") that can talk to Super Power Ducks!

What!? There are no Super Power Ducks? So sad, but never mind, we have really powerful storage services [here](https://github.com/apache/opendal/issues/5). Welcome to pick one to implement. I promise you wonâ€™t have to `gagaga!()` this time.
