# 

opendal/types/

metadata.rs

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
18use crate::raw::*;
19use crate::*;
20use std::collections::HashMap;
21
22/// Metadata contains all the information related to a specific path.
23///
24/// Depending on the context of the requests, the metadata for the same path may vary. For example, two
25/// versions of the same path might have different content lengths. Keep in mind that metadata is always
26/// tied to the given context and is not a global state.
27///
28/// ## File Versions
29///
30/// In systems that support versioning, such as AWS S3, the metadata may represent a specific version
31/// of a file.
32///
33/// Users can access [`Metadata::version`] to retrieve the file's version, if available. They can also
34/// use [`Metadata::is_current`] and [`Metadata::is_deleted`] to determine whether the metadata
35/// corresponds to the latest version or a deleted one.
36///
37/// The all possible combinations of `is_current` and `is_deleted` are as follows:
38///
39/// | `is_current`  | `is_deleted` | description                                                                                                                                                                                                                                                                                                                                                                          |
40/// |---------------|--------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
41/// | `Some(true)`  | `false`      | **The metadata's associated version is the latest, current version.** This is the normal state, indicating that this version is the most up-to-date and accessible version.                                                                                                                                                                                                          |
42/// | `Some(true)`  | `true`       | **The metadata's associated version is the latest, deleted version (Latest Delete Marker or Soft Deleted).** This is particularly important in object storage systems like S3. It signifies that this version is the **most recent delete marker**, indicating the object has been deleted. Subsequent GET requests will return 404 errors unless a specific version ID is provided. |
43/// | `Some(false)` | `false`      | **The metadata's associated version is neither the latest version nor deleted.** This indicates that this version is a previous version, still accessible by specifying its version ID.                                                                                                                                                                                              |
44/// | `Some(false)` | `true`       | **The metadata's associated version is not the latest version and is deleted.** This represents a historical version that has been marked for deletion. Users will need to specify the version ID to access it, and accessing it may be subject to specific delete marker behavior (e.g., in S3, it might not return actual data but a specific delete marker response).             |
45/// | `None`        | `false`      | **The metadata's associated file is not deleted, but its version status is either unknown or it is not the latest version.** This likely indicates that versioning is not enabled for this file, or versioning information is unavailable.                                                                                                                                           |
46/// | `None`        | `true`       | **The metadata's associated file is deleted, but its version status is either unknown or it is not the latest version.** This typically means the file was deleted without versioning enabled, or its versioning information is unavailable. This may represent an actual data deletion operation rather than an S3 delete marker.                                                   |
47#[derive(Debug, Clone, Eq, PartialEq, Default)]
48pub struct Metadata {
49    mode: EntryMode,
50
51    is_current: Option<bool>,
52    is_deleted: bool,
53
54    cache_control: Option<String>,
55    content_disposition: Option<String>,
56    content_length: Option<u64>,
57    content_md5: Option<String>,
58    content_range: Option<BytesContentRange>,
59    content_type: Option<String>,
60    content_encoding: Option<String>,
61    etag: Option<String>,
62    last_modified: Option<Timestamp>,
63    version: Option<String>,
64
65    user_metadata: Option<HashMap<String, String>>,
66}
67
68impl Metadata {
69    /// Create a new metadata
70    pub fn new(mode: EntryMode) -> Self {
71        Self {
72            mode,
73
74            is_current: None,
75            is_deleted: false,
76
77            cache_control: None,
78            content_length: None,
79            content_md5: None,
80            content_type: None,
81            content_encoding: None,
82            content_range: None,
83            last_modified: None,
84            etag: None,
85            content_disposition: None,
86            version: None,
87            user_metadata: None,
88        }
89    }
90
91    /// mode represent this entry's mode.
92    pub fn mode(&self) -> EntryMode {
93        self.mode
94    }
95
96    /// Set mode for entry.
97    pub fn set_mode(&mut self, v: EntryMode) -> &mut Self {
98        self.mode = v;
99        self
100    }
101
102    /// Set mode for entry.
103    pub fn with_mode(mut self, v: EntryMode) -> Self {
104        self.mode = v;
105        self
106    }
107
108    /// Returns `true` if this metadata is for a file.
109    pub fn is_file(&self) -> bool {
110        matches!(self.mode, EntryMode::FILE)
111    }
112
113    /// Returns `true` if this metadata is for a directory.
114    pub fn is_dir(&self) -> bool {
115        matches!(self.mode, EntryMode::DIR)
116    }
117
118    /// Checks whether the metadata corresponds to the most recent version of the file.
119    ///
120    /// This function is particularly useful when working with versioned objects,
121    /// such as those stored in systems like AWS S3 with versioning enabled. It helps
122    /// determine if the retrieved metadata represents the current state of the file
123    /// or an older version.
124    ///
125    /// Refer to docs in [`Metadata`] for more information about file versions.
126    ///
127    /// # Return Value
128    ///
129    /// The function returns an `Option<bool>` which can have the following values:
130    ///
131    /// - `Some(true)`:  Indicates that the metadata **is** associated with the latest version of the file.
132    ///   The metadata is current and reflects the most up-to-date state.
133    /// - `Some(false)`: Indicates that the metadata **is not** associated with the latest version of the file.
134    ///   The metadata belongs to an older version, and there might be a more recent version available.
135    /// - `None`:      Indicates that the currency of the metadata **cannot be determined**. This might occur if
136    ///   versioning is not supported or enabled, or if there is insufficient information to ascertain the version status.
137    pub fn is_current(&self) -> Option<bool> {
138        self.is_current
139    }
140
141    /// Set the `is_current` status of this entry.
142    ///
143    /// By default, this value will be `None`. Please avoid using this API if it's unclear whether the entry is current.
144    /// Set it to `true` if it is known to be the latest; otherwise, set it to `false`.
145    pub fn set_is_current(&mut self, is_current: bool) -> &mut Self {
146        self.is_current = Some(is_current);
147        self
148    }
149
150    /// Set the `is_current` status of this entry.
151    ///
152    /// By default, this value will be `None`. Please avoid using this API if it's unclear whether the entry is current.
153    /// Set it to `true` if it is known to be the latest; otherwise, set it to `false`.
154    pub fn with_is_current(mut self, is_current: Option<bool>) -> Self {
155        self.is_current = is_current;
156        self
157    }
158
159    /// Checks if the file (or version) associated with this metadata has been deleted.
160    ///
161    /// This function returns `true` if the file represented by this metadata has been marked for
162    /// deletion or has been permanently deleted.
163    /// It returns `false` otherwise, indicating that the file (or version) is still present and accessible.
164    ///
165    /// Refer to docs in [`Metadata`] for more information about file versions.
166    ///
167    /// # Returns
168    ///
169    /// `bool`: `true` if the object is considered deleted, `false` otherwise.
170    pub fn is_deleted(&self) -> bool {
171        self.is_deleted
172    }
173
174    /// Set the deleted status of this entry.
175    pub fn set_is_deleted(&mut self, v: bool) -> &mut Self {
176        self.is_deleted = v;
177        self
178    }
179
180    /// Set the deleted status of this entry.
181    pub fn with_is_deleted(mut self, v: bool) -> Self {
182        self.is_deleted = v;
183        self
184    }
185
186    /// Cache control of this entry.
187    ///
188    /// Cache-Control is defined by [RFC 7234](https://httpwg.org/specs/rfc7234.html#header.cache-control)
189    /// Refer to [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) for more information.
190    pub fn cache_control(&self) -> Option<&str> {
191        self.cache_control.as_deref()
192    }
193
194    /// Set cache control of this entry.
195    ///
196    /// Cache-Control is defined by [RFC 7234](https://httpwg.org/specs/rfc7234.html#header.cache-control)
197    /// Refer to [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) for more information.
198    pub fn set_cache_control(&mut self, v: &str) -> &mut Self {
199        self.cache_control = Some(v.to_string());
200        self
201    }
202
203    /// Set cache control of this entry.
204    ///
205    /// Cache-Control is defined by [RFC 7234](https://httpwg.org/specs/rfc7234.html#header.cache-control)
206    /// Refer to [MDN Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) for more information.
207    pub fn with_cache_control(mut self, v: String) -> Self {
208        self.cache_control = Some(v);
209        self
210    }
211
212    /// Content length of this entry.
213    ///
214    /// `Content-Length` is defined by [RFC 7230](https://httpwg.org/specs/rfc7230.html#header.content-length)
215    ///
216    /// Refer to [MDN Content-Length](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Length) for more information.
217    ///
218    /// # Returns
219    ///
220    /// Content length of this entry. It will be `0` if the content length is not set by the storage services.
221    pub fn content_length(&self) -> u64 {
222        self.content_length.unwrap_or_default()
223    }
224
225    /// Set content length of this entry.
226    pub fn set_content_length(&mut self, v: u64) -> &mut Self {
227        self.content_length = Some(v);
228        self
229    }
230
231    /// Set content length of this entry.
232    pub fn with_content_length(mut self, v: u64) -> Self {
233        self.content_length = Some(v);
234        self
235    }
236
237    /// Content MD5 of this entry.
238    ///
239    /// Content MD5 is defined by [RFC 2616](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html).
240    /// And removed by [RFC 7231](https://www.rfc-editor.org/rfc/rfc7231).
241    ///
242    /// OpenDAL will try its best to set this value, but not guarantee this value is the md5 of content.
243    pub fn content_md5(&self) -> Option<&str> {
244        self.content_md5.as_deref()
245    }
246
247    /// Set content MD5 of this entry.
248    pub fn set_content_md5(&mut self, v: &str) -> &mut Self {
249        self.content_md5 = Some(v.to_string());
250        self
251    }
252
253    /// Set content MD5 of this entry.
254    pub fn with_content_md5(mut self, v: String) -> Self {
255        self.content_md5 = Some(v);
256        self
257    }
258
259    /// Content Type of this entry.
260    ///
261    /// Content Type is defined by [RFC 9110](https://httpwg.org/specs/rfc9110.html#field.content-type).
262    ///
263    /// Refer to [MDN Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type) for more information.
264    pub fn content_type(&self) -> Option<&str> {
265        self.content_type.as_deref()
266    }
267
268    /// Set Content Type of this entry.
269    pub fn set_content_type(&mut self, v: &str) -> &mut Self {
270        self.content_type = Some(v.to_string());
271        self
272    }
273
274    /// Set Content Type of this entry.
275    pub fn with_content_type(mut self, v: String) -> Self {
276        self.content_type = Some(v);
277        self
278    }
279
280    /// Content Encoding of this entry.
281    ///
282    /// Content Encoding is defined by [RFC 7231](https://httpwg.org/specs/rfc7231.html#header.content-encoding)
283    ///
284    /// Refer to [MDN Content-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding) for more information.
285    pub fn content_encoding(&self) -> Option<&str> {
286        self.content_encoding.as_deref()
287    }
288
289    /// Set Content Encoding of this entry.
290    pub fn set_content_encoding(&mut self, v: &str) -> &mut Self {
291        self.content_encoding = Some(v.to_string());
292        self
293    }
294
295    /// Content Range of this entry.
296    ///
297    /// Content Range is defined by [RFC 9110](https://httpwg.org/specs/rfc9110.html#field.content-range).
298    ///
299    /// Refer to [MDN Content-Range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range) for more information.
300    pub fn content_range(&self) -> Option<BytesContentRange> {
301        self.content_range
302    }
303
304    /// Set Content Range of this entry.
305    pub fn set_content_range(&mut self, v: BytesContentRange) -> &mut Self {
306        self.content_range = Some(v);
307        self
308    }
309
310    /// Set Content Range of this entry.
311    pub fn with_content_range(mut self, v: BytesContentRange) -> Self {
312        self.content_range = Some(v);
313        self
314    }
315
316    /// Last modified of this entry.
317    ///
318    /// `Last-Modified` is defined by [RFC 7232](https://httpwg.org/specs/rfc7232.html#header.last-modified)
319    ///
320    /// Refer to [MDN Last-Modified](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Last-Modified) for more information.
321    pub fn last_modified(&self) -> Option<Timestamp> {
322        self.last_modified
323    }
324
325    /// Set Last modified of this entry.
326    pub fn set_last_modified(&mut self, v: Timestamp) -> &mut Self {
327        self.last_modified = Some(v);
328        self
329    }
330
331    /// Set Last modified of this entry.
332    pub fn with_last_modified(mut self, v: Timestamp) -> Self {
333        self.last_modified = Some(v);
334        self
335    }
336
337    /// ETag of this entry.
338    ///
339    /// `ETag` is defined by [RFC 7232](https://httpwg.org/specs/rfc7232.html#header.etag)
340    ///
341    /// Refer to [MDN ETag](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag) for more information.
342    ///
343    /// OpenDAL will return this value AS-IS like the following:
344    ///
345    /// - `"33a64df551425fcc55e4d42a148795d9f25f89d4"`
346    /// - `W/"0815"`
347    ///
348    /// `"` is part of etag.
349    pub fn etag(&self) -> Option<&str> {
350        self.etag.as_deref()
351    }
352
353    /// Set ETag of this entry.
354    pub fn set_etag(&mut self, v: &str) -> &mut Self {
355        self.etag = Some(v.to_string());
356        self
357    }
358
359    /// Set ETag of this entry.
360    pub fn with_etag(mut self, v: String) -> Self {
361        self.etag = Some(v);
362        self
363    }
364
365    /// Content-Disposition of this entry
366    ///
367    /// `Content-Disposition` is defined by [RFC 2616](https://www.rfc-editor/rfcs/2616) and
368    /// clarified usage in [RFC 6266](https://www.rfc-editor/6266).
369    ///
370    /// Refer to [MDN Content-Disposition](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition) for more information.
371    ///
372    /// OpenDAL will return this value AS-IS like the following:
373    ///
374    /// - "inline"
375    /// - "attachment"
376    /// - "attachment; filename=\"filename.jpg\""
377    pub fn content_disposition(&self) -> Option<&str> {
378        self.content_disposition.as_deref()
379    }
380
381    /// Set Content-Disposition of this entry
382    pub fn set_content_disposition(&mut self, v: &str) -> &mut Self {
383        self.content_disposition = Some(v.to_string());
384        self
385    }
386
387    /// Set Content-Disposition of this entry
388    pub fn with_content_disposition(mut self, v: String) -> Self {
389        self.content_disposition = Some(v);
390        self
391    }
392
393    /// Retrieves the `version` of the file, if available.
394    ///
395    /// The version is typically used in systems that support object versioning, such as AWS S3.
396    ///
397    /// # Returns
398    ///
399    /// - `Some(&str)`: If the file has a version associated with it,
400    ///   this function returns `Some` containing a reference to the version ID string.
401    /// - `None`: If the file does not have a version, or if versioning is
402    ///   not supported or enabled for the underlying storage system, this function
403    ///   returns `None`.
404    pub fn version(&self) -> Option<&str> {
405        self.version.as_deref()
406    }
407
408    /// Set the version of the file
409    pub fn set_version(&mut self, v: &str) -> &mut Self {
410        self.version = Some(v.to_string());
411        self
412    }
413
414    /// With the version of the file.
415    pub fn with_version(mut self, v: String) -> Self {
416        self.version = Some(v);
417        self
418    }
419
420    /// User defined metadata of this entry
421    ///
422    /// The prefix of the user defined metadata key(for example: in oss, it's x-oss-meta-)
423    /// is remove from the key
424    pub fn user_metadata(&self) -> Option<&HashMap<String, String>> {
425        self.user_metadata.as_ref()
426    }
427
428    /// With user defined metadata of this entry
429    pub fn with_user_metadata(mut self, data: HashMap<String, String>) -> Self {
430        self.user_metadata = Some(data);
431        self
432    }
433}
```
