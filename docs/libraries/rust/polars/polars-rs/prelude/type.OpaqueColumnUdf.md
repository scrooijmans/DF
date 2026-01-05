# Type Alias OpaqueColumnUdf Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/expr/expr_dyn_fn.rs.html#135" class="src">Source</a>

``` rust
pub type OpaqueColumnUdf = LazySerde<SpecialEq<Arc<dyn AnonymousColumnsUdf>>>;
```

Available on **crate feature `lazy`** only.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#aliased-type" class="anchor">§</a>

``` rust
pub enum OpaqueColumnUdf {
    Deserialized(SpecialEq<Arc<dyn AnonymousColumnsUdf>>),
    Bytes(Bytes),
    Named {
        name: String,
        payload: Option<Bytes>,
        value: Option<SpecialEq<Arc<dyn AnonymousColumnsUdf>>>,
    },
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variant.Deserialized" class="anchor">§</a>

### Deserialized(<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq">SpecialEq</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf">AnonymousColumnsUdf</a>\>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variant.Bytes" class="anchor">§</a>

### Bytes(<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variant.Named" class="anchor">§</a>

### Named

Named functions allow for serializing arbitrary Rust functions as long as both sides know ahead of time which function it is. There is a registry of functions that both sides know and every time we need serialize we serialize the function by name in the registry.

Used by cloud.

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variant.Named.field.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variant.Named.field.payload" class="anchor field">§</a>`payload: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes"><code>Bytes</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/type.OpaqueColumnUdf.html#variant.Named.field.value" class="anchor field">§</a>`value: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq"><code>SpecialEq</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf"><code>AnonymousColumnsUdf</code></a>`>>>`
