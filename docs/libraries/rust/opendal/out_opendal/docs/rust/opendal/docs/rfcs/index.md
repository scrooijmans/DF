# Module rfcs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#18-282" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#rfcs---opendal-active-rfc-list" class="doc-anchor">Â§</a>RFCs - OpenDAL Active RFC List

RFCs power OpenDALâ€™s development.

The â€œRFCâ€? (request for comments) process is intended to provide a consistent and controlled path for changes to OpenDAL (such as new features) so that all stakeholders can be confident about the direction of the project.

Many changes, including bug fixes and documentation improvements, can be implemented and reviewed via the normal GitHub pull request workflow.

Some changes, though, are â€œsubstantialâ€? and we ask that these be put through a bit of a design process and produce a consensus among the OpenDAL community.

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#which-kinds-of-changes-require-an-rfc" class="doc-anchor">Â§</a>Which kinds of changes require an RFC?

Any substantial change or addition to the project that would require a significant amount of work to implement should generally be an RFC.

Some examples include:

- A new feature that creates a new public API or raw API.
- The removal of features that already shipped as part of the release.
- A big refactor of existing code or reorganization of code into new modules.

Those are just a few examples. Ultimately, the judgment call of what constitutes a big enough change to warrant an RFC is left to the project maintainers.

If you submit a pull request to implement a new feature without going through the RFC process, it may be closed with a polite request to submit an RFC first.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#before-creating-the-rfc" class="doc-anchor">Â§</a>Before creating the RFC

Preparing in advance before submitting an RFC hastily can increase its chances of being accepted. If you have proposals to make, it is advisable to engage in some preliminary groundwork to facilitate a smoother process.

It is great to seek feedback from other project developers first, as this can help validate the viability of the RFC. To ensure a sustained impact on the project, it is important to work together and reach a consensus.

Common preparatory steps include presenting your idea on platforms such as GitHub [issues](https://github.com/apache/opendal/issues/) or [discussions](https://github.com/apache/opendal/discussions/categories/ideas), or engaging in discussions through our [email list](https://opendal.apache.org/community/#mailing-list) or [Discord server](https://opendal.apache.org/discord).

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#the-rfc-process" class="doc-anchor">Â§</a>The RFC process

- Fork the [OpenDAL repo](https://github.com/apache/opendal) and create your branch from `main`.
- Copy \[`0000_example.md`\] to `0000-my-feature.md` (where â€œmy-featureâ€? is descriptive). Donâ€™t assign an RFC number yet; This is going to be the PR number, and weâ€™ll rename the file accordingly if the RFC is accepted.
- Submit a pull request. As a pull request, the RFC will receive design feedback from the larger community, and the author should be prepared to revise it in response.
- Now that your RFC has an open pull request, use the issue number of this PR to update your `0000-` prefix to that number.
- Build consensus and integrate feedback. RFCs that have broad support are much more likely to make progress than those that donâ€™t receive any comments. Feel free to reach OpenDAL maintainers for help.
- RFCs rarely go through this process unchanged, especially as alternatives and drawbacks are shown. You can make edits, big and small, to the RFC to clarify or change the design, but make changes as new commits to the pull request, and leave a comment on the pull request explaining your changes. Specifically, do not squash or rebase commits after they are visible on the pull request.
- The RFC pull request lasts for three days after the last update. After that, the RFC will be accepted or declined based on the consensus reached in the discussion.
- For the accepting of an RFC, we will require approval from at least three maintainers.
- Once the RFC is accepted, please create a tracking issue and update links in RFC. And then the PR will be merged and the RFC will become â€˜activeâ€™ status.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#implementing-an-rfc" class="doc-anchor">Â§</a>Implementing an RFC

An active RFC does not indicate the priority assigned to its implementation, nor does it imply that a developer has been specifically assigned the task of implementing the feature.

The RFC author is encouraged to submit an implementation after the RFC has been accepted. Nevertheless, it is not obligatory for them to do so.

Accepted RFCs may represent features that can wait until a developer chooses to work on them. Each accepted RFC is associated with an issue in the OpenDAL repository, which tracks its implementation.

If you are interested in implementing an RFC but are unsure if someone else is already working on it, feel free to inquire by leaving a comment on the associated issue.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#some-useful-tips" class="doc-anchor">Â§</a>Some useful tips

- The author of an RFC may not be the same one as the implementer. Therefore, when submitting an RFC, it is advisable to include sufficient information.
- If modifications are needed for an accepted RFC, please submit a new pull request or create a new RFC to propose changes.

## Modules<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/index.html#modules" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0000_example/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0000_example">rfc_0000_example</a>  
RFC example

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0041_object_native_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0041_object_native_api">rfc_0041_object_native_api</a>  
Object native API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0044_error_handle/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0044_error_handle">rfc_0044_error_handle</a>  
Error handle

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0057_auto_region/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0057_auto_region">rfc_0057_auto_region</a>  
Auto region

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0069_object_stream/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0069_object_stream">rfc_0069_object_stream</a>  
Object stream

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0090_limited_reader/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0090_limited_reader">rfc_0090_limited_reader</a>  
Limited reader

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0112_path_normalization/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0112_path_normalization">rfc_0112_path_normalization</a>  
Path normalization

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0191_async_streaming_io/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0191_async_streaming_io">rfc_0191_async_streaming_io</a>  
Async streaming IO

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0203_remove_credential/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0203_remove_credential">rfc_0203_remove_credential</a>  
Remove credential

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0221_create_dir/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0221_create_dir">rfc_0221_create_dir</a>  
Create dir

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0247_retryable_error/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0247_retryable_error">rfc_0247_retryable_error</a>  
Retryable error

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0293_object_id/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0293_object_id">rfc_0293_object_id</a>  
Object ID

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0337_dir_entry/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0337_dir_entry">rfc_0337_dir_entry</a>  
Dir entry

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0409_accessor_capabilities/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0409_accessor_capabilities">rfc_0409_accessor_capabilities</a>  
Accessor capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0413_presign/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0413_presign">rfc_0413_presign</a>  
Presign

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0423_command_line_interface/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0423_command_line_interface">rfc_0423_command_line_interface</a>  
Command line interface

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0429_init_from_iter/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0429_init_from_iter">rfc_0429_init_from_iter</a>  
Init from iter

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0438_multipart/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0438_multipart">rfc_0438_multipart</a>  
Multipart

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0443_gateway/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0443_gateway">rfc_0443_gateway</a>  
Gateway

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0501_new_builder/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0501_new_builder">rfc_0501_new_builder</a>  
New builder

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0554_write_refactor/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0554_write_refactor">rfc_0554_write_refactor</a>  
Write refactor

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0561_list_metadata_reuse/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0561_list_metadata_reuse">rfc_0561_list_metadata_reuse</a>  
List metadata reuse

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0599_blocking_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0599_blocking_api">rfc_0599_blocking_api</a>  
Blocking API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0623_redis_service/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0623_redis_service">rfc_0623_redis_service</a>  
Redis service

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0627_split_capabilities/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0627_split_capabilities">rfc_0627_split_capabilities</a>  
Split capabilities

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0661_path_in_accessor/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0661_path_in_accessor">rfc_0661_path_in_accessor</a>  
Path in accessor

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0793_generic_kv_services/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0793_generic_kv_services">rfc_0793_generic_kv_services</a>  
Generic KV services

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0926_object_reader/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0926_object_reader">rfc_0926_object_reader</a>  
Object reader

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_0977_refactor_error/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_0977_refactor_error">rfc_0977_refactor_error</a>  
Refactor error

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1085_object_handler/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_1085_object_handler">rfc_1085_object_handler</a>  
Object handler

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1391_object_metadataer/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_1391_object_metadataer">rfc_1391_object_metadataer</a>  
Object metadataer

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1398_query_based_metadata/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_1398_query_based_metadata">rfc_1398_query_based_metadata</a>  
Query based metadata

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1420_object_writer/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_1420_object_writer">rfc_1420_object_writer</a>  
Object writer

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1477_remove_object_concept/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_1477_remove_object_concept">rfc_1477_remove_object_concept</a>  
Remove object concept

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_1735_operation_extension/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_1735_operation_extension">rfc_1735_operation_extension</a>  
Operation extension

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2083_writer_sink_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2083_writer_sink_api">rfc_2083_writer_sink_api</a>  
Writer sink API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2133_append_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2133_append_api">rfc_2133_append_api</a>  
Append API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2299_chain_based_operator_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2299_chain_based_operator_api">rfc_2299_chain_based_operator_api</a>  
Chain based operator API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2602_object_versioning/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2602_object_versioning">rfc_2602_object_versioning</a>  
Object versioning

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2758_merge_append_into_write/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2758_merge_append_into_write">rfc_2758_merge_append_into_write</a>  
Merge append into write

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2774_lister_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2774_lister_api">rfc_2774_lister_api</a>  
Lister API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2779_list_with_metakey/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2779_list_with_metakey">rfc_2779_list_with_metakey</a>  
List with metakey

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_2852_native_capability/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_2852_native_capability">rfc_2852_native_capability</a>  
Native capability

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3017_remove_write_copy_from/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3017_remove_write_copy_from">rfc_3017_remove_write_copy_from</a>  
Remove write copy from

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3197_config/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3197_config">rfc_3197_config</a>  
Config

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3232_align_list_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3232_align_list_api">rfc_3232_align_list_api</a>  
Align list API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3243_list_prefix/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3243_list_prefix">rfc_3243_list_prefix</a>  
List prefix

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3356_lazy_reader/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3356_lazy_reader">rfc_3356_lazy_reader</a>  
Lazy reader

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3526_list_recursive/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3526_list_recursive">rfc_3526_list_recursive</a>  
List recursive

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3574_concurrent_stat_in_list/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3574_concurrent_stat_in_list">rfc_3574_concurrent_stat_in_list</a>  
Concurrent stat in list

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3734_buffered_reader/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3734_buffered_reader">rfc_3734_buffered_reader</a>  
Buffered Reader

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3898_concurrent_writer/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3898_concurrent_writer">rfc_3898_concurrent_writer</a>  
Concurrent Writer

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_3911_deleter_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_3911_deleter_api">rfc_3911_deleter_api</a>  
Deleter API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4382_range_based_read/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_4382_range_based_read">rfc_4382_range_based_read</a>  
Range Based Read API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_4638_executor/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_4638_executor">rfc_4638_executor</a>  
Executor API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5314_remove_metakey/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5314_remove_metakey">rfc_5314_remove_metakey</a>  
Remove metakey

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5444_operator_from_uri/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5444_operator_from_uri">rfc_5444_operator_from_uri</a>  
Operator from uri

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5479_context/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5479_context">rfc_5479_context</a>  
Context

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5485_conditional_reader/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5485_conditional_reader">rfc_5485_conditional_reader</a>  
Conditional Reader

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5495_list_with_deleted/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5495_list_with_deleted">rfc_5495_list_with_deleted</a>  
List With Deleted

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5556_write_returns_metadata/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5556_write_returns_metadata">rfc_5556_write_returns_metadata</a>  
Write Returns Metadata

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_5871_read_returns_metadata/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_5871_read_returns_metadata">rfc_5871_read_returns_metadata</a>  
Read Returns Metadata

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6189_remove_native_blocking/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_6189_remove_native_blocking">rfc_6189_remove_native_blocking</a>  
Remove Native Blocking

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6209_glob_support/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_6209_glob_support">rfc_6209_glob_support</a>  
Glob support

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6213_options_api/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_6213_options_api">rfc_6213_options_api</a>  
Options API

<a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html" class="mod" title="mod opendal::docs::rfcs::rfc_6678_simulate_layer">rfc_6678_simulate_layer</a>  
Simulate Layer
