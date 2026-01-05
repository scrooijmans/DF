# Struct MapFieldNames Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/map_builder.rs.html#70" class="src">Source</a>

``` rust
pub struct MapFieldNames {
    pub entry: String,
    pub key: String,
    pub value: String,
}
```

Expand description

The [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") names for a [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray")

## Fields<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#structfield.entry" class="anchor field">§</a>`entry: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

[`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") name for map entries

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#structfield.key" class="anchor field">§</a>`key: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

[`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") name for map key

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#structfield.value" class="anchor field">§</a>`value: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

[`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") name for map value

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#impl-Clone-for-MapFieldNames" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#impl-Debug-for-MapFieldNames" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#impl-Default-for-MapFieldNames" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html#blanket-implementations" class="anchor">§</a>
