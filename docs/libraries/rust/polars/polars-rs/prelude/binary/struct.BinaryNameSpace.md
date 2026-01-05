# Struct BinaryNameSpace Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/binary.rs.html#3" class="src">Source</a>

``` rust
pub struct BinaryNameSpace(/* private fields */);
```

Available on **crate feature `lazy`** only.

Expand description

Specialized expressions for [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") of [`DataType::String`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.String "variant polars::prelude::DataType::String").

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#impl-BinaryNameSpace" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html" class="struct" title="struct polars::prelude::binary::BinaryNameSpace">BinaryNameSpace</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.contains_literal" class="fn">contains_literal</a>(self, pat: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Check if a binary value contains a literal binary.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.ends_with" class="fn">ends_with</a>(self, sub: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Check if a binary value ends with the given sequence.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.starts_with" class="fn">starts_with</a>(self, sub: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Check if a binary value starts with the given sequence.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.size_bytes" class="fn">size_bytes</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

Return the size (number of bytes) in each element.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.hex_decode" class="fn">hex_decode</a>(self, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.hex_encode" class="fn">hex_encode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.base64_decode" class="fn">base64_decode</a>(self, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.base64_encode" class="fn">base64_encode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#method.reinterpret" class="fn">reinterpret</a>( self, to_type: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>\>, is_little_endian: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/binary/struct.BinaryNameSpace.html#blanket-implementations" class="anchor">§</a>
