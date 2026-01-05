# 

opendal/services/cos/

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
19use std::sync::Arc;
20
21use http::Response;
22use http::StatusCode;
23use http::Uri;
24use log::debug;
25use reqsign::TencentCosConfig;
26use reqsign::TencentCosCredentialLoader;
27use reqsign::TencentCosSigner;
28
29use super::COS_SCHEME;
30use super::core::*;
31use super::delete::CosDeleter;
32use super::error::parse_error;
33use super::lister::CosLister;
34use super::lister::CosListers;
35use super::lister::CosObjectVersionsLister;
36use super::writer::CosWriter;
37use super::writer::CosWriters;
38use crate::raw::oio::PageLister;
39use crate::raw::*;
40use crate::services::CosConfig;
41use crate::*;
42
43/// Tencent-Cloud COS services support.
44#[doc = include_str!("docs.md")]
45#[derive(Default, Clone)]
46pub struct CosBuilder {
47    pub(super) config: CosConfig,
48
49    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
50    pub(super) http_client: Option<HttpClient>,
51}
52
53impl Debug for CosBuilder {
54    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
55        f.debug_struct("CosBuilder")
56            .field("config", &self.config)
57            .finish()
58    }
59}
60
61impl CosBuilder {
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
75    /// Set endpoint of this backend.
76    ///
77    /// NOTE: no bucket or account id in endpoint, we will trim them if exists.
78    ///
79    /// # Examples
80    ///
81    /// - `https://cos.ap-singapore.myqcloud.com`
82    pub fn endpoint(mut self, endpoint: &str) -> Self {
83        if !endpoint.is_empty() {
84            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
85        }
86
87        self
88    }
89
90    /// Set secret_id of this backend.
91    /// - If it is set, we will take user's input first.
92    /// - If not, we will try to load it from environment.
93    pub fn secret_id(mut self, secret_id: &str) -> Self {
94        if !secret_id.is_empty() {
95            self.config.secret_id = Some(secret_id.to_string());
96        }
97
98        self
99    }
100
101    /// Set secret_key of this backend.
102    /// - If it is set, we will take user's input first.
103    /// - If not, we will try to load it from environment.
104    pub fn secret_key(mut self, secret_key: &str) -> Self {
105        if !secret_key.is_empty() {
106            self.config.secret_key = Some(secret_key.to_string());
107        }
108
109        self
110    }
111
112    /// Set bucket of this backend.
113    /// The param is required.
114    pub fn bucket(mut self, bucket: &str) -> Self {
115        if !bucket.is_empty() {
116            self.config.bucket = Some(bucket.to_string());
117        }
118
119        self
120    }
121
122    /// Set bucket versioning status for this backend
123    pub fn enable_versioning(mut self, enabled: bool) -> Self {
124        self.config.enable_versioning = enabled;
125
126        self
127    }
128
129    /// Disable config load so that opendal will not load config from
130    /// environment.
131    ///
132    /// For examples:
133    ///
134    /// - envs like `TENCENTCLOUD_SECRET_ID`
135    pub fn disable_config_load(mut self) -> Self {
136        self.config.disable_config_load = true;
137        self
138    }
139
140    /// Specify the http client that used by this service.
141    ///
142    /// # Notes
143    ///
144    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
145    /// during minor updates.
146    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
147    #[allow(deprecated)]
148    pub fn http_client(mut self, client: HttpClient) -> Self {
149        self.http_client = Some(client);
150        self
151    }
152}
153
154impl Builder for CosBuilder {
155    type Config = CosConfig;
156
157    fn build(self) -> Result<impl Access> {
158        debug!("backend build started: {:?}", &self);
159
160        let root = normalize_root(&self.config.root.unwrap_or_default());
161        debug!("backend use root {root}");
162
163        let bucket = match &self.config.bucket {
164            Some(bucket) => Ok(bucket.to_string()),
165            None => Err(
166                Error::new(ErrorKind::ConfigInvalid, "The bucket is misconfigured")
167                    .with_context("service", Scheme::Cos),
168            ),
169        }?;
170        debug!("backend use bucket {}", &bucket);
171
172        let uri = match &self.config.endpoint {
173            Some(endpoint) => endpoint.parse::<Uri>().map_err(|err| {
174                Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
175                    .with_context("service", Scheme::Cos)
176                    .with_context("endpoint", endpoint)
177                    .set_source(err)
178            }),
179            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
180                .with_context("service", Scheme::Cos)),
181        }?;
182
183        let scheme = match uri.scheme_str() {
184            Some(scheme) => scheme.to_string(),
185            None => "https".to_string(),
186        };
187
188        // If endpoint contains bucket name, we should trim them.
189        let endpoint = uri.host().unwrap().replace(&format!("//{bucket}."), "//");
190        debug!("backend use endpoint {}", &endpoint);
191
192        let mut cfg = TencentCosConfig::default();
193        if !self.config.disable_config_load {
194            cfg = cfg.from_env();
195        }
196
197        if let Some(v) = self.config.secret_id {
198            cfg.secret_id = Some(v);
199        }
200        if let Some(v) = self.config.secret_key {
201            cfg.secret_key = Some(v);
202        }
203
204        let cred_loader = TencentCosCredentialLoader::new(GLOBAL_REQWEST_CLIENT.clone(), cfg);
205
206        let signer = TencentCosSigner::new();
207
208        Ok(CosBackend {
209            core: Arc::new(CosCore {
210                info: {
211                    let am = AccessorInfo::default();
212                    am.set_scheme(COS_SCHEME)
213                        .set_root(&root)
214                        .set_name(&bucket)
215                        .set_native_capability(Capability {
216                            stat: true,
217                            stat_with_if_match: true,
218                            stat_with_if_none_match: true,
219                            stat_with_version: self.config.enable_versioning,
220
221                            read: true,
222
223                            read_with_if_match: true,
224                            read_with_if_none_match: true,
225                            read_with_if_modified_since: true,
226                            read_with_if_unmodified_since: true,
227                            read_with_version: self.config.enable_versioning,
228
229                            write: true,
230                            write_can_empty: true,
231                            write_can_append: true,
232                            write_can_multi: true,
233                            write_with_content_type: true,
234                            write_with_cache_control: true,
235                            write_with_content_disposition: true,
236                            // Cos doesn't support forbid overwrite while version has been enabled.
237                            write_with_if_not_exists: !self.config.enable_versioning,
238                            // The min multipart size of COS is 1 MiB.
239                            //
240                            // ref: <https://www.tencentcloud.com/document/product/436/14112>
241                            write_multi_min_size: Some(1024 * 1024),
242                            // The max multipart size of COS is 5 GiB.
243                            //
244                            // ref: <https://www.tencentcloud.com/document/product/436/14112>
245                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
246                                Some(5 * 1024 * 1024 * 1024)
247                            } else {
248                                Some(usize::MAX)
249                            },
250                            write_with_user_metadata: true,
251
252                            delete: true,
253                            delete_with_version: self.config.enable_versioning,
254                            copy: true,
255
256                            list: true,
257                            list_with_recursive: true,
258                            list_with_versions: self.config.enable_versioning,
259                            list_with_deleted: self.config.enable_versioning,
260
261                            presign: true,
262                            presign_stat: true,
263                            presign_read: true,
264                            presign_write: true,
265
266                            shared: true,
267
268                            ..Default::default()
269                        });
270
271                    // allow deprecated api here for compatibility
272                    #[allow(deprecated)]
273                    if let Some(client) = self.http_client {
274                        am.update_http_client(|_| client);
275                    }
276
277                    am.into()
278                },
279                bucket: bucket.clone(),
280                root,
281                endpoint: format!("{}://{}.{}", &scheme, &bucket, &endpoint),
282                signer,
283                loader: cred_loader,
284            }),
285        })
286    }
287}
288
289/// Backend for Tencent-Cloud COS services.
290#[derive(Debug, Clone)]
291pub struct CosBackend {
292    core: Arc<CosCore>,
293}
294
295impl Access for CosBackend {
296    type Reader = HttpBody;
297    type Writer = CosWriters;
298    type Lister = CosListers;
299    type Deleter = oio::OneShotDeleter<CosDeleter>;
300
301    fn info(&self) -> Arc<AccessorInfo> {
302        self.core.info.clone()
303    }
304
305    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
306        let resp = self.core.cos_head_object(path, &args).await?;
307
308        let status = resp.status();
309
310        match status {
311            StatusCode::OK => {
312                let headers = resp.headers();
313                let mut meta = parse_into_metadata(path, headers)?;
314
315                let user_meta = parse_prefixed_headers(headers, "x-cos-meta-");
316                if !user_meta.is_empty() {
317                    meta = meta.with_user_metadata(user_meta);
318                }
319
320                if let Some(v) = parse_header_to_str(headers, constants::X_COS_VERSION_ID)? {
321                    if v != "null" {
322                        meta.set_version(v);
323                    }
324                }
325
326                Ok(RpStat::new(meta))
327            }
328            _ => Err(parse_error(resp)),
329        }
330    }
331
332    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
333        let resp = self.core.cos_get_object(path, args.range(), &args).await?;
334
335        let status = resp.status();
336
337        match status {
338            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
339                Ok((RpRead::default(), resp.into_body()))
340            }
341            _ => {
342                let (part, mut body) = resp.into_parts();
343                let buf = body.to_buffer().await?;
344                Err(parse_error(Response::from_parts(part, buf)))
345            }
346        }
347    }
348
349    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
350        let writer = CosWriter::new(self.core.clone(), path, args.clone());
351
352        let w = if args.append() {
353            CosWriters::Two(oio::AppendWriter::new(writer))
354        } else {
355            CosWriters::One(oio::MultipartWriter::new(
356                self.core.info.clone(),
357                writer,
358                args.concurrent(),
359            ))
360        };
361
362        Ok((RpWrite::default(), w))
363    }
364
365    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
366        Ok((
367            RpDelete::default(),
368            oio::OneShotDeleter::new(CosDeleter::new(self.core.clone())),
369        ))
370    }
371
372    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
373        let l = if args.versions() || args.deleted() {
374            TwoWays::Two(PageLister::new(CosObjectVersionsLister::new(
375                self.core.clone(),
376                path,
377                args,
378            )))
379        } else {
380            TwoWays::One(PageLister::new(CosLister::new(
381                self.core.clone(),
382                path,
383                args.recursive(),
384                args.limit(),
385            )))
386        };
387
388        Ok((RpList::default(), l))
389    }
390
391    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
392        let resp = self.core.cos_copy_object(from, to).await?;
393
394        let status = resp.status();
395
396        match status {
397            StatusCode::OK => Ok(RpCopy::default()),
398            _ => Err(parse_error(resp)),
399        }
400    }
401
402    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
403        let req = match args.operation() {
404            PresignOperation::Stat(v) => self.core.cos_head_object_request(path, v),
405            PresignOperation::Read(v) => {
406                self.core
407                    .cos_get_object_request(path, BytesRange::default(), v)
408            }
409            PresignOperation::Write(v) => {
410                self.core
411                    .cos_put_object_request(path, None, v, Buffer::new())
412            }
413            PresignOperation::Delete(_) => Err(Error::new(
414                ErrorKind::Unsupported,
415                "operation is not supported",
416            )),
417        };
418        let mut req = req?;
419        self.core.sign_query(&mut req, args.expire()).await?;
420
421        // We don't need this request anymore, consume it directly.
422        let (parts, _) = req.into_parts();
423
424        Ok(RpPresign::new(PresignedRequest::new(
425            parts.method,
426            parts.uri,
427            parts.headers,
428        )))
429    }
430}
```
