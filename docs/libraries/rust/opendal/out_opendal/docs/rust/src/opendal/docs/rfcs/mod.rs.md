# 

opendal/docs/rfcs/

mod.rs

``` rust
1// Licensed to the Apache Software Foundation (ASF) under one
2// or more contributor license agreements.  See the NOTICE file
3// distributed with this work for additional information
4// regarding copyright ownership.  The ASF licenses this file
5// to you under the Apache License, Version 2.0 (the
6// "License"); you may not use this file except in compliance
7// with the License.  You may obtain a copy of the License at
8//
9//   http://www.apache.org/licenses/LICENSE-2.0
10//
11// Unless required by applicable law or agreed to in writing,
12// software distributed under the License is distributed on an
13// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
14// KIND, either express or implied.  See the License for the
15// specific language governing permissions and limitations
16// under the License.
17
18#![doc = include_str!("README.md")]
19
20/// RFC example
21#[doc = include_str!("0000_example.md")]
22pub mod rfc_0000_example {}
23
24/// Object native API
25#[doc = include_str!("0041_object_native_api.md")]
26pub mod rfc_0041_object_native_api {}
27
28/// Error handle
29#[doc = include_str!("0044_error_handle.md")]
30pub mod rfc_0044_error_handle {}
31
32/// Auto region
33#[doc = include_str!("0057_auto_region.md")]
34pub mod rfc_0057_auto_region {}
35
36/// Object stream
37#[doc = include_str!("0069_object_stream.md")]
38pub mod rfc_0069_object_stream {}
39
40/// Limited reader
41#[doc = include_str!("0090_limited_reader.md")]
42pub mod rfc_0090_limited_reader {}
43
44/// Path normalization
45#[doc = include_str!("0112_path_normalization.md")]
46pub mod rfc_0112_path_normalization {}
47
48/// Async streaming IO
49#[doc = include_str!("0191_async_streaming_io.md")]
50pub mod rfc_0191_async_streaming_io {}
51
52/// Remove credential
53#[doc = include_str!("0203_remove_credential.md")]
54pub mod rfc_0203_remove_credential {}
55
56/// Create dir
57#[doc = include_str!("0221_create_dir.md")]
58pub mod rfc_0221_create_dir {}
59
60/// Retryable error
61#[doc = include_str!("0247_retryable_error.md")]
62pub mod rfc_0247_retryable_error {}
63
64/// Object ID
65#[doc = include_str!("0293_object_id.md")]
66pub mod rfc_0293_object_id {}
67
68/// Dir entry
69#[doc = include_str!("0337_dir_entry.md")]
70pub mod rfc_0337_dir_entry {}
71
72/// Accessor capabilities
73#[doc = include_str!("0409_accessor_capabilities.md")]
74pub mod rfc_0409_accessor_capabilities {}
75
76/// Presign
77#[doc = include_str!("0413_presign.md")]
78pub mod rfc_0413_presign {}
79
80/// Command line interface
81#[doc = include_str!("0423_command_line_interface.md")]
82pub mod rfc_0423_command_line_interface {}
83
84/// Init from iter
85#[doc = include_str!("0429_init_from_iter.md")]
86pub mod rfc_0429_init_from_iter {}
87
88/// Multipart
89#[doc = include_str!("0438_multipart.md")]
90pub mod rfc_0438_multipart {}
91
92/// Gateway
93#[doc = include_str!("0443_gateway.md")]
94pub mod rfc_0443_gateway {}
95
96/// New builder
97#[doc = include_str!("0501_new_builder.md")]
98pub mod rfc_0501_new_builder {}
99
100/// Write refactor
101#[doc = include_str!("0554_write_refactor.md")]
102pub mod rfc_0554_write_refactor {}
103
104/// List metadata reuse
105#[doc = include_str!("0561_list_metadata_reuse.md")]
106pub mod rfc_0561_list_metadata_reuse {}
107
108/// Blocking API
109#[doc = include_str!("0599_blocking_api.md")]
110pub mod rfc_0599_blocking_api {}
111
112/// Redis service
113#[doc = include_str!("0623_redis_service.md")]
114pub mod rfc_0623_redis_service {}
115
116/// Split capabilities
117#[doc = include_str!("0627_split_capabilities.md")]
118pub mod rfc_0627_split_capabilities {}
119
120/// Path in accessor
121#[doc = include_str!("0661_path_in_accessor.md")]
122pub mod rfc_0661_path_in_accessor {}
123
124/// Generic KV services
125#[doc = include_str!("0793_generic_kv_services.md")]
126pub mod rfc_0793_generic_kv_services {}
127
128/// Object reader
129#[doc = include_str!("0926_object_reader.md")]
130pub mod rfc_0926_object_reader {}
131
132/// Refactor error
133#[doc = include_str!("0977_refactor_error.md")]
134pub mod rfc_0977_refactor_error {}
135
136/// Object handler
137#[doc = include_str!("1085_object_handler.md")]
138pub mod rfc_1085_object_handler {}
139
140/// Object metadataer
141#[doc = include_str!("1391_object_metadataer.md")]
142pub mod rfc_1391_object_metadataer {}
143
144/// Query based metadata
145#[doc = include_str!("1398_query_based_metadata.md")]
146pub mod rfc_1398_query_based_metadata {}
147
148/// Object writer
149#[doc = include_str!("1420_object_writer.md")]
150pub mod rfc_1420_object_writer {}
151
152/// Remove object concept
153#[doc = include_str!("1477_remove_object_concept.md")]
154pub mod rfc_1477_remove_object_concept {}
155
156/// Operation extension
157#[doc = include_str!("1735_operation_extension.md")]
158pub mod rfc_1735_operation_extension {}
159
160/// Writer sink API
161#[doc = include_str!("2083_writer_sink_api.md")]
162pub mod rfc_2083_writer_sink_api {}
163
164/// Append API
165#[doc = include_str!("2133_append_api.md")]
166pub mod rfc_2133_append_api {}
167
168/// Chain based operator API
169#[doc = include_str!("2299_chain_based_operator_api.md")]
170pub mod rfc_2299_chain_based_operator_api {}
171
172/// Object versioning
173#[doc = include_str!("2602_object_versioning.md")]
174pub mod rfc_2602_object_versioning {}
175
176/// Merge append into write
177#[doc = include_str!("2758_merge_append_into_write.md")]
178pub mod rfc_2758_merge_append_into_write {}
179
180/// Lister API
181#[doc = include_str!("2774_lister_api.md")]
182pub mod rfc_2774_lister_api {}
183
184/// List with metakey
185#[doc = include_str!("2779_list_with_metakey.md")]
186pub mod rfc_2779_list_with_metakey {}
187
188/// Native capability
189#[doc = include_str!("2852_native_capability.md")]
190pub mod rfc_2852_native_capability {}
191
192/// Remove write copy from
193#[doc = include_str!("3017_remove_write_copy_from.md")]
194pub mod rfc_3017_remove_write_copy_from {}
195
196/// Config
197#[doc = include_str!("3197_config.md")]
198pub mod rfc_3197_config {}
199
200/// Align list API
201#[doc = include_str!("3232_align_list_api.md")]
202pub mod rfc_3232_align_list_api {}
203
204/// List prefix
205#[doc = include_str!("3243_list_prefix.md")]
206pub mod rfc_3243_list_prefix {}
207
208/// Lazy reader
209#[doc = include_str!("3356_lazy_reader.md")]
210pub mod rfc_3356_lazy_reader {}
211
212/// List recursive
213#[doc = include_str!("3526_list_recursive.md")]
214pub mod rfc_3526_list_recursive {}
215
216/// Concurrent stat in list
217#[doc = include_str!("3574_concurrent_stat_in_list.md")]
218pub mod rfc_3574_concurrent_stat_in_list {}
219
220/// Buffered Reader
221#[doc = include_str!("3734_buffered_reader.md")]
222pub mod rfc_3734_buffered_reader {}
223
224/// Concurrent Writer
225#[doc = include_str!("3898_concurrent_writer.md")]
226pub mod rfc_3898_concurrent_writer {}
227
228/// Deleter API
229#[doc = include_str!("3911_deleter_api.md")]
230pub mod rfc_3911_deleter_api {}
231
232/// Range Based Read API
233#[doc = include_str!("4382_range_based_read.md")]
234pub mod rfc_4382_range_based_read {}
235
236/// Executor API
237#[doc = include_str!("4638_executor.md")]
238pub mod rfc_4638_executor {}
239
240/// Remove metakey
241#[doc = include_str!("5314_remove_metakey.md")]
242pub mod rfc_5314_remove_metakey {}
243
244/// Operator from uri
245#[doc = include_str!("5444_operator_from_uri.md")]
246pub mod rfc_5444_operator_from_uri {}
247
248/// Context
249#[doc = include_str!("5479_context.md")]
250pub mod rfc_5479_context {}
251
252/// Conditional Reader
253#[doc = include_str!("5485_conditional_reader.md")]
254pub mod rfc_5485_conditional_reader {}
255
256/// List With Deleted
257#[doc = include_str!("5495_list_with_deleted.md")]
258pub mod rfc_5495_list_with_deleted {}
259
260/// Write Returns Metadata
261#[doc = include_str!("5556_write_returns_metadata.md")]
262pub mod rfc_5556_write_returns_metadata {}
263
264/// Read Returns Metadata
265#[doc = include_str!("5871_read_returns_metadata.md")]
266pub mod rfc_5871_read_returns_metadata {}
267
268/// Remove Native Blocking
269#[doc = include_str!("6189_remove_native_blocking.md")]
270pub mod rfc_6189_remove_native_blocking {}
271
272/// Glob support
273#[doc = include_str!("6209_glob_support.md")]
274pub mod rfc_6209_glob_support {}
275
276/// Options API
277#[doc = include_str!("6213_options_api.md")]
278pub mod rfc_6213_options_api {}
279
280/// Simulate Layer
281#[doc = include_str!("6678_simulate_layer.md")]
282pub mod rfc_6678_simulate_layer {}
```
