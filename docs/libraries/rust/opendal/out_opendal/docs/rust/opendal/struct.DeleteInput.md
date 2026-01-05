# Struct DeleteInput Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/delete/input.rs.html#24-29" class="src">Source</a>

``` rust
#[non_exhaustive]pub struct DeleteInput {
    pub path: String,
    pub version: Option<String>,
}
```

Expand description

DeleteInput is the input for delete operations.

## Fields (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#fields" class="anchor">Â§</a>

This struct is marked as non-exhaustive

Non-exhaustive structs could have additional fields added in future. Therefore, non-exhaustive structs cannot be constructed in external crates using the traditional `Struct { .. }` syntax; cannot be matched against without a wildcard `..`; and struct update syntax will not work.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#structfield.path" class="anchor field">Â§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The path of the path to delete.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#structfield.version" class="anchor field">Â§</a>`version: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The version of the path to delete.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#impl-Debug-for-DeleteInput" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#impl-Default-for-DeleteInput" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#impl-IntoDeleteInput-for-DeleteInput" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html" class="trait" title="trait opendal::IntoDeleteInput">IntoDeleteInput</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

Implement `IntoDeleteInput` for `DeleteInput` self.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#method.into_delete_input" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoDeleteInput.html#tymethod.into_delete_input" class="fn">into_delete_input</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html" class="struct" title="struct opendal::DeleteInput">DeleteInput</a>

Convert `self` into a `DeleteInput`.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.DeleteInput.html#blanket-implementations" class="anchor">Â§</a>
