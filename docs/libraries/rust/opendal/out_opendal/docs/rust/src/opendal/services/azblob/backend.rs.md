# 

opendal/services/azblob/

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
22use base64::Engine;
23use base64::prelude::BASE64_STANDARD;
24use http::Response;
25use http::StatusCode;
26use log::debug;
27use reqsign::AzureStorageConfig;
28use reqsign::AzureStorageLoader;
29use reqsign::AzureStorageSigner;
30use sha2::Digest;
31use sha2::Sha256;
32
33use super::AZBLOB_SCHEME;
34use super::core::AzblobCore;
35use super::core::constants::X_MS_META_PREFIX;
36use super::core::constants::X_MS_VERSION_ID;
37use super::delete::AzblobDeleter;
38use super::error::parse_error;
39use super::lister::AzblobLister;
40use super::writer::AzblobWriter;
41use super::writer::AzblobWriters;
42use crate::raw::*;
43use crate::services::AzblobConfig;
44use crate::*;
45const AZBLOB_BATCH_LIMIT: usize = 256;
46
47impl From<AzureStorageConfig> for AzblobConfig {
48    fn from(value: AzureStorageConfig) -> Self {
49        Self {
50            endpoint: value.endpoint,
51            account_name: value.account_name,
52            account_key: value.account_key,
53            sas_token: value.sas_token,
54            ..Default::default()
55        }
56    }
57}
58
59#[doc = include_str!("docs.md")]
60#[derive(Default, Clone)]
61pub struct AzblobBuilder {
62    pub(super) config: AzblobConfig,
63
64    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
65    pub(super) http_client: Option<HttpClient>,
66}
67
68impl Debug for AzblobBuilder {
69    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
70        let mut ds = f.debug_struct("AzblobBuilder");
71
72        ds.field("config", &self.config);
73
74        ds.finish()
75    }
76}
77
78impl AzblobBuilder {
79    /// Set root of this backend.
80    ///
81    /// All operations will happen under this root.
82    pub fn root(mut self, root: &str) -> Self {
83        self.config.root = if root.is_empty() {
84            None
85        } else {
86            Some(root.to_string())
87        };
88
89        self
90    }
91
92    /// Set container name of this backend.
93    pub fn container(mut self, container: &str) -> Self {
94        self.config.container = container.to_string();
95
96        self
97    }
98
99    /// Set endpoint of this backend
100    ///
101    /// Endpoint must be full uri, e.g.
102    ///
103    /// - Azblob: `https://accountname.blob.core.windows.net`
104    /// - Azurite: `http://127.0.0.1:10000/devstoreaccount1`
105    pub fn endpoint(mut self, endpoint: &str) -> Self {
106        if !endpoint.is_empty() {
107            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
108            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
109        }
110
111        self
112    }
113
114    /// Set account_name of this backend.
115    ///
116    /// - If account_name is set, we will take user's input first.
117    /// - If not, we will try to load it from environment.
118    pub fn account_name(mut self, account_name: &str) -> Self {
119        if !account_name.is_empty() {
120            self.config.account_name = Some(account_name.to_string());
121        }
122
123        self
124    }
125
126    /// Set account_key of this backend.
127    ///
128    /// - If account_key is set, we will take user's input first.
129    /// - If not, we will try to load it from environment.
130    pub fn account_key(mut self, account_key: &str) -> Self {
131        if !account_key.is_empty() {
132            self.config.account_key = Some(account_key.to_string());
133        }
134
135        self
136    }
137
138    /// Set encryption_key of this backend.
139    ///
140    /// # Args
141    ///
142    /// `v`: Base64-encoded key that matches algorithm specified in `encryption_algorithm`.
143    ///
144    /// # Note
145    ///
146    /// This function is the low-level setting for SSE related features.
147    ///
148    /// SSE related options should be set carefully to make them works.
149    /// Please use `server_side_encryption_with_*` helpers if even possible.
150    pub fn encryption_key(mut self, v: &str) -> Self {
151        if !v.is_empty() {
152            self.config.encryption_key = Some(v.to_string());
153        }
154
155        self
156    }
157
158    /// Set encryption_key_sha256 of this backend.
159    ///
160    /// # Args
161    ///
162    /// `v`: Base64-encoded SHA256 digest of the key specified in encryption_key.
163    ///
164    /// # Note
165    ///
166    /// This function is the low-level setting for SSE related features.
167    ///
168    /// SSE related options should be set carefully to make them works.
169    /// Please use `server_side_encryption_with_*` helpers if even possible.
170    pub fn encryption_key_sha256(mut self, v: &str) -> Self {
171        if !v.is_empty() {
172            self.config.encryption_key_sha256 = Some(v.to_string());
173        }
174
175        self
176    }
177
178    /// Set encryption_algorithm of this backend.
179    ///
180    /// # Args
181    ///
182    /// `v`: server-side encryption algorithm. (Available values: `AES256`)
183    ///
184    /// # Note
185    ///
186    /// This function is the low-level setting for SSE related features.
187    ///
188    /// SSE related options should be set carefully to make them works.
189    /// Please use `server_side_encryption_with_*` helpers if even possible.
190    pub fn encryption_algorithm(mut self, v: &str) -> Self {
191        if !v.is_empty() {
192            self.config.encryption_algorithm = Some(v.to_string());
193        }
194
195        self
196    }
197
198    /// Enable server side encryption with customer key.
199    ///
200    /// As known as: CPK
201    ///
202    /// # Args
203    ///
204    /// `key`: Base64-encoded SHA256 digest of the key specified in encryption_key.
205    ///
206    /// # Note
207    ///
208    /// Function that helps the user to set the server-side customer-provided encryption key, the key's SHA256, and the algorithm.
209    /// See [Server-side encryption with customer-provided keys (CPK)](https://learn.microsoft.com/en-us/azure/storage/blobs/encryption-customer-provided-keys)
210    /// for more info.
211    pub fn server_side_encryption_with_customer_key(mut self, key: &[u8]) -> Self {
212        // Only AES256 is supported for now
213        self.config.encryption_algorithm = Some("AES256".to_string());
214        self.config.encryption_key = Some(BASE64_STANDARD.encode(key));
215        self.config.encryption_key_sha256 =
216            Some(BASE64_STANDARD.encode(Sha256::digest(key).as_slice()));
217        self
218    }
219
220    /// Set sas_token of this backend.
221    ///
222    /// - If sas_token is set, we will take user's input first.
223    /// - If not, we will try to load it from environment.
224    ///
225    /// See [Grant limited access to Azure Storage resources using shared access signatures (SAS)](https://learn.microsoft.com/en-us/azure/storage/common/storage-sas-overview)
226    /// for more info.
227    pub fn sas_token(mut self, sas_token: &str) -> Self {
228        if !sas_token.is_empty() {
229            self.config.sas_token = Some(sas_token.to_string());
230        }
231
232        self
233    }
234
235    /// Specify the http client that used by this service.
236    ///
237    /// # Notes
238    ///
239    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
240    /// during minor updates.
241    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
242    #[allow(deprecated)]
243    pub fn http_client(mut self, client: HttpClient) -> Self {
244        self.http_client = Some(client);
245        self
246    }
247
248    /// Set maximum batch operations of this backend.
249    pub fn batch_max_operations(mut self, batch_max_operations: usize) -> Self {
250        self.config.batch_max_operations = Some(batch_max_operations);
251
252        self
253    }
254
255    /// from_connection_string will make a builder from connection string
256    ///
257    /// connection string looks like:
258    ///
259    /// ```txt
260    /// DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;
261    /// AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;
262    /// BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;
263    /// QueueEndpoint=http://127.0.0.1:10001/devstoreaccount1;
264    /// TableEndpoint=http://127.0.0.1:10002/devstoreaccount1;
265    /// ```
266    ///
267    /// Or
268    ///
269    /// ```txt
270    /// DefaultEndpointsProtocol=https;
271    /// AccountName=storagesample;
272    /// AccountKey=<account-key>;
273    /// EndpointSuffix=core.chinacloudapi.cn;
274    /// ```
275    ///
276    /// For reference: [Configure Azure Storage connection strings](https://learn.microsoft.com/en-us/azure/storage/common/storage-configure-connection-string)
277    ///
278    /// # Note
279    ///
280    /// Connection strings can only configure the endpoint, account name and
281    /// authentication information. Users still need to configure container name.
282    pub fn from_connection_string(conn: &str) -> Result<Self> {
283        let config =
284            raw::azure_config_from_connection_string(conn, raw::AzureStorageService::Blob)?;
285
286        Ok(AzblobConfig::from(config).into_builder())
287    }
288}
289
290impl Builder for AzblobBuilder {
291    type Config = AzblobConfig;
292
293    fn build(self) -> Result<impl Access> {
294        debug!("backend build started: {:?}", &self);
295
296        let root = normalize_root(&self.config.root.unwrap_or_default());
297        debug!("backend use root {root}");
298
299        // Handle endpoint, region and container name.
300        let container = match self.config.container.is_empty() {
301            false => Ok(&self.config.container),
302            true => Err(Error::new(ErrorKind::ConfigInvalid, "container is empty")
303                .with_operation("Builder::build")
304                .with_context("service", Scheme::Azblob)),
305        }?;
306        debug!("backend use container {}", &container);
307
308        let endpoint = match &self.config.endpoint {
309            Some(endpoint) => Ok(endpoint.clone()),
310            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
311                .with_operation("Builder::build")
312                .with_context("service", Scheme::Azblob)),
313        }?;
314        debug!("backend use endpoint {}", &container);
315
316        #[cfg(target_arch = "wasm32")]
317        let mut config_loader = AzureStorageConfig::default();
318        #[cfg(not(target_arch = "wasm32"))]
319        let mut config_loader = AzureStorageConfig::default().from_env();
320
321        if let Some(v) = self
322            .config
323            .account_name
324            .clone()
325            .or_else(|| raw::azure_account_name_from_endpoint(endpoint.as_str()))
326        {
327            config_loader.account_name = Some(v);
328        }
329
330        if let Some(v) = self.config.account_key.clone() {
331            // Validate that account_key can be decoded as base64
332            if let Err(e) = BASE64_STANDARD.decode(&v) {
333                return Err(Error::new(
334                    ErrorKind::ConfigInvalid,
335                    format!("invalid account_key: cannot decode as base64: {e}"),
336                )
337                .with_operation("Builder::build")
338                .with_context("service", Scheme::Azblob)
339                .with_context("key", "account_key"));
340            }
341            config_loader.account_key = Some(v);
342        }
343
344        if let Some(v) = self.config.sas_token.clone() {
345            config_loader.sas_token = Some(v);
346        }
347
348        let encryption_key =
349            match &self.config.encryption_key {
350                None => None,
351                Some(v) => Some(build_header_value(v).map_err(|err| {
352                    err.with_context("key", "server_side_encryption_customer_key")
353                })?),
354            };
355
356        let encryption_key_sha256 = match &self.config.encryption_key_sha256 {
357            None => None,
358            Some(v) => Some(build_header_value(v).map_err(|err| {
359                err.with_context("key", "server_side_encryption_customer_key_sha256")
360            })?),
361        };
362
363        let encryption_algorithm = match &self.config.encryption_algorithm {
364            None => None,
365            Some(v) => {
366                if v == "AES256" {
367                    Some(build_header_value(v).map_err(|err| {
368                        err.with_context("key", "server_side_encryption_customer_algorithm")
369                    })?)
370                } else {
371                    return Err(Error::new(
372                        ErrorKind::ConfigInvalid,
373                        "encryption_algorithm value must be AES256",
374                    ));
375                }
376            }
377        };
378
379        let cred_loader = AzureStorageLoader::new(config_loader);
380
381        let signer = AzureStorageSigner::new();
382
383        Ok(AzblobBackend {
384            core: Arc::new(AzblobCore {
385                info: {
386                    let am = AccessorInfo::default();
387                    am.set_scheme(AZBLOB_SCHEME)
388                        .set_root(&root)
389                        .set_name(container)
390                        .set_native_capability(Capability {
391                            stat: true,
392                            stat_with_if_match: true,
393                            stat_with_if_none_match: true,
394
395                            read: true,
396
397                            read_with_if_match: true,
398                            read_with_if_none_match: true,
399                            read_with_override_content_disposition: true,
400                            read_with_if_modified_since: true,
401                            read_with_if_unmodified_since: true,
402
403                            write: true,
404                            write_can_append: true,
405                            write_can_empty: true,
406                            write_can_multi: true,
407                            write_with_cache_control: true,
408                            write_with_content_type: true,
409                            write_with_if_not_exists: true,
410                            write_with_if_none_match: true,
411                            write_with_user_metadata: true,
412
413                            delete: true,
414                            delete_max_size: Some(AZBLOB_BATCH_LIMIT),
415
416                            copy: true,
417                            copy_with_if_not_exists: true,
418
419                            list: true,
420                            list_with_recursive: true,
421
422                            presign: self.config.sas_token.is_some(),
423                            presign_stat: self.config.sas_token.is_some(),
424                            presign_read: self.config.sas_token.is_some(),
425                            presign_write: self.config.sas_token.is_some(),
426
427                            shared: true,
428
429                            ..Default::default()
430                        });
431
432                    // allow deprecated api here for compatibility
433                    #[allow(deprecated)]
434                    if let Some(client) = self.http_client {
435                        am.update_http_client(|_| client);
436                    }
437
438                    am.into()
439                },
440                root,
441                endpoint,
442                encryption_key,
443                encryption_key_sha256,
444                encryption_algorithm,
445                container: self.config.container.clone(),
446
447                loader: cred_loader,
448                signer,
449            }),
450        })
451    }
452}
453
454/// Backend for azblob services.
455#[derive(Debug, Clone)]
456pub struct AzblobBackend {
457    core: Arc<AzblobCore>,
458}
459
460impl Access for AzblobBackend {
461    type Reader = HttpBody;
462    type Writer = AzblobWriters;
463    type Lister = oio::PageLister<AzblobLister>;
464    type Deleter = oio::BatchDeleter<AzblobDeleter>;
465
466    fn info(&self) -> Arc<AccessorInfo> {
467        self.core.info.clone()
468    }
469
470    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
471        let resp = self.core.azblob_get_blob_properties(path, &args).await?;
472
473        let status = resp.status();
474
475        match status {
476            StatusCode::OK => {
477                let headers = resp.headers();
478                let mut meta = parse_into_metadata(path, headers)?;
479                if let Some(version_id) = parse_header_to_str(headers, X_MS_VERSION_ID)? {
480                    meta.set_version(version_id);
481                }
482
483                let user_meta = parse_prefixed_headers(headers, X_MS_META_PREFIX);
484                if !user_meta.is_empty() {
485                    meta = meta.with_user_metadata(user_meta);
486                }
487
488                Ok(RpStat::new(meta))
489            }
490            _ => Err(parse_error(resp)),
491        }
492    }
493
494    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
495        let resp = self.core.azblob_get_blob(path, args.range(), &args).await?;
496
497        let status = resp.status();
498        match status {
499            StatusCode::OK | StatusCode::PARTIAL_CONTENT => Ok((RpRead::new(), resp.into_body())),
500            _ => {
501                let (part, mut body) = resp.into_parts();
502                let buf = body.to_buffer().await?;
503                Err(parse_error(Response::from_parts(part, buf)))
504            }
505        }
506    }
507
508    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
509        let w = AzblobWriter::new(self.core.clone(), args.clone(), path.to_string());
510        let w = if args.append() {
511            AzblobWriters::Two(oio::AppendWriter::new(w))
512        } else {
513            AzblobWriters::One(oio::BlockWriter::new(
514                self.core.info.clone(),
515                w,
516                args.concurrent(),
517            ))
518        };
519
520        Ok((RpWrite::default(), w))
521    }
522
523    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
524        Ok((
525            RpDelete::default(),
526            oio::BatchDeleter::new(AzblobDeleter::new(self.core.clone())),
527        ))
528    }
529
530    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
531        let l = AzblobLister::new(
532            self.core.clone(),
533            path.to_string(),
534            args.recursive(),
535            args.limit(),
536        );
537
538        Ok((RpList::default(), oio::PageLister::new(l)))
539    }
540
541    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
542        let resp = self.core.azblob_copy_blob(from, to, args).await?;
543
544        let status = resp.status();
545
546        match status {
547            StatusCode::ACCEPTED => Ok(RpCopy::default()),
548            _ => Err(parse_error(resp)),
549        }
550    }
551
552    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
553        let req = match args.operation() {
554            PresignOperation::Stat(v) => self.core.azblob_head_blob_request(path, v),
555            PresignOperation::Read(v) => {
556                self.core
557                    .azblob_get_blob_request(path, BytesRange::default(), v)
558            }
559            PresignOperation::Write(_) => {
560                self.core
561                    .azblob_put_blob_request(path, None, &OpWrite::default(), Buffer::new())
562            }
563            PresignOperation::Delete(_) => Err(Error::new(
564                ErrorKind::Unsupported,
565                "operation is not supported",
566            )),
567        };
568
569        let mut req = req?;
570
571        self.core.sign_query(&mut req).await?;
572
573        let (parts, _) = req.into_parts();
574
575        Ok(RpPresign::new(PresignedRequest::new(
576            parts.method,
577            parts.uri,
578            parts.headers,
579        )))
580    }
581}
```
