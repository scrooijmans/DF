# Trait Literal Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/literal.rs.html#54" class="src">Source</a>

``` rust
pub trait Literal {
    // Required method
    fn lit(&self) -> Expr;
}
```

Expand description

Trait for converting a type to a [`Literal`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html "trait datafusion::logical_expr::Literal") literal expression.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

convert the value to a Literal expression

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-%26str" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-%26String" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-%26%5Bu8%5D" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-bool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

literal expression containing a bool

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

literal expression containing an f32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

literal expression containing an f64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

literal expression containing an i8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

literal expression containing an i16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

literal expression containing an i32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

literal expression containing an i64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

literal expression containing a u8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

literal expression containing a u16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

literal expression containing a u32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

literal expression containing a u64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-String" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-14" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-Vec%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-15" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

literal expression containing an i8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-16" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

literal expression containing an i16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-17" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

literal expression containing an i32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-18" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

literal expression containing an i64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-19" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

literal expression containing a u8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-20" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

literal expression containing a u16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-21" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

literal expression containing a u32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-22" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-NonZero%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

literal expression containing a u64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#method.lit-23" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#impl-Literal-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>
