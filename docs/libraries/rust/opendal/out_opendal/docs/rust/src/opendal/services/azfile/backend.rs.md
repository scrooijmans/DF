# 

opendal/services/azfile/

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
29use super::AZFILE_SCHEME;
30use super::core::AzfileCore;
31use super::delete::AzfileDeleter;
32use super::error::parse_error;
33use super::lister::AzfileLister;
34use super::writer::AzfileWriter;
35use super::writer::AzfileWriters;
36use crate::raw::*;
37use crate::services::AzfileConfig;
38use crate::*;
39impl From<AzureStorageConfig> for AzfileConfig {
40    fn from(config: AzureStorageConfig) -> Self {
41        AzfileConfig {
42            account_name: config.account_name,
43            account_key: config.account_key,
44            sas_token: config.sas_token,
45            endpoint: config.endpoint,
46            root: None,                // root is not part of AzureStorageConfig
47            share_name: String::new(), // share_name is not part of AzureStorageConfig
48        }
49    }
50}
51
52/// Azure File services support.
53#[doc = include_str!("docs.md")]
54#[derive(Default, Clone)]
55pub struct AzfileBuilder {
56    pub(super) config: AzfileConfig,
57
58    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
59    pub(super) http_client: Option<HttpClient>,
60}
61
62impl Debug for AzfileBuilder {
63    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
64        let mut ds = f.debug_struct("AzfileBuilder");
65
66        ds.field("config", &self.config);
67
68        ds.finish()
69    }
70}
71
72impl AzfileBuilder {
73    /// Set root of this backend.
74    ///
75    /// All operations will happen under this root.
76    pub fn root(mut self, root: &str) -> Self {
77        self.config.root = if root.is_empty() {
78            None
79        } else {
80            Some(root.to_string())
81        };
82
83        self
84    }
85
86    /// Set endpoint of this backend.
87    pub fn endpoint(mut self, endpoint: &str) -> Self {
88        if !endpoint.is_empty() {
89            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
90            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
91        }
92
93        self
94    }
95
96    /// Set account_name of this backend.
97    ///
98    /// - If account_name is set, we will take user's input first.
99    /// - If not, we will try to load it from environment.
100    pub fn account_name(mut self, account_name: &str) -> Self {
101        if !account_name.is_empty() {
102            self.config.account_name = Some(account_name.to_string());
103        }
104
105        self
106    }
107
108    /// Set account_key of this backend.
109    ///
110    /// - If account_key is set, we will take user's input first.
111    /// - If not, we will try to load it from environment.
112    pub fn account_key(mut self, account_key: &str) -> Self {
113        if !account_key.is_empty() {
114            self.config.account_key = Some(account_key.to_string());
115        }
116
117        self
118    }
119
120    /// Set file share name of this backend.
121    ///
122    /// # Notes
123    /// You can find more about from: <https://learn.microsoft.com/en-us/rest/api/storageservices/operations-on-shares--file-service>
124    pub fn share_name(mut self, share_name: &str) -> Self {
125        if !share_name.is_empty() {
126            self.config.share_name = share_name.to_string();
127        }
128
129        self
130    }
131
132    /// Specify the http client that used by this service.
133    ///
134    /// # Notes
135    ///
136    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
137    /// during minor updates.
138    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
139    #[allow(deprecated)]
140    pub fn http_client(mut self, client: HttpClient) -> Self {
141        self.http_client = Some(client);
142        self
143    }
144
145    /// Create a new `AfileBuilder` instance from an [Azure Storage connection string][1].
146    ///
147    /// [1]: https://learn.microsoft.com/en-us/azure/storage/common/storage-configure-connection-string
148    ///
149    /// # Example
150    /// ```
151    /// use opendal::Builder;
152    /// use opendal::services::Azfile;
153    ///
154    /// let conn_str = "AccountName=example;DefaultEndpointsProtocol=https;EndpointSuffix=core.windows.net";
155    ///
156    /// let mut config = Azfile::from_connection_string(&conn_str)
157    ///     .unwrap()
158    ///     // Add additional configuration if needed
159    ///     .share_name("myShare")
160    ///     .build()
161    ///     .unwrap();
162    /// ```
163    pub fn from_connection_string(conn_str: &str) -> Result<Self> {
164        let config =
165            raw::azure_config_from_connection_string(conn_str, raw::AzureStorageService::File)?;
166
167        Ok(AzfileConfig::from(config).into_builder())
168    }
169}
170
171impl Builder for AzfileBuilder {
172    type Config = AzfileConfig;
173
174    fn build(self) -> Result<impl Access> {
175        debug!("backend build started: {:?}", &self);
176
177        let root = normalize_root(&self.config.root.unwrap_or_default());
178        debug!("backend use root {root}");
179
180        let endpoint = match &self.config.endpoint {
181            Some(endpoint) => Ok(endpoint.clone()),
182            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
183                .with_operation("Builder::build")
184                .with_context("service", Scheme::Azfile)),
185        }?;
186        debug!("backend use endpoint {}", &endpoint);
187
188        let account_name_option = self
189            .config
190            .account_name
191            .clone()
192            .or_else(|| raw::azure_account_name_from_endpoint(endpoint.as_str()));
193
194        let account_name = match account_name_option {
195            Some(account_name) => Ok(account_name),
196            None => Err(
197                Error::new(ErrorKind::ConfigInvalid, "account_name is empty")
198                    .with_operation("Builder::build")
199                    .with_context("service", Scheme::Azfile),
200            ),
201        }?;
202
203        let config_loader = AzureStorageConfig {
204            account_name: Some(account_name),
205            account_key: self.config.account_key.clone(),
206            sas_token: self.config.sas_token.clone(),
207            ..Default::default()
208        };
209
210        let cred_loader = AzureStorageLoader::new(config_loader);
211        let signer = AzureStorageSigner::new();
212        Ok(AzfileBackend {
213            core: Arc::new(AzfileCore {
214                info: {
215                    let am = AccessorInfo::default();
216                    am.set_scheme(AZFILE_SCHEME)
217                        .set_root(&root)
218                        .set_native_capability(Capability {
219                            stat: true,
220
221                            read: true,
222
223                            write: true,
224                            create_dir: true,
225                            delete: true,
226                            rename: true,
227
228                            list: true,
229
230                            shared: true,
231
232                            ..Default::default()
233                        });
234
235                    // allow deprecated api here for compatibility
236                    #[allow(deprecated)]
237                    if let Some(client) = self.http_client {
238                        am.update_http_client(|_| client);
239                    }
240
241                    am.into()
242                },
243                root,
244                endpoint,
245                loader: cred_loader,
246                signer,
247                share_name: self.config.share_name.clone(),
248            }),
249        })
250    }
251}
252
253/// Backend for azfile services.
254#[derive(Debug, Clone)]
255pub struct AzfileBackend {
256    core: Arc<AzfileCore>,
257}
258
259impl Access for AzfileBackend {
260    type Reader = HttpBody;
261    type Writer = AzfileWriters;
262    type Lister = oio::PageLister<AzfileLister>;
263    type Deleter = oio::OneShotDeleter<AzfileDeleter>;
264
265    fn info(&self) -> Arc<AccessorInfo> {
266        self.core.info.clone()
267    }
268
269    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
270        self.core.ensure_parent_dir_exists(path).await?;
271        let resp = self.core.azfile_create_dir(path).await?;
272        let status = resp.status();
273
274        match status {
275            StatusCode::CREATED => Ok(RpCreateDir::default()),
276            _ => {
277                // we cannot just check status code because 409 Conflict has two meaning:
278                // 1. If a directory by the same name is being deleted when Create Directory is called, the server returns status code 409 (Conflict)
279                // 2. If a directory or file with the same name already exists, the operation fails with status code 409 (Conflict).
280                // but we just need case 2 (already exists)
281                // ref: https://learn.microsoft.com/en-us/rest/api/storageservices/create-directory
282                if resp
283                    .headers()
284                    .get("x-ms-error-code")
285                    .map(|value| value.to_str().unwrap_or(""))
286                    .unwrap_or_else(|| "")
287                    == "ResourceAlreadyExists"
288                {
289                    Ok(RpCreateDir::default())
290                } else {
291                    Err(parse_error(resp))
292                }
293            }
294        }
295    }
296
297    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
298        let resp = if path.ends_with('/') {
299            self.core.azfile_get_directory_properties(path).await?
300        } else {
301            self.core.azfile_get_file_properties(path).await?
302        };
303
304        let status = resp.status();
305        match status {
306            StatusCode::OK => {
307                let meta = parse_into_metadata(path, resp.headers())?;
308                Ok(RpStat::new(meta))
309            }
310            _ => Err(parse_error(resp)),
311        }
312    }
313
314    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
315        let resp = self.core.azfile_read(path, args.range()).await?;
316
317        let status = resp.status();
318        match status {
319            StatusCode::OK | StatusCode::PARTIAL_CONTENT => Ok((RpRead::new(), resp.into_body())),
320            _ => {
321                let (part, mut body) = resp.into_parts();
322                let buf = body.to_buffer().await?;
323                Err(parse_error(Response::from_parts(part, buf)))
324            }
325        }
326    }
327
328    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
329        self.core.ensure_parent_dir_exists(path).await?;
330        let w = AzfileWriter::new(self.core.clone(), args.clone(), path.to_string());
331        let w = if args.append() {
332            AzfileWriters::Two(oio::AppendWriter::new(w))
333        } else {
334            AzfileWriters::One(oio::OneShotWriter::new(w))
335        };
336        Ok((RpWrite::default(), w))
337    }
338
339    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
340        Ok((
341            RpDelete::default(),
342            oio::OneShotDeleter::new(AzfileDeleter::new(self.core.clone())),
343        ))
344    }
345
346    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
347        let l = AzfileLister::new(self.core.clone(), path.to_string(), args.limit());
348
349        Ok((RpList::default(), oio::PageLister::new(l)))
350    }
351
352    async fn rename(&self, from: &str, to: &str, _: OpRename) -> Result<RpRename> {
353        self.core.ensure_parent_dir_exists(to).await?;
354        let resp = self.core.azfile_rename(from, to).await?;
355        let status = resp.status();
356        match status {
357            StatusCode::OK => Ok(RpRename::default()),
358            _ => Err(parse_error(resp)),
359        }
360    }
361}
```
