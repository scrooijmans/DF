# Struct FieldMetadata Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr.rs.html#448" class="src">Source</a>

``` rust
pub struct FieldMetadata { /* private fields */ }
```

Expand description

Literal metadata

Stores metadata associated with a literal expressions and is designed to be fast to `clone`.

This structure is used to store metadata associated with a literal expression, and it corresponds to the `metadata` field on [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#example-create-fieldmetadata-from-a-field" class="doc-anchor">§</a>Example: Create [`FieldMetadata`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html "struct datafusion::logical_expr::expr::FieldMetadata") from a [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field")

``` rust
// Create a new `FieldMetadata` instance from a `Field`
let metadata = FieldMetadata::new_from_field(&field);
// There is also a `From` impl:
let metadata = FieldMetadata::from(&field);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#example-update-a-field-with-fieldmetadata" class="doc-anchor">§</a>Example: Update a [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") with [`FieldMetadata`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html "struct datafusion::logical_expr::expr::FieldMetadata")

``` rust
// Add any metadata from `FieldMetadata` to `Field`
let updated_field = metadata.add_to_field(field);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-FieldMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.new_empty" class="fn">new_empty</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Create a new empty metadata instance.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.merge_options" class="fn">merge_options</a>( m: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>\>, n: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>\>

Merges two optional `FieldMetadata` instances, overwriting any existing keys in `m` with keys from `n` if present.

This function is commonly used in alias operations, particularly for literals with metadata. When creating an alias expression, the metadata from the original expression (such as a literal) is combined with any metadata specified on the alias.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#arguments" class="doc-anchor">§</a>Arguments

- `m` - The first metadata (typically from the original expression like a literal)
- `n` - The second metadata (typically from the alias definition)

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#merge-strategy" class="doc-anchor">§</a>Merge Strategy

- If both metadata instances exist, they are merged with `n` taking precedence
- Keys from `n` will overwrite keys from `m` if they have the same name
- If only one metadata instance exists, it is returned unchanged
- If neither exists, `None` is returned

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#example-usage" class="doc-anchor">§</a>Example usage

``` rust
use datafusion_expr::expr::FieldMetadata;
use std::collections::BTreeMap;

// Create metadata for a literal expression
let literal_metadata = Some(FieldMetadata::from(BTreeMap::from([
    ("source".to_string(), "constant".to_string()),
    ("type".to_string(), "int".to_string()),
])));

// Create metadata for an alias
let alias_metadata = Some(FieldMetadata::from(BTreeMap::from([
    ("description".to_string(), "answer".to_string()),
    ("source".to_string(), "user".to_string()), // This will override literal's "source"
])));

// Merge the metadata
let merged = FieldMetadata::merge_options(
    literal_metadata.as_ref(),
    alias_metadata.as_ref(),
);

// Result contains: {"source": "user", "type": "int", "description": "answer"}
assert!(merged.is_some());
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.new_from_field" class="fn">new_from_field</a>(field: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Create a new metadata instance from a `Field`’s metadata.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.new" class="fn">new</a>(inner: <a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Create a new metadata instance from a map of string keys to string values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get the inner metadata as a reference to a `BTreeMap`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.into_inner" class="fn">into_inner</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>

Return the inner metadata

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.extend" class="fn">extend</a>(&mut self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>)

Adds metadata from `other` into `self`, overwriting any existing keys.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the metadata is empty.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of key-value pairs in the metadata.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.to_hashmap" class="fn">to_hashmap</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Convert this `FieldMetadata` into a `HashMap<String, String>`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.add_to_field" class="fn">add_to_field</a>(&self, field: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>

Updates the metadata on the Field with this metadata, if it is not empty.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-Clone-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-Debug-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-Default-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-From%3C%26Field%3E-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(field: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-From%3C%26HashMap%3CString,+String%3E%3E-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

From reference

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(map: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-From%3C%26HashMap%3CString,+String%3E%3E-for-FieldMetadata-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(map: &<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-From%3CBTreeMap%3CString,+String%3E%3E-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(inner: <a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-From%3CHashMap%3CString,+String%3E%3E-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(map: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-From%3CHashMap%3CString,+String%3E%3E-for-FieldMetadata-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

From hashbrown map

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(map: <a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-Hash-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-PartialEq-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-PartialOrd-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-Eq-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#impl-StructuralPartialEq-for-FieldMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html#blanket-implementations" class="anchor">§</a>
