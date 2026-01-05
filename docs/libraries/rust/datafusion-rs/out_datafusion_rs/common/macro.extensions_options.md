# Macro extensions_options Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1489" class="src">Source</a>

``` rust
macro_rules! extensions_options {
    (
     $(#[doc = $struct_d:tt])*
     $vis:vis struct $struct_name:ident {
        $(
        $(#[doc = $d:tt])*
        $field_vis:vis $field_name:ident : $field_type:ty, default = $default:expr
        )*$(,)*
    }
    ) => { ... };
}
```

Expand description

Convenience macro to create [`ExtensionsOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions").

The created structure implements the following traits:

- [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")
- [`Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")
- [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default")
- [`ExtensionOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.extensions_options.html#usage" class="doc-anchor">§</a>Usage

The syntax is:

``` text
extensions_options! {
     /// Struct docs (optional).
    [<vis>] struct <StructName> {
        /// Field docs (optional)
        [<vis>] <field_name>: <field_type>, default = <default_value>

        ... more fields
    }
}
```

The placeholders are:

- `[<vis>]`: Optional visibility modifier like `pub` or `pub(crate)`.
- `<StructName>`: Struct name like `MyStruct`.
- `<field_name>`: Field name like `my_field`.
- `<field_type>`: Field type like `u8`.
- `<default_value>`: Default value matching the field type like `42`.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.extensions_options.html#example" class="doc-anchor">§</a>Example

See also a full example on the [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") documentation

``` rust
use datafusion_common::extensions_options;

extensions_options! {
    /// My own config options.
    pub struct MyConfig {
        /// Should "foo" be replaced by "bar"?
        pub foo_to_bar: bool, default = true

        /// How many "baz" should be created?
        pub baz_count: usize, default = 1337
    }
}
```
