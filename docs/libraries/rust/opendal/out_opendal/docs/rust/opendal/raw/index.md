# Module raw Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/mod.rs.html#18-101" class="src">Source</a>

Expand description

Raw modules provide raw APIs that used by underlying services

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#notes" class="doc-anchor">Â§</a>Notes

- Only developers who want to develop new services or layers need to access raw APIs.
- Raw APIs should only be accessed via `opendal::raw::Xxxx`, any public API should never expose raw API directly.
- Raw APIs are far less stable than public API, please donâ€™t rely on them whenever possible.

## Modules<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#modules" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/index.html" class="mod" title="mod opendal::raw::adapters">adapters</a>  
Providing adapters and its implementations.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/index.html" class="mod" title="mod opendal::raw::oio">oio</a>  
`oio` provides OpenDALâ€™s raw traits and types that opendal returns as output.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/tests/index.html" class="mod" title="mod opendal::raw::tests">tests</a>`tests`  
Utilities for opendal testing.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>  
Info for the accessor. Users can use this struct to retrieve information about the underlying backend.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AtomicContentLength.html" class="struct" title="struct opendal::raw::AtomicContentLength">AtomicContentLength</a>  
AtomicContentLength is a wrapper of AtomicU64 that used to store content length.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>  
BytesContentRange is the content range of bytes.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>  
BytesRange(offset, size) carries a range of content.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html" class="struct" title="struct opendal::raw::ConcurrentTasks">ConcurrentTasks</a>  
ConcurrentTasks is used to execute tasks concurrently.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConfigDeserializer.html" class="struct" title="struct opendal::raw::ConfigDeserializer">ConfigDeserializer</a>  
ConfigDeserializer is used to deserialize given configs from `HashMap<String, String>`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.FormDataPart.html" class="struct" title="struct opendal::raw::FormDataPart">FormDataPart</a>  
FormDataPart is a builder for multipart/form-data part.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>  
The streaming body that OpenDALâ€™s HttpClient returned.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>  
A HTTP client instance for OpenDALâ€™s services.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.MixedPart.html" class="struct" title="struct opendal::raw::MixedPart">MixedPart</a>  
MixedPart is a builder for multipart/mixed part.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Multipart.html" class="struct" title="struct opendal::raw::Multipart">Multipart</a>  
Multipart is a builder for multipart/form-data.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>  
Args for `copy` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>  
Args for `create` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>  
Args for `delete` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDeleter.html" class="struct" title="struct opendal::raw::OpDeleter">OpDeleter</a>  
Args for `delete` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>  
Args for `list` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>  
Args for `presign` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>  
Args for `read` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpReader.html" class="struct" title="struct opendal::raw::OpReader">OpReader</a>  
Args for reader operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>  
Args for `rename` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>  
Args for `stat` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>  
Args for `write` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWriter.html" class="struct" title="struct opendal::raw::OpWriter">OpWriter</a>  
Args for `writer` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PathCacher.html" class="struct" title="struct opendal::raw::PathCacher">PathCacher</a>`internal-path-cache`  
PathCacher is a cache for path query.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.PresignedRequest.html" class="struct" title="struct opendal::raw::PresignedRequest">PresignedRequest</a>  
PresignedRequest is a presigned request return by `presign`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.QueryPairsWriter.html" class="struct" title="struct opendal::raw::QueryPairsWriter">QueryPairsWriter</a>  
QueryPairsWriter is used to write query pairs to a url.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RelatedPart.html" class="struct" title="struct opendal::raw::RelatedPart">RelatedPart</a>  
RelatedPart is a builder for multipart/related part.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>  
Reply for `copy` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>  
Reply for `create_dir` operation

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>  
Reply for `delete` operation

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>  
Reply for `list` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>  
Reply for `presign` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>  
Reply for `read` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>  
Reply for `rename` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>  
Reply for `stat` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>  
Reply for `write` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.Timestamp.html" class="struct" title="struct opendal::raw::Timestamp">Timestamp</a>  
An instant in time represented as the number of nanoseconds since the Unix epoch.

## Enums<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#enums" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html" class="enum" title="enum opendal::raw::FourWays">FourWays</a>  
FourWays is used to implement traits that based on four ways.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>  
Operation is the name of the operation that is being performed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.PresignOperation.html" class="enum" title="enum opendal::raw::PresignOperation">PresignOperation</a>  
Presign operation used for presign.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.ThreeWays.html" class="enum" title="enum opendal::raw::ThreeWays">ThreeWays</a>  
ThreeWays is used to implement traits that based on three ways.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>  
TwoWays is used to implement traits that based on two ways.

## Constants<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#constants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/constant.VERSION.html" class="constant" title="constant opendal::raw::VERSION">VERSION</a>  
VERSION is the compiled version of OpenDAL.

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>  
Underlying trait of all backends for implementers.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html" class="trait" title="trait opendal::raw::AccessDyn">AccessDyn</a>  
`AccessDyn` is the dyn version of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") make it possible to use as `Box<dyn AccessDyn>`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html" class="trait" title="trait opendal::raw::HttpFetch">HttpFetch</a>  
HttpFetch is the trait to fetch a request in async way. User should implement this trait to provide their own http client.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>  
Layer is used to intercept the operations on the underlying storage.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a>  
LayeredAccess is layered accessor that forward all not implemented method to inner.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>  
MaybeSend is a marker to determine whether a type is `Send` or not. We use this trait to wrap the `Send` requirement for wasm32 target.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Part.html" class="trait" title="trait opendal::raw::Part">Part</a>  
Part is a trait for multipart part.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.PathQuery.html" class="trait" title="trait opendal::raw::PathQuery">PathQuery</a>`internal-path-cache`  
The trait required for path cacher.

## Functions<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#functions" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_abs_path.html" class="fn" title="fn opendal::raw::build_abs_path">build_abs_path</a>  
build_abs_path will build an absolute path with root.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_header_value.html" class="fn" title="fn opendal::raw::build_header_value">build_header_value</a>  
Build header value from given string.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_rel_path.html" class="fn" title="fn opendal::raw::build_rel_path">build_rel_path</a>  
build_rel_path will build a relative path towards root.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_rooted_abs_path.html" class="fn" title="fn opendal::raw::build_rooted_abs_path">build_rooted_abs_path</a>  
build_rooted_abs_path will build an absolute path with root.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.build_tmp_path_of.html" class="fn" title="fn opendal::raw::build_tmp_path_of">build_tmp_path_of</a>  
Build a temporary path of a file path.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.format_authorization_by_basic.html" class="fn" title="fn opendal::raw::format_authorization_by_basic">format_authorization_by_basic</a>  
format authorization header by basic auth.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.format_authorization_by_bearer.html" class="fn" title="fn opendal::raw::format_authorization_by_bearer">format_authorization_by_bearer</a>  
format authorization header by bearer token.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.format_content_md5.html" class="fn" title="fn opendal::raw::format_content_md5">format_content_md5</a>  
format content md5 header by given input.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.get_basename.html" class="fn" title="fn opendal::raw::get_basename">get_basename</a>  
Get basename from path.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.get_parent.html" class="fn" title="fn opendal::raw::get_parent">get_parent</a>  
Get parent from path.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_http_uri_invalid_error.html" class="fn" title="fn opendal::raw::new_http_uri_invalid_error">new_http_uri_invalid_error</a>  
Parse http uri invalid error in to opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_json_deserialize_error.html" class="fn" title="fn opendal::raw::new_json_deserialize_error">new_json_deserialize_error</a>  
Parse json deserialize error into opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_json_serialize_error.html" class="fn" title="fn opendal::raw::new_json_serialize_error">new_json_serialize_error</a>  
Parse json serialize error into opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_request_build_error.html" class="fn" title="fn opendal::raw::new_request_build_error">new_request_build_error</a>  
Create a new error happened during building request.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_request_credential_error.html" class="fn" title="fn opendal::raw::new_request_credential_error">new_request_credential_error</a>  
Create a new error happened during signing request.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_request_sign_error.html" class="fn" title="fn opendal::raw::new_request_sign_error">new_request_sign_error</a>  
Create a new error happened during signing request.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_std_io_error.html" class="fn" title="fn opendal::raw::new_std_io_error">new_std_io_error</a>  
Parse std io error into opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_task_join_error.html" class="fn" title="fn opendal::raw::new_task_join_error">new_task_join_error</a>`internal-tokio-rt`  
Parse tokio error into opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_xml_deserialize_error.html" class="fn" title="fn opendal::raw::new_xml_deserialize_error">new_xml_deserialize_error</a>  
Parse xml deserialize error into opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.new_xml_serialize_error.html" class="fn" title="fn opendal::raw::new_xml_serialize_error">new_xml_serialize_error</a>  
Parse xml serialize error into opendal::Error.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.normalize_path.html" class="fn" title="fn opendal::raw::normalize_path">normalize_path</a>  
Make sure all operation are constructed by normalized path:

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.normalize_root.html" class="fn" title="fn opendal::raw::normalize_root">normalize_root</a>  
Make sure root is normalized to style like `/abc/def/`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_content_disposition.html" class="fn" title="fn opendal::raw::parse_content_disposition">parse_content_disposition</a>  
Parse Content-Disposition for header map

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_content_encoding.html" class="fn" title="fn opendal::raw::parse_content_encoding">parse_content_encoding</a>  
Parse content encoding from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_content_length.html" class="fn" title="fn opendal::raw::parse_content_length">parse_content_length</a>  
Parse content length from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_content_md5.html" class="fn" title="fn opendal::raw::parse_content_md5">parse_content_md5</a>  
Parse content md5 from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_content_range.html" class="fn" title="fn opendal::raw::parse_content_range">parse_content_range</a>  
Parse content range from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_content_type.html" class="fn" title="fn opendal::raw::parse_content_type">parse_content_type</a>  
Parse content type from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_etag.html" class="fn" title="fn opendal::raw::parse_etag">parse_etag</a>  
Parse etag from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_header_to_str.html" class="fn" title="fn opendal::raw::parse_header_to_str">parse_header_to_str</a>  
Parse header value to string according to name.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_into_metadata.html" class="fn" title="fn opendal::raw::parse_into_metadata">parse_into_metadata</a>  
parse_into_metadata will parse standards http headers into Metadata.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_last_modified.html" class="fn" title="fn opendal::raw::parse_last_modified">parse_last_modified</a>  
Parse last modified from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_location.html" class="fn" title="fn opendal::raw::parse_location">parse_location</a>  
Parse redirect location from header map

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_multipart_boundary.html" class="fn" title="fn opendal::raw::parse_multipart_boundary">parse_multipart_boundary</a>  
Parse multipart boundary from header map.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.parse_prefixed_headers.html" class="fn" title="fn opendal::raw::parse_prefixed_headers">parse_prefixed_headers</a>  
Parse prefixed headers and return a map with the prefix of each header removed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.percent_decode_path.html" class="fn" title="fn opendal::raw::percent_decode_path">percent_decode_path</a>  
percent_decode_path will do percent decoding for http decode path.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.percent_encode_path.html" class="fn" title="fn opendal::raw::percent_encode_path">percent_encode_path</a>  
percent_encode_path will do percent encoding for http encode path.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.signed_to_duration.html" class="fn" title="fn opendal::raw::signed_to_duration">signed_to_duration</a>  
Convert an unsigned [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration") into a jiff \[`SignedDuration`\]. Parse a duration encoded either as ISO-8601 (e.g. `PT5M`) or friendly (e.g. `5m`).

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.validate_path.html" class="fn" title="fn opendal::raw::validate_path">validate_path</a>  
Validate given path is match with given EntryMode.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/fn.with_error_response_context.html" class="fn" title="fn opendal::raw::with_error_response_context">with_error_response_context</a>  
Add response context to error.

## Type Aliases<a href="https://opendal.apache.org/docs/rust/opendal/raw/index.html#types" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html" class="type" title="type opendal::raw::Accessor">Accessor</a>  
Accessor is the type erased accessor with `Arc<dyn Accessor>`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>  
BoxedFuture is the type alias of \[`futures::future::BoxFuture`\].

<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>  
BoxedStaticFuture is the type alias of \[`futures::future::BoxFuture`\].

<a href="https://opendal.apache.org/docs/rust/opendal/raw/type.HttpFetcher.html" class="type" title="type opendal::raw::HttpFetcher">HttpFetcher</a>  
HttpFetcher is a type erased [`HttpFetch`](https://opendal.apache.org/docs/rust/opendal/raw/trait.HttpFetch.html "trait opendal::raw::HttpFetch").
