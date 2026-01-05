# Struct PaginatedListResult Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/list.rs.html#56-61" class="src">Source</a>

``` rust
pub struct PaginatedListResult {
    pub result: ListResult,
    pub page_token: Option<String>,
}
```

Expand description

A [`ListResult`](https://docs.rs/object_store/latest/object_store/struct.ListResult.html "struct object_store::ListResult") with optional pagination token

## Fields<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#structfield.result" class="anchor field">§</a>`result: `<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html" class="struct" title="struct object_store::ListResult"><code>ListResult</code></a>

The list result

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#structfield.page_token" class="anchor field">§</a>`page_token: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

If result set truncated, the pagination token to fetch next results

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#impl-Debug-for-PaginatedListResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html" class="struct" title="struct object_store::list::PaginatedListResult">PaginatedListResult</a>

<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/list/struct.PaginatedListResult.html#blanket-implementations" class="anchor">§</a>
