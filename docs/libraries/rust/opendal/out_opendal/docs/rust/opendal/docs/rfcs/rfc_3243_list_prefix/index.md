# Module rfc_3243_list_prefix Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#206" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

List prefix

- Proposal Name: `list_prefix`
- Start Date: 2023-10-08
- RFC PR: [apache/opendal#3243](https://github.com/apache/opendal/pull/3243)
- Tracking Issue: [apache/opendal#3247](https://github.com/apache/opendal/issues/3247)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#summary" class="doc-anchor">Â§</a>Summary

Allow users to specify a prefix and remove the requirement that the path must end with `/`.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#motivation" class="doc-anchor">Â§</a>Motivation

OpenDAL uses `/` to distinguish between a file and a directory. This design is necessary for object storage services such as S3 and GCS, where both `abc` (file) and `abc/` (directory) can coexist. We require users to provide the correct path to the API. For instance, when using `read("abc/")`, it returns `IsADirectory`, whereas with `list("abc/")` it returns `NotADirectory`. This behavior may be perplexing for users.

As a side-effect of this design, OpenDAL always return exist for `stat("not_exist/")` since there is no way for OpenDAL to check if `not_exist/file_example` is exist via `HeadObject` call.

There are some issues and pull requests related to those issues.

- [Invalid metadata for dir objects in s3](https://github.com/apache/opendal/issues/3199)
- [`is_exist` always return true for key end with â€˜/â€™, in S3 service](https://github.com/apache/opendal/issues/2086)

POSIX-like file systems also have their own issues, as they lack native support for listing a prefix.

Give file tree like the following:

``` shell
abc/
abc/def
abc/xyz/
```

Calling `list("ab")` will return `NotFound` after we removing the requirement that the path must end with `/`.

So I propose the following changes of OpenDAL API behaviors:

- Remove the requirement that the path for `list` must end with `/`.
- Object storage services will use `list_object` API to check if a dir is exist.
- Simulate the list prefix behavior for POSIX-like file systems.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

Given the following file tree:

``` shell
abc/
abc/def_file
abc/def_dir/
abc/def_dir/xyz_file
abc/def_dir/xyz_dir/
```

While listing a path:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr>
<th>Case</th>
<th>Path</th>
<th>Result</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>list dir</td>
<td><code>abc/</code></td>
<td><code>abc/def_file</code><br />
<code>abc/def_dir/</code></td>
<td>children that matches the dir</td>
</tr>
<tr>
<td>list prefix</td>
<td><code>abc/def</code></td>
<td><code>abc/def_file</code><br />
<code>abc/def_dir/</code></td>
<td>children that matches the prefix</td>
</tr>
<tr>
<td>list file</td>
<td><code>abc/def_file</code></td>
<td><code>abc/def_file</code></td>
<td>the only children that matches the path</td>
</tr>
<tr>
<td>list dir without <code>/</code></td>
<td><code>abc/def_dir</code></td>
<td><code>abc/def_dir/</code></td>
<td>the only children that matches the path</td>
</tr>
<tr>
<td>list file ends with <code>/</code></td>
<td><code>abc/def_file/</code></td>
<td>EMPTY</td>
<td>no children matches the dir</td>
</tr>
<tr>
<td>list not exist dir</td>
<td><code>def/</code></td>
<td>EMPTY</td>
<td>no children found matches the dir</td>
</tr>
<tr>
<td>list not exist file</td>
<td><code>def</code></td>
<td>EMPTY</td>
<td>no children found matches the prefix</td>
</tr>
</tbody>
</table>

While listing a path with `delimiter` set to `""`:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr>
<th>Case</th>
<th>Path</th>
<th>Result</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>list dir</td>
<td><code>abc/</code></td>
<td><code>abc/def_file</code><br />
<code>abc/def_dir/</code><br />
<code>abc/def_dir/xyz_file</code><br />
<code>abc/def_dir/xyz_dir/</code></td>
<td>children that matches the dir</td>
</tr>
<tr>
<td>list prefix</td>
<td><code>abc/def</code></td>
<td><code>abc/def_file</code><br />
<code>abc/def_dir/</code><br />
<code>abc/def_dir/xyz_file</code><br />
<code>abc/def_dir/xyz_dir/</code></td>
<td>children that matches the prefix</td>
</tr>
<tr>
<td>list file</td>
<td><code>abc/def_file</code></td>
<td><code>abc/def_file</code></td>
<td>the only children that matches the path</td>
</tr>
<tr>
<td>list dir without <code>/</code></td>
<td><code>abc/def_dir</code></td>
<td><code>abc/def_dir/</code><br />
<code>abc/def_dir/xyz_file</code><br />
<code>abc/def_dir/xyz_dir/</code></td>
<td>children that matches the path</td>
</tr>
<tr>
<td>list file ends with <code>/</code></td>
<td><code>abc/def_file/</code></td>
<td>EMPTY</td>
<td>no children matches the dir</td>
</tr>
<tr>
<td>list not exist dir</td>
<td><code>def/</code></td>
<td>EMPTY</td>
<td>no children found matches the dir</td>
</tr>
<tr>
<td>list not exist file</td>
<td><code>def</code></td>
<td>EMPTY</td>
<td>no children found matches the prefix</td>
</tr>
</tbody>
</table>

While stat a path:

| Case | Path | Result |
|----|----|----|
| stat existing dir | `abc/` | Metadata with dir mode |
| stat existing file | `abc/def_file` | Metadata with file mode |
| stat dir without `/` | `abc/def_dir` | Error `NotFound` or metadata with dir mode |
| stat file with `/` | `abc/def_file/` | Error `NotFound` |
| stat not existing path | `xyz` | Error `NotFound` |

While create dir on a path:

| Case                        | Path   | Result                     |
|-----------------------------|--------|----------------------------|
| create dir on existing dir  | `abc/` | Ok                         |
| create dir on existing file | `abc`  | Error with `NotADirectory` |
| create dir with `/`         | `xyz/` | Ok                         |
| create dir without `/`      | `xyz`  | Ok with `xyz/` created     |

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

For POSIX-like services, we will:

- Simulate the list prefix behavior by listing the parent dir and filter the children that matches the prefix.
- Return `NotFound` while stat an existing file with `/`

For object storage services, we will:

- Use `list_object` API while stat a path ends with `/`.
  - Return dir metadata if the dir is exist or there is at least a children.
  - Return `NotFound` if the dir is not exist and there is no children.
- Check path before create dir with a path not ends with `/`.
  - Return `NotADirectory` if the path is exist.
  - Create the dir with `/` appended.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

None

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

None
