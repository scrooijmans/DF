# Module rfc_0501_new_builder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#98" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

New builder

- Proposal Name: `new_builder`
- Start Date: 2022-08-03
- RFC PR: [apache/opendal#501](https://github.com/apache/opendal/pull/501)
- Tracking Issue: [apache/opendal#502](https://github.com/apache/opendal/issues/502)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#summary" class="doc-anchor">Â§</a>Summary

Allow users to build services without async.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Most services share a similar builder API to construct backends.

``` rust
impl Builder {
    pub async fn finish(&mut self) -> Result<Arc<dyn Accessor>> {}
}
```

We have `async` here so that every user who wants to build services backend must go through an async runtime. Even for `memory` backend:

``` rust
impl Builder {
    /// Consume builder to build a memory backend.
    pub async fn finish(&mut self) -> Result<Arc<dyn Accessor>> {
        Ok(Arc::new(Backend::default()))
    }
}
```

Only `s3` services need to call async functions `detect_region` to get the correct region.

So, we can provide blocking `Builder` APIs and move async-related logic out for users to call out. This way, our users can build services without playing with async runtime.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

After this change, all our services builder will add a new API:

``` rust
impl Builder {
    pub fn build(&mut self) -> Result<Backend> {}
}
```

Along with this change, our `Operator` will accept `impl Accessor + 'static` instead of `Arc<dyn Accessor>` anymore:

``` rust
impl Operator {
    pub fn new(accessor: impl Accessor + 'static) -> Self {}
}
```

Also, we will implement `From<impl Accessor + 'static>` for `Operator`:

``` rust
impl<A> From<A> for Operator
where
    A: Accessor + 'static,
{
    fn from(acc: A) -> Self {
        Operator::newx(acc)
    }
}
```

We can initiate an operator quicker:

``` diff
- let op: Operator = Operator::new(fs::Backend::build().finish().await?);
+ let op: Operator = fs::Builder::new().build()?.into();
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will add the following APIs:

- All builders will add `build(&mut self) -> Result<Backend>`
- `impl<A> From<A> for Operator where A: Accessor + 'static`

We will deprecate the following APIs:

- All builders `finish()` API (should be replaced by `build()`)
- All services `build()` API (should be replaced by `Builder::new()` or `Builder::default()`)

We will change the following APIs:

- Operator: `new(accessor: Arc<dyn Accessor>)` -\> `fn new(accessor: impl dyn Accessor + 'static)`
- Operator: `async fn from_iter()` -\> `fn from_iter()`
- Operator: `async fn from_env()` -\> `fn from_env()`

Most services will work the same, except for `s3`: `s3` depends on `detect_region` to check the correct region if the user doesnâ€™t input. After this change, `s3::Builder.build()` will return error if `region` is missing. Users should call `detect_region` by themselves to get the region.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None.
