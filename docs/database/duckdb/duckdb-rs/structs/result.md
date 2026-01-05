# Result in duckdb - Rust

## [duckdb](../duckdb/index.html)1.4.1

## [Result](#)

### [Aliased Type](#aliased-type)

### [Variants](#variants)

- [Err](#variant.Err "Err")
- [Ok](#variant.Ok "Ok")

### [Trait Implementations](#trait-implementations)

- [OptionalExt<T>](#impl-OptionalExt%3CT%3E-for-Result%3CT,+Error%3E "OptionalExt<T>")

## [In crate duckdb](index.html)

[duckdb](index.html)

## Type Alias Result 

[Source](about:blank/src/duckdb/lib.rs.html#172)

```
pub type Result<T, E = Error> = Result<T, E>;
```

Expand description

A typedef of the result returned by many methods.

## Aliased Type[§](#aliased-type)

```
pub enum Result<T, E = Error> {
    Ok(T),
    Err(E),
}
```

## Variants[§](#variants)

[§](#variant.Ok)1.0.0

### Ok(T)

Contains the success value

[§](#variant.Err)1.0.0

### Err(E)

Contains the error value

## Trait Implementations[§](#trait-implementations)

[Source](about:blank/src/duckdb/lib.rs.html#184-192)
[§](#impl-OptionalExt%3CT%3E-for-Result%3CT,+Error%3E)

### impl<T> [OptionalExt](trait.OptionalExt.html "trait duckdb::OptionalExt")<T> for [Result](type.Result.html "type duckdb::Result")<T>

[Source](about:blank/src/duckdb/lib.rs.html#185-191)
[§](#method.optional)

#### fn [optional](about:blank/trait.OptionalExt.html#tymethod.optional)(self) -> [Result](type.Result.html "type duckdb::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

Converts a `Result<T>` into a `Result<Option<T>>`. [Read more](about:blank/trait.OptionalExt.html#tymethod.optional)
