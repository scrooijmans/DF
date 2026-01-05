# Struct D1 Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/d1/backend.rs.html#29-33" class="src">Source</a>

``` rust
pub struct D1 { /* private fields */ }
```

Expand description

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☐ create_dir
- ☒ stat
- ☒ read
- ☒ write
- ☒ delete
- ☐ copy
- ☐ rename
- ☐ list
- ☐ ~~presign~~

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the working directory of `OpenDAL`
- `token`: Set the token of cloudflare api
- `account_id`: Set the account id of cloudflare api
- `database_id`: Set the database id of cloudflare api
- `table`: Set the table of D1 Database
- `key_field`: Set the key field of D1 Database
- `value_field`: Set the value field of D1 Database

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::D1;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = D1::default()
        .token("token")
        .account_id("account_id")
        .database_id("database_id")
        .table("table")
        .key_field("key_field")
        .value_field("value_field");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#impl-D1Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1Builder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

Set api token for the cloudflare d1 service.

create a api token from [here](https://dash.cloudflare.com/profile/api-tokens)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.account_id" class="fn">account_id</a>(self, account_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

Set the account identifier for the cloudflare d1 service.

get the account identifier from Workers & Pages -\> Overview -\> Account ID If not specified, it will return an error when building.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.database_id" class="fn">database_id</a>(self, database_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

Set the database identifier for the cloudflare d1 service.

get the database identifier from Workers & Pages -\> D1 -\> \[Your Database\] -\> Database ID If not specified, it will return an error when building.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

set the working directory, all operations will be performed under it.

default: â€œ/â€?

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.table" class="fn">table</a>(self, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

Set the table name of the d1 service to read/write.

If not specified, it will return an error when building.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.key_field" class="fn">key_field</a>(self, key_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

Set the key field name of the d1 service to read/write.

Default to `key` if not specified.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.value_field" class="fn">value_field</a>(self, value_field: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-d1`** only.

Set the value field name of the d1 service to read/write.

Default to `value` if not specified.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#impl-Builder-for-D1Builder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1Builder</a>

Available on **crate feature `services-d1`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1Config.html" class="struct" title="struct opendal::services::D1Config">D1Config</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#impl-Default-for-D1Builder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1Builder</a>

Available on **crate feature `services-d1`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html" class="struct" title="struct opendal::services::D1">D1Builder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.D1.html#blanket-implementations" class="anchor">Â§</a>
