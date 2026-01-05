# Struct SchemaBuilder Copy item path

<a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/src/arrow_schema/schema.rs.html#29" class="src">Source</a>

``` rust
pub struct SchemaBuilder { /* private fields */ }
```

Expand description

A builder to facilitate building a [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema") from iteratively from [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Creates a new empty [`SchemaBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html "struct datafusion::common::arrow::datatypes::SchemaBuilder")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Creates a new empty [`SchemaBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html "struct datafusion::common::arrow::datatypes::SchemaBuilder") with space for `capacity` fields

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.push" class="fn">push</a>(&mut self, field: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>)

Appends a [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") to this [`SchemaBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html "struct datafusion::common::arrow::datatypes::SchemaBuilder") without checking for collision

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.remove" class="fn">remove</a>(&mut self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>

Removes and returns the [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") as index `idx`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#panics" class="doc-anchor">§</a>Panics

Panics if index out of bounds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.field" class="fn">field</a>(&mut self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>

Returns an immutable reference to the [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") at index `idx`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#panics-1" class="doc-anchor">§</a>Panics

Panics if index out of bounds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.field_mut" class="fn">field_mut</a>(&mut self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &mut <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>

Returns a mutable reference to the [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") at index `idx`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#panics-2" class="doc-anchor">§</a>Panics

Panics if index out of bounds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.metadata" class="fn">metadata</a>(&mut self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns an immutable reference to the Map of custom metadata key-value pairs.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.metadata_mut" class="fn">metadata_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Returns a mutable reference to the Map of custom metadata key-value pairs.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.reverse" class="fn">reverse</a>(&mut self)

Reverse the fileds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.try_merge" class="fn">try_merge</a>(&mut self, field: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Appends a [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef") to this [`SchemaBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html "struct datafusion::common::arrow::datatypes::SchemaBuilder") checking for collision

If an existing field exists with the same name, calls [`Field::try_merge`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.try_merge "method datafusion::common::arrow::datatypes::Field::try_merge")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

Consume this [`SchemaBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html "struct datafusion::common::arrow::datatypes::SchemaBuilder") yielding the final [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-Debug-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-Default-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-Extend%3CArc%3CField%3E%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-Extend%3CField%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.extend-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.extend_one-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.extend_reserve-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-From%3C%26Fields%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-From%3C%26Schema%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-From%3CFields%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#impl-From%3CSchema%3E-for-SchemaBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::datatypes::SchemaBuilder">SchemaBuilder</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.SchemaBuilder.html#blanket-implementations" class="anchor">§</a>
