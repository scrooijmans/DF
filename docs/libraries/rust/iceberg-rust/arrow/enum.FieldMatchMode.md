# Enum FieldMatchMode Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/value.rs.html#440-445" class="src">Source</a>

``` rust
pub enum FieldMatchMode {
    Id,
    Name,
}
```

Expand description

Defines how Arrow fields are matched with Iceberg fields when converting data.

This enum provides two strategies for matching fields:

- `Id`: Match fields by their ID, which is stored in Arrow field metadata.
- `Name`: Match fields by their name, ignoring the field ID.

The ID matching mode is the default and preferred approach as it’s more robust against schema evolution where field names might change but IDs remain stable. The name matching mode can be useful in scenarios where field IDs are not available or when working with systems that don’t preserve field IDs.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#variant.Id" class="anchor">§</a>

### Id

Match fields by their ID stored in Arrow field metadata

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#variant.Name" class="anchor">§</a>

### Name

Match fields by their name, ignoring field IDs

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#impl-FieldMatchMode" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#method.match_field" class="fn">match_field</a>( &self, arrow_field: &<a href="https://docs.rs/arrow-schema/55.2.0/x86_64-unknown-linux-gnu/arrow_schema/field/type.FieldRef.html" class="type" title="type arrow_schema::field::FieldRef">FieldRef</a>, iceberg_field: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Determines if an Arrow field matches an Iceberg field based on the matching mode.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#impl-Clone-for-FieldMatchMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#impl-Debug-for-FieldMatchMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#impl-Copy-for-FieldMatchMode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html#blanket-implementations" class="anchor">§</a>
