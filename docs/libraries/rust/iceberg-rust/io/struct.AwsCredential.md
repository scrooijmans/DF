# Struct AwsCredential Copy item path

<a href="https://docs.rs/reqsign/0.16.5/x86_64-unknown-linux-gnu/src/reqsign/aws/credential.rs.html#29" class="src">Source</a>

``` rust
pub struct AwsCredential {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub expires_in: Option<DateTime<Utc>>,
}
```

Expand description

Credential that holds the `access_key` and `secret_key`.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#structfield.access_key_id" class="anchor field">§</a>`access_key_id: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Access key id for aws services.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#structfield.secret_access_key" class="anchor field">§</a>`secret_access_key: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Secret access key for aws services.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#structfield.session_token" class="anchor field">§</a>`session_token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Session token for aws services.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#structfield.expires_in" class="anchor field">§</a>`expires_in: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime"><code>DateTime</code></a>`<`<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc"><code>Utc</code></a>`>>`

Expiration time for this credential.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#impl-AwsCredential" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">Credential</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#method.is_valid" class="fn">is_valid</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

is current cred is valid?

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#panics" class="doc-anchor">§</a>Panics

Panics if the time delta calculation overflows (which should not happen in practice).

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#impl-Clone-for-AwsCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">Credential</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">Credential</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#impl-Default-for-AwsCredential" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">Credential</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html" class="struct" title="struct iceberg::io::AwsCredential">Credential</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.AwsCredential.html#blanket-implementations" class="anchor">§</a>
