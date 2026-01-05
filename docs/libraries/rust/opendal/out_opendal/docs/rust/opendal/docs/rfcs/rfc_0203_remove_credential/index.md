# Module rfc_0203_remove_credential Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#54" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Remove credential

- Proposal Name: `remove_credential`
- Start Date: 2022-04-02
- RFC PR: [apache/opendal#203](https://github.com/apache/opendal/pull/203)
- Tracking Issue: [apache/opendal#203](https://github.com/apache/opendal/issues/203)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#summary" class="doc-anchor">Â§</a>Summary

Remove the concept of credential.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#motivation" class="doc-anchor">Â§</a>Motivation

`Credential` intends to carry service credentials like `access_key_id` and `secret_access_key`. At OpenDAL, we designed a global `Credential` enum for services and users to use.

``` rust
pub enum Credential {
    /// Plain refers to no credential has been provided, fallback to services'
    /// default logic.
    Plain,
    /// Basic refers to HTTP Basic Authentication.
    Basic { username: String, password: String },
    /// HMAC, also known as Access Key/Secret Key authentication.
    HMAC {
        access_key_id: String,
        secret_access_key: String,
    },
    /// Token refers to static API token.
    Token(String),
}
```

However, every service only supports one kind of `Credential` with different `Credential` load methods covered by [reqsign](https://github.com/Xuanwo/reqsign). As a result, only `HMAC` is used. Both users and services need to write the same logic again and again.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

`Credential` will be removed, and the services builder will provide native credential representation directly.

For s3:

``` rust
pub fn access_key_id(&mut self, v: &str) -> &mut Self {}
pub fn secret_access_key(&mut self, v: &str) -> &mut Self {}
```

For azblob:

``` rust
pub fn account_name(&mut self, account_name: &str) -> &mut Self {}
pub fn account_key(&mut self, account_key: &str) -> &mut Self {}
```

All builders must implement `Debug` by hand and redact sensitive fields to avoid credentials being a leak.

``` rust
impl Debug for Builder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("Builder");

        ds.field("root", &self.root);
        ds.field("container", &self.container);
        ds.field("endpoint", &self.endpoint);

        if self.account_name.is_some() {
            ds.field("account_name", &"<redacted>");
        }
        if self.account_key.is_some() {
            ds.field("account_key", &"<redacted>");
        }

        ds.finish_non_exhaustive()
    }
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

Simple change without reference-level explanation needs.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

API Breakage.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
