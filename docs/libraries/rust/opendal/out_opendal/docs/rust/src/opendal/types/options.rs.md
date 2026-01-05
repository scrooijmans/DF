# 

opendal/types/

options.rs

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
18//! Options module provides options definitions for operations.
19
20use crate::raw::{BytesRange, Timestamp};
21use std::collections::HashMap;
22
23/// Options for delete operations.
24#[derive(Debug, Clone, Default, Eq, PartialEq)]
25pub struct DeleteOptions {
26    /// The version of the file to delete.
27    pub version: Option<String>,
28}
29
30/// Options for list operations.
31
32#[derive(Debug, Clone, Default, Eq, PartialEq)]
33pub struct ListOptions {
34    /// The limit passed to underlying service to specify the max results
35    /// that could return per-request.
36    ///
37    /// Users could use this to control the memory usage of list operation.
38    pub limit: Option<usize>,
39    /// The start_after passes to underlying service to specify the specified key
40    /// to start listing from.
41    pub start_after: Option<String>,
42    /// The recursive is used to control whether the list operation is recursive.
43    ///
44    /// - If `false`, list operation will only list the entries under the given path.
45    /// - If `true`, list operation will list all entries that starts with given path.
46    ///
47    /// Default to `false`.
48    pub recursive: bool,
49    /// The version is used to control whether the object versions should be returned.
50    ///
51    /// - If `false`, list operation will not return with object versions
52    /// - If `true`, list operation will return with object versions if object versioning is supported
53    ///   by the underlying service
54    ///
55    /// Default to `false`
56    pub versions: bool,
57    /// The deleted is used to control whether the deleted objects should be returned.
58    ///
59    /// - If `false`, list operation will not return with deleted objects
60    /// - If `true`, list operation will return with deleted objects if object versioning is supported
61    ///   by the underlying service
62    ///
63    /// Default to `false`
64    pub deleted: bool,
65}
66
67/// Options for read operations.
68#[derive(Debug, Clone, Default, Eq, PartialEq)]
69pub struct ReadOptions {
70    /// Set `range` for this operation.
71    ///
72    /// If we have a file with size `n`.
73    ///
74    /// - `..` means read bytes in range `[0, n)` of file.
75    /// - `0..1024` and `..1024` means read bytes in range `[0, 1024)` of file
76    /// - `1024..` means read bytes in range `[1024, n)` of file
77    ///
78    /// The type implements `From<RangeBounds<u64>>`, so users can use `(1024..).into()` instead.
79    pub range: BytesRange,
80    /// Set `version` for this operation.
81    ///
82    /// This option can be used to retrieve the data of a specified version of the given path.
83    ///
84    /// If the version doesn't exist, an error with kind [`ErrorKind::NotFound`] will be returned.
85    pub version: Option<String>,
86
87    /// Set `if_match` for this operation.
88    ///
89    /// This option can be used to check if the file's `ETag` matches the given `ETag`.
90    ///
91    /// If file exists and it's etag doesn't match, an error with kind [`ErrorKind::ConditionNotMatch`]
92    /// will be returned.
93    pub if_match: Option<String>,
94    /// Set `if_none_match` for this operation.
95    ///
96    /// This option can be used to check if the file's `ETag` doesn't match the given `ETag`.
97    ///
98    /// If file exists and it's etag match, an error with kind [`ErrorKind::ConditionNotMatch`]
99    /// will be returned.
100    pub if_none_match: Option<String>,
101    /// Set `if_modified_since` for this operation.
102    ///
103    /// This option can be used to check if the file has been modified since the given timestamp.
104    ///
105    /// If file exists and it hasn't been modified since the specified time, an error with kind
106    /// [`ErrorKind::ConditionNotMatch`] will be returned.
107    pub if_modified_since: Option<Timestamp>,
108    /// Set `if_unmodified_since` for this operation.
109    ///
110    /// This feature can be used to check if the file hasn't been modified since the given timestamp.
111    ///
112    /// If file exists and it has been modified since the specified time, an error with kind
113    /// [`ErrorKind::ConditionNotMatch`] will be returned.
114    pub if_unmodified_since: Option<Timestamp>,
115
116    /// Set `concurrent` for the operation.
117    ///
118    /// OpenDAL by default to read file without concurrent. This is not efficient for cases when users
119    /// read large chunks of data. By setting `concurrent`, opendal will reading files concurrently
120    /// on support storage services.
121    ///
122    /// By setting `concurrent`, opendal will fetch chunks concurrently with
123    /// the give chunk size.
124    ///
125    /// Refer to [`crate::docs::performance`] for more details.
126    pub concurrent: usize,
127    /// Set `chunk` for the operation.
128    ///
129    /// OpenDAL will use services' preferred chunk size by default. Users can set chunk based on their own needs.
130    ///
131    /// Refer to [`crate::docs::performance`] for more details.
132    pub chunk: Option<usize>,
133    /// Controls the optimization strategy for range reads in [`Reader::fetch`].
134    ///
135    /// When performing range reads, if the gap between two requested ranges is smaller than
136    /// the configured `gap` size, OpenDAL will merge these ranges into a single read request
137    /// and discard the unrequested data in between. This helps reduce the number of API calls
138    /// to remote storage services.
139    ///
140    /// This optimization is particularly useful when performing multiple small range reads
141    /// that are close to each other, as it reduces the overhead of multiple network requests
142    /// at the cost of transferring some additional data.
143    ///
144    /// Refer to [`crate::docs::performance`] for more details.
145    pub gap: Option<usize>,
146
147    /// Specify the content-type header that should be sent back by the operation.
148    ///
149    /// This option is only meaningful when used along with presign.
150    pub override_content_type: Option<String>,
151    /// Specify the `cache-control` header that should be sent back by the operation.
152    ///
153    /// This option is only meaningful when used along with presign.
154    pub override_cache_control: Option<String>,
155    /// Specify the `content-disposition` header that should be sent back by the operation.
156    ///
157    /// This option is only meaningful when used along with presign.
158    pub override_content_disposition: Option<String>,
159}
160
161/// Options for reader operations.
162#[derive(Debug, Clone, Default, Eq, PartialEq)]
163pub struct ReaderOptions {
164    /// Set `version` for this operation.
165    ///
166    /// This option can be used to retrieve the data of a specified version of the given path.
167    ///
168    /// If the version doesn't exist, an error with kind [`ErrorKind::NotFound`] will be returned.
169    pub version: Option<String>,
170
171    /// Set `if_match` for this operation.
172    ///
173    /// This option can be used to check if the file's `ETag` matches the given `ETag`.
174    ///
175    /// If file exists and it's etag doesn't match, an error with kind [`ErrorKind::ConditionNotMatch`]
176    /// will be returned.
177    pub if_match: Option<String>,
178    /// Set `if_none_match` for this operation.
179    ///
180    /// This option can be used to check if the file's `ETag` doesn't match the given `ETag`.
181    ///
182    /// If file exists and it's etag match, an error with kind [`ErrorKind::ConditionNotMatch`]
183    /// will be returned.
184    pub if_none_match: Option<String>,
185    /// Set `if_modified_since` for this operation.
186    ///
187    /// This option can be used to check if the file has been modified since the given timestamp.
188    ///
189    /// If file exists and it hasn't been modified since the specified time, an error with kind
190    /// [`ErrorKind::ConditionNotMatch`] will be returned.
191    pub if_modified_since: Option<Timestamp>,
192    /// Set `if_unmodified_since` for this operation.
193    ///
194    /// This feature can be used to check if the file hasn't been modified since the given timestamp.
195    ///
196    /// If file exists and it has been modified since the specified time, an error with kind
197    /// [`ErrorKind::ConditionNotMatch`] will be returned.
198    pub if_unmodified_since: Option<Timestamp>,
199
200    /// Set `concurrent` for the operation.
201    ///
202    /// OpenDAL by default to read file without concurrent. This is not efficient for cases when users
203    /// read large chunks of data. By setting `concurrent`, opendal will reading files concurrently
204    /// on support storage services.
205    ///
206    /// By setting `concurrent`, opendal will fetch chunks concurrently with
207    /// the give chunk size.
208    ///
209    /// Refer to [`crate::docs::performance`] for more details.
210    pub concurrent: usize,
211    /// Set `chunk` for the operation.
212    ///
213    /// OpenDAL will use services' preferred chunk size by default. Users can set chunk based on their own needs.
214    ///
215    /// Refer to [`crate::docs::performance`] for more details.
216    pub chunk: Option<usize>,
217    /// Controls the optimization strategy for range reads in [`Reader::fetch`].
218    ///
219    /// When performing range reads, if the gap between two requested ranges is smaller than
220    /// the configured `gap` size, OpenDAL will merge these ranges into a single read request
221    /// and discard the unrequested data in between. This helps reduce the number of API calls
222    /// to remote storage services.
223    ///
224    /// This optimization is particularly useful when performing multiple small range reads
225    /// that are close to each other, as it reduces the overhead of multiple network requests
226    /// at the cost of transferring some additional data.
227    ///
228    /// Refer to [`crate::docs::performance`] for more details.
229    pub gap: Option<usize>,
230    /// Controls the number of prefetched bytes ranges that can be buffered in memory
231    /// during concurrent reading.
232    ///
233    /// When performing concurrent reads with `Reader`, this option limits how many
234    /// completed-but-not-yet-read chunks can be buffered. Once the number of buffered
235    /// chunks reaches this limit, no new read tasks will be spawned until some of the
236    /// buffered chunks are consumed.
237    ///
238    /// - Default value: 0 (no prefetching, strict back-pressure control)
239    /// - Set to a higher value to allow more aggressive prefetching at the cost of memory
240    ///
241    /// This option helps prevent memory exhaustion when reading large files with high
242    /// concurrency settings.
243    pub prefetch: usize,
244}
245
246/// Options for stat operations.
247#[derive(Debug, Clone, Default, Eq, PartialEq)]
248pub struct StatOptions {
249    /// Set `version` for this operation.
250    ///
251    /// This options can be used to retrieve the data of a specified version of the given path.
252    ///
253    /// If the version doesn't exist, an error with kind [`ErrorKind::NotFound`] will be returned.
254    pub version: Option<String>,
255
256    /// Set `if_match` for this operation.
257    ///
258    /// This option can be used to check if the file's `ETag` matches the given `ETag`.
259    ///
260    /// If file exists and it's etag doesn't match, an error with kind [`ErrorKind::ConditionNotMatch`]
261    /// will be returned.
262    pub if_match: Option<String>,
263    /// Set `if_none_match` for this operation.
264    ///
265    /// This option can be used to check if the file's `ETag` doesn't match the given `ETag`.
266    ///
267    /// If file exists and it's etag match, an error with kind [`ErrorKind::ConditionNotMatch`]
268    /// will be returned.
269    pub if_none_match: Option<String>,
270    /// Set `if_modified_since` for this operation.
271    ///
272    /// This option can be used to check if the file has been modified since the given timestamp.
273    ///
274    /// If file exists and it hasn't been modified since the specified time, an error with kind
275    /// [`ErrorKind::ConditionNotMatch`] will be returned.
276    pub if_modified_since: Option<Timestamp>,
277    /// Set `if_unmodified_since` for this operation.
278    ///
279    /// This feature can be used to check if the file hasn't been modified since the given timestamp.
280    ///
281    /// If file exists and it has been modified since the specified time, an error with kind
282    /// [`ErrorKind::ConditionNotMatch`] will be returned.
283    pub if_unmodified_since: Option<Timestamp>,
284
285    /// Specify the content-type header that should be sent back by the operation.
286    ///
287    /// This option is only meaningful when used along with presign.
288    pub override_content_type: Option<String>,
289    /// Specify the `cache-control` header that should be sent back by the operation.
290    ///
291    /// This option is only meaningful when used along with presign.
292    pub override_cache_control: Option<String>,
293    /// Specify the `content-disposition` header that should be sent back by the operation.
294    ///
295    /// This option is only meaningful when used along with presign.
296    pub override_content_disposition: Option<String>,
297}
298
299/// Options for write operations.
300#[derive(Debug, Clone, Default, Eq, PartialEq)]
301pub struct WriteOptions {
302    /// Sets append mode for this operation.
303    ///
304    /// ### Capability
305    ///
306    /// Check [`Capability::write_can_append`] before using this option.
307    ///
308    /// ### Behavior
309    ///
310    /// - By default, write operations overwrite existing files
311    /// - When append is set to true:
312    ///   - New data will be appended to the end of existing file
313    ///   - If file doesn't exist, it will be created
314    /// - If not supported, will return an error
315    ///
316    /// This operation allows adding data to existing files instead of overwriting them.
317    pub append: bool,
318
319    /// Sets Cache-Control header for this write operation.
320    ///
321    /// ### Capability
322    ///
323    /// Check [`Capability::write_with_cache_control`] before using this feature.
324    ///
325    /// ### Behavior
326    ///
327    /// - If supported, sets Cache-Control as system metadata on the target file
328    /// - The value should follow HTTP Cache-Control header format
329    /// - If not supported, the value will be ignored
330    ///
331    /// This operation allows controlling caching behavior for the written content.
332    ///
333    /// ### Use Cases
334    ///
335    /// - Setting browser cache duration
336    /// - Configuring CDN behavior
337    /// - Optimizing content delivery
338    /// - Managing cache invalidation
339    ///
340    /// ### References
341    ///
342    /// - [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control)
343    /// - [RFC 7234 Section 5.2](https://tools.ietf.org/html/rfc7234#section-5.2)
344    pub cache_control: Option<String>,
345    /// Sets `Content-Type` header for this write operation.
346    ///
347    /// ## Capability
348    ///
349    /// Check [`Capability::write_with_content_type`] before using this feature.
350    ///
351    /// ### Behavior
352    ///
353    /// - If supported, sets Content-Type as system metadata on the target file
354    /// - The value should follow MIME type format (e.g. "text/plain", "image/jpeg")
355    /// - If not supported, the value will be ignored
356    ///
357    /// This operation allows specifying the media type of the content being written.
358    pub content_type: Option<String>,
359    /// Sets Content-Disposition header for this write request.
360    ///
361    /// ### Capability
362    ///
363    /// Check [`Capability::write_with_content_disposition`] before using this feature.
364    ///
365    /// ### Behavior
366    ///
367    /// - If supported, sets Content-Disposition as system metadata on the target file
368    /// - The value should follow HTTP Content-Disposition header format
369    /// - Common values include:
370    ///   - `inline` - Content displayed within browser
371    ///   - `attachment` - Content downloaded as file
372    ///   - `attachment; filename="example.jpg"` - Downloaded with specified filename
373    /// - If not supported, the value will be ignored
374    ///
375    /// This operation allows controlling how the content should be displayed or downloaded.
376    pub content_disposition: Option<String>,
377    /// Sets Content-Encoding header for this write request.
378    ///
379    /// ### Capability
380    ///
381    /// Check [`Capability::write_with_content_encoding`] before using this feature.
382    ///
383    /// ### Behavior
384    ///
385    /// - If supported, sets Content-Encoding as system metadata on the target file
386    /// - The value should follow HTTP Content-Encoding header format
387    /// - Common values include:
388    ///   - `gzip` - Content encoded using gzip compression
389    ///   - `deflate` - Content encoded using deflate compression
390    ///   - `br` - Content encoded using Brotli compression
391    ///   - `identity` - No encoding applied (default value)
392    /// - If not supported, the value will be ignored
393    ///
394    /// This operation allows specifying the encoding applied to the content being written.
395    pub content_encoding: Option<String>,
396    /// Sets user metadata for this write request.
397    ///
398    /// ### Capability
399    ///
400    /// Check [`Capability::write_with_user_metadata`] before using this feature.
401    ///
402    /// ### Behavior
403    ///
404    /// - If supported, the user metadata will be attached to the object during write
405    /// - Accepts key-value pairs where both key and value are strings
406    /// - Keys are case-insensitive in most services
407    /// - Services may have limitations for user metadata, for example:
408    ///   - Key length is typically limited (e.g., 1024 bytes)
409    ///   - Value length is typically limited (e.g., 4096 bytes)
410    ///   - Total metadata size might be limited
411    ///   - Some characters might be forbidden in keys
412    /// - If not supported, the metadata will be ignored
413    ///
414    /// User metadata provides a way to attach custom metadata to objects during write operations.
415    /// This metadata can be retrieved later when reading the object.
416    pub user_metadata: Option<HashMap<String, String>>,
417
418    /// Sets If-Match header for this write request.
419    ///
420    /// ### Capability
421    ///
422    /// Check [`Capability::write_with_if_match`] before using this feature.
423    ///
424    /// ### Behavior
425    ///
426    /// - If supported, the write operation will only succeed if the target's ETag matches the specified value
427    /// - The value should be a valid ETag string
428    /// - Common values include:
429    ///   - A specific ETag value like `"686897696a7c876b7e"`
430    ///   - `*` - Matches any existing resource
431    /// - If not supported, the value will be ignored
432    ///
433    /// This operation provides conditional write functionality based on ETag matching,
434    /// helping prevent unintended overwrites in concurrent scenarios.
435    pub if_match: Option<String>,
436    /// Sets If-None-Match header for this write request.
437    ///
438    /// Note: Certain services, like `s3`, support `if_not_exists` but not `if_none_match`.
439    /// Use `if_not_exists` if you only want to check whether a file exists.
440    ///
441    /// ### Capability
442    ///
443    /// Check [`Capability::write_with_if_none_match`] before using this feature.
444    ///
445    /// ### Behavior
446    ///
447    /// - If supported, the write operation will only succeed if the target's ETag does not match the specified value
448    /// - The value should be a valid ETag string
449    /// - Common values include:
450    ///   - A specific ETag value like `"686897696a7c876b7e"`
451    ///   - `*` - Matches if the resource does not exist
452    /// - If not supported, the value will be ignored
453    ///
454    /// This operation provides conditional write functionality based on ETag non-matching,
455    /// useful for preventing overwriting existing resources or ensuring unique writes.
456    pub if_none_match: Option<String>,
457    /// Sets the condition that write operation will succeed only if target does not exist.
458    ///
459    /// ### Capability
460    ///
461    /// Check [`Capability::write_with_if_not_exists`] before using this feature.
462    ///
463    /// ### Behavior
464    ///
465    /// - If supported, the write operation will only succeed if the target path does not exist
466    /// - Will return error if target already exists
467    /// - If not supported, the value will be ignored
468    ///
469    /// This operation provides a way to ensure write operations only create new resources
470    /// without overwriting existing ones, useful for implementing "create if not exists" logic.
471    pub if_not_exists: bool,
472
473    /// Sets concurrent write operations for this writer.
474    ///
475    /// ## Behavior
476    ///
477    /// - By default, OpenDAL writes files sequentially
478    /// - When concurrent is set:
479    ///   - Multiple write operations can execute in parallel
480    ///   - Write operations return immediately without waiting if tasks space are available
481    ///   - Close operation ensures all writes complete in order
482    ///   - Memory usage increases with concurrency level
483    /// - If not supported, falls back to sequential writes
484    ///
485    /// This feature significantly improves performance when:
486    /// - Writing large files
487    /// - Network latency is high
488    /// - Storage service supports concurrent uploads like multipart uploads
489    ///
490    /// ## Performance Impact
491    ///
492    /// Setting appropriate concurrency can:
493    /// - Increase write throughput
494    /// - Reduce total write time
495    /// - Better utilize available bandwidth
496    /// - Trade memory for performance
497    pub concurrent: usize,
498    /// Sets chunk size for buffered writes.
499    ///
500    /// ### Capability
501    ///
502    /// Check [`Capability::write_multi_min_size`] and [`Capability::write_multi_max_size`] for size limits.
503    ///
504    /// ### Behavior
505    ///
506    /// - By default, OpenDAL sets optimal chunk size based on service capabilities
507    /// - When chunk size is set:
508    ///   - Data will be buffered until reaching chunk size
509    ///   - One API call will be made per chunk
510    ///   - Last chunk may be smaller than chunk size
511    /// - Important considerations:
512    ///   - Some services require minimum chunk sizes (e.g. S3's EntityTooSmall error)
513    ///   - Smaller chunks increase API calls and costs
514    ///   - Larger chunks increase memory usage, but improve performance and reduce costs
515    ///
516    /// ### Performance Impact
517    ///
518    /// Setting appropriate chunk size can:
519    /// - Reduce number of API calls
520    /// - Improve overall throughput
521    /// - Lower operation costs
522    /// - Better utilize network bandwidth
523    pub chunk: Option<usize>,
524}
525
526/// Options for copy operations.
527#[derive(Debug, Clone, Default, Eq, PartialEq)]
528pub struct CopyOptions {
529    /// Sets the condition that copy operation will succeed only if target does not exist.
530    ///
531    /// ### Capability
532    ///
533    /// Check [`Capability::copy_with_if_not_exists`] before using this feature.
534    ///
535    /// ### Behavior
536    ///
537    /// - If supported, the copy operation will only succeed if the target path does not exist
538    /// - Will return error if target already exists
539    /// - If not supported, the value will be ignored
540    ///
541    /// This operation provides a way to ensure copy operations only create new resources
542    /// without overwriting existing ones, useful for implementing "copy if not exists" logic.
543    pub if_not_exists: bool,
544}
```
