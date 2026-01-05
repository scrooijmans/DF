# Module buffer Copy item path

<a href="https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/src/arrow_buffer/lib.rs.html#30" class="src">Source</a>

Expand description

Types of shared memory region

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::BooleanBuffer">BooleanBuffer</a>  
A slice-able [`Buffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html "struct datafusion::common::arrow::buffer::Buffer") containing bit-packed booleans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>  
A contiguous memory region that can be shared with other buffers and across thread boundaries that stores Arrow data.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::MutableBuffer">MutableBuffer</a>  
A [`MutableBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.MutableBuffer.html "struct datafusion::common::arrow::buffer::MutableBuffer") is Arrow’s interface to build a [`Buffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html "struct datafusion::common::arrow::buffer::Buffer") out of items or slices of items.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>  
A [`BooleanBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.BooleanBuffer.html "struct datafusion::common::arrow::buffer::BooleanBuffer") used to encode validity for Arrow arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>  
A non-empty buffer of monotonically increasing, positive integers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.RunEndBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::RunEndBuffer">RunEndBuffer</a>  
A slice-able buffer of monotonically increasing, positive integers used to store run-ends

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::ScalarBuffer">ScalarBuffer</a>  
A strongly-typed [`Buffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html "struct datafusion::common::arrow::buffer::Buffer") supporting zero-copy cloning and slicing

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.bitwise_bin_op_helper.html" class="fn" title="fn datafusion::common::arrow::buffer::bitwise_bin_op_helper">bitwise_bin_op_helper</a>  
Apply a bitwise operation `op` to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.bitwise_quaternary_op_helper.html" class="fn" title="fn datafusion::common::arrow::buffer::bitwise_quaternary_op_helper">bitwise_quaternary_op_helper</a>  
Apply a bitwise operation `op` to four inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.bitwise_unary_op_helper.html" class="fn" title="fn datafusion::common::arrow::buffer::bitwise_unary_op_helper">bitwise_unary_op_helper</a>  
Apply a bitwise operation `op` to one input and return the result as a Buffer. The input is treated as a bitmap, meaning that offset and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.buffer_bin_and.html" class="fn" title="fn datafusion::common::arrow::buffer::buffer_bin_and">buffer_bin_and</a>  
Apply a bitwise and to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.buffer_bin_and_not.html" class="fn" title="fn datafusion::common::arrow::buffer::buffer_bin_and_not">buffer_bin_and_not</a>  
Apply a bitwise and_not to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.buffer_bin_or.html" class="fn" title="fn datafusion::common::arrow::buffer::buffer_bin_or">buffer_bin_or</a>  
Apply a bitwise or to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.buffer_bin_xor.html" class="fn" title="fn datafusion::common::arrow::buffer::buffer_bin_xor">buffer_bin_xor</a>  
Apply a bitwise xor to two inputs and return the result as a Buffer. The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/fn.buffer_unary_not.html" class="fn" title="fn datafusion::common::arrow::buffer::buffer_unary_not">buffer_unary_not</a>  
Apply a bitwise not to one input and return the result as a Buffer. The input is treated as a bitmap, meaning that offset and length are specified in number of bits.
