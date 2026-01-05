# Struct ViewMetadataBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/view_metadata_builder.rs.html#45-54" class="src">Source</a>

``` rust
pub struct ViewMetadataBuilder { /* private fields */ }
```

Expand description

Manipulating view metadata.

For this builder the order of called functions matters. All operations applied to the `ViewMetadata` are tracked in `changes` as a chronologically ordered vec of `ViewUpdate`. If an operation does not lead to a change of the `ViewMetadata`, the corresponding update is omitted from `changes`.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#impl-ViewMetadataBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html" class="struct" title="struct iceberg::spec::ViewMetadataBuilder">ViewMetadataBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.new" class="fn">new</a>( location: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>, view_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>, format_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewFormatVersion.html" class="enum" title="enum iceberg::spec::ViewFormatVersion">ViewFormatVersion</a>, properties: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a new view metadata builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.new_from_metadata" class="fn">new_from_metadata</a>(previous: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>) -\> Self

Creates a new view metadata builder from the given metadata to modify it.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.from_view_creation" class="fn">from_view_creation</a>(view_creation: <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.ViewCreation.html" class="struct" title="struct iceberg::ViewCreation">ViewCreation</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Creates a new view metadata builder from the given view creation.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.upgrade_format_version" class="fn">upgrade_format_version</a>( self, format_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewFormatVersion.html" class="enum" title="enum iceberg::spec::ViewFormatVersion">ViewFormatVersion</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Upgrade `FormatVersion`. Downgrades are not allowed.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#errors" class="doc-anchor">§</a>Errors

- Cannot downgrade to older format versions.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.set_location" class="fn">set_location</a>(self, location: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Set the location of the view, stripping any trailing slashes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.set_current_version_id" class="fn">set_current_version_id</a>(self, version_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Set an existing view version as the current version.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#errors-1" class="doc-anchor">§</a>Errors

- The specified `version_id` does not exist.
- The specified `version_id` is `-1` but no version has been added.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.set_current_version" class="fn">set_current_version</a>( self, view_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a new view version and set it as current.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.add_version" class="fn">add_version</a>(self, view_version: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Add a new version to the view.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#errors-2" class="doc-anchor">§</a>Errors

- The schema ID of the version is set to `-1`, but no schema has been added.
- The schema ID of the specified version is unknown.
- Multiple queries for the same dialect are added.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.add_schema" class="fn">add_schema</a>(self, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>) -\> Self

Add a new schema to the view.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.set_properties" class="fn">set_properties</a>(self, updates: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Update properties of the view.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.remove_properties" class="fn">remove_properties</a>(self, removals: &\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]) -\> Self

Remove properties from the view

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.assign_uuid" class="fn">assign_uuid</a>(self, uuid: <a href="https://docs.rs/uuid/1.18.1/x86_64-unknown-linux-gnu/uuid/struct.Uuid.html" class="struct" title="struct uuid::Uuid">Uuid</a>) -\> Self

Assign a new UUID to the view.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<ViewMetadataBuildResult\>

Build the `ViewMetadata` from the changes.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#impl-Clone-for-ViewMetadataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html" class="struct" title="struct iceberg::spec::ViewMetadataBuilder">ViewMetadataBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html" class="struct" title="struct iceberg::spec::ViewMetadataBuilder">ViewMetadataBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#impl-Debug-for-ViewMetadataBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html" class="struct" title="struct iceberg::spec::ViewMetadataBuilder">ViewMetadataBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html#blanket-implementations" class="anchor">§</a>
