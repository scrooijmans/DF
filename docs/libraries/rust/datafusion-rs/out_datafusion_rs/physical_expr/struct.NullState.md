# Struct NullState Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/aggregate/groups_accumulator/accumulate.rs.html#53" class="src">Source</a>

``` rust
pub struct NullState { /* private fields */ }
```

Expand description

Track the accumulator null state per row: if any values for that group were null and if any values have been seen at all for that group.

This is part of the inner loop for many [`GroupsAccumulator`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.GroupsAccumulator.html "trait datafusion::logical_expr::GroupsAccumulator")s, and thus the performance is critical and so there are multiple specialized implementations, invoked depending on the specific combinations of the input.

Typically there are 4 potential combinations of inputs must be special cased for performance:

- With / Without filter
- With / Without nulls in the input

If the input has nulls, then the accumulator must potentially handle each input null value specially (e.g. for `SUM` to mark the corresponding sum as null)

If there are filters present, `NullState` tracks if it has seen *any* value for that group (as some values may be filtered out). Without a filter, the accumulator is only passed groups that had at least one value to accumulate so they do not need to track if they have seen values for a particular group.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#impl-NullState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html" class="struct" title="struct datafusion::physical_expr::NullState">NullState</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html" class="struct" title="struct datafusion::physical_expr::NullState">NullState</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

return the size of all buffers allocated by this null state, not including self

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.accumulate" class="fn">accumulate</a>\<T, F\>( &mut self, group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], values: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct datafusion::common::arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>, opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value_fn: F, )

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Invokes `value_fn(group_index, value)` for each non null, non filtered value of `value`, while tracking which groups have seen null inputs and which groups have seen any inputs if necessary

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#arguments" class="doc-anchor">§</a>Arguments:

- `values`: the input arguments to the accumulator
- `group_indices`: To which groups do the rows in `values` belong, (aka group_index)
- `opt_filter`: if present, only rows for which is Some(true) are included
- `value_fn`: function invoked for (group_index, value) where value is non null

See [`accumulate`](https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_aggregate_common/aggregate/groups_accumulator/accumulate/fn.accumulate.html "fn datafusion_functions_aggregate_common::aggregate::groups_accumulator::accumulate::accumulate"), for more details on how value_fn is called

When value_fn is called it also sets

1.  `self.seen_values[group_index]` to true for all rows that had a non null value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.accumulate_boolean" class="fn">accumulate_boolean</a>\<F\>( &mut self, group_indices: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], values: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>, opt_filter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>\>, total_num_groups: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value_fn: F, )

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Invokes `value_fn(group_index, value)` for each non null, non filtered value in `values`, while tracking which groups have seen null inputs and which groups have seen any inputs, for [`BooleanArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html "struct datafusion::common::arrow::array::BooleanArray")s.

Since `BooleanArray` is not a [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") it must be handled specially.

See [`Self::accumulate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.accumulate "method datafusion::physical_expr::NullState::accumulate"), which handles `PrimitiveArray`s, for more details on other arguments.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.build" class="fn">build</a>(&mut self, emit_to: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.EmitTo.html" class="enum" title="enum datafusion::logical_expr::EmitTo">EmitTo</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>

Creates the a [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer") representing which group_indices should have null values (because they never saw any values) for the `emit_to` rows.

resets the internal state appropriately

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#impl-Debug-for-NullState" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html" class="struct" title="struct datafusion::physical_expr::NullState">NullState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#impl-Default-for-NullState" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html" class="struct" title="struct datafusion::physical_expr::NullState">NullState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html" class="struct" title="struct datafusion::physical_expr::NullState">NullState</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.NullState.html#blanket-implementations" class="anchor">§</a>
