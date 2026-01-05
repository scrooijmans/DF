# Struct FileGroupPartitioner Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/file_groups.rs.html#129" class="src">Source</a>

``` rust
pub struct FileGroupPartitioner { /* private fields */ }
```

Expand description

Repartition input files into `target_partitions` partitions, if total file size exceed `repartition_file_min_size`

This partitions evenly by file byte range, and does not have any knowledge of how data is laid out in specific files. The specific `FileOpener` are responsible for the actual partitioning on specific data source type. (e.g. the `CsvOpener` will read lines overlap with byte range as well as handle boundaries to ensure all lines will be read exactly once)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#example" class="doc-anchor">§</a>Example

For example, if there are two files `A` and `B` that we wish to read with 4 partitions (with 4 threads) they will be divided as follows:

``` text
                                   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
│  File A (7MB)   │   ────────▶     ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
└─────────────────┘                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│  File B (1MB)   │                  ┌─────────────────┐
│                 │                │ │     File A      │ │
└─────────────────┘                  │  Range: 6-7MB   │
                                   │ └─────────────────┘ │
                                     ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#maintaining-order" class="doc-anchor">§</a>Maintaining Order

Within each group files are read sequentially. Thus, if the overall order of tuples must be preserved, multiple files can not be mixed in the same group.

In this case, the code will split the largest files evenly into any available empty groups, but the overall distribution may not be as even as if the order did not need to be preserved.

``` text
                                  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
┌─────────────────┐                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│  File A (6MB)   │   ────────▶      │                 │
│    (ordered)    │                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
└─────────────────┘                  │                 │
┌─────────────────┐                │ └─────────────────┘ │
│  File B (1MB)   │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│    (ordered)    │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
└─────────────────┘                  ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#impl-FileGroupPartitioner" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

Creates a new [`FileGroupPartitioner`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html "struct datafusion::datasource::physical_plan::FileGroupPartitioner") with default values:

1.  `target_partitions = 1`
2.  `repartition_file_min_size = 10MB`
3.  `preserve_order_within_groups = false`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.with_target_partitions" class="fn">with_target_partitions</a>( self, target_partitions: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

Set the target partitions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.with_repartition_file_min_size" class="fn">with_repartition_file_min_size</a>( self, repartition_file_min_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

Set the minimum size at which to repartition a file

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.with_preserve_order_within_groups" class="fn">with_preserve_order_within_groups</a>( self, preserve_order_within_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

Set whether the order of tuples within a file must be preserved

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.repartition_file_groups" class="fn">repartition_file_groups</a>( &self, file_groups: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroup.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroup">FileGroup</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroup.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroup">FileGroup</a>\>\>

Repartition input files according to the settings on this [`FileGroupPartitioner`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html "struct datafusion::datasource::physical_plan::FileGroupPartitioner").

If no repartitioning is needed or possible, return `None`.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#impl-Clone-for-FileGroupPartitioner" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#impl-Debug-for-FileGroupPartitioner" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#impl-Default-for-FileGroupPartitioner" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#impl-Copy-for-FileGroupPartitioner" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html" class="struct" title="struct datafusion::datasource::physical_plan::FileGroupPartitioner">FileGroupPartitioner</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileGroupPartitioner.html#blanket-implementations" class="anchor">§</a>
