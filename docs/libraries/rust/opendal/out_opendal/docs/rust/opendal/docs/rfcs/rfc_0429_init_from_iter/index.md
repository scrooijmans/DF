# Module rfc_0429_init_from_iter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#86" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Init from iter

- Proposal Name: `init_from_iter`
- Start Date: 2022-07-10
- RFC PR: [apache/opendal#429](https://github.com/apache/opendal/pull/429)
- Tracking Issue: [apache/opendal#430](https://github.com/apache/opendal/issues/430)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#summary" class="doc-anchor">Â§</a>Summary

Allow initializing opendal operators from an iterator.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#motivation" class="doc-anchor">Â§</a>Motivation

To init OpenDAL operators, users have to init an accessor first.

``` rust
let root = &env::var("OPENDAL_S3_ROOT").unwrap_or_else(|_| "/".to_string());
let root = format!("/{}/{}", root, uuid::Uuid::new_v4());

let mut builder = opendal::services::s3::Backend::build();
builder.root(&root);
builder.bucket(&env::var("OPENDAL_S3_BUCKET").expect("OPENDAL_S3_BUCKET must set"));
builder.endpoint(&env::var("OPENDAL_S3_ENDPOINT").unwrap_or_default());
builder.access_key_id(&env::var("OPENDAL_S3_ACCESS_KEY_ID").unwrap_or_default());
builder.secret_access_key(&env::var("OPENDAL_S3_SECRET_ACCESS_KEY").unwrap_or_default());
builder
    .server_side_encryption(&env::var("OPENDAL_S3_SERVER_SIDE_ENCRYPTION").unwrap_or_default());
builder.server_side_encryption_customer_algorithm(
    &env::var("OPENDAL_S3_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM").unwrap_or_default(),
);
builder.server_side_encryption_customer_key(
    &env::var("OPENDAL_S3_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY").unwrap_or_default(),
);
builder.server_side_encryption_customer_key_md5(
    &env::var("OPENDAL_S3_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5").unwrap_or_default(),
);
builder.server_side_encryption_aws_kms_key_id(
    &env::var("OPENDAL_S3_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID").unwrap_or_default(),
);
if env::var("OPENDAL_S3_ENABLE_VIRTUAL_HOST_STYLE").unwrap_or_default() == "on" {
    builder.enable_virtual_host_style();
}
Ok(Some(builder.finish().await?))
```

We can simplify this logic if opendal has its native `from_iter` support.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

Users can init an operator like the following:

``` rust
// OPENDAL_S3_BUCKET = <bucket>
// OPENDAL_S3_ENDPOINT = <endpoint>
let op = Operator::from_env(Scheme::S3)?;
```

Or from a prefixed env:

``` rust
// OIL_PROFILE_<name>_S3_BUCKET = <bucket>
// OIL_PROFILE_<name>_S3_ENDPOINT = <endpoint>
let op = Operator::from_env(Scheme::S3, "OIL_PROFILE_<name>")?;
```

Also, we call the underlying function directly:

``` rust
// var it: impl Iterator<Item=(String, String)>
let op = Operator::from_iter(Scheme::S3, it)?;
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

Internally, every serviceâ€™s backend will implement the following functions:

``` rust
fn from_iter(it: impl Iterator<Item=(String, String)>) -> Backend {}
```

Note: itâ€™s not a public API of `Accessor`, and it will never be. Instead, we will use this function inside the crate to keep the ability to refactor or even remove it.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html#connection-string" class="doc-anchor">Â§</a>Connection string

It sounds a good idea to implement something like:

``` rust
let op = Operator::open("s3://bucket?region=test")?
```

But there are no valid use cases. Letâ€™s implement this in the future if needed.
