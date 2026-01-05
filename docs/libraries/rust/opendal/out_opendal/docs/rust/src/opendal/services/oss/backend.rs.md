# 

opendal/services/oss/

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
22use http::Response;
23use http::StatusCode;
24use http::Uri;
25use log::debug;
26use reqsign::AliyunConfig;
27use reqsign::AliyunLoader;
28use reqsign::AliyunOssSigner;
29
30use super::OSS_SCHEME;
31use super::core::*;
32use super::delete::OssDeleter;
33use super::error::parse_error;
34use super::lister::OssLister;
35use super::lister::OssListers;
36use super::lister::OssObjectVersionsLister;
37use super::writer::OssWriter;
38use super::writer::OssWriters;
39use crate::raw::*;
40use crate::services::OssConfig;
41use crate::*;
42const DEFAULT_BATCH_MAX_OPERATIONS: usize = 1000;
43
44/// Aliyun Object Storage Service (OSS) support
45#[doc = include_str!("docs.md")]
46#[derive(Default)]
47pub struct OssBuilder {
48    pub(super) config: OssConfig,
49
50    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
51    pub(super) http_client: Option<HttpClient>,
52}
53
54impl Debug for OssBuilder {
55    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
56        let mut d = f.debug_struct("OssBuilder");
57
58        d.field("config", &self.config);
59        d.finish_non_exhaustive()
60    }
61}
62
63impl OssBuilder {
64    /// Set root of this backend.
65    ///
66    /// All operations will happen under this root.
67    pub fn root(mut self, root: &str) -> Self {
68        self.config.root = if root.is_empty() {
69            None
70        } else {
71            Some(root.to_string())
72        };
73
74        self
75    }
76
77    /// Set bucket name of this backend.
78    pub fn bucket(mut self, bucket: &str) -> Self {
79        self.config.bucket = bucket.to_string();
80
81        self
82    }
83
84    /// Set endpoint of this backend.
85    pub fn endpoint(mut self, endpoint: &str) -> Self {
86        if !endpoint.is_empty() {
87            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
88            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string())
89        }
90
91        self
92    }
93
94    /// Set addressing style for the endpoint.
95    ///
96    /// Available values: `virtual`, `cname`, `path`.
97    ///
98    /// - `virtual`: Use virtual addressing style, i.e. `http://bucket.oss-<region>.aliyuncs.com/object`
99    /// - `cname`: Use cname addressing style, i.e. `http://mydomain.com/object` with mydomain.com bound to your bucket.
100    /// - `path`: Use path addressing style. i.e. `http://oss-<region>.aliyuncs.com/bucket/object`
101    ///
102    /// - If not set, default value is `virtual`.
103    pub fn addressing_style(mut self, addressing_style: &str) -> Self {
104        self.config.addressing_style = Some(addressing_style.to_string());
105
106        self
107    }
108
109    /// Set bucket versioning status for this backend
110    pub fn enable_versioning(mut self, enabled: bool) -> Self {
111        self.config.enable_versioning = enabled;
112
113        self
114    }
115
116    /// Set an endpoint for generating presigned urls.
117    ///
118    /// You can offer a public endpoint like <https://oss-cn-beijing.aliyuncs.com> to return a presinged url for
119    /// public accessors, along with an internal endpoint like <https://oss-cn-beijing-internal.aliyuncs.com>
120    /// to access objects in a faster path.
121    ///
122    /// - If presign_endpoint is set, we will use presign_endpoint on generating presigned urls.
123    /// - if not, we will use endpoint as default.
124    pub fn presign_endpoint(mut self, endpoint: &str) -> Self {
125        if !endpoint.is_empty() {
126            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
127            self.config.presign_endpoint = Some(endpoint.trim_end_matches('/').to_string())
128        }
129
130        self
131    }
132
133    /// Set addressing style for presign endpoint.
134    ///
135    /// Similar to setting addressing style for endpoint.
136    ///
137    /// - If both presign_endpoint and presign_addressing_style are not set, they are the same as endpoint's configurations.
138    ///
139    /// - If presign_endpoint is set, but presign_addressing_style is not set, default value is `virtual`.
140    pub fn presign_addressing_style(mut self, addressing_style: &str) -> Self {
141        self.config.presign_addressing_style = Some(addressing_style.to_string());
142
143        self
144    }
145
146    /// Set access_key_id of this backend.
147    ///
148    /// - If access_key_id is set, we will take user's input first.
149    /// - If not, we will try to load it from environment.
150    pub fn access_key_id(mut self, v: &str) -> Self {
151        if !v.is_empty() {
152            self.config.access_key_id = Some(v.to_string())
153        }
154
155        self
156    }
157
158    /// Set access_key_secret of this backend.
159    ///
160    /// - If access_key_secret is set, we will take user's input first.
161    /// - If not, we will try to load it from environment.
162    pub fn access_key_secret(mut self, v: &str) -> Self {
163        if !v.is_empty() {
164            self.config.access_key_secret = Some(v.to_string())
165        }
166
167        self
168    }
169
170    /// Set security_token for this backend.
171    ///
172    /// - If security_token is set, we will take user's input first.
173    /// - If not, we will try to load it from environment.
174    pub fn security_token(mut self, security_token: &str) -> Self {
175        if !security_token.is_empty() {
176            self.config.security_token = Some(security_token.to_string())
177        }
178
179        self
180    }
181
182    /// Specify the http client that used by this service.
183    ///
184    /// # Notes
185    ///
186    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
187    /// during minor updates.
188    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
189    #[allow(deprecated)]
190    pub fn http_client(mut self, client: HttpClient) -> Self {
191        self.http_client = Some(client);
192        self
193    }
194
195    /// preprocess the endpoint option
196    fn parse_endpoint(
197        &self,
198        endpoint: &Option<String>,
199        bucket: &str,
200        addressing_style: AddressingStyle,
201    ) -> Result<(String, String)> {
202        let (endpoint, host) = match endpoint.clone() {
203            Some(ep) => {
204                let uri = ep.parse::<Uri>().map_err(|err| {
205                    Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
206                        .with_context("service", Scheme::Oss)
207                        .with_context("endpoint", &ep)
208                        .set_source(err)
209                })?;
210                let host = uri.host().ok_or_else(|| {
211                    Error::new(ErrorKind::ConfigInvalid, "endpoint host is empty")
212                        .with_context("service", Scheme::Oss)
213                        .with_context("endpoint", &ep)
214                })?;
215                let full_host = match addressing_style {
216                    AddressingStyle::Virtual => {
217                        if let Some(port) = uri.port_u16() {
218                            format!("{bucket}.{host}:{port}")
219                        } else {
220                            format!("{bucket}.{host}")
221                        }
222                    }
223                    AddressingStyle::Cname | AddressingStyle::Path => {
224                        if let Some(port) = uri.port_u16() {
225                            format!("{host}:{port}")
226                        } else {
227                            host.to_string()
228                        }
229                    }
230                };
231                if let Some(port) = uri.port_u16() {
232                    format!("{bucket}.{host}:{port}")
233                } else {
234                    format!("{bucket}.{host}")
235                };
236                let endpoint = match uri.scheme_str() {
237                    Some(scheme_str) => match scheme_str {
238                        "http" | "https" => format!("{scheme_str}://{full_host}"),
239                        _ => {
240                            return Err(Error::new(
241                                ErrorKind::ConfigInvalid,
242                                "endpoint protocol is invalid",
243                            )
244                            .with_context("service", Scheme::Oss));
245                        }
246                    },
247                    None => format!("https://{full_host}"),
248                };
249                let endpoint = match addressing_style {
250                    AddressingStyle::Path => format!("{}/{}", endpoint, bucket),
251                    AddressingStyle::Cname | AddressingStyle::Virtual => endpoint,
252                };
253                (endpoint, full_host)
254            }
255            None => {
256                return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
257                    .with_context("service", Scheme::Oss));
258            }
259        };
260        Ok((endpoint, host))
261    }
262
263    /// Set server_side_encryption for this backend.
264    ///
265    /// Available values: `AES256`, `KMS`.
266    ///
267    /// Reference: <https://www.alibabacloud.com/help/en/object-storage-service/latest/server-side-encryption-5>
268    /// Brief explanation:
269    /// There are two server-side encryption methods available:
270    /// SSE-AES256:
271    ///     1. Configure the bucket encryption mode as OSS-managed and specify the encryption algorithm as AES256.
272    ///     2. Include the `x-oss-server-side-encryption` parameter in the request and set its value to AES256.
273    /// SSE-KMS:
274    ///     1. To use this service, you need to first enable KMS.
275    ///     2. Configure the bucket encryption mode as KMS, and specify the specific CMK ID for BYOK (Bring Your Own Key)
276    ///        or not specify the specific CMK ID for OSS-managed KMS key.
277    ///     3. Include the `x-oss-server-side-encryption` parameter in the request and set its value to KMS.
278    ///     4. If a specific CMK ID is specified, include the `x-oss-server-side-encryption-key-id` parameter in the request, and set its value to the specified CMK ID.
279    pub fn server_side_encryption(mut self, v: &str) -> Self {
280        if !v.is_empty() {
281            self.config.server_side_encryption = Some(v.to_string())
282        }
283        self
284    }
285
286    /// Set server_side_encryption_key_id for this backend.
287    ///
288    /// # Notes
289    ///
290    /// This option only takes effect when server_side_encryption equals to KMS.
291    pub fn server_side_encryption_key_id(mut self, v: &str) -> Self {
292        if !v.is_empty() {
293            self.config.server_side_encryption_key_id = Some(v.to_string())
294        }
295        self
296    }
297
298    /// Set maximum batch operations of this backend.
299    #[deprecated(
300        since = "0.52.0",
301        note = "Please use `delete_max_size` instead of `batch_max_operations`"
302    )]
303    pub fn batch_max_operations(mut self, delete_max_size: usize) -> Self {
304        self.config.delete_max_size = Some(delete_max_size);
305
306        self
307    }
308
309    /// Set maximum delete operations of this backend.
310    pub fn delete_max_size(mut self, delete_max_size: usize) -> Self {
311        self.config.delete_max_size = Some(delete_max_size);
312
313        self
314    }
315
316    /// Allow anonymous will allow opendal to send request without signing
317    /// when credential is not loaded.
318    pub fn allow_anonymous(mut self) -> Self {
319        self.config.allow_anonymous = true;
320        self
321    }
322
323    /// Set role_arn for this backend.
324    ///
325    /// If `role_arn` is set, we will use already known config as source
326    /// credential to assume role with `role_arn`.
327    pub fn role_arn(mut self, role_arn: &str) -> Self {
328        if !role_arn.is_empty() {
329            self.config.role_arn = Some(role_arn.to_string())
330        }
331
332        self
333    }
334
335    /// Set role_session_name for this backend.
336    pub fn role_session_name(mut self, role_session_name: &str) -> Self {
337        if !role_session_name.is_empty() {
338            self.config.role_session_name = Some(role_session_name.to_string())
339        }
340
341        self
342    }
343
344    /// Set oidc_provider_arn for this backend.
345    pub fn oidc_provider_arn(mut self, oidc_provider_arn: &str) -> Self {
346        if !oidc_provider_arn.is_empty() {
347            self.config.oidc_provider_arn = Some(oidc_provider_arn.to_string())
348        }
349
350        self
351    }
352
353    /// Set oidc_token_file for this backend.
354    pub fn oidc_token_file(mut self, oidc_token_file: &str) -> Self {
355        if !oidc_token_file.is_empty() {
356            self.config.oidc_token_file = Some(oidc_token_file.to_string())
357        }
358
359        self
360    }
361
362    /// Set sts_endpoint for this backend.
363    pub fn sts_endpoint(mut self, sts_endpoint: &str) -> Self {
364        if !sts_endpoint.is_empty() {
365            self.config.sts_endpoint = Some(sts_endpoint.to_string())
366        }
367
368        self
369    }
370}
371
372enum AddressingStyle {
373    Path,
374    Cname,
375    Virtual,
376}
377
378impl TryFrom<&Option<String>> for AddressingStyle {
379    type Error = Error;
380
381    fn try_from(value: &Option<String>) -> Result<Self> {
382        match value.as_deref() {
383            None | Some("virtual") => Ok(AddressingStyle::Virtual),
384            Some("path") => Ok(AddressingStyle::Path),
385            Some("cname") => Ok(AddressingStyle::Cname),
386            Some(v) => Err(Error::new(
387                ErrorKind::ConfigInvalid,
388                "Invalid addressing style, available: `virtual`, `path`, `cname`",
389            )
390            .with_context("service", Scheme::Oss)
391            .with_context("addressing_style", v)),
392        }
393    }
394}
395
396impl Builder for OssBuilder {
397    type Config = OssConfig;
398
399    fn build(self) -> Result<impl Access> {
400        debug!("backend build started: {:?}", &self);
401
402        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
403        debug!("backend use root {}", &root);
404
405        // Handle endpoint, region and bucket name.
406        let bucket = match self.config.bucket.is_empty() {
407            false => Ok(&self.config.bucket),
408            true => Err(
409                Error::new(ErrorKind::ConfigInvalid, "The bucket is misconfigured")
410                    .with_context("service", Scheme::Oss),
411            ),
412        }?;
413
414        // Retrieve endpoint and host by parsing the endpoint option and bucket. If presign_endpoint is not
415        // set, take endpoint as default presign_endpoint.
416        let (endpoint, host) = self.parse_endpoint(
417            &self.config.endpoint,
418            bucket,
419            (&self.config.addressing_style).try_into()?,
420        )?;
421        debug!("backend use bucket {}, endpoint: {}", &bucket, &endpoint);
422
423        let presign_endpoint = if self.config.presign_endpoint.is_some() {
424            self.parse_endpoint(
425                &self.config.presign_endpoint,
426                bucket,
427                (&self.config.presign_addressing_style).try_into()?,
428            )?
429            .0
430        } else {
431            endpoint.clone()
432        };
433        debug!("backend use presign_endpoint: {}", &presign_endpoint);
434
435        let server_side_encryption = match &self.config.server_side_encryption {
436            None => None,
437            Some(v) => Some(
438                build_header_value(v)
439                    .map_err(|err| err.with_context("key", "server_side_encryption"))?,
440            ),
441        };
442
443        let server_side_encryption_key_id = match &self.config.server_side_encryption_key_id {
444            None => None,
445            Some(v) => Some(
446                build_header_value(v)
447                    .map_err(|err| err.with_context("key", "server_side_encryption_key_id"))?,
448            ),
449        };
450
451        let mut cfg = AliyunConfig::default();
452        // Load cfg from env first.
453        cfg = cfg.from_env();
454
455        if let Some(v) = self.config.access_key_id {
456            cfg.access_key_id = Some(v);
457        }
458
459        if let Some(v) = self.config.access_key_secret {
460            cfg.access_key_secret = Some(v);
461        }
462
463        if let Some(v) = self.config.security_token {
464            cfg.security_token = Some(v);
465        }
466
467        if let Some(v) = self.config.role_arn {
468            cfg.role_arn = Some(v);
469        }
470
471        // override default role_session_name if set
472        if let Some(v) = self.config.role_session_name {
473            cfg.role_session_name = v;
474        }
475
476        if let Some(v) = self.config.oidc_provider_arn {
477            cfg.oidc_provider_arn = Some(v);
478        }
479
480        if let Some(v) = self.config.oidc_token_file {
481            cfg.oidc_token_file = Some(v);
482        }
483
484        if let Some(v) = self.config.sts_endpoint {
485            cfg.sts_endpoint = Some(v);
486        }
487
488        let loader = AliyunLoader::new(GLOBAL_REQWEST_CLIENT.clone(), cfg);
489
490        let signer = AliyunOssSigner::new(bucket);
491
492        let delete_max_size = self
493            .config
494            .delete_max_size
495            .unwrap_or(DEFAULT_BATCH_MAX_OPERATIONS);
496
497        Ok(OssBackend {
498            core: Arc::new(OssCore {
499                info: {
500                    let am = AccessorInfo::default();
501                    am.set_scheme(OSS_SCHEME)
502                        .set_root(&root)
503                        .set_name(bucket)
504                        .set_native_capability(Capability {
505                            stat: true,
506                            stat_with_if_match: true,
507                            stat_with_if_none_match: true,
508                            stat_with_version: self.config.enable_versioning,
509
510                            read: true,
511
512                            read_with_if_match: true,
513                            read_with_if_none_match: true,
514                            read_with_version: self.config.enable_versioning,
515                            read_with_if_modified_since: true,
516                            read_with_if_unmodified_since: true,
517
518                            write: true,
519                            write_can_empty: true,
520                            write_can_append: true,
521                            write_can_multi: true,
522                            write_with_cache_control: true,
523                            write_with_content_type: true,
524                            write_with_content_disposition: true,
525                            // TODO: set this to false while version has been enabled.
526                            write_with_if_not_exists: !self.config.enable_versioning,
527
528                            // The min multipart size of OSS is 100 KiB.
529                            //
530                            // ref: <https://www.alibabacloud.com/help/en/oss/user-guide/multipart-upload-12>
531                            write_multi_min_size: Some(100 * 1024),
532                            // The max multipart size of OSS is 5 GiB.
533                            //
534                            // ref: <https://www.alibabacloud.com/help/en/oss/user-guide/multipart-upload-12>
535                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
536                                Some(5 * 1024 * 1024 * 1024)
537                            } else {
538                                Some(usize::MAX)
539                            },
540                            write_with_user_metadata: true,
541
542                            delete: true,
543                            delete_with_version: self.config.enable_versioning,
544                            delete_max_size: Some(delete_max_size),
545
546                            copy: true,
547
548                            list: true,
549                            list_with_limit: true,
550                            list_with_start_after: true,
551                            list_with_recursive: true,
552                            list_with_versions: self.config.enable_versioning,
553                            list_with_deleted: self.config.enable_versioning,
554
555                            presign: true,
556                            presign_stat: true,
557                            presign_read: true,
558                            presign_write: true,
559
560                            shared: true,
561
562                            ..Default::default()
563                        });
564
565                    // allow deprecated api here for compatibility
566                    #[allow(deprecated)]
567                    if let Some(client) = self.http_client {
568                        am.update_http_client(|_| client);
569                    }
570
571                    am.into()
572                },
573                root,
574                bucket: bucket.to_owned(),
575                endpoint,
576                host,
577                presign_endpoint,
578                allow_anonymous: self.config.allow_anonymous,
579                signer,
580                loader,
581                server_side_encryption,
582                server_side_encryption_key_id,
583            }),
584        })
585    }
586}
587
588#[derive(Debug, Clone)]
589/// Aliyun Object Storage Service backend
590pub struct OssBackend {
591    core: Arc<OssCore>,
592}
593
594impl Access for OssBackend {
595    type Reader = HttpBody;
596    type Writer = OssWriters;
597    type Lister = OssListers;
598    type Deleter = oio::BatchDeleter<OssDeleter>;
599
600    fn info(&self) -> Arc<AccessorInfo> {
601        self.core.info.clone()
602    }
603
604    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
605        let resp = self.core.oss_head_object(path, &args).await?;
606
607        let status = resp.status();
608
609        match status {
610            StatusCode::OK => {
611                let headers = resp.headers();
612                let mut meta = self.core.parse_metadata(path, resp.headers())?;
613
614                if let Some(v) = parse_header_to_str(headers, constants::X_OSS_VERSION_ID)? {
615                    meta.set_version(v);
616                }
617
618                Ok(RpStat::new(meta))
619            }
620            _ => Err(parse_error(resp)),
621        }
622    }
623
624    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
625        let resp = self.core.oss_get_object(path, &args).await?;
626
627        let status = resp.status();
628
629        match status {
630            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
631                Ok((RpRead::default(), resp.into_body()))
632            }
633            _ => {
634                let (part, mut body) = resp.into_parts();
635                let buf = body.to_buffer().await?;
636                Err(parse_error(Response::from_parts(part, buf)))
637            }
638        }
639    }
640
641    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
642        let writer = OssWriter::new(self.core.clone(), path, args.clone());
643
644        let w = if args.append() {
645            OssWriters::Two(oio::AppendWriter::new(writer))
646        } else {
647            OssWriters::One(oio::MultipartWriter::new(
648                self.core.info.clone(),
649                writer,
650                args.concurrent(),
651            ))
652        };
653
654        Ok((RpWrite::default(), w))
655    }
656
657    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
658        Ok((
659            RpDelete::default(),
660            oio::BatchDeleter::new(OssDeleter::new(self.core.clone())),
661        ))
662    }
663
664    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
665        let l = if args.versions() || args.deleted() {
666            TwoWays::Two(oio::PageLister::new(OssObjectVersionsLister::new(
667                self.core.clone(),
668                path,
669                args,
670            )))
671        } else {
672            TwoWays::One(oio::PageLister::new(OssLister::new(
673                self.core.clone(),
674                path,
675                args.recursive(),
676                args.limit(),
677                args.start_after(),
678            )))
679        };
680
681        Ok((RpList::default(), l))
682    }
683
684    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
685        let resp = self.core.oss_copy_object(from, to).await?;
686        let status = resp.status();
687
688        match status {
689            StatusCode::OK => Ok(RpCopy::default()),
690            _ => Err(parse_error(resp)),
691        }
692    }
693
694    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
695        // We will not send this request out, just for signing.
696        let req = match args.operation() {
697            PresignOperation::Stat(v) => self.core.oss_head_object_request(path, true, v),
698            PresignOperation::Read(v) => self.core.oss_get_object_request(path, true, v),
699            PresignOperation::Write(v) => {
700                self.core
701                    .oss_put_object_request(path, None, v, Buffer::new(), true)
702            }
703            PresignOperation::Delete(_) => Err(Error::new(
704                ErrorKind::Unsupported,
705                "operation is not supported",
706            )),
707        };
708        let mut req = req?;
709
710        self.core.sign_query(&mut req, args.expire()).await?;
711
712        // We don't need this request anymore, consume it directly.
713        let (parts, _) = req.into_parts();
714
715        Ok(RpPresign::new(PresignedRequest::new(
716            parts.method,
717            parts.uri,
718            parts.headers,
719        )))
720    }
721}
```
