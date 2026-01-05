# Struct ArrowBytesViewMap Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/binary_view_map.rs.html#117" class="src">Source</a>

``` rust
pub struct ArrowBytesViewMap<V>where
    V: Debug + PartialEq + Eq + Clone + Copy + Default,{ /* private fields */ }
```

Expand description

Optimized map for storing Arrow “byte view” types (`StringView`, `BinaryView`) values that can produce the set of keys on output as `GenericBinaryViewArray` without copies.

Equivalent to `HashSet<String, V>` but with better performance if you need to emit the keys as an Arrow `StringViewArray` / `BinaryViewArray`. For other purposes it is the same as a `HashMap<String, V>`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#generic-arguments" class="doc-anchor">§</a>Generic Arguments

- `V`: payload type

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#description" class="doc-anchor">§</a>Description

This is a specialized HashMap with the following properties:

1.  Optimized for storing and emitting Arrow byte types (e.g. `StringViewArray` / `BinaryViewArray`) very efficiently by minimizing copying of the string values themselves, both when inserting and when emitting the final array.

2.  Retains the insertion order of entries in the final array. The values are in the same order as they were inserted.

Note this structure can be used as a `HashSet` by specifying the value type as `()`, as is done by [`ArrowBytesViewSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewSet.html "struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewSet").

This map is used by the special `COUNT DISTINCT` aggregate function to store the distinct values, and by the `GROUP BY` operator to store group values when they are a single string array.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#impl-ArrowBytesViewMap%3CV%3E" class="anchor">§</a>

### impl\<V\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap">ArrowBytesViewMap</a>\<V\>

where V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.new" class="fn">new</a>(output_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/enum.OutputType.html" class="enum" title="enum datafusion::physical_expr_common::binary_map::OutputType">OutputType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap">ArrowBytesViewMap</a>\<V\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.take" class="fn">take</a>(&mut self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap">ArrowBytesViewMap</a>\<V\>

Return the contents of this map and replace it with a new empty map with the same output type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.insert_if_new" class="fn">insert_if_new</a>\<MP, OP\>( &mut self, values: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, make_payload_fn: MP, observe_payload_fn: OP, )

where MP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>) -\> V, OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(V),

Inserts each value from `values` into the map, invoking `payload_fn` for each value if *not* already present, deferring the allocation of the payload until it is needed.

Note that this is different than a normal map that would replace the existing entry

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#arguments" class="doc-anchor">§</a>Arguments:

`values`: array whose values are inserted

`make_payload_fn`: invoked for each value that is not already present to create the payload, in order of the values in `values`

`observe_payload_fn`: invoked once, for each value in `values`, that was already present in the map, with corresponding payload value.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#returns" class="doc-anchor">§</a>Returns

The payload value for the entry, either the existing value or the newly inserted value

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#safety" class="doc-anchor">§</a>Safety:

Note that `make_payload_fn` and `observe_payload_fn` are only invoked with valid values from `values`, not for the `NULL` value.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.into_state" class="fn">into_state</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Converts this set into a `StringViewArray`, or `BinaryViewArray`, containing each distinct value that was inserted. This is done without copying the values.

The values are guaranteed to be returned in the same order in which they were first seen.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Total number of entries (including null, if present)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Is the set empty?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.non_null_len" class="fn">non_null_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Number of non null entries

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the total size, in bytes, of memory used to store the data in this set, not including `self`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#impl-Debug-for-ArrowBytesViewMap%3CV%3E" class="anchor">§</a>

### impl\<V\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html" class="struct" title="struct datafusion::physical_expr_common::binary_view_map::ArrowBytesViewMap">ArrowBytesViewMap</a>\<V\>

where V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_view_map/struct.ArrowBytesViewMap.html#blanket-implementations" class="anchor">§</a>
