# Struct PaginatedListOptions Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/list.rs.html#27-52" class="src">Source</a>

``` rust
pub struct PaginatedListOptions {
    pub offset: Option<String>,
    pub delimiter: Option<Cow<'static, str>>,
    pub max_keys: Option<usize>,
    pub page_token: Option<String>,
    pub extensions: Extensions,
}
```

Expand description

Options for a paginated list request

## Fields<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#structfield.offset" class="anchor field">§</a>`offset: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Path to start listing from

Note: Not all stores support this

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#structfield.delimiter" class="anchor field">§</a>`delimiter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow"><code>Cow</code></a>`<'static, `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>>`

A delimiter use to group keys with a common prefix

Note: Some stores only support `/`

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#structfield.max_keys" class="anchor field">§</a>`max_keys: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

The maximum number of paths to return

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#structfield.page_token" class="anchor field">§</a>`page_token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

A page token from a previous request

Note: Behaviour is implementation defined if the previous request used a different prefix or options

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#structfield.extensions" class="anchor field">§</a>`extensions: `<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions"><code>Extensions</code></a>

Implementation-specific extensions. Intended for use by implementations that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#impl-Clone-for-PaginatedListOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html" class="struct" title="struct object_store::list::PaginatedListOptions">PaginatedListOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html" class="struct" title="struct object_store::list::PaginatedListOptions">PaginatedListOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#impl-Debug-for-PaginatedListOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html" class="struct" title="struct object_store::list::PaginatedListOptions">PaginatedListOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#impl-Default-for-PaginatedListOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html" class="struct" title="struct object_store::list::PaginatedListOptions">PaginatedListOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html" class="struct" title="struct object_store::list::PaginatedListOptions">PaginatedListOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListOptions.html#blanket-implementations" class="anchor">§</a>
