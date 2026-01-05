# 

opendal/services/obs/

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
18use std::collections::HashMap;
19use std::fmt::Debug;
20use std::fmt::Formatter;
21use std::sync::Arc;
22
23use http::Response;
24use http::StatusCode;
25use http::Uri;
26use log::debug;
27use reqsign::HuaweicloudObsConfig;
28use reqsign::HuaweicloudObsCredentialLoader;
29use reqsign::HuaweicloudObsSigner;
30
31use super::OBS_SCHEME;
32use super::core::ObsCore;
33use super::core::constants;
34use super::delete::ObsDeleter;
35use super::error::parse_error;
36use super::lister::ObsLister;
37use super::writer::ObsWriter;
38use super::writer::ObsWriters;
39use crate::raw::*;
40use crate::services::ObsConfig;
41use crate::*;
42
43/// Huawei-Cloud Object Storage Service (OBS) support
44#[doc = include_str!("docs.md")]
45#[derive(Default, Clone)]
46pub struct ObsBuilder {
47    pub(super) config: ObsConfig,
48
49    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
50    pub(super) http_client: Option<HttpClient>,
51}
52
53impl Debug for ObsBuilder {
54    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
55        let mut d = f.debug_struct("ObsBuilder");
56        d.field("config", &self.config);
57        d.finish_non_exhaustive()
58    }
59}
60
61impl ObsBuilder {
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
77    /// Both huaweicloud default domain and user domain endpoints are allowed.
78    /// Please DO NOT add the bucket name to the endpoint.
79    ///
80    /// - `https://obs.cn-north-4.myhuaweicloud.com`
81    /// - `obs.cn-north-4.myhuaweicloud.com` (https by default)
82    /// - `https://custom.obs.com` (port should not be set)
83    pub fn endpoint(mut self, endpoint: &str) -> Self {
84        if !endpoint.is_empty() {
85            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
86        }
87
88        self
89    }
90
91    /// Set access_key_id of this backend.
92    /// - If it is set, we will take user's input first.
93    /// - If not, we will try to load it from environment.
94    pub fn access_key_id(mut self, access_key_id: &str) -> Self {
95        if !access_key_id.is_empty() {
96            self.config.access_key_id = Some(access_key_id.to_string());
97        }
98
99        self
100    }
101
102    /// Set secret_access_key of this backend.
103    /// - If it is set, we will take user's input first.
104    /// - If not, we will try to load it from environment.
105    pub fn secret_access_key(mut self, secret_access_key: &str) -> Self {
106        if !secret_access_key.is_empty() {
107            self.config.secret_access_key = Some(secret_access_key.to_string());
108        }
109
110        self
111    }
112
113    /// Set bucket of this backend.
114    /// The param is required.
115    pub fn bucket(mut self, bucket: &str) -> Self {
116        if !bucket.is_empty() {
117            self.config.bucket = Some(bucket.to_string());
118        }
119
120        self
121    }
122
123    /// Set bucket versioning status for this backend
124    pub fn enable_versioning(mut self, enabled: bool) -> Self {
125        self.config.enable_versioning = enabled;
126
127        self
128    }
129
130    /// Specify the http client that used by this service.
131    ///
132    /// # Notes
133    ///
134    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
135    /// during minor updates.
136    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
137    #[allow(deprecated)]
138    pub fn http_client(mut self, client: HttpClient) -> Self {
139        self.http_client = Some(client);
140        self
141    }
142}
143
144impl Builder for ObsBuilder {
145    type Config = ObsConfig;
146
147    fn build(self) -> Result<impl Access> {
148        debug!("backend build started: {:?}", &self);
149
150        let root = normalize_root(&self.config.root.unwrap_or_default());
151        debug!("backend use root {root}");
152
153        let bucket = match &self.config.bucket {
154            Some(bucket) => Ok(bucket.to_string()),
155            None => Err(
156                Error::new(ErrorKind::ConfigInvalid, "The bucket is misconfigured")
157                    .with_context("service", Scheme::Obs),
158            ),
159        }?;
160        debug!("backend use bucket {}", &bucket);
161
162        let uri = match &self.config.endpoint {
163            Some(endpoint) => endpoint.parse::<Uri>().map_err(|err| {
164                Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
165                    .with_context("service", Scheme::Obs)
166                    .set_source(err)
167            }),
168            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
169                .with_context("service", Scheme::Obs)),
170        }?;
171
172        let scheme = match uri.scheme_str() {
173            Some(scheme) => scheme.to_string(),
174            None => "https".to_string(),
175        };
176
177        let (endpoint, is_obs_default) = {
178            let host = uri.host().unwrap_or_default().to_string();
179            if host.starts_with("obs.")
180                && (host.ends_with(".myhuaweicloud.com") || host.ends_with(".huawei.com"))
181            {
182                (format!("{bucket}.{host}"), true)
183            } else {
184                (host, false)
185            }
186        };
187        debug!("backend use endpoint {}", &endpoint);
188
189        let mut cfg = HuaweicloudObsConfig::default();
190        // Load cfg from env first.
191        cfg = cfg.from_env();
192
193        if let Some(v) = self.config.access_key_id {
194            cfg.access_key_id = Some(v);
195        }
196
197        if let Some(v) = self.config.secret_access_key {
198            cfg.secret_access_key = Some(v);
199        }
200
201        let loader = HuaweicloudObsCredentialLoader::new(cfg);
202
203        // Set the bucket name in CanonicalizedResource.
204        // 1. If the bucket is bound to a user domain name, use the user domain name as the bucket name,
205        // for example, `/obs.ccc.com/object`. `obs.ccc.com` is the user domain name bound to the bucket.
206        // 2. If you do not access OBS using a user domain name, this field is in the format of `/bucket/object`.
207        //
208        // Please refer to this doc for more details:
209        // https://support.huaweicloud.com/intl/en-us/api-obs/obs_04_0010.html
210        let signer = HuaweicloudObsSigner::new(if is_obs_default { &bucket } else { &endpoint });
211
212        debug!("backend build finished");
213        Ok(ObsBackend {
214            core: Arc::new(ObsCore {
215                info: {
216                    let am = AccessorInfo::default();
217                    am.set_scheme(OBS_SCHEME)
218                        .set_root(&root)
219                        .set_name(&bucket)
220                        .set_native_capability(Capability {
221                            stat: true,
222                            stat_with_if_match: true,
223                            stat_with_if_none_match: true,
224
225                            read: true,
226
227                            read_with_if_match: true,
228                            read_with_if_none_match: true,
229
230                            write: true,
231                            write_can_empty: true,
232                            write_can_append: true,
233                            write_can_multi: true,
234                            write_with_content_type: true,
235                            write_with_cache_control: true,
236                            // The min multipart size of OBS is 5 MiB.
237                            //
238                            // ref: <https://support.huaweicloud.com/intl/en-us/ugobs-obs/obs_41_0021.html>
239                            write_multi_min_size: Some(5 * 1024 * 1024),
240                            // The max multipart size of OBS is 5 GiB.
241                            //
242                            // ref: <https://support.huaweicloud.com/intl/en-us/ugobs-obs/obs_41_0021.html>
243                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
244                                Some(5 * 1024 * 1024 * 1024)
245                            } else {
246                                Some(usize::MAX)
247                            },
248                            write_with_user_metadata: true,
249
250                            delete: true,
251                            copy: true,
252
253                            list: true,
254                            list_with_recursive: true,
255
256                            presign: true,
257                            presign_stat: true,
258                            presign_read: true,
259                            presign_write: true,
260
261                            shared: true,
262
263                            ..Default::default()
264                        });
265
266                    // allow deprecated api here for compatibility
267                    #[allow(deprecated)]
268                    if let Some(client) = self.http_client {
269                        am.update_http_client(|_| client);
270                    }
271
272                    am.into()
273                },
274                bucket,
275                root,
276                endpoint: format!("{}://{}", &scheme, &endpoint),
277                signer,
278                loader,
279            }),
280        })
281    }
282}
283
284/// Backend for Huaweicloud OBS services.
285#[derive(Debug, Clone)]
286pub struct ObsBackend {
287    core: Arc<ObsCore>,
288}
289
290impl Access for ObsBackend {
291    type Reader = HttpBody;
292    type Writer = ObsWriters;
293    type Lister = oio::PageLister<ObsLister>;
294    type Deleter = oio::OneShotDeleter<ObsDeleter>;
295
296    fn info(&self) -> Arc<AccessorInfo> {
297        self.core.info.clone()
298    }
299
300    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
301        let resp = self.core.obs_head_object(path, &args).await?;
302        let headers = resp.headers();
303
304        let status = resp.status();
305
306        // The response is very similar to azblob.
307        match status {
308            StatusCode::OK => {
309                let mut meta = parse_into_metadata(path, headers)?;
310                let user_meta = headers
311                    .iter()
312                    .filter_map(|(name, _)| {
313                        name.as_str()
314                            .strip_prefix(constants::X_OBS_META_PREFIX)
315                            .and_then(|stripped_key| {
316                                parse_header_to_str(headers, name)
317                                    .unwrap_or(None)
318                                    .map(|val| (stripped_key.to_string(), val.to_string()))
319                            })
320                    })
321                    .collect::<HashMap<_, _>>();
322
323                if !user_meta.is_empty() {
324                    meta = meta.with_user_metadata(user_meta);
325                }
326
327                if let Some(v) = parse_header_to_str(headers, constants::X_OBS_VERSION_ID)? {
328                    meta.set_version(v);
329                }
330
331                Ok(RpStat::new(meta))
332            }
333            StatusCode::NOT_FOUND if path.ends_with('/') => {
334                Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
335            }
336            _ => Err(parse_error(resp)),
337        }
338    }
339
340    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
341        let resp = self.core.obs_get_object(path, args.range(), &args).await?;
342
343        let status = resp.status();
344
345        match status {
346            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
347                Ok((RpRead::default(), resp.into_body()))
348            }
349            _ => {
350                let (part, mut body) = resp.into_parts();
351                let buf = body.to_buffer().await?;
352                Err(parse_error(Response::from_parts(part, buf)))
353            }
354        }
355    }
356
357    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
358        let writer = ObsWriter::new(self.core.clone(), path, args.clone());
359
360        let w = if args.append() {
361            ObsWriters::Two(oio::AppendWriter::new(writer))
362        } else {
363            ObsWriters::One(oio::MultipartWriter::new(
364                self.core.info.clone(),
365                writer,
366                args.concurrent(),
367            ))
368        };
369
370        Ok((RpWrite::default(), w))
371    }
372
373    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
374        Ok((
375            RpDelete::default(),
376            oio::OneShotDeleter::new(ObsDeleter::new(self.core.clone())),
377        ))
378    }
379
380    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
381        let l = ObsLister::new(self.core.clone(), path, args.recursive(), args.limit());
382        Ok((RpList::default(), oio::PageLister::new(l)))
383    }
384
385    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
386        let resp = self.core.obs_copy_object(from, to).await?;
387
388        let status = resp.status();
389
390        match status {
391            StatusCode::OK => Ok(RpCopy::default()),
392            _ => Err(parse_error(resp)),
393        }
394    }
395
396    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
397        let req = match args.operation() {
398            PresignOperation::Stat(v) => self.core.obs_head_object_request(path, v),
399            PresignOperation::Read(v) => {
400                self.core
401                    .obs_get_object_request(path, BytesRange::default(), v)
402            }
403            PresignOperation::Write(v) => {
404                self.core
405                    .obs_put_object_request(path, None, v, Buffer::new())
406            }
407            PresignOperation::Delete(_) => Err(Error::new(
408                ErrorKind::Unsupported,
409                "operation is not supported",
410            )),
411        };
412        let mut req = req?;
413        self.core.sign_query(&mut req, args.expire()).await?;
414
415        // We don't need this request anymore, consume it directly.
416        let (parts, _) = req.into_parts();
417
418        Ok(RpPresign::new(PresignedRequest::new(
419            parts.method,
420            parts.uri,
421            parts.headers,
422        )))
423    }
424}
```
