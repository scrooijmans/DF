# Trait ConfigExtension Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1224" class="src">Source</a>

``` rust
pub trait ConfigExtension: ExtensionOptions {
    const PREFIX: &'static str;
}
```

Expand description

[`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") provides a mechanism to store third-party configuration within DataFusion [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

This mechanism can be used to pass configuration to user defined functions or optimizer passes

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html#example" class="doc-anchor">§</a>Example

``` rust
use datafusion_common::{
    config::ConfigExtension, extensions_options,
    config::ConfigOptions,
};
 // Define a new configuration struct using the `extensions_options` macro
 extensions_options! {
    /// My own config options.
    pub struct MyConfig {
        /// Should "foo" be replaced by "bar"?
        pub foo_to_bar: bool, default = true

        /// How many "baz" should be created?
        pub baz_count: usize, default = 1337
    }
 }

 impl ConfigExtension for MyConfig {
    const PREFIX: &'static str = "my_config";
 }

 // set up config struct and register extension
 let mut config = ConfigOptions::default();
 config.extensions.insert(MyConfig::default());

 // overwrite config default
 config.set("my_config.baz_count", "42").unwrap();

 // check config state
 let my_config = config.extensions.get::<MyConfig>().unwrap();
 assert!(my_config.foo_to_bar,);
 assert_eq!(my_config.baz_count, 42,);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html#note" class="doc-anchor">§</a>Note:

Unfortunately associated constants are not currently object-safe, and so this extends the object-safe [`ExtensionOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions")

## Required Associated Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Configuration namespace prefix to use

All values under this will be prefixed with `$PREFIX + "."`

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html#implementors" class="anchor">§</a>
