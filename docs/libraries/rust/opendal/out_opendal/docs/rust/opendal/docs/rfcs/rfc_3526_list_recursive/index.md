# Module rfc_3526_list_recursive Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#214" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

List recursive

- Proposal Name: `list_recursive`
- Start Date: 2023-11-08
- RFC PR: [apache/opendal#3526](https://github.com/apache/opendal/pull/3526)
- Tracking Issue: [apache/opendal#0000](https://github.com/apache/opendal/issues/0000)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#summary" class="doc-anchor">Â§</a>Summary

Use `recursive` to replace `delimiter`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL add `delimiter` in `list` to allow users to control the list behavior:

- `delimiter == "/"` means use `/` as delimiter of path, it behaves like list current dir.
- `delimiter == ""` means donâ€™t set delimiter of path, it behaves like list current dir and all itâ€™s children.

Ideally, we should allow users to input any delimiter such as `|`, `-`, and `+`.

The `delimiter` concept can be challenging for users unfamiliar with object storage services. Currently, only `/` and empty spaces are accepted as delimiters, despite not being fully implemented across all services. We need to inform users that `delimiter == "/"` is used to list the current directory, while `delimiter == ""` is used for recursive listing. This may not be immediately clear.

So, why not use `recursive` directly for more clear API behavior?

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

OpenDAL will use `recursive` to replace `delimiter`. Default behavior is not changed, so users that using `op.list()` is not affected.

For users who is using `op.list_with(path).delimiter(delimiter)`:

- `op.list_with(path).delimiter("")` -\> `op.list_with(path).recursive(true)`
- `op.list_with(path).delimiter("/")` -\> `op.list_with(path).recursive(false)`
- `op.list_with(path).delimiter(other_value)`: not supported anymore.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

We will add `recursive` as a new arg in `OpList` and remove all fields related to `delimiter`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#cant-support-to-use----and--as-delimiter" class="doc-anchor">Â§</a>Canâ€™t support to use `|`, `-`, and `+` as delimiter

We never support this feature before.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html#add-delete-with-recursive-support" class="doc-anchor">Â§</a>Add delete with recursive support

Some services have native support for delete with recursive, such as azfile. We can add this feature in the future if needed.
