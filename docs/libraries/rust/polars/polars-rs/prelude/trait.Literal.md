# Trait Literal Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/plans/lit.rs.html#407" class="src">Source</a>

``` rust
pub trait Literal {
    // Required method
    fn lit(self) -> Expr;
}
```

Available on **crate feature `lazy`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

[Literal](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#variant.Literal "variant polars::prelude::Expr::Literal") expression.

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-%26str" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-%26%5Bu8%5D" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-bool" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-13" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-String" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-14" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-Vec%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-15" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-NaiveDate" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-16" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-NaiveDateTime" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-17" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-TimeDelta" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#method.lit-18" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#tymethod.lit" class="fn">lit</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-LiteralValue" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.LiteralValue.html" class="enum" title="enum polars::prelude::LiteralValue">LiteralValue</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-Duration" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-Null" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Null.html" class="struct" title="struct polars::prelude::Null">Null</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-PlSmallStr" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-Scalar" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html#impl-Literal-for-Series" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.Literal.html" class="trait" title="trait polars::prelude::Literal">Literal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>
