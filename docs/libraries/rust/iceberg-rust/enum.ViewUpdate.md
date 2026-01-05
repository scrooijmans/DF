# Enum ViewUpdate Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/catalog/mod.rs.html#836-889" class="src">Source</a>

``` rust
pub enum ViewUpdate {
    AssignUuid {
        uuid: Uuid,
    },
    UpgradeFormatVersion {
        format_version: ViewFormatVersion,
    },
    AddSchema {
        schema: Schema,
        last_column_id: Option<i32>,
    },
    SetLocation {
        location: String,
    },
    SetProperties {
        updates: HashMap<String, String>,
    },
    RemoveProperties {
        removals: Vec<String>,
    },
    AddViewVersion {
        view_version: ViewVersion,
    },
    SetCurrentViewVersion {
        view_version_id: i32,
    },
}
```

Expand description

ViewUpdate represents an update to a view in the catalog.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AssignUuid" class="anchor">§</a>

### AssignUuid

Assign a new UUID to the view

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AssignUuid.field.uuid" class="anchor field">§</a>`uuid: `<a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid"><code>Uuid</code></a>

The new UUID to assign.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.UpgradeFormatVersion" class="anchor">§</a>

### UpgradeFormatVersion

Upgrade view’s format version

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.UpgradeFormatVersion.field.format_version" class="anchor field">§</a>`format_version: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewFormatVersion.html" class="enum" title="enum iceberg::spec::ViewFormatVersion"><code>ViewFormatVersion</code></a>

Target format upgrade to.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AddSchema" class="anchor">§</a>

### AddSchema

Add a new schema to the view

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AddSchema.field.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema"><code>Schema</code></a>

The schema to add.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AddSchema.field.last_column_id" class="anchor field">§</a>`last_column_id: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

The last column id of the view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.SetLocation" class="anchor">§</a>

### SetLocation

Set view’s current schema

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.SetLocation.field.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

New location for view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.SetProperties" class="anchor">§</a>

### SetProperties

Set view’s properties

Matching keys are updated, and non-matching keys are left unchanged.

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.SetProperties.field.updates" class="anchor field">§</a>`updates: `<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Properties to update for view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.RemoveProperties" class="anchor">§</a>

### RemoveProperties

Remove view’s properties

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.RemoveProperties.field.removals" class="anchor field">§</a>`removals: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Properties to remove

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AddViewVersion" class="anchor">§</a>

### AddViewVersion

Add a new version to the view

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.AddViewVersion.field.view_version" class="anchor field">§</a>`view_version: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion"><code>ViewVersion</code></a>

The view version to add.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.SetCurrentViewVersion" class="anchor">§</a>

### SetCurrentViewVersion

Set view’s current version

#### Fields

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#variant.SetCurrentViewVersion.field.view_version_id" class="anchor field">§</a>`view_version_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

View version id to set as current, or -1 to set last added version

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#impl-Clone-for-ViewUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#impl-Debug-for-ViewUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#impl-Deserialize%3C&#39;de%3E-for-ViewUpdate" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#impl-PartialEq-for-ViewUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#impl-Serialize-for-ViewUpdate" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#impl-StructuralPartialEq-for-ViewUpdate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html" class="enum" title="enum iceberg::ViewUpdate">ViewUpdate</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ViewUpdate.html#blanket-implementations" class="anchor">§</a>
