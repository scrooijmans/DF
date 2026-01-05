# Module core Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/lib.rs.html#104" class="src">Source</a>

Expand description

Core datafusion expressions These are always available and not controlled by a feature flag “core” DataFusion functions

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/arrow_cast/index.html" class="mod" title="mod datafusion::functions::core::arrow_cast">arrow_cast</a>  
[`ArrowCastFunc`](https://docs.rs/datafusion/50.2.0/datafusion/functions/core/arrow_cast/struct.ArrowCastFunc.html "struct datafusion::functions::core::arrow_cast::ArrowCastFunc"): Implementation of the `arrow_cast`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/arrowtypeof/index.html" class="mod" title="mod datafusion::functions::core::arrowtypeof">arrowtypeof</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/coalesce/index.html" class="mod" title="mod datafusion::functions::core::coalesce">coalesce</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/index.html" class="mod" title="mod datafusion::functions::core::expr_ext">expr_ext</a>  
Extension methods for Expr.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_fn/index.html" class="mod" title="mod datafusion::functions::core::expr_fn">expr_fn</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/getfield/index.html" class="mod" title="mod datafusion::functions::core::getfield">getfield</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/greatest/index.html" class="mod" title="mod datafusion::functions::core::greatest">greatest</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/least/index.html" class="mod" title="mod datafusion::functions::core::least">least</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/named_struct/index.html" class="mod" title="mod datafusion::functions::core::named_struct">named_struct</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/nullif/index.html" class="mod" title="mod datafusion::functions::core::nullif">nullif</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/nvl/index.html" class="mod" title="mod datafusion::functions::core::nvl">nvl</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/nvl2/index.html" class="mod" title="mod datafusion::functions::core::nvl2">nvl2</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/overlay/index.html" class="mod" title="mod datafusion::functions::core::overlay">overlay</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/planner/index.html" class="mod" title="mod datafusion::functions::core::planner">planner</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/struct/index.html" class="mod" title="mod datafusion::functions::core::struct">struct</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/union_extract/index.html" class="mod" title="mod datafusion::functions::core::union_extract">union_extract</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/union_tag/index.html" class="mod" title="mod datafusion::functions::core::union_tag">union_tag</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/version/index.html" class="mod" title="mod datafusion::functions::core::version">version</a>  
[`VersionFunc`](https://docs.rs/datafusion/50.2.0/datafusion/functions/core/version/struct.VersionFunc.html "struct datafusion::functions::core::version::VersionFunc"): Implementation of the `version` function.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.arrow_cast.html" class="fn" title="fn datafusion::functions::core::arrow_cast">arrow_cast</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of arrow_cast

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.arrow_typeof.html" class="fn" title="fn datafusion::functions::core::arrow_typeof">arrow_typeof</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of arrow_typeof

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.coalesce.html" class="fn" title="fn datafusion::functions::core::coalesce">coalesce</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of coalesce

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.functions.html" class="fn" title="fn datafusion::functions::core::functions">functions</a>  
Returns all DataFusion functions defined in this package

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.get_field.html" class="fn" title="fn datafusion::functions::core::get_field">get_field</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of get_field

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.greatest.html" class="fn" title="fn datafusion::functions::core::greatest">greatest</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of greatest

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.least.html" class="fn" title="fn datafusion::functions::core::least">least</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of least

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.named_struct.html" class="fn" title="fn datafusion::functions::core::named_struct">named_struct</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of named_struct

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.nullif.html" class="fn" title="fn datafusion::functions::core::nullif">nullif</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of nullif

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.nvl.html" class="fn" title="fn datafusion::functions::core::nvl">nvl</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of nvl

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.nvl2.html" class="fn" title="fn datafusion::functions::core::nvl2">nvl2</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of nvl2

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.overlay.html" class="fn" title="fn datafusion::functions::core::overlay">overlay</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of overlay

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.struct.html" class="fn" title="fn datafusion::functions::core::struct">struct</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of r#struct

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.union_extract.html" class="fn" title="fn datafusion::functions::core::union_extract">union_extract</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of union_extract

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.union_tag.html" class="fn" title="fn datafusion::functions::core::union_tag">union_tag</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of union_tag

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/fn.version.html" class="fn" title="fn datafusion::functions::core::version">version</a>  
Return a [`ScalarUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.ScalarUDF.html "struct datafusion::logical_expr::ScalarUDF") implementation of version
