# Module casts Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/lib.rs.html#36" class="src">Source</a>

Expand description

Utilities for casting scalar literals to different data types

This module contains functions for casting ScalarValue literals to different data types, originally extracted from the optimizer’s unwrap_cast module to be shared between logical and physical layers.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/casts/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/casts/fn.is_supported_type.html" class="fn" title="fn datafusion::logical_expr_common::casts::is_supported_type">is_supported_type</a>  
Returns true if unwrap_cast_in_comparison supports this data type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/casts/fn.try_cast_literal_to_type.html" class="fn" title="fn datafusion::logical_expr_common::casts::try_cast_literal_to_type">try_cast_literal_to_type</a>  
Convert a literal value from one data type to another
