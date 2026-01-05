# DefaultCrsTransform in geoarrow_schema::crs - Rust

## Struct DefaultCrsTransform

[Source](about:blank/src/geoarrow_schema/crs.rs.html#212)

```
pub struct DefaultCrsTransform {}
```

Expand description

A default implementation for [CrsTransform](trait.CrsTransform.html "trait geoarrow_schema::crs::CrsTransform") which does not do any CRS conversion.

Instead of raising an error, this will **silently drop any CRS information when writing data**.

## Trait Implementations[§](#trait-implementations)

## Auto Trait Implementations[§](#synthetic-implementations)

[§](#impl-Freeze-for-DefaultCrsTransform)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

[§](#impl-RefUnwindSafe-for-DefaultCrsTransform)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

[§](#impl-Send-for-DefaultCrsTransform)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

[§](#impl-Sync-for-DefaultCrsTransform)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

[§](#impl-Unpin-for-DefaultCrsTransform)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

[§](#impl-UnwindSafe-for-DefaultCrsTransform)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

## Blanket Implementations[§](#blanket-implementations)
