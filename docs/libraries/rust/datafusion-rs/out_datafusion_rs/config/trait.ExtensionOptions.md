# Trait ExtensionOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1234" class="src">Source</a>

``` rust
pub trait ExtensionOptions:
    Send
    + Sync
    + Debug
    + 'static {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
    fn cloned(&self) -> Box<dyn ExtensionOptions>;
    fn set(&mut self, key: &str, value: &str) -> Result<(), DataFusionError>;
    fn entries(&self) -> Vec<ConfigEntry>;
}
```

Expand description

An object-safe API for storing arbitrary configuration.

See [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") for user defined configuration

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Return `self` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")

This is needed until trait upcasting is stabilized

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Return `self` as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")

This is needed until trait upcasting is stabilized

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#tymethod.cloned" class="fn">cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html" class="trait" title="trait datafusion::config::ExtensionOptions">ExtensionOptions</a>\>

Return a deep clone of this [`ExtensionOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions")

It is important this does not share mutable state to avoid consistency issues with configuration changing whilst queries are executing

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Set the given `key`, `value` pair

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#tymethod.entries" class="fn">entries</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html" class="struct" title="struct datafusion::config::ConfigEntry">ConfigEntry</a>\>

Returns the [`ConfigEntry`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html "struct datafusion::config::ConfigEntry") stored in this [`ExtensionOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html "trait datafusion::config::ExtensionOptions")

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html#implementors" class="anchor">§</a>
