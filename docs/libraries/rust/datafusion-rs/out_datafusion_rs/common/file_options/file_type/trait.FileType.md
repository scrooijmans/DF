# Trait FileType Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/file_options/file_type.rs.html#42" class="src">Source</a>

``` rust
pub trait FileType:
    GetExt
    + Display
    + Send
    + Sync {
    // Required method
    fn as_any(&self) -> &(dyn Any + 'static);
}
```

Expand description

Defines the functionality needed for logical planning for a type of file which will be read or written to storage.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the table source as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html#impl-FileType-for-DefaultFileType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/file_options/file_type/trait.FileType.html" class="trait" title="trait datafusion::common::file_options::file_type::FileType">FileType</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/struct.DefaultFileType.html" class="struct" title="struct datafusion::datasource::file_format::DefaultFileType">DefaultFileType</a>
