# Struct DictionaryEncodingArgs Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#4175" class="src">Source</a>

``` rust
pub struct DictionaryEncodingArgs<'a> {
    pub id: i64,
    pub indexType: Option<WIPOffset<Int<'a>>>,
    pub isOrdered: bool,
    pub dictionaryKind: DictionaryKind,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#structfield.id" class="anchor field">§</a>`id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#structfield.indexType" class="anchor field">§</a>`indexType: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset"><code>WIPOffset</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int"><code>Int</code></a>`<'a>>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#structfield.isOrdered" class="anchor field">§</a>`isOrdered: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#structfield.dictionaryKind" class="anchor field">§</a>`dictionaryKind: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryKind.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryKind"><code>DictionaryKind</code></a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#impl-Default-for-DictionaryEncodingArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryEncodingArgs">DictionaryEncodingArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryEncodingArgs">DictionaryEncodingArgs</a>\<'a\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html#blanket-implementations" class="anchor">§</a>
