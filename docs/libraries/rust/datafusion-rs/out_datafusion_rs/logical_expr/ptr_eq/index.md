# Module ptr_eq Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#70" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ptr_eq/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ptr_eq/struct.PtrEq.html" class="struct" title="struct datafusion::logical_expr::ptr_eq::PtrEq">PtrEq</a>  
A wrapper around a pointer that implements `Eq` and `Hash` comparing the underlying pointer address.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ptr_eq/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ptr_eq/fn.arc_ptr_eq.html" class="fn" title="fn datafusion::logical_expr::ptr_eq::arc_ptr_eq">arc_ptr_eq</a>  
Compares two `Arc` pointers for equality based on their underlying pointers values. This is not equivalent to [`Arc::ptr_eq`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.ptr_eq "associated function alloc::sync::Arc::ptr_eq") for fat pointers, see that method for more information.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ptr_eq/fn.arc_ptr_hash.html" class="fn" title="fn datafusion::logical_expr::ptr_eq::arc_ptr_hash">arc_ptr_hash</a>  
Hashes an `Arc` pointer based on its underlying pointer value. The general contract for this function is that if [`arc_ptr_eq`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ptr_eq/fn.arc_ptr_eq.html "fn datafusion::logical_expr::ptr_eq::arc_ptr_eq") returns `true` for two `Arc`s, then this function should return the same hash value for both.
