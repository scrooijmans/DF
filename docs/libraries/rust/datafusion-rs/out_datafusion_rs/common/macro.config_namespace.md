# Macro config_namespace Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#118" class="src">Source</a>

``` rust
macro_rules! config_namespace {
    (
        $(#[doc = $struct_d:tt])* // Struct-level documentation attributes
        $(#[deprecated($($struct_depr:tt)*)])? // Optional struct-level deprecated attribute
        $(#[allow($($struct_de:tt)*)])?
        $vis:vis struct $struct_name:ident {
            $(
                $(#[doc = $d:tt])* // Field-level documentation attributes
                $(#[deprecated($($field_depr:tt)*)])? // Optional field-level deprecated attribute
                $(#[allow($($field_de:tt)*)])?
                $field_vis:vis $field_name:ident : $field_type:ty,
                $(warn = $warn:expr,)?
                $(transform = $transform:expr,)?
                default = $default:expr
            )*$(,)*
        }
    ) => { ... };
}
```

Expand description

A macro that wraps a configuration struct and automatically derives [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and [`ConfigField`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html "trait datafusion::config::ConfigField") for it, allowing it to be used in the [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") configuration tree.

`transform` is used to normalize values before parsing.

For example,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_namespace.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
config_namespace! {
   /// Amazing config
   pub struct MyConfig {
       /// Field 1 doc
       field1: String, transform = str::to_lowercase, default = "".to_string()

       /// Field 2 doc
       field2: usize, default = 232

       /// Field 3 doc
       field3: Option<usize>, default = None
   }
}
```

Will generate

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/macro.config_namespace.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
/// Amazing config
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MyConfig {
    /// Field 1 doc
    field1: String,
    /// Field 2 doc
    field2: usize,
    /// Field 3 doc
    field3: Option<usize>,
}
impl ConfigField for MyConfig {
    fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let (key, rem) = key.split_once('.').unwrap_or((key, ""));
        match key {
            "field1" => {
                let value = str::to_lowercase(value);
                self.field1.set(rem, value.as_ref())
            },
            "field2" => self.field2.set(rem, value.as_ref()),
            "field3" => self.field3.set(rem, value.as_ref()),
            _ => _internal_err!(
                "Config value \"{}\" not found on MyConfig",
                key
            ),
        }
    }

    fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str) {
        let key = format!("{}.field1", key_prefix);
        let desc = "Field 1 doc";
        self.field1.visit(v, key.as_str(), desc);
        let key = format!("{}.field2", key_prefix);
        let desc = "Field 2 doc";
        self.field2.visit(v, key.as_str(), desc);
        let key = format!("{}.field3", key_prefix);
        let desc = "Field 3 doc";
        self.field3.visit(v, key.as_str(), desc);
    }
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            field1: "".to_string(),
            field2: 232,
            field3: None,
        }
    }
}
```

NB: Misplaced commas may result in nonsensical errors
