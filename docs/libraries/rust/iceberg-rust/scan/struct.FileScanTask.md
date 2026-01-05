# Struct FileScanTask Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/scan/task.rs.html#30-57" class="src">Source</a>

``` rust
pub struct FileScanTask {
    pub start: u64,
    pub length: u64,
    pub record_count: Option<u64>,
    pub data_file_path: String,
    pub data_file_format: DataFileFormat,
    pub schema: SchemaRef,
    pub project_field_ids: Vec<i32>,
    pub predicate: Option<BoundPredicate>,
    pub deletes: Vec<FileScanTaskDeleteFile>,
}
```

Expand description

A task to scan part of file.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.start" class="anchor field">§</a>`start: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>

The start offset of the file to scan.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.length" class="anchor field">§</a>`length: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>

The length of the file to scan.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.record_count" class="anchor field">§</a>`record_count: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

The number of records in the file to scan.

This is an optional field, and only available if we are reading the entire data file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.data_file_path" class="anchor field">§</a>`data_file_path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The data file path corresponding to the task.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.data_file_format" class="anchor field">§</a>`data_file_format: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileFormat.html" class="enum" title="enum iceberg::spec::DataFileFormat"><code>DataFileFormat</code></a>

The format of the file to scan.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef"><code>SchemaRef</code></a>

The schema of the file to scan.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.project_field_ids" class="anchor field">§</a>`project_field_ids: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

The field ids to project.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.predicate" class="anchor field">§</a>`predicate: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate"><code>BoundPredicate</code></a>`>`

The predicate to filter.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#structfield.deletes" class="anchor field">§</a>`deletes: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTaskDeleteFile.html" class="struct" title="struct iceberg::scan::FileScanTaskDeleteFile"><code>FileScanTaskDeleteFile</code></a>`>`

The list of delete files that may need to be applied to this data file

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-FileScanTask" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.data_file_path" class="fn">data_file_path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the data file path of this file scan task.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.project_field_ids" class="fn">project_field_ids</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\]

Returns the project field id of this file scan task.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.predicate" class="fn">predicate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>\>

Returns the predicate of this file scan task.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>

Returns the schema of this file scan task as a reference

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.schema_ref" class="fn">schema_ref</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>

Returns the schema of this file scan task as a SchemaRef

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-Clone-for-FileScanTask" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-Debug-for-FileScanTask" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-Deserialize%3C&#39;de%3E-for-FileScanTask" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-PartialEq-for-FileScanTask" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-Serialize-for-FileScanTask" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#impl-StructuralPartialEq-for-FileScanTask" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html" class="struct" title="struct iceberg::scan::FileScanTask">FileScanTask</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html#blanket-implementations" class="anchor">§</a>
