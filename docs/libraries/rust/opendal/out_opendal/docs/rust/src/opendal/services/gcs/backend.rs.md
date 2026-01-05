# 

opendal/services/gcs/

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
24use log::debug;
25use reqsign::GoogleCredentialLoader;
26use reqsign::GoogleSigner;
27use reqsign::GoogleTokenLoad;
28use reqsign::GoogleTokenLoader;
29
30use super::GCS_SCHEME;
31use super::core::*;
32use super::delete::GcsDeleter;
33use super::error::parse_error;
34use super::lister::GcsLister;
35use super::writer::GcsWriter;
36use super::writer::GcsWriters;
37use crate::raw::oio::BatchDeleter;
38use crate::raw::*;
39use crate::services::GcsConfig;
40use crate::*;
41const DEFAULT_GCS_ENDPOINT: &str = "https://storage.googleapis.com";
42const DEFAULT_GCS_SCOPE: &str = "https://www.googleapis.com/auth/devstorage.read_write";
43
44/// [Google Cloud Storage](https://cloud.google.com/storage) services support.
45#[doc = include_str!("docs.md")]
46#[derive(Default)]
47pub struct GcsBuilder {
48    pub(super) config: GcsConfig,
49
50    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
51    pub(super) http_client: Option<HttpClient>,
52    pub(super) customized_token_loader: Option<Box<dyn GoogleTokenLoad>>,
53}
54
55impl Debug for GcsBuilder {
56    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
57        let mut ds = f.debug_struct("GcsBuilder");
58
59        ds.field("config", &self.config);
60        ds.finish_non_exhaustive()
61    }
62}
63
64impl GcsBuilder {
65    /// set the working directory root of backend
66    pub fn root(mut self, root: &str) -> Self {
67        self.config.root = if root.is_empty() {
68            None
69        } else {
70            Some(root.to_string())
71        };
72
73        self
74    }
75
76    /// set the container's name
77    pub fn bucket(mut self, bucket: &str) -> Self {
78        self.config.bucket = bucket.to_string();
79        self
80    }
81
82    /// set the GCS service scope
83    ///
84    /// If not set, we will use `https://www.googleapis.com/auth/devstorage.read_write`.
85    ///
86    /// # Valid scope examples
87    ///
88    /// - read-only: `https://www.googleapis.com/auth/devstorage.read_only`
89    /// - read-write: `https://www.googleapis.com/auth/devstorage.read_write`
90    /// - full-control: `https://www.googleapis.com/auth/devstorage.full_control`
91    ///
92    /// Reference: [Cloud Storage authentication](https://cloud.google.com/storage/docs/authentication)
93    pub fn scope(mut self, scope: &str) -> Self {
94        if !scope.is_empty() {
95            self.config.scope = Some(scope.to_string())
96        };
97        self
98    }
99
100    /// Set the GCS service account.
101    ///
102    /// service account will be used for fetch token from vm metadata.
103    /// If not set, we will try to fetch with `default` service account.
104    pub fn service_account(mut self, service_account: &str) -> Self {
105        if !service_account.is_empty() {
106            self.config.service_account = Some(service_account.to_string())
107        };
108        self
109    }
110
111    /// set the endpoint GCS service uses
112    pub fn endpoint(mut self, endpoint: &str) -> Self {
113        if !endpoint.is_empty() {
114            self.config.endpoint = Some(endpoint.to_string())
115        };
116        self
117    }
118
119    /// set the base64 hashed credentials string used for OAuth2 authentication.
120    ///
121    /// this method allows to specify the credentials directly as a base64 hashed string.
122    /// alternatively, you can use `credential_path()` to provide the local path to a credentials file.
123    /// we will use one of `credential` and `credential_path` to complete the OAuth2 authentication.
124    ///
125    /// Reference: [Google Cloud Storage Authentication](https://cloud.google.com/docs/authentication).
126    pub fn credential(mut self, credential: &str) -> Self {
127        if !credential.is_empty() {
128            self.config.credential = Some(credential.to_string())
129        };
130        self
131    }
132
133    /// set the local path to credentials file which is used for OAuth2 authentication.
134    ///
135    /// credentials file contains the original credentials that have not been base64 hashed.
136    /// we will use one of `credential` and `credential_path` to complete the OAuth2 authentication.
137    ///
138    /// Reference: [Google Cloud Storage Authentication](https://cloud.google.com/docs/authentication).
139    pub fn credential_path(mut self, path: &str) -> Self {
140        if !path.is_empty() {
141            self.config.credential_path = Some(path.to_string())
142        };
143        self
144    }
145
146    /// Specify the http client that used by this service.
147    ///
148    /// # Notes
149    ///
150    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
151    /// during minor updates.
152    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
153    #[allow(deprecated)]
154    pub fn http_client(mut self, client: HttpClient) -> Self {
155        self.http_client = Some(client);
156        self
157    }
158
159    /// Specify the customized token loader used by this service.
160    pub fn customized_token_loader(mut self, token_load: Box<dyn GoogleTokenLoad>) -> Self {
161        self.customized_token_loader = Some(token_load);
162        self
163    }
164
165    /// Provide the OAuth2 token to use.
166    pub fn token(mut self, token: String) -> Self {
167        self.config.token = Some(token);
168        self
169    }
170
171    /// Disable attempting to load credentials from the GCE metadata server.
172    pub fn disable_vm_metadata(mut self) -> Self {
173        self.config.disable_vm_metadata = true;
174        self
175    }
176
177    /// Disable loading configuration from the environment.
178    pub fn disable_config_load(mut self) -> Self {
179        self.config.disable_config_load = true;
180        self
181    }
182
183    /// Set the predefined acl for GCS.
184    ///
185    /// Available values are:
186    /// - `authenticatedRead`
187    /// - `bucketOwnerFullControl`
188    /// - `bucketOwnerRead`
189    /// - `private`
190    /// - `projectPrivate`
191    /// - `publicRead`
192    pub fn predefined_acl(mut self, acl: &str) -> Self {
193        if !acl.is_empty() {
194            self.config.predefined_acl = Some(acl.to_string())
195        };
196        self
197    }
198
199    /// Set the default storage class for GCS.
200    ///
201    /// Available values are:
202    /// - `STANDARD`
203    /// - `NEARLINE`
204    /// - `COLDLINE`
205    /// - `ARCHIVE`
206    pub fn default_storage_class(mut self, class: &str) -> Self {
207        if !class.is_empty() {
208            self.config.default_storage_class = Some(class.to_string())
209        };
210        self
211    }
212
213    /// Allow anonymous requests.
214    ///
215    /// This is typically used for buckets which are open to the public or GCS
216    /// storage emulators.
217    pub fn allow_anonymous(mut self) -> Self {
218        self.config.allow_anonymous = true;
219        self
220    }
221}
222
223impl Builder for GcsBuilder {
224    type Config = GcsConfig;
225
226    fn build(self) -> Result<impl Access> {
227        debug!("backend build started: {self:?}");
228
229        let root = normalize_root(&self.config.root.unwrap_or_default());
230        debug!("backend use root {root}");
231
232        // Handle endpoint and bucket name
233        let bucket = match self.config.bucket.is_empty() {
234            false => Ok(&self.config.bucket),
235            true => Err(
236                Error::new(ErrorKind::ConfigInvalid, "The bucket is misconfigured")
237                    .with_operation("Builder::build")
238                    .with_context("service", Scheme::Gcs),
239            ),
240        }?;
241
242        // TODO: server side encryption
243
244        let endpoint = self
245            .config
246            .endpoint
247            .clone()
248            .unwrap_or_else(|| DEFAULT_GCS_ENDPOINT.to_string());
249        debug!("backend use endpoint: {endpoint}");
250
251        let mut cred_loader = GoogleCredentialLoader::default();
252        if let Some(cred) = &self.config.credential {
253            cred_loader = cred_loader.with_content(cred);
254        }
255        if let Some(cred) = &self.config.credential_path {
256            cred_loader = cred_loader.with_path(cred);
257        }
258        #[cfg(target_arch = "wasm32")]
259        {
260            cred_loader = cred_loader.with_disable_env();
261            cred_loader = cred_loader.with_disable_well_known_location();
262        }
263
264        if self.config.disable_config_load {
265            cred_loader = cred_loader
266                .with_disable_env()
267                .with_disable_well_known_location();
268        }
269
270        let scope = if let Some(scope) = &self.config.scope {
271            scope
272        } else {
273            DEFAULT_GCS_SCOPE
274        };
275
276        let mut token_loader = GoogleTokenLoader::new(scope, GLOBAL_REQWEST_CLIENT.clone());
277        if let Some(account) = &self.config.service_account {
278            token_loader = token_loader.with_service_account(account);
279        }
280        if let Ok(Some(cred)) = cred_loader.load() {
281            token_loader = token_loader.with_credentials(cred)
282        }
283        if let Some(loader) = self.customized_token_loader {
284            token_loader = token_loader.with_customized_token_loader(loader)
285        }
286
287        if self.config.disable_vm_metadata {
288            token_loader = token_loader.with_disable_vm_metadata(true);
289        }
290
291        let signer = GoogleSigner::new("storage");
292
293        let backend = GcsBackend {
294            core: Arc::new(GcsCore {
295                info: {
296                    let am = AccessorInfo::default();
297                    am.set_scheme(GCS_SCHEME)
298                        .set_root(&root)
299                        .set_name(bucket)
300                        .set_native_capability(Capability {
301                            stat: true,
302                            stat_with_if_match: true,
303                            stat_with_if_none_match: true,
304
305                            read: true,
306
307                            read_with_if_match: true,
308                            read_with_if_none_match: true,
309
310                            write: true,
311                            write_can_empty: true,
312                            write_can_multi: true,
313                            write_with_cache_control: true,
314                            write_with_content_type: true,
315                            write_with_content_encoding: true,
316                            write_with_user_metadata: true,
317                            write_with_if_not_exists: true,
318
319                            // The min multipart size of Gcs is 5 MiB.
320                            //
321                            // ref: <https://cloud.google.com/storage/docs/xml-api/put-object-multipart>
322                            write_multi_min_size: Some(5 * 1024 * 1024),
323                            // The max multipart size of Gcs is 5 GiB.
324                            //
325                            // ref: <https://cloud.google.com/storage/docs/xml-api/put-object-multipart>
326                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
327                                Some(5 * 1024 * 1024 * 1024)
328                            } else {
329                                Some(usize::MAX)
330                            },
331
332                            delete: true,
333                            delete_max_size: Some(100),
334                            copy: true,
335
336                            list: true,
337                            list_with_limit: true,
338                            list_with_start_after: true,
339                            list_with_recursive: true,
340
341                            presign: true,
342                            presign_stat: true,
343                            presign_read: true,
344                            presign_write: true,
345
346                            shared: true,
347
348                            ..Default::default()
349                        });
350
351                    // allow deprecated api here for compatibility
352                    #[allow(deprecated)]
353                    if let Some(client) = self.http_client {
354                        am.update_http_client(|_| client);
355                    }
356
357                    am.into()
358                },
359                endpoint,
360                bucket: bucket.to_string(),
361                root,
362                signer,
363                token_loader,
364                token: self.config.token,
365                scope: scope.to_string(),
366                credential_loader: cred_loader,
367                predefined_acl: self.config.predefined_acl.clone(),
368                default_storage_class: self.config.default_storage_class.clone(),
369                allow_anonymous: self.config.allow_anonymous,
370            }),
371        };
372
373        Ok(backend)
374    }
375}
376
377/// GCS storage backend
378#[derive(Clone, Debug)]
379pub struct GcsBackend {
380    core: Arc<GcsCore>,
381}
382
383impl Access for GcsBackend {
384    type Reader = HttpBody;
385    type Writer = GcsWriters;
386    type Lister = oio::PageLister<GcsLister>;
387    type Deleter = oio::BatchDeleter<GcsDeleter>;
388
389    fn info(&self) -> Arc<AccessorInfo> {
390        self.core.info.clone()
391    }
392
393    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
394        let resp = self.core.gcs_get_object_metadata(path, &args).await?;
395
396        if !resp.status().is_success() {
397            return Err(parse_error(resp));
398        }
399
400        let slc = resp.into_body();
401        let m = GcsCore::build_metadata_from_object_response(path, slc)?;
402
403        Ok(RpStat::new(m))
404    }
405
406    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
407        let resp = self.core.gcs_get_object(path, args.range(), &args).await?;
408
409        let status = resp.status();
410
411        match status {
412            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
413                Ok((RpRead::default(), resp.into_body()))
414            }
415            _ => {
416                let (part, mut body) = resp.into_parts();
417                let buf = body.to_buffer().await?;
418                Err(parse_error(Response::from_parts(part, buf)))
419            }
420        }
421    }
422
423    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
424        let concurrent = args.concurrent();
425        let w = GcsWriter::new(self.core.clone(), path, args);
426        let w = oio::MultipartWriter::new(self.core.info.clone(), w, concurrent);
427
428        Ok((RpWrite::default(), w))
429    }
430
431    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
432        Ok((
433            RpDelete::default(),
434            BatchDeleter::new(GcsDeleter::new(self.core.clone())),
435        ))
436    }
437
438    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
439        let l = GcsLister::new(
440            self.core.clone(),
441            path,
442            args.recursive(),
443            args.limit(),
444            args.start_after(),
445        );
446
447        Ok((RpList::default(), oio::PageLister::new(l)))
448    }
449
450    async fn copy(&self, from: &str, to: &str, _: OpCopy) -> Result<RpCopy> {
451        let resp = self.core.gcs_copy_object(from, to).await?;
452
453        if resp.status().is_success() {
454            Ok(RpCopy::default())
455        } else {
456            Err(parse_error(resp))
457        }
458    }
459
460    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
461        // We will not send this request out, just for signing.
462        let req = match args.operation() {
463            PresignOperation::Stat(v) => self.core.gcs_head_object_xml_request(path, v),
464            PresignOperation::Read(v) => self.core.gcs_get_object_xml_request(path, v),
465            PresignOperation::Write(v) => {
466                self.core
467                    .gcs_insert_object_xml_request(path, v, Buffer::new())
468            }
469            PresignOperation::Delete(_) => Err(Error::new(
470                ErrorKind::Unsupported,
471                "operation is not supported",
472            )),
473        };
474        let mut req = req?;
475        self.core.sign_query(&mut req, args.expire())?;
476
477        // We don't need this request anymore, consume it directly.
478        let (parts, _) = req.into_parts();
479
480        Ok(RpPresign::new(PresignedRequest::new(
481            parts.method,
482            parts.uri,
483            parts.headers,
484        )))
485    }
486}
```
