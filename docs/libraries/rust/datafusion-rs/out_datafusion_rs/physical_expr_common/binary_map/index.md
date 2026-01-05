# Module binary_map Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/lib.rs.html#32" class="src">Source</a>

Expand description

[`ArrowBytesMap`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesMap.html "struct datafusion::physical_expr_common::binary_map::ArrowBytesMap") and [`ArrowBytesSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html "struct datafusion::physical_expr_common::binary_map::ArrowBytesSet") for storing maps/sets of values from StringArray / LargeStringArray / BinaryArray / LargeBinaryArray.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesMap.html" class="struct" title="struct datafusion::physical_expr_common::binary_map::ArrowBytesMap">ArrowBytesMap</a>  
Optimized map for storing Arrow “bytes” types (`String`, `LargeString`, `Binary`, and `LargeBinary`) values that can produce the set of keys on output as `GenericBinaryArray` without copies.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/struct.ArrowBytesSet.html" class="struct" title="struct datafusion::physical_expr_common::binary_map::ArrowBytesSet">ArrowBytesSet</a>  
HashSet optimized for storing string or binary values that can produce that the final set as a GenericStringArray with minimal copies.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/enum.OutputType.html" class="enum" title="enum datafusion::physical_expr_common::binary_map::OutputType">OutputType</a>  
Should the output be a String or Binary?

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/binary_map/constant.INITIAL_BUFFER_CAPACITY.html" class="constant" title="constant datafusion::physical_expr_common::binary_map::INITIAL_BUFFER_CAPACITY">INITIAL_BUFFER_CAPACITY</a>  
The initial size, in bytes, of the string data
