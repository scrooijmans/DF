# Type Alias Result Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/error.rs.html#46" class="src">Source</a>

``` rust
pub type Result<T, E = Error> = Result<T, E>;
```

Expand description

Result that is a wrapper of `Result<T, opendal::Error>`

## Aliased Type<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html#aliased-type" class="anchor">Â§</a>

``` rust
pub enum Result<T, E = Error> {
    Ok(T),
    Err(E),
}
```

## Variants<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html#variants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html#variant.Ok" class="anchor">Â§</a>1.0.0

### Ok(T)

Contains the success value

<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html#variant.Err" class="anchor">Â§</a>1.0.0

### Err(E)

Contains the error value
