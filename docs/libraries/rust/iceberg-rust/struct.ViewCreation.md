# Struct ViewCreation Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#809-830" class="src">Source</a>

``` rust
pub struct ViewCreation {
    pub name: String,
    pub location: String,
    pub representations: ViewRepresentations,
    pub schema: Schema,
    pub properties: HashMap<String, String>,
    pub default_namespace: NamespaceIdent,
    pub default_catalog: Option<String>,
    pub summary: HashMap<String, String>,
}
```

Expand description

ViewCreation represents the creation of a view in the catalog.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The name of the view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The view’s base location; used to create metadata file locations

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.representations" class="anchor field">§</a>`representations: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations"><code>ViewRepresentations</code></a>

Representations for the view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema"><code>Schema</code></a>

The schema of the view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.properties" class="anchor field">§</a>`properties: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

The properties of the view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.default_namespace" class="anchor field">§</a>`default_namespace: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.NamespaceIdent.html" class="struct" title="struct iceberg::NamespaceIdent"><code>NamespaceIdent</code></a>

The default namespace to use when a reference in the SELECT is a single identifier

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.default_catalog" class="anchor field">§</a>`default_catalog: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Default catalog to use when a reference in the SELECT does not contain a catalog

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#structfield.summary" class="anchor field">§</a>`summary: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

A string to string map of summary metadata about the version Typical keys are “engine-name” and “engine-version”

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#impl-ViewCreation" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html" class="struct" title="struct iceberg::ViewCreation">ViewCreation</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#method.builder" class="fn">builder</a>() -\> ViewCreationBuilder\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>)\>

Create a builder for building `ViewCreation`. On the builder, call `.name(...)`, `.location(...)`, `.representations(...)`, `.schema(...)`, `.properties(...)`(optional), `.default_namespace(...)`, `.default_catalog(...)`(optional), `.summary(...)`(optional) to set the values of the fields. Finally, call `.build()` to create the instance of `ViewCreation`.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#impl-Debug-for-ViewCreation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html" class="struct" title="struct iceberg::ViewCreation">ViewCreation</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html#blanket-implementations" class="anchor">§</a>
