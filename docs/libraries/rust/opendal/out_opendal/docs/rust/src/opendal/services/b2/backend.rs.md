# 

opendal/services/b2/

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
21
22use http::Request;
23use http::Response;
24use http::StatusCode;
25use log::debug;
26use tokio::sync::RwLock;
27
28use super::B2_SCHEME;
29use super::core::B2Core;
30use super::core::B2Signer;
31use super::core::constants;
32use super::core::parse_file_info;
33use super::delete::B2Deleter;
34use super::error::parse_error;
35use super::lister::B2Lister;
36use super::writer::B2Writer;
37use super::writer::B2Writers;
38use crate::raw::*;
39use crate::services::B2Config;
40use crate::*;
41
42/// [b2](https://www.backblaze.com/cloud-storage) services support.
43#[doc = include_str!("docs.md")]
44#[derive(Default)]
45pub struct B2Builder {
46    pub(super) config: B2Config,
47
48    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
49    pub(super) http_client: Option<HttpClient>,
50}
51
52impl Debug for B2Builder {
53    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
54        let mut d = f.debug_struct("B2Builder");
55
56        d.field("config", &self.config);
57        d.finish_non_exhaustive()
58    }
59}
60
61impl B2Builder {
62    /// Set root of this backend.
63    ///
64    /// All operations will happen under this root.
65    pub fn root(mut self, root: &str) -> Self {
66        self.config.root = if root.is_empty() {
67            None
68        } else {
69            Some(root.to_string())
70        };
71
72        self
73    }
74
75    /// application_key_id of this backend.
76    pub fn application_key_id(mut self, application_key_id: &str) -> Self {
77        self.config.application_key_id = if application_key_id.is_empty() {
78            None
79        } else {
80            Some(application_key_id.to_string())
81        };
82
83        self
84    }
85
86    /// application_key of this backend.
87    pub fn application_key(mut self, application_key: &str) -> Self {
88        self.config.application_key = if application_key.is_empty() {
89            None
90        } else {
91            Some(application_key.to_string())
92        };
93
94        self
95    }
96
97    /// Set bucket name of this backend.
98    /// You can find it in <https://secure.backblaze.com/b2_buckets.html>
99    pub fn bucket(mut self, bucket: &str) -> Self {
100        self.config.bucket = bucket.to_string();
101
102        self
103    }
104
105    /// Set bucket id of this backend.
106    /// You can find it in <https://secure.backblaze.com/b2_buckets.html>
107    pub fn bucket_id(mut self, bucket_id: &str) -> Self {
108        self.config.bucket_id = bucket_id.to_string();
109
110        self
111    }
112
113    /// Specify the http client that used by this service.
114    ///
115    /// # Notes
116    ///
117    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
118    /// during minor updates.
119    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
120    #[allow(deprecated)]
121    pub fn http_client(mut self, client: HttpClient) -> Self {
122        self.http_client = Some(client);
123        self
124    }
125}
126
127impl Builder for B2Builder {
128    type Config = B2Config;
129
130    /// Builds the backend and returns the result of B2Backend.
131    fn build(self) -> Result<impl Access> {
132        debug!("backend build started: {:?}", &self);
133
134        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
135        debug!("backend use root {}", &root);
136
137        // Handle bucket.
138        if self.config.bucket.is_empty() {
139            return Err(Error::new(ErrorKind::ConfigInvalid, "bucket is empty")
140                .with_operation("Builder::build")
141                .with_context("service", Scheme::B2));
142        }
143
144        debug!("backend use bucket {}", &self.config.bucket);
145
146        // Handle bucket_id.
147        if self.config.bucket_id.is_empty() {
148            return Err(Error::new(ErrorKind::ConfigInvalid, "bucket_id is empty")
149                .with_operation("Builder::build")
150                .with_context("service", Scheme::B2));
151        }
152
153        debug!("backend bucket_id {}", &self.config.bucket_id);
154
155        let application_key_id = match &self.config.application_key_id {
156            Some(application_key_id) => Ok(application_key_id.clone()),
157            None => Err(
158                Error::new(ErrorKind::ConfigInvalid, "application_key_id is empty")
159                    .with_operation("Builder::build")
160                    .with_context("service", Scheme::B2),
161            ),
162        }?;
163
164        let application_key = match &self.config.application_key {
165            Some(key_id) => Ok(key_id.clone()),
166            None => Err(
167                Error::new(ErrorKind::ConfigInvalid, "application_key is empty")
168                    .with_operation("Builder::build")
169                    .with_context("service", Scheme::B2),
170            ),
171        }?;
172
173        let signer = B2Signer {
174            application_key_id,
175            application_key,
176            ..Default::default()
177        };
178
179        Ok(B2Backend {
180            core: Arc::new(B2Core {
181                info: {
182                    let am = AccessorInfo::default();
183                    am.set_scheme(B2_SCHEME)
184                        .set_root(&root)
185                        .set_native_capability(Capability {
186                            stat: true,
187
188                            read: true,
189
190                            write: true,
191                            write_can_empty: true,
192                            write_can_multi: true,
193                            write_with_content_type: true,
194                            // The min multipart size of b2 is 5 MiB.
195                            //
196                            // ref: <https://www.backblaze.com/docs/cloud-storage-large-files>
197                            write_multi_min_size: Some(5 * 1024 * 1024),
198                            // The max multipart size of b2 is 5 Gb.
199                            //
200                            // ref: <https://www.backblaze.com/docs/cloud-storage-large-files>
201                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
202                                Some(5 * 1024 * 1024 * 1024)
203                            } else {
204                                Some(usize::MAX)
205                            },
206
207                            delete: true,
208                            copy: true,
209
210                            list: true,
211                            list_with_limit: true,
212                            list_with_start_after: true,
213                            list_with_recursive: true,
214
215                            presign: true,
216                            presign_read: true,
217                            presign_write: true,
218                            presign_stat: true,
219
220                            shared: true,
221
222                            ..Default::default()
223                        });
224
225                    // allow deprecated api here for compatibility
226                    #[allow(deprecated)]
227                    if let Some(client) = self.http_client {
228                        am.update_http_client(|_| client);
229                    }
230
231                    am.into()
232                },
233                signer: Arc::new(RwLock::new(signer)),
234                root,
235
236                bucket: self.config.bucket.clone(),
237                bucket_id: self.config.bucket_id.clone(),
238            }),
239        })
240    }
241}
242
243/// Backend for b2 services.
244#[derive(Debug, Clone)]
245pub struct B2Backend {
246    core: Arc<B2Core>,
247}
248
249impl Access for B2Backend {
250    type Reader = HttpBody;
251    type Writer = B2Writers;
252    type Lister = oio::PageLister<B2Lister>;
253    type Deleter = oio::OneShotDeleter<B2Deleter>;
254
255    fn info(&self) -> Arc<AccessorInfo> {
256        self.core.info.clone()
257    }
258
259    /// B2 have a get_file_info api required a file_id field, but field_id need call list api, list api also return file info
260    /// So we call list api to get file info
261    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
262        // Stat root always returns a DIR.
263        if path == "/" {
264            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
265        }
266
267        let delimiter = if path.ends_with('/') { Some("/") } else { None };
268
269        let file_info = self.core.get_file_info(path, delimiter).await?;
270        let meta = parse_file_info(&file_info);
271        Ok(RpStat::new(meta))
272    }
273
274    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
275        let resp = self
276            .core
277            .download_file_by_name(path, args.range(), &args)
278            .await?;
279
280        let status = resp.status();
281        match status {
282            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
283                Ok((RpRead::default(), resp.into_body()))
284            }
285            _ => {
286                let (part, mut body) = resp.into_parts();
287                let buf = body.to_buffer().await?;
288                Err(parse_error(Response::from_parts(part, buf)))
289            }
290        }
291    }
292
293    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
294        let concurrent = args.concurrent();
295        let writer = B2Writer::new(self.core.clone(), path, args);
296
297        let w = oio::MultipartWriter::new(self.core.info.clone(), writer, concurrent);
298
299        Ok((RpWrite::default(), w))
300    }
301
302    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
303        Ok((
304            RpDelete::default(),
305            oio::OneShotDeleter::new(B2Deleter::new(self.core.clone())),
306        ))
307    }
308
309    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
310        Ok((
311            RpList::default(),
312            oio::PageLister::new(B2Lister::new(
313                self.core.clone(),
314                path,
315                args.recursive(),
316                args.limit(),
317                args.start_after(),
318            )),
319        ))
320    }
321
322    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
323        let file_info = self.core.get_file_info(from, None).await?;
324
325        let source_file_id = file_info.file_id;
326
327        let Some(source_file_id) = source_file_id else {
328            return Err(Error::new(ErrorKind::IsADirectory, "is a directory"));
329        };
330
331        let resp = self.core.copy_file(source_file_id, to).await?;
332
333        let status = resp.status();
334
335        match status {
336            StatusCode::OK => Ok(RpCopy::default()),
337            _ => Err(parse_error(resp)),
338        }
339    }
340
341    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
342        match args.operation() {
343            PresignOperation::Stat(_) => {
344                let resp = self
345                    .core
346                    .get_download_authorization(path, args.expire())
347                    .await?;
348                let path = build_abs_path(&self.core.root, path);
349
350                let auth_info = self.core.get_auth_info().await?;
351
352                let url = format!(
353                    "{}/file/{}/{}?Authorization={}",
354                    auth_info.download_url, self.core.bucket, path, resp.authorization_token
355                );
356
357                let req = Request::get(url);
358
359                let req = req.body(Buffer::new()).map_err(new_request_build_error)?;
360
361                // We don't need this request anymore, consume
362                let (parts, _) = req.into_parts();
363
364                Ok(RpPresign::new(PresignedRequest::new(
365                    parts.method,
366                    parts.uri,
367                    parts.headers,
368                )))
369            }
370            PresignOperation::Read(_) => {
371                let resp = self
372                    .core
373                    .get_download_authorization(path, args.expire())
374                    .await?;
375                let path = build_abs_path(&self.core.root, path);
376
377                let auth_info = self.core.get_auth_info().await?;
378
379                let url = format!(
380                    "{}/file/{}/{}?Authorization={}",
381                    auth_info.download_url, self.core.bucket, path, resp.authorization_token
382                );
383
384                let req = Request::get(url);
385
386                let req = req.body(Buffer::new()).map_err(new_request_build_error)?;
387
388                // We don't need this request anymore, consume
389                let (parts, _) = req.into_parts();
390
391                Ok(RpPresign::new(PresignedRequest::new(
392                    parts.method,
393                    parts.uri,
394                    parts.headers,
395                )))
396            }
397            PresignOperation::Write(_) => {
398                let resp = self.core.get_upload_url().await?;
399
400                let mut req = Request::post(&resp.upload_url);
401
402                req = req.header(http::header::AUTHORIZATION, resp.authorization_token);
403                req = req.header("X-Bz-File-Name", build_abs_path(&self.core.root, path));
404                req = req.header(http::header::CONTENT_TYPE, "b2/x-auto");
405                req = req.header(constants::X_BZ_CONTENT_SHA1, "do_not_verify");
406
407                let req = req.body(Buffer::new()).map_err(new_request_build_error)?;
408                // We don't need this request anymore, consume it directly.
409                let (parts, _) = req.into_parts();
410
411                Ok(RpPresign::new(PresignedRequest::new(
412                    parts.method,
413                    parts.uri,
414                    parts.headers,
415                )))
416            }
417            PresignOperation::Delete(_) => Err(Error::new(
418                ErrorKind::Unsupported,
419                "operation is not supported",
420            )),
421        }
422    }
423}
```
