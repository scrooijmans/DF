# infer_downcast_type in geoarrow_cast::downcast - Rust

## Function infer_downcast_type

[Source](about:blank/src/geoarrow_cast/downcast.rs.html#82-99)

```
pub fn infer_downcast_type<'a>(
    arrays: impl Iterator<Item = &'a dyn GeoArrowArray>,
) -> GeoArrowResult<Option<(NativeType, Dimension)>>
```

Expand description

Infer the simplest, most-compact native geometry type from the provided arrays, if any.

This accepts an [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") of [`GeoArrowArray`](https://docs.rs/geoarrow-array/0.5.0/x86_64-unknown-linux-gnu/geoarrow_array/trait_/trait.GeoArrowArray.html "trait geoarrow_array::trait_::GeoArrowArray") because it’s important to have schema stability across batches of a chunked GeoArrow array. You don’t want to separately downcast different batches because they could have different mixtures of geometry types.

A return value of `None` means that there is no common native type (other than `Geometry`) to downcast to. So your input data can be represented as a `GeometryArray` or as a serialized array.

After inferring a common type, use [`cast`](../cast/fn.cast.html "fn geoarrow_cast::cast::cast") to cast input to a specific output type.

### [§](#examples)Examples

Let’s say we have a WKB array with unknown data. We can use `infer_downcast_type` to find the simplest geometry type that fits our data.

```
use geoarrow_cast::cast::cast;
use geoarrow_cast::downcast::{NativeType, infer_downcast_type};
use wkt::wkt;

let mut builder = WkbBuilder::<i32>::new(Default::default());

builder.push_geometry(Some(&wkt!(POINT (0. 1.))));
builder.push_geometry(Some(&wkt!(POINT (2. 3.))));
builder.push_geometry(Some(&wkt!(POINT (4. 5.))));

let wkb_array = builder.finish();

let (native_type, dim) = infer_downcast_type(std::iter::once(&wkb_array as _))
    .unwrap()
    .unwrap();
assert_eq!(native_type, NativeType::Point);
assert_eq!(dim, Dimension::XY);

let point_type = PointType::new(Dimension::XY, Default::default());
cast(&wkb_array, &GeoArrowType::Point(point_type)).unwrap();
```

However, if you have geometry types in your array that aren’t compatible with a single GeoArrow native type, you’ll get `None` back from `infer_downcast_type`.

```
use geoarrow_cast::downcast::infer_downcast_type;
use wkt::wkt;

let wkb_type = WkbType::new(Default::default());
let mut builder = WkbBuilder::<i32>::new(wkb_type);

builder.push_geometry(Some(&wkt!(POINT (0. 1.))));
builder.push_geometry(Some(&wkt!(LINESTRING (2. 3., 4. 5.))));

let wkb_array = builder.finish();

assert_eq!(
    infer_downcast_type(std::iter::once(&wkb_array as _)).unwrap(),
    None
);
```
