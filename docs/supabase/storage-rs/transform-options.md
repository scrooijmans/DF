# TransformOptions in supabase_storage_rs::models - Rust

[supabase_storage_rs](../index.html)::[models](index.html)

## Struct TransformOptions

[Source](about:blank/src/supabase_storage_rs/models.rs.html#184-190)

```
pub struct TransformOptions<'a> {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub resize: Option<&'a str>,
    pub format: Option<&'a str>,
    pub quality: Option<u8>,
}
```

## Fields[§](#fields)

[§](#structfield.width)`width: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)>`[§](#structfield.height)`height: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)>`[§](#structfield.resize)`resize: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`[§](#structfield.format)`format: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`[§](#structfield.quality)`quality: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)>`

## Trait Implementations[§](#trait-implementations)

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#impl-Clone-for-TransformOptions%3C'a%3E)

### impl<'a> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#method.clone)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

Returns a copy of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174)
[§](#method.clone_from)

#### fn [clone_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#impl-Debug-for-TransformOptions%3C'a%3E)

### impl<'a> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#method.fmt)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html "type core::fmt::Result")

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#impl-Deserialize%3C'de%3E-for-TransformOptions%3C'a%3E)

### impl<'de: 'a, 'a> [Deserialize](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html "trait serde::de::Deserialize")<'de> for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#method.deserialize)

#### fn [deserialize](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>(\_\_deserializer: \_\_D) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, \_\_D::[Error](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error "type serde::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html "trait serde::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#impl-PartialEq-for-TransformOptions%3C'a%3E)

### impl<'a> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#method.eq)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#261)
[§](#method.ne)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#impl-Serialize-for-TransformOptions%3C'a%3E)

### impl<'a> [Serialize](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html "trait serde::ser::Serialize") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#method.serialize)

#### fn [serialize](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>(&self, \_\_serializer: \_\_S) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\_\_S::[Ok](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok "type serde::ser::Serializer::Ok"), \_\_S::[Error](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error "type serde::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html "trait serde::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

[Source](about:blank/src/supabase_storage_rs/models.rs.html#183)
[§](#impl-StructuralPartialEq-for-TransformOptions%3C'a%3E)

### impl<'a> [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

## Auto Trait Implementations[§](#synthetic-implementations)

[§](#impl-Freeze-for-TransformOptions%3C'a%3E)

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[§](#impl-RefUnwindSafe-for-TransformOptions%3C'a%3E)

### impl<'a> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[§](#impl-Send-for-TransformOptions%3C'a%3E)

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[§](#impl-Sync-for-TransformOptions%3C'a%3E)

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[§](#impl-Unpin-for-TransformOptions%3C'a%3E)

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

[§](#impl-UnwindSafe-for-TransformOptions%3C'a%3E)

### impl<'a> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [TransformOptions](struct.TransformOptions.html "struct supabase_storage_rs::models::TransformOptions")<'a>

## Blanket Implementations[§](#blanket-implementations)

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)
[§](#impl-Any-for-T)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)
[§](#method.type_id)

#### fn [type_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#209)
[§](#impl-Borrow%3CT%3E-for-T)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#211)
[§](#method.borrow)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#217)
[§](#impl-BorrowMut%3CT%3E-for-T)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#218)
[§](#method.borrow_mut)

#### fn [borrow_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#273)
[§](#impl-CloneToUninit-for-T)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#275)
[§](#method.clone_to_uninit)

#### unsafe fn [clone_to_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dst: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dst`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#765)
[§](#impl-From%3CT%3E-for-T)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#768)
[§](#method.from)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)
[§](#impl-Instrument-for-T)

### impl<T> [Instrument](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html "trait tracing::instrument::Instrument") for T

[Source](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)
[§](#method.instrument)

#### fn [instrument](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)(self, span: [Span](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html "struct tracing::span::Span")) -> [Instrumented](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/struct.Instrumented.html "struct tracing::instrument::Instrumented")<Self>

Instruments this type with the provided [`Span`](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html "struct tracing::span::Span"), returning an `Instrumented` wrapper. [Read more](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)
[§](#method.in_current_span)

#### fn [in_current_span](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/struct.Instrumented.html "struct tracing::instrument::Instrumented")<Self>

Instruments this type with the [current](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current "associated function tracing::span::Span::current") [`Span`](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html "struct tracing::span::Span"), returning an `Instrumented` wrapper. [Read more](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#748-750)
[§](#impl-Into%3CU%3E-for-T)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#758)
[§](#method.into)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#82-84)
[§](#impl-ToOwned-for-T)

### impl<T> [ToOwned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#86)
[§](#associatedtype.Owned)

#### type [Owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#87)
[§](#method.to_owned)

#### fn [to_owned](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#91)
[§](#method.clone_into)

#### fn [clone_into](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#805-807)
[§](#impl-TryFrom%3CU%3E-for-T)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#809)
[§](#associatedtype.Error-1)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#812)
[§](#method.try_from)

#### fn [try_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790-792)
[§](#impl-TryInto%3CU%3E-for-T)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#794)
[§](#associatedtype.Error)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#797)
[§](#method.try_into)

#### fn [try_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)
[§](#impl-WithSubscriber-for-T)

### impl<T> [WithSubscriber](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html "trait tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)
[§](#method.with_subscriber)

#### fn [with_subscriber](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch")<Self>

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](https://docs.rs/tracing-core/0.1.33/x86_64-unknown-linux-gnu/tracing_core/dispatcher/struct.Dispatch.html "struct tracing_core::dispatcher::Dispatch")\>,

Attaches the provided [`Subscriber`](https://docs.rs/tracing-core/0.1.33/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber") to this type, returning a [`WithDispatch`](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch") wrapper. [Read more](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)
[§](#method.with_current_subscriber)

#### fn [with_current_subscriber](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch")<Self>

Attaches the current [default](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber "mod tracing::dispatcher") [`Subscriber`](https://docs.rs/tracing-core/0.1.33/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber") to this type, returning a [`WithDispatch`](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html "struct tracing::instrument::WithDispatch") wrapper. [Read more](https://docs.rs/tracing/0.1.40/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[Source](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/src/serde/de/mod.rs.html#614)
[§](#impl-DeserializeOwned-for-T)

### impl<T> [DeserializeOwned](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.DeserializeOwned.html "trait serde::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde/1.0.215/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html "trait serde::de::Deserialize")<'de>,

[Source](https://docs.rs/yoke/0.7.5/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)
[§](#impl-ErasedDestructor-for-T)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.7.5/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/icu_provider/1.5.0/x86_64-unknown-linux-gnu/src/icu_provider/any.rs.html#32)
[§](#impl-MaybeSendSync-for-T)

### impl<T> [MaybeSendSync](https://docs.rs/icu_provider/1.5.0/x86_64-unknown-linux-gnu/icu_provider/any/trait.MaybeSendSync.html "trait icu_provider::any::MaybeSendSync") for T
