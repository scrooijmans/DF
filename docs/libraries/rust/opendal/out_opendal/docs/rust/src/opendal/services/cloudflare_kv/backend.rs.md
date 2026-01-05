# 

opendal/services/cloudflare_kv/

backend.rs

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
18use std::fmt::Debug;
19use std::fmt::Formatter;
20use std::sync::Arc;
21use std::time::Duration;
22
23use super::CLOUDFLARE_KV_SCHEME;
24use crate::ErrorKind;
25use crate::raw::*;
26use crate::services::CloudflareKvConfig;
27use crate::services::cloudflare_kv::core::CloudflareKvCore;
28use crate::services::cloudflare_kv::delete::CloudflareKvDeleter;
29use crate::services::cloudflare_kv::error::parse_error;
30use crate::services::cloudflare_kv::lister::CloudflareKvLister;
31use crate::services::cloudflare_kv::model::*;
32use crate::services::cloudflare_kv::writer::CloudflareWriter;
33use crate::*;
34use bytes::Buf;
35use http::StatusCode;
36
37#[doc = include_str!("docs.md")]
38#[derive(Default)]
39pub struct CloudflareKvBuilder {
40    pub(super) config: CloudflareKvConfig,
41
42    /// The HTTP client used to communicate with CloudFlare.
43    pub(super) http_client: Option<HttpClient>,
44}
45
46impl Debug for CloudflareKvBuilder {
47    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
48        f.debug_struct("CloudFlareKvBuilder")
49            .field("config", &self.config)
50            .finish()
51    }
52}
53
54impl CloudflareKvBuilder {
55    /// Set the token used to authenticate with CloudFlare.
56    pub fn api_token(mut self, api_token: &str) -> Self {
57        if !api_token.is_empty() {
58            self.config.api_token = Some(api_token.to_string())
59        }
60        self
61    }
62
63    /// Set the account ID used to authenticate with CloudFlare.
64    pub fn account_id(mut self, account_id: &str) -> Self {
65        if !account_id.is_empty() {
66            self.config.account_id = Some(account_id.to_string())
67        }
68        self
69    }
70
71    /// Set the namespace ID.
72    pub fn namespace_id(mut self, namespace_id: &str) -> Self {
73        if !namespace_id.is_empty() {
74            self.config.namespace_id = Some(namespace_id.to_string())
75        }
76        self
77    }
78
79    /// Set the default ttl for cloudflare_kv services.
80    ///
81    /// If set, we will specify `EX` for write operations.
82    pub fn default_ttl(mut self, ttl: Duration) -> Self {
83        self.config.default_ttl = Some(ttl);
84        self
85    }
86
87    /// Set the root within this backend.
88    pub fn root(mut self, root: &str) -> Self {
89        self.config.root = if root.is_empty() {
90            None
91        } else {
92            Some(root.to_string())
93        };
94
95        self
96    }
97}
98
99impl Builder for CloudflareKvBuilder {
100    type Config = CloudflareKvConfig;
101
102    fn build(self) -> Result<impl Access> {
103        let api_token = match &self.config.api_token {
104            Some(api_token) => format_authorization_by_bearer(api_token)?,
105            None => {
106                return Err(Error::new(
107                    ErrorKind::ConfigInvalid,
108                    "api_token is required",
109                ));
110            }
111        };
112
113        let Some(account_id) = self.config.account_id.clone() else {
114            return Err(Error::new(
115                ErrorKind::ConfigInvalid,
116                "account_id is required",
117            ));
118        };
119
120        let Some(namespace_id) = self.config.namespace_id.clone() else {
121            return Err(Error::new(
122                ErrorKind::ConfigInvalid,
123                "namespace_id is required",
124            ));
125        };
126
127        // Validate default TTL is at least 60 seconds if specified
128        if let Some(ttl) = self.config.default_ttl {
129            if ttl < Duration::from_secs(60) {
130                return Err(Error::new(
131                    ErrorKind::ConfigInvalid,
132                    "Default TTL must be at least 60 seconds",
133                ));
134            }
135        }
136
137        let root = normalize_root(
138            self.config
139                .root
140                .clone()
141                .unwrap_or_else(|| "/".to_string())
142                .as_str(),
143        );
144
145        Ok(CloudflareKvAccessor {
146            core: Arc::new(CloudflareKvCore {
147                api_token,
148                account_id,
149                namespace_id,
150                expiration_ttl: self.config.default_ttl,
151                info: {
152                    let am = AccessorInfo::default();
153                    am.set_scheme(CLOUDFLARE_KV_SCHEME)
154                        .set_root(&root)
155                        .set_native_capability(Capability {
156                            create_dir: true,
157
158                            stat: true,
159                            stat_with_if_match: true,
160                            stat_with_if_none_match: true,
161                            stat_with_if_modified_since: true,
162                            stat_with_if_unmodified_since: true,
163
164                            read: true,
165                            read_with_if_match: true,
166                            read_with_if_none_match: true,
167                            read_with_if_modified_since: true,
168                            read_with_if_unmodified_since: true,
169
170                            write: true,
171                            write_can_empty: true,
172                            write_total_max_size: Some(25 * 1024 * 1024),
173
174                            list: true,
175                            list_with_limit: true,
176                            list_with_recursive: true,
177
178                            delete: true,
179                            delete_max_size: Some(10000),
180
181                            shared: false,
182
183                            ..Default::default()
184                        });
185
186                    // allow deprecated api here for compatibility
187                    #[allow(deprecated)]
188                    if let Some(client) = self.http_client {
189                        am.update_http_client(|_| client);
190                    }
191
192                    am.into()
193                },
194            }),
195        })
196    }
197}
198
199#[derive(Debug, Clone)]
200pub struct CloudflareKvAccessor {
201    core: std::sync::Arc<CloudflareKvCore>,
202}
203
204impl Access for CloudflareKvAccessor {
205    type Reader = Buffer;
206    type Writer = oio::OneShotWriter<CloudflareWriter>;
207    type Lister = oio::PageLister<CloudflareKvLister>;
208    type Deleter = oio::BatchDeleter<CloudflareKvDeleter>;
209
210    fn info(&self) -> std::sync::Arc<AccessorInfo> {
211        self.core.info.clone()
212    }
213
214    async fn create_dir(&self, path: &str, _args: OpCreateDir) -> Result<RpCreateDir> {
215        let path = build_abs_path(&self.core.info.root(), path);
216
217        if path == build_abs_path(&self.core.info.root(), "") {
218            return Ok(RpCreateDir::default());
219        }
220
221        // Split path into segments and create directories for each level
222        let segments: Vec<&str> = path
223            .trim_start_matches('/')
224            .trim_end_matches('/')
225            .split('/')
226            .collect();
227
228        // Create each directory level
229        let mut current_path = String::from("/");
230        for segment in segments {
231            // Build the current directory path
232            if !current_path.ends_with('/') {
233                current_path.push('/');
234            }
235            current_path.push_str(segment);
236            current_path.push('/');
237
238            // Create metadata for current directory
239            let cf_kv_metadata = CfKvMetadata {
240                etag: build_tmp_path_of(&current_path),
241                last_modified: Timestamp::now().to_string(),
242                content_length: 0,
243                is_dir: true,
244            };
245
246            // Set the directory entry
247            self.core
248                .set(&current_path, Buffer::new(), cf_kv_metadata)
249                .await?;
250        }
251
252        Ok(RpCreateDir::default())
253    }
254
255    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
256        let path = build_abs_path(&self.core.info.root(), path);
257        let new_path = path.trim_end_matches('/');
258
259        let resp = self.core.metadata(new_path).await?;
260
261        // Handle non-OK response
262        if resp.status() != StatusCode::OK {
263            // Special handling for potential directory paths
264            if path.ends_with('/') && resp.status() == StatusCode::NOT_FOUND {
265                // Try listing the path to check if it's a directory
266                let list_resp = self.core.list(&path, None, None).await?;
267
268                if list_resp.status() == StatusCode::OK {
269                    let list_body = list_resp.into_body();
270                    let list_result: CfKvListResponse = serde_json::from_reader(list_body.reader())
271                        .map_err(new_json_deserialize_error)?;
272
273                    // If listing returns results, treat as directory
274                    if let Some(entries) = list_result.result {
275                        if !entries.is_empty() {
276                            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
277                        }
278                    }
279
280                    // Empty or no results means not found
281                    return Err(Error::new(
282                        ErrorKind::NotFound,
283                        "key not found in CloudFlare KV",
284                    ));
285                }
286            }
287
288            // For all other error cases, parse the error response
289            return Err(parse_error(resp));
290        }
291
292        let resp_body = resp.into_body();
293        let cf_response: CfKvStatResponse =
294            serde_json::from_reader(resp_body.reader()).map_err(new_json_deserialize_error)?;
295
296        if !cf_response.success {
297            return Err(Error::new(
298                ErrorKind::Unexpected,
299                "cloudflare_kv stat this key failed for reason we don't know",
300            ));
301        }
302
303        let metadata = match cf_response.result {
304            Some(metadata) => {
305                if path.ends_with('/') && !metadata.is_dir {
306                    return Err(Error::new(
307                        ErrorKind::NotFound,
308                        "key not found in CloudFlare KV",
309                    ));
310                } else {
311                    metadata
312                }
313            }
314            None => {
315                return Err(Error::new(
316                    ErrorKind::NotFound,
317                    "key not found in CloudFlare KV",
318                ));
319            }
320        };
321
322        // Check if_match condition
323        if let Some(if_match) = &args.if_match() {
324            if if_match != &metadata.etag {
325                return Err(Error::new(ErrorKind::ConditionNotMatch, "etag mismatch"));
326            }
327        }
328
329        // Check if_none_match condition
330        if let Some(if_none_match) = &args.if_none_match() {
331            if if_none_match == &metadata.etag {
332                return Err(Error::new(
333                    ErrorKind::ConditionNotMatch,
334                    "etag match when expected none match",
335                ));
336            }
337        }
338
339        // Parse since time once for both time-based conditions
340        let last_modified = metadata
341            .last_modified
342            .parse::<Timestamp>()
343            .map_err(|_| Error::new(ErrorKind::Unsupported, "invalid since format"))?;
344
345        // Check modified_since condition
346        if let Some(modified_since) = &args.if_modified_since() {
347            if !last_modified.gt(modified_since) {
348                return Err(Error::new(
349                    ErrorKind::ConditionNotMatch,
350                    "not modified since specified time",
351                ));
352            }
353        }
354
355        // Check unmodified_since condition
356        if let Some(unmodified_since) = &args.if_unmodified_since() {
357            if !last_modified.le(unmodified_since) {
358                return Err(Error::new(
359                    ErrorKind::ConditionNotMatch,
360                    "modified since specified time",
361                ));
362            }
363        }
364
365        let meta = Metadata::new(if metadata.is_dir {
366            EntryMode::DIR
367        } else {
368            EntryMode::FILE
369        })
370        .with_etag(metadata.etag)
371        .with_content_length(metadata.content_length as u64)
372        .with_last_modified(metadata.last_modified.parse::<Timestamp>()?);
373
374        Ok(RpStat::new(meta))
375    }
376
377    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
378        let path = build_abs_path(&self.core.info.root(), path);
379        let resp = self.core.get(&path).await?;
380
381        let status = resp.status();
382
383        if status != StatusCode::OK {
384            return Err(parse_error(resp));
385        }
386
387        let resp_body = resp.into_body();
388
389        if args.if_match().is_some()
390            || args.if_none_match().is_some()
391            || args.if_modified_since().is_some()
392            || args.if_unmodified_since().is_some()
393        {
394            let meta_resp = self.core.metadata(&path).await?;
395
396            if meta_resp.status() != StatusCode::OK {
397                return Err(parse_error(meta_resp));
398            }
399
400            let cf_response: CfKvStatResponse =
401                serde_json::from_reader(meta_resp.into_body().reader())
402                    .map_err(new_json_deserialize_error)?;
403
404            if !cf_response.success && cf_response.result.is_some() {
405                return Err(Error::new(
406                    ErrorKind::Unexpected,
407                    "cloudflare_kv read this key failed for reason we don't know",
408                ));
409            }
410
411            let metadata = cf_response.result.unwrap();
412
413            // Check if_match condition
414            if let Some(if_match) = &args.if_match() {
415                if if_match != &metadata.etag {
416                    return Err(Error::new(ErrorKind::ConditionNotMatch, "etag mismatch"));
417                }
418            }
419
420            // Check if_none_match condition
421            if let Some(if_none_match) = &args.if_none_match() {
422                if if_none_match == &metadata.etag {
423                    return Err(Error::new(
424                        ErrorKind::ConditionNotMatch,
425                        "etag match when expected none match",
426                    ));
427                }
428            }
429
430            // Parse since time once for both time-based conditions
431            let last_modified = metadata
432                .last_modified
433                .parse::<Timestamp>()
434                .map_err(|_| Error::new(ErrorKind::Unsupported, "invalid since format"))?;
435
436            // Check modified_since condition
437            if let Some(modified_since) = &args.if_modified_since() {
438                if !last_modified.gt(modified_since) {
439                    return Err(Error::new(
440                        ErrorKind::ConditionNotMatch,
441                        "not modified since specified time",
442                    ));
443                }
444            }
445
446            // Check unmodified_since condition
447            if let Some(unmodified_since) = &args.if_unmodified_since() {
448                if !last_modified.le(unmodified_since) {
449                    return Err(Error::new(
450                        ErrorKind::ConditionNotMatch,
451                        "modified since specified time",
452                    ));
453                }
454            }
455        }
456
457        let range = args.range();
458        let buffer = if range.is_full() {
459            resp_body
460        } else {
461            let start = range.offset() as usize;
462            let end = match range.size() {
463                Some(size) => (range.offset() + size) as usize,
464                None => resp_body.len(),
465            };
466            resp_body.slice(start..end.min(resp_body.len()))
467        };
468        Ok((RpRead::new(), buffer))
469    }
470
471    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
472        let path = build_abs_path(&self.core.info.root(), path);
473        let writer = CloudflareWriter::new(self.core.clone(), path);
474
475        let w = oio::OneShotWriter::new(writer);
476
477        Ok((RpWrite::default(), w))
478    }
479
480    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
481        Ok((
482            RpDelete::default(),
483            oio::BatchDeleter::new(CloudflareKvDeleter::new(self.core.clone())),
484        ))
485    }
486
487    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
488        let path = build_abs_path(&self.core.info.root(), path);
489
490        let limit = match args.limit() {
491            Some(limit) => {
492                // The list limit of cloudflare_kv is limited to 10..1000.
493                if !(10..=1000).contains(&limit) {
494                    1000
495                } else {
496                    limit
497                }
498            }
499            None => 1000,
500        };
501
502        let l = CloudflareKvLister::new(self.core.clone(), &path, args.recursive(), Some(limit));
503
504        Ok((RpList::default(), oio::PageLister::new(l)))
505    }
506}
```
