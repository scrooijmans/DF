# Macro config_field Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1364" class="src">Source</a>

``` rust
macro_rules! config_field {
    ($t:ty) => { ... };
    ($t:ty, $arg:ident => $transform:expr) => { ... };
}
```

Expand description

Macro that generates [`ConfigField`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html "trait datafusion::config::ConfigField") for a given type.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_field.html#usage" class="doc-anchor">§</a>Usage

This always requires [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") to be implemented for the given type.

There are two ways to invoke this macro. The first one uses [`default_config_transform`](https://docs.rs/datafusion/50.2.0/datafusion/config/fn.default_config_transform.html "fn datafusion::config::default_config_transform")/[`FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") to parse the data:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_field.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
config_field(MyType);
```

Note that the parsing error MUST implement [`std::error::Error`](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error")!

Or you can specify how you want to parse an [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") into the type:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_field.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
fn parse_it(s: &str) -> Result<MyType> {
    ...
}

config_field(
    MyType,
    value => parse_it(value)
);
```
