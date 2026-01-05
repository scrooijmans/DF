# Trait Part Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/multipart.rs.html#154-165" class="src">Source</a>

``` rust
pub trait Part: Sized + 'static {
    const TYPE: &'static str;

    // Required methods
    fn format(self) -> Buffer â;
    fn parse(s: &str) -> Result<Self>;
}
```

Expand description

Part is a trait for multipart part.

## Required Associated Constants<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#required-associated-consts" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

TYPE is the type of multipart.

Current available types are: `form-data` and `mixed`

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.format" class="fn">format</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#" class="tooltip" data-notable-ty="Buffer">â“˜</a>

format will generates the bytes.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#tymethod.parse" class="fn">parse</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

parse will parse the bytes into a part.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#impl-Part-for-FormDataPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html" class="struct" title="struct opendal::raw::FormDataPart">FormDataPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE-1" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "form-data"

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#impl-Part-for-MixedPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html" class="struct" title="struct opendal::raw::MixedPart">MixedPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE-2" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "mixed"

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#impl-Part-for-RelatedPart" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html" class="struct" title="struct opendal::raw::RelatedPart">RelatedPart</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE-3" class="anchor">Â§</a>

#### const <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html#associatedconstant.TYPE" class="constant">TYPE</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "related"
