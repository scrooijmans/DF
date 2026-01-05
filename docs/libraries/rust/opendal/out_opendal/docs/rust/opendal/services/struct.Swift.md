# Struct Swift Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/swift/backend.rs.html#41-43" class="src">Source</a>

``` rust
pub struct Swift { /* private fields */ }
```

Expand description

[OpenStack Swift](https://docs.openstack.org/api-ref/object-store/#)â€™s REST API support. For more information about swift-compatible services, refer to [Compatible Services](https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#compatible-services).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☐ ~~rename~~
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#configurations" class="doc-anchor">Â§</a>Configurations

- `endpoint`: Set the endpoint for backend.
- `container`: Swift container.
- `token`: Swift personal access token.

Refer to [`SwiftBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html "struct opendal::services::Swift")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#examples" class="doc-anchor">Â§</a>Examples

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Swift;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create Swift backend builder
    let mut builder = Swift::default() 
        // Set the root for swift, all operations will happen under this root
        .root("/path/to/dir")
        // set the endpoint of Swift backend
        .endpoint("https://openstack-controller.example.com:8080/v1/account")
        // set the container name of Swift workspace
        .container("container")
        // set the auth token for builder
        .token("token");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#compatible-services" class="doc-anchor">Â§</a>Compatible Services

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#openstack-swift" class="doc-anchor">Â§</a>OpenStack Swift

[OpenStack Swift](https://docs.openstack.org/swift/latest/) is the default implementations of swift services.

To connect to OpenStack Swift, we need to set:

- `endpoint`: The endpoint of OpenStack Swift, for example: `http://127.0.0.1:8080/v1/AUTH_test`.
- `container`: The name of OpenStack Swift container.
- `token`: OpenStack Swift container personal access token.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
builder.endpoint("http://127.0.0.1:8080/v1/AUTH_test");
builder.container("container");
builder.token("token");
```

`endpoint` is the full URL that serves as the access point to all containers under an OpenStack Swift account. It represents the entry point for accessing the resources of the account. Alongside `endpoint`, `token` is used as a credential to verify the userâ€™s identity and authorize access to the relevant resources. Both `endpoint` and `token` can be obtained through OpenStack Swift authentication service.

`endpoint` consists of server address and port, API version, authenticated account ID. For instance, it might appear as follows:

- `http://127.0.0.1:8080/v1/AUTH_test`.
- `http://192.168.66.88:8080/swift/v1`.
- `https://openstack-controller.example.com:8080/v1/account`.

Please note that the exact format of `endpoint` may vary depending on the deployment configuration and version of swift services. Users can refer to the specific services documentation for the correct `endpoint` format and authentication method.

For more information, refer:

- [OpenStack Swift API](https://docs.openstack.org/api-ref/object-store/).
- [OpenStack Swift Authentication](https://docs.openstack.org/swift/latest/api/object_api_v1_overview.html).

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#ceph-rados-gateway" class="doc-anchor">Â§</a>Ceph Rados Gateway

[Ceph Rados Gateway](https://docs.ceph.com/en/quincy/radosgw/) supports a RESTful API that is compatible with the basic data access model of OpenStack Swift API.

To connect to Ceph Rados Gateway, we need to set:

- `endpoint`: The endpoint of swift services, for example: `http://127.0.0.1:8080/swift/v1`.
- `container`: The name of swift container.
- `token`: swift container personal access token.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#" class="tooltip" title="This example is not tested">â“˜</a>

``` rust
builder.endpoint("http://127.0.0.1:8080/swift/v1");
builder.container("container");
builder.token("token");
```

For more information, refer:

- [Ceph Rados Gateway Swift API](https://docs.ceph.com/en/latest/radosgw/swift/#api).
- [Ceph Rados Gateway Swift Authentication](https://docs.ceph.com/en/latest/radosgw/swift/auth/).

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#impl-SwiftBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-swift`** only.

Set the remote address of this backend

Endpoints should be full uri, e.g.

- `http://127.0.0.1:8080/v1/AUTH_test`
- `http://192.168.66.88:8080/swift/v1`
- `https://openstack-controller.example.com:8080/v1/ccount`

If user inputs endpoint without scheme, we will prepend `https://` to it.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.container" class="fn">container</a>(self, container: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-swift`** only.

Set container of this backend.

All operations will happen under this container. It is required. e.g. `snapshots`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-swift`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-swift`** only.

Set the token of this backend.

Default to empty string.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#impl-Builder-for-SwiftBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Available on **crate feature `services-swift`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Build a SwiftBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SwiftConfig.html" class="struct" title="struct opendal::services::SwiftConfig">SwiftConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#impl-Clone-for-SwiftBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Available on **crate feature `services-swift`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#impl-Debug-for-SwiftBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Available on **crate feature `services-swift`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#impl-Default-for-SwiftBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Available on **crate feature `services-swift`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html" class="struct" title="struct opendal::services::Swift">SwiftBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Swift.html#blanket-implementations" class="anchor">Â§</a>
