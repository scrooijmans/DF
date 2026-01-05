# Module rounding Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#54" class="src">Source</a>

Expand description

Floating point rounding mode utility library TODO: Remove this custom implementation and the “libc” dependency when floating-point rounding mode manipulation functions become available in Rust.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/trait.FloatBits.html" class="trait" title="trait datafusion::common::rounding::FloatBits">FloatBits</a>  
A trait to manipulate floating-point types with bitwise operations. Provides functions to convert a floating-point value to/from its bitwise representation as well as utility methods to handle special values.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/fn.alter_fp_rounding_mode.html" class="fn" title="fn datafusion::common::rounding::alter_fp_rounding_mode">alter_fp_rounding_mode</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/fn.next_down.html" class="fn" title="fn datafusion::common::rounding::next_down">next_down</a>  
Returns the next representable floating-point value smaller than the input value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/rounding/fn.next_up.html" class="fn" title="fn datafusion::common::rounding::next_up">next_up</a>  
Returns the next representable floating-point value greater than the input value.
