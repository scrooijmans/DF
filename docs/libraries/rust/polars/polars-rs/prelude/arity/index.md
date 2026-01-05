# Module arity Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#13" class="src">Source</a>

## Traits<a href="https://docs.rs/polars/latest/polars/prelude/arity/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.BinaryFnMut.html" class="trait" title="trait polars::prelude::arity::BinaryFnMut">BinaryFnMut</a>

<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.TernaryFnMut.html" class="trait" title="trait polars::prelude::arity::TernaryFnMut">TernaryFnMut</a>

<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html" class="trait" title="trait polars::prelude::arity::UnaryFnMut">UnaryFnMut</a>

## Functions<a href="https://docs.rs/polars/latest/polars/prelude/arity/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.apply_binary_kernel_broadcast.html" class="fn" title="fn polars::prelude::arity::apply_binary_kernel_broadcast">apply_binary_kernel_broadcast</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.apply_binary_kernel_broadcast_owned.html" class="fn" title="fn polars::prelude::arity::apply_binary_kernel_broadcast_owned">apply_binary_kernel_broadcast_owned</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary.html" class="fn" title="fn polars::prelude::arity::binary">binary</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_elementwise.html" class="fn" title="fn polars::prelude::arity::binary_elementwise">binary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_elementwise_for_each.html" class="fn" title="fn polars::prelude::arity::binary_elementwise_for_each">binary_elementwise_for_each</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_elementwise_into_string_amortized.html" class="fn" title="fn polars::prelude::arity::binary_elementwise_into_string_amortized">binary_elementwise_into_string_amortized</a>  
Apply elementwise binary function which produces string, amortising allocations.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_elementwise_values.html" class="fn" title="fn polars::prelude::arity::binary_elementwise_values">binary_elementwise_values</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_mut_values.html" class="fn" title="fn polars::prelude::arity::binary_mut_values">binary_mut_values</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_mut_with_options.html" class="fn" title="fn polars::prelude::arity::binary_mut_with_options">binary_mut_with_options</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_owned.html" class="fn" title="fn polars::prelude::arity::binary_owned">binary_owned</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_to_series.html" class="fn" title="fn polars::prelude::arity::binary_to_series">binary_to_series</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.binary_unchecked_same_type.html" class="fn" title="fn polars::prelude::arity::binary_unchecked_same_type">binary_unchecked_same_type</a><sup>⚠</sup>  
Applies a kernel that produces `ArrayRef` of the same type.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.broadcast_binary_elementwise.html" class="fn" title="fn polars::prelude::arity::broadcast_binary_elementwise">broadcast_binary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.broadcast_binary_elementwise_values.html" class="fn" title="fn polars::prelude::arity::broadcast_binary_elementwise_values">broadcast_binary_elementwise_values</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.broadcast_try_binary_elementwise.html" class="fn" title="fn polars::prelude::arity::broadcast_try_binary_elementwise">broadcast_try_binary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.ternary_elementwise.html" class="fn" title="fn polars::prelude::arity::ternary_elementwise">ternary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_binary.html" class="fn" title="fn polars::prelude::arity::try_binary">try_binary</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_binary_elementwise.html" class="fn" title="fn polars::prelude::arity::try_binary_elementwise">try_binary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_binary_mut_with_options.html" class="fn" title="fn polars::prelude::arity::try_binary_mut_with_options">try_binary_mut_with_options</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_binary_unchecked_same_type.html" class="fn" title="fn polars::prelude::arity::try_binary_unchecked_same_type">try_binary_unchecked_same_type</a><sup>⚠</sup>  
Applies a kernel that produces `ArrayRef` of the same type.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_ternary_elementwise.html" class="fn" title="fn polars::prelude::arity::try_ternary_elementwise">try_ternary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_unary_elementwise.html" class="fn" title="fn polars::prelude::arity::try_unary_elementwise">try_unary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_unary_elementwise_values.html" class="fn" title="fn polars::prelude::arity::try_unary_elementwise_values">try_unary_elementwise_values</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.try_unary_mut_with_options.html" class="fn" title="fn polars::prelude::arity::try_unary_mut_with_options">try_unary_mut_with_options</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.unary_elementwise.html" class="fn" title="fn polars::prelude::arity::unary_elementwise">unary_elementwise</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.unary_elementwise_values.html" class="fn" title="fn polars::prelude::arity::unary_elementwise_values">unary_elementwise_values</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.unary_kernel.html" class="fn" title="fn polars::prelude::arity::unary_kernel">unary_kernel</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.unary_kernel_owned.html" class="fn" title="fn polars::prelude::arity::unary_kernel_owned">unary_kernel_owned</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.unary_mut_values.html" class="fn" title="fn polars::prelude::arity::unary_mut_values">unary_mut_values</a>  
Applies a kernel that produces `Array` types.

<a href="https://docs.rs/polars/latest/polars/prelude/arity/fn.unary_mut_with_options.html" class="fn" title="fn polars::prelude::arity::unary_mut_with_options">unary_mut_with_options</a>  
Applies a kernel that produces `Array` types.
