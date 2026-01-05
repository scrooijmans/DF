# Type Alias Result Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1313" class="src">Source</a>

``` rust
pub type Result<T, E = Error> = Result<T, E>;
```

Expand description

A specialized `Result` for object store-related errors

## Aliased Type<a href="https://docs.rs/object_store/latest/object_store/type.Result.html#aliased-type" class="anchor">§</a>

``` rust
pub enum Result<T, E = Error> {
    Ok(T),
    Err(E),
}
```

## Variants<a href="https://docs.rs/object_store/latest/object_store/type.Result.html#variants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/type.Result.html#variant.Ok" class="anchor">§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://docs.rs/object_store/latest/object_store/type.Result.html#variant.Err" class="anchor">§</a>1.0.0

### Err(E)

Contains the error value
