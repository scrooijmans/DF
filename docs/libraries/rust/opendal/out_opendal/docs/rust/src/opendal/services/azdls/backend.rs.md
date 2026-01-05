# 

opendal/services/azdls/

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
25use reqsign::AzureStorageConfig;
26use reqsign::AzureStorageLoader;
27use reqsign::AzureStorageSigner;
28
29use super::AZDLS_SCHEME;
30use super::core::AzdlsCore;
31use super::core::DIRECTORY;
32use super::delete::AzdlsDeleter;
33use super::error::parse_error;
34use super::lister::AzdlsLister;
35use super::writer::AzdlsWriter;
36use super::writer::AzdlsWriters;
37use crate::raw::*;
38use crate::services::AzdlsConfig;
39use crate::*;
40impl From<AzureStorageConfig> for AzdlsConfig {
41    fn from(config: AzureStorageConfig) -> Self {
42        AzdlsConfig {
43            endpoint: config.endpoint,
44            account_name: config.account_name,
45            account_key: config.account_key,
46            client_secret: config.client_secret,
47            tenant_id: config.tenant_id,
48            client_id: config.client_id,
49            sas_token: config.sas_token,
50            authority_host: config.authority_host,
51            ..Default::default()
52        }
53    }
54}
55
56/// Azure Data Lake Storage Gen2 Support.
57#[doc = include_str!("docs.md")]
58#[derive(Default, Clone)]
59pub struct AzdlsBuilder {
60    pub(super) config: AzdlsConfig,
61
62    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
63    pub(super) http_client: Option<HttpClient>,
64}
65
66impl Debug for AzdlsBuilder {
67    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
68        let mut ds = f.debug_struct("AzdlsBuilder");
69
70        ds.field("config", &self.config);
71
72        ds.finish()
73    }
74}
75
76impl AzdlsBuilder {
77    /// Set root of this backend.
78    ///
79    /// All operations will happen under this root.
80    pub fn root(mut self, root: &str) -> Self {
81        self.config.root = if root.is_empty() {
82            None
83        } else {
84            Some(root.to_string())
85        };
86
87        self
88    }
89
90    /// Set filesystem name of this backend.
91    pub fn filesystem(mut self, filesystem: &str) -> Self {
92        self.config.filesystem = filesystem.to_string();
93
94        self
95    }
96
97    /// Set endpoint of this backend.
98    ///
99    /// Endpoint must be full uri, e.g.
100    ///
101    /// - Azblob: `https://accountname.blob.core.windows.net`
102    /// - Azurite: `http://127.0.0.1:10000/devstoreaccount1`
103    pub fn endpoint(mut self, endpoint: &str) -> Self {
104        if !endpoint.is_empty() {
105            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
106            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
107        }
108
109        self
110    }
111
112    /// Set account_name of this backend.
113    ///
114    /// - If account_name is set, we will take user's input first.
115    /// - If not, we will try to load it from environment.
116    pub fn account_name(mut self, account_name: &str) -> Self {
117        if !account_name.is_empty() {
118            self.config.account_name = Some(account_name.to_string());
119        }
120
121        self
122    }
123
124    /// Set account_key of this backend.
125    ///
126    /// - If account_key is set, we will take user's input first.
127    /// - If not, we will try to load it from environment.
128    pub fn account_key(mut self, account_key: &str) -> Self {
129        if !account_key.is_empty() {
130            self.config.account_key = Some(account_key.to_string());
131        }
132
133        self
134    }
135
136    /// Set client_secret of this backend.
137    ///
138    /// - If client_secret is set, we will take user's input first.
139    /// - If not, we will try to load it from environment.
140    /// - required for client_credentials authentication
141    pub fn client_secret(mut self, client_secret: &str) -> Self {
142        if !client_secret.is_empty() {
143            self.config.client_secret = Some(client_secret.to_string());
144        }
145
146        self
147    }
148
149    /// Set tenant_id of this backend.
150    ///
151    /// - If tenant_id is set, we will take user's input first.
152    /// - If not, we will try to load it from environment.
153    /// - required for client_credentials authentication
154    pub fn tenant_id(mut self, tenant_id: &str) -> Self {
155        if !tenant_id.is_empty() {
156            self.config.tenant_id = Some(tenant_id.to_string());
157        }
158
159        self
160    }
161
162    /// Set client_id of this backend.
163    ///
164    /// - If client_id is set, we will take user's input first.
165    /// - If not, we will try to load it from environment.
166    /// - required for client_credentials authentication
167    pub fn client_id(mut self, client_id: &str) -> Self {
168        if !client_id.is_empty() {
169            self.config.client_id = Some(client_id.to_string());
170        }
171
172        self
173    }
174
175    /// Set the sas_token of this backend.
176    pub fn sas_token(mut self, sas_token: &str) -> Self {
177        if !sas_token.is_empty() {
178            self.config.sas_token = Some(sas_token.to_string());
179        }
180
181        self
182    }
183
184    /// Set authority_host of this backend.
185    ///
186    /// - If authority_host is set, we will take user's input first.
187    /// - If not, we will try to load it from environment.
188    /// - default value: `https://login.microsoftonline.com`
189    pub fn authority_host(mut self, authority_host: &str) -> Self {
190        if !authority_host.is_empty() {
191            self.config.authority_host = Some(authority_host.to_string());
192        }
193
194        self
195    }
196
197    /// Specify the http client that used by this service.
198    ///
199    /// # Notes
200    ///
201    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
202    /// during minor updates.
203    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
204    #[allow(deprecated)]
205    pub fn http_client(mut self, client: HttpClient) -> Self {
206        self.http_client = Some(client);
207        self
208    }
209
210    /// Create a new `AzdlsBuilder` instance from an [Azure Storage connection string][1].
211    ///
212    /// [1]: https://learn.microsoft.com/en-us/azure/storage/common/storage-configure-connection-string
213    ///
214    /// # Example
215    /// ```
216    /// use opendal::Builder;
217    /// use opendal::services::Azdls;
218    ///
219    /// let conn_str = "AccountName=example;DefaultEndpointsProtocol=https;EndpointSuffix=core.windows.net";
220    ///
221    /// let mut config = Azdls::from_connection_string(&conn_str)
222    ///     .unwrap()
223    ///     // Add additional configuration if needed
224    ///     .filesystem("myFilesystem")
225    ///     .client_id("myClientId")
226    ///     .client_secret("myClientSecret")
227    ///     .tenant_id("myTenantId")
228    ///     .build()
229    ///     .unwrap();
230    /// ```
231    pub fn from_connection_string(conn_str: &str) -> Result<Self> {
232        let config =
233            raw::azure_config_from_connection_string(conn_str, raw::AzureStorageService::Adls)?;
234
235        Ok(AzdlsConfig::from(config).into_builder())
236    }
237}
238
239impl Builder for AzdlsBuilder {
240    type Config = AzdlsConfig;
241
242    fn build(self) -> Result<impl Access> {
243        debug!("backend build started: {:?}", &self);
244
245        let root = normalize_root(&self.config.root.unwrap_or_default());
246        debug!("backend use root {root}");
247
248        // Handle endpoint, region and container name.
249        let filesystem = match self.config.filesystem.is_empty() {
250            false => Ok(&self.config.filesystem),
251            true => Err(Error::new(ErrorKind::ConfigInvalid, "filesystem is empty")
252                .with_operation("Builder::build")
253                .with_context("service", Scheme::Azdls)),
254        }?;
255        debug!("backend use filesystem {}", &filesystem);
256
257        let endpoint = match &self.config.endpoint {
258            Some(endpoint) => Ok(endpoint.clone().trim_end_matches('/').to_string()),
259            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
260                .with_operation("Builder::build")
261                .with_context("service", Scheme::Azdls)),
262        }?;
263        debug!("backend use endpoint {}", &endpoint);
264
265        let config_loader = AzureStorageConfig {
266            account_name: self
267                .config
268                .account_name
269                .clone()
270                .or_else(|| raw::azure_account_name_from_endpoint(endpoint.as_str())),
271            account_key: self.config.account_key.clone(),
272            sas_token: self.config.sas_token,
273            client_id: self.config.client_id.clone(),
274            client_secret: self.config.client_secret.clone(),
275            tenant_id: self.config.tenant_id.clone(),
276            authority_host: self.config.authority_host.clone(),
277            ..Default::default()
278        };
279
280        let cred_loader = AzureStorageLoader::new(config_loader);
281        let signer = AzureStorageSigner::new();
282        Ok(AzdlsBackend {
283            core: Arc::new(AzdlsCore {
284                info: {
285                    let am = AccessorInfo::default();
286                    am.set_scheme(AZDLS_SCHEME)
287                        .set_root(&root)
288                        .set_name(filesystem)
289                        .set_native_capability(Capability {
290                            stat: true,
291
292                            read: true,
293
294                            write: true,
295                            write_can_append: true,
296                            write_with_if_none_match: true,
297                            write_with_if_not_exists: true,
298
299                            create_dir: true,
300                            delete: true,
301                            rename: true,
302
303                            list: true,
304
305                            shared: true,
306
307                            ..Default::default()
308                        });
309
310                    // allow deprecated api here for compatibility
311                    #[allow(deprecated)]
312                    if let Some(client) = self.http_client {
313                        am.update_http_client(|_| client);
314                    }
315
316                    am.into()
317                },
318                filesystem: self.config.filesystem.clone(),
319                root,
320                endpoint,
321                loader: cred_loader,
322                signer,
323            }),
324        })
325    }
326}
327
328/// Backend for azblob services.
329#[derive(Debug, Clone)]
330pub struct AzdlsBackend {
331    core: Arc<AzdlsCore>,
332}
333
334impl Access for AzdlsBackend {
335    type Reader = HttpBody;
336    type Writer = AzdlsWriters;
337    type Lister = oio::PageLister<AzdlsLister>;
338    type Deleter = oio::OneShotDeleter<AzdlsDeleter>;
339
340    fn info(&self) -> Arc<AccessorInfo> {
341        self.core.info.clone()
342    }
343
344    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
345        let resp = self
346            .core
347            .azdls_create(path, DIRECTORY, &OpWrite::default())
348            .await?;
349
350        let status = resp.status();
351        match status {
352            StatusCode::CREATED | StatusCode::OK => Ok(RpCreateDir::default()),
353            _ => Err(parse_error(resp)),
354        }
355    }
356
357    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
358        // Stat root always returns a DIR.
359        // TODO: include metadata for the root (#4746)
360        if path == "/" {
361            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
362        }
363
364        let metadata = self.core.azdls_stat_metadata(path).await?;
365        Ok(RpStat::new(metadata))
366    }
367
368    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
369        let resp = self.core.azdls_read(path, args.range()).await?;
370
371        let status = resp.status();
372        match status {
373            StatusCode::OK | StatusCode::PARTIAL_CONTENT => Ok((RpRead::new(), resp.into_body())),
374            _ => {
375                let (part, mut body) = resp.into_parts();
376                let buf = body.to_buffer().await?;
377                Err(parse_error(Response::from_parts(part, buf)))
378            }
379        }
380    }
381
382    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
383        let w = AzdlsWriter::new(self.core.clone(), args.clone(), path.to_string());
384        let w = if args.append() {
385            AzdlsWriters::Two(oio::AppendWriter::new(w))
386        } else {
387            AzdlsWriters::One(oio::OneShotWriter::new(w))
388        };
389        Ok((RpWrite::default(), w))
390    }
391
392    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
393        Ok((
394            RpDelete::default(),
395            oio::OneShotDeleter::new(AzdlsDeleter::new(self.core.clone())),
396        ))
397    }
398
399    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
400        let l = AzdlsLister::new(self.core.clone(), path.to_string(), args.limit());
401
402        Ok((RpList::default(), oio::PageLister::new(l)))
403    }
404
405    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
406        if let Some(resp) = self.core.azdls_ensure_parent_path(to).await? {
407            let status = resp.status();
408            match status {
409                StatusCode::CREATED | StatusCode::CONFLICT => {}
410                _ => return Err(parse_error(resp)),
411            }
412        }
413
414        let resp = self.core.azdls_rename(from, to).await?;
415
416        let status = resp.status();
417
418        match status {
419            StatusCode::CREATED => Ok(RpRename::default()),
420            _ => Err(parse_error(resp)),
421        }
422    }
423}
```
