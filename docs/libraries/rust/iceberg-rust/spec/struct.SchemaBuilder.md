# Struct SchemaBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/schema/mod.rs.html#86-92" class="src">Source</a>

``` rust
pub struct SchemaBuilder { /* private fields */ }
```

Expand description

Schema builder.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#impl-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html" class="struct" title="struct iceberg::spec::SchemaBuilder">SchemaBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#method.with_fields" class="fn">with_fields</a>( self, fields: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>\>, ) -\> Self

Add fields to schema builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#method.with_schema_id" class="fn">with_schema_id</a>(self, schema_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

Set schema id.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#method.with_identifier_field_ids" class="fn">with_identifier_field_ids</a>( self, ids: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>, ) -\> Self

Set identifier field ids.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#method.with_alias" class="fn">with_alias</a>(self, alias_to_id: <a href="https://docs.rs/bimap/0.6.3/x86_64-unknown-linux-gnu/bimap/hash/struct.BiHashMap.html" class="struct" title="struct bimap::hash::BiHashMap">BiHashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>) -\> Self

Set alias to filed id mapping.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>\>

Builds the schema.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#impl-Debug-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html" class="struct" title="struct iceberg::spec::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html#blanket-implementations" class="anchor">§</a>
