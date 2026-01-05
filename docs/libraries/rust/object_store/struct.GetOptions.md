# Struct GetOptions Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#939-994" class="src">Source</a>

``` rust
pub struct GetOptions {
    pub if_match: Option<String>,
    pub if_none_match: Option<String>,
    pub if_modified_since: Option<DateTime<Utc>>,
    pub if_unmodified_since: Option<DateTime<Utc>>,
    pub range: Option<GetRange>,
    pub version: Option<String>,
    pub head: bool,
    pub extensions: Extensions,
}
```

Expand description

Options for a get request, such as range

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.if_match" class="anchor field">§</a>`if_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Request will succeed if the `ObjectMeta::e_tag` matches otherwise returning [`Error::Precondition`](https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Precondition "variant object_store::Error::Precondition")

See <https://datatracker.ietf.org/doc/html/rfc9110#name-if-match>

Examples:

``` text
If-Match: "xyzzy"
If-Match: "xyzzy", "r2d2xxxx", "c3piozzzz"
If-Match: *
```

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.if_none_match" class="anchor field">§</a>`if_none_match: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Request will succeed if the `ObjectMeta::e_tag` does not match otherwise returning [`Error::NotModified`](https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotModified "variant object_store::Error::NotModified")

See <https://datatracker.ietf.org/doc/html/rfc9110#section-13.1.2>

Examples:

``` text
If-None-Match: "xyzzy"
If-None-Match: "xyzzy", "r2d2xxxx", "c3piozzzz"
If-None-Match: *
```

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.if_modified_since" class="anchor field">§</a>`if_modified_since: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime"><code>DateTime</code></a>`<`<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc"><code>Utc</code></a>`>>`

Request will succeed if the object has been modified since

<https://datatracker.ietf.org/doc/html/rfc9110#section-13.1.3>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.if_unmodified_since" class="anchor field">§</a>`if_unmodified_since: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime"><code>DateTime</code></a>`<`<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc"><code>Utc</code></a>`>>`

Request will succeed if the object has not been modified since otherwise returning [`Error::Precondition`](https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.Precondition "variant object_store::Error::Precondition")

Some stores, such as S3, will only return `NotModified` for exact timestamp matches, instead of for any timestamp greater than or equal.

<https://datatracker.ietf.org/doc/html/rfc9110#section-13.1.4>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.range" class="anchor field">§</a>`range: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange"><code>GetRange</code></a>`>`

Request transfer of only the specified range of bytes otherwise returning [`Error::NotModified`](https://docs.rs/object_store/latest/object_store/enum.Error.html#variant.NotModified "variant object_store::Error::NotModified")

<https://datatracker.ietf.org/doc/html/rfc9110#name-range>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.version" class="anchor field">§</a>`version: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Request a particular object version

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.head" class="anchor field">§</a>`head: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Request transfer of no content

<https://datatracker.ietf.org/doc/html/rfc9110#name-head>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#structfield.extensions" class="anchor field">§</a>`extensions: `<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions"><code>Extensions</code></a>

Implementation-specific extensions. Intended for use by [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementations that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#impl-GetOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#method.check_preconditions" class="fn">check_preconditions</a>(&self, meta: &<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Returns an error if the modification conditions on this request are not satisfied

<https://datatracker.ietf.org/doc/html/rfc7232#section-6>

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#impl-Clone-for-GetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#impl-Debug-for-GetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#impl-Default-for-GetOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html#blanket-implementations" class="anchor">§</a>
