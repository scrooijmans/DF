# Trait TimestampLiteral Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/literal.rs.html#60" class="src">Source</a>

``` rust
pub trait TimestampLiteral {
    // Required method
    fn lit_timestamp_nano(&self) -> Expr;
}
```

Expand description

Trait for converting a type to a literal timestamp

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

literal expression containing an i8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

literal expression containing an i16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

literal expression containing an i32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

literal expression containing an i64

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

literal expression containing a u8

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

literal expression containing a u16

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#impl-TimestampLiteral-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html" class="trait" title="trait datafusion::logical_expr::TimestampLiteral">TimestampLiteral</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

literal expression containing a u32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#method.lit_timestamp_nano-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#tymethod.lit_timestamp_nano" class="fn">lit_timestamp_nano</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TimestampLiteral.html#implementors" class="anchor">§</a>
