# Struct ManifestWriter Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/writer.rs.html#104-123" class="src">Source</a>

``` rust
pub struct ManifestWriter { /* private fields */ }
```

Expand description

A manifest writer.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#impl-ManifestWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html" class="struct" title="struct iceberg::spec::ManifestWriter">ManifestWriter</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#method.add_file" class="fn">add_file</a>( &mut self, data_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, sequence_number: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Add file as an added entry with a specific sequence number. The entry’s snapshot ID will be this manifest’s snapshot ID. The entry’s data sequence number will be the provided data sequence number. The entry’s file sequence number will be assigned at commit.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#method.add_delete_file" class="fn">add_delete_file</a>( &mut self, data_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, sequence_number: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, file_sequence_number: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Add a file as delete manifest entry. The entry’s snapshot ID will be this manifest’s snapshot ID. However, the original data and file sequence numbers of the file must be preserved when the file is marked as deleted.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#method.add_existing_file" class="fn">add_existing_file</a>( &mut self, data_file: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, snapshot_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, sequence_number: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, file_sequence_number: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Add an file as existing manifest entry. The original data and file sequence numbers, snapshot ID, which were assigned at commit, must be preserved when adding an existing entry.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#method.write_manifest_file" class="fn">write_manifest_file</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>\>

Write manifest file and return it.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html#blanket-implementations" class="anchor">§</a>
