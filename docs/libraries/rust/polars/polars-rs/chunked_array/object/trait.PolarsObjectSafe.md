# Trait PolarsObjectSafe Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/object/mod.rs.html#32" class="src">Source</a>

``` rust
pub trait PolarsObjectSafe:
    Any
    + Debug
    + Send
    + Sync
    + Display {
    // Required methods
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn to_boxed(&self) -> Box<dyn PolarsObjectSafe>;
    fn equal(&self, other: &(dyn PolarsObjectSafe + 'static)) -> bool;
}
```

Expand description

Trimmed down object safe polars object

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#tymethod.type_name" class="fn">type_name</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#tymethod.to_boxed" class="fn">to_boxed</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#tymethod.equal" class="fn">equal</a>(&self, other: &(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#impl-PartialEq-for-%26dyn+PolarsObjectSafe" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for &(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &&(dyn <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html#impl-PolarsObjectSafe-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/object/trait.PolarsObjectSafe.html" class="trait" title="trait polars::chunked_array::object::PolarsObjectSafe">PolarsObjectSafe</a> for T

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,
