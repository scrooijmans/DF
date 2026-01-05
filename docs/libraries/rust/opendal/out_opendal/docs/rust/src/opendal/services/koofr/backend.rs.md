# 

opendal/services/koofr/

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
22use bytes::Buf;
23use http::Response;
24use http::StatusCode;
25use log::debug;
26use tokio::sync::Mutex;
27use tokio::sync::OnceCell;
28
29use super::KOOFR_SCHEME;
30use super::core::File;
31use super::core::KoofrCore;
32use super::core::KoofrSigner;
33use super::delete::KoofrDeleter;
34use super::error::parse_error;
35use super::lister::KoofrLister;
36use super::writer::KoofrWriter;
37use super::writer::KoofrWriters;
38use crate::raw::*;
39use crate::services::KoofrConfig;
40use crate::*;
41
42/// [Koofr](https://app.koofr.net/) services support.
43#[doc = include_str!("docs.md")]
44#[derive(Default)]
45pub struct KoofrBuilder {
46    pub(super) config: KoofrConfig,
47
48    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
49    pub(super) http_client: Option<HttpClient>,
50}
51
52impl Debug for KoofrBuilder {
53    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
54        let mut d = f.debug_struct("KoofrBuilder");
55
56        d.field("config", &self.config);
57        d.finish_non_exhaustive()
58    }
59}
60
61impl KoofrBuilder {
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
75    /// endpoint.
76    ///
77    /// It is required. e.g. `https://api.koofr.net/`
78    pub fn endpoint(mut self, endpoint: &str) -> Self {
79        self.config.endpoint = endpoint.to_string();
80
81        self
82    }
83
84    /// email.
85    ///
86    /// It is required. e.g. `test@example.com`
87    pub fn email(mut self, email: &str) -> Self {
88        self.config.email = email.to_string();
89
90        self
91    }
92
93    /// Koofr application password.
94    ///
95    /// Go to <https://app.koofr.net/app/admin/preferences/password>.
96    /// Click "Generate Password" button to generate a new application password.
97    ///
98    /// # Notes
99    ///
100    /// This is not user's Koofr account password.
101    /// Please use the application password instead.
102    /// Please also remind users of this.
103    pub fn password(mut self, password: &str) -> Self {
104        self.config.password = if password.is_empty() {
105            None
106        } else {
107            Some(password.to_string())
108        };
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
127impl Builder for KoofrBuilder {
128    type Config = KoofrConfig;
129
130    /// Builds the backend and returns the result of KoofrBackend.
131    fn build(self) -> Result<impl Access> {
132        debug!("backend build started: {:?}", &self);
133
134        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
135        debug!("backend use root {}", &root);
136
137        if self.config.endpoint.is_empty() {
138            return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
139                .with_operation("Builder::build")
140                .with_context("service", Scheme::Koofr));
141        }
142
143        debug!("backend use endpoint {}", &self.config.endpoint);
144
145        if self.config.email.is_empty() {
146            return Err(Error::new(ErrorKind::ConfigInvalid, "email is empty")
147                .with_operation("Builder::build")
148                .with_context("service", Scheme::Koofr));
149        }
150
151        debug!("backend use email {}", &self.config.email);
152
153        let password = match &self.config.password {
154            Some(password) => Ok(password.clone()),
155            None => Err(Error::new(ErrorKind::ConfigInvalid, "password is empty")
156                .with_operation("Builder::build")
157                .with_context("service", Scheme::Koofr)),
158        }?;
159
160        let signer = Arc::new(Mutex::new(KoofrSigner::default()));
161
162        Ok(KoofrBackend {
163            core: Arc::new(KoofrCore {
164                info: {
165                    let am = AccessorInfo::default();
166                    am.set_scheme(KOOFR_SCHEME)
167                        .set_root(&root)
168                        .set_native_capability(Capability {
169                            stat: true,
170
171                            create_dir: true,
172
173                            read: true,
174
175                            write: true,
176                            write_can_empty: true,
177
178                            delete: true,
179
180                            rename: true,
181
182                            copy: true,
183
184                            list: true,
185
186                            shared: true,
187
188                            ..Default::default()
189                        });
190
191                    // allow deprecated api here for compatibility
192                    #[allow(deprecated)]
193                    if let Some(client) = self.http_client {
194                        am.update_http_client(|_| client);
195                    }
196
197                    am.into()
198                },
199                root,
200                endpoint: self.config.endpoint.clone(),
201                email: self.config.email.clone(),
202                password,
203                mount_id: OnceCell::new(),
204                signer,
205            }),
206        })
207    }
208}
209
210/// Backend for Koofr services.
211#[derive(Debug, Clone)]
212pub struct KoofrBackend {
213    core: Arc<KoofrCore>,
214}
215
216impl Access for KoofrBackend {
217    type Reader = HttpBody;
218    type Writer = KoofrWriters;
219    type Lister = oio::PageLister<KoofrLister>;
220    type Deleter = oio::OneShotDeleter<KoofrDeleter>;
221
222    fn info(&self) -> Arc<AccessorInfo> {
223        self.core.info.clone()
224    }
225
226    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
227        self.core.ensure_dir_exists(path).await?;
228        self.core
229            .create_dir(&build_abs_path(&self.core.root, path))
230            .await?;
231        Ok(RpCreateDir::default())
232    }
233
234    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
235        let path = build_rooted_abs_path(&self.core.root, path);
236        let resp = self.core.info(&path).await?;
237
238        let status = resp.status();
239
240        match status {
241            StatusCode::OK => {
242                let bs = resp.into_body();
243
244                let file: File =
245                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
246
247                let mode = if file.ty == "dir" {
248                    EntryMode::DIR
249                } else {
250                    EntryMode::FILE
251                };
252
253                let mut md = Metadata::new(mode);
254
255                md.set_content_length(file.size)
256                    .set_content_type(&file.content_type)
257                    .set_last_modified(Timestamp::from_millisecond(file.modified)?);
258
259                Ok(RpStat::new(md))
260            }
261            _ => Err(parse_error(resp)),
262        }
263    }
264
265    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
266        let resp = self.core.get(path, args.range()).await?;
267
268        let status = resp.status();
269        match status {
270            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
271                Ok((RpRead::default(), resp.into_body()))
272            }
273            _ => {
274                let (part, mut body) = resp.into_parts();
275                let buf = body.to_buffer().await?;
276                Err(parse_error(Response::from_parts(part, buf)))
277            }
278        }
279    }
280
281    async fn write(&self, path: &str, _args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
282        let writer = KoofrWriter::new(self.core.clone(), path.to_string());
283
284        let w = oio::OneShotWriter::new(writer);
285
286        Ok((RpWrite::default(), w))
287    }
288
289    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
290        Ok((
291            RpDelete::default(),
292            oio::OneShotDeleter::new(KoofrDeleter::new(self.core.clone())),
293        ))
294    }
295
296    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
297        let l = KoofrLister::new(self.core.clone(), path);
298        Ok((RpList::default(), oio::PageLister::new(l)))
299    }
300
301    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
302        self.core.ensure_dir_exists(to).await?;
303
304        if from == to {
305            return Ok(RpCopy::default());
306        }
307
308        let resp = self.core.remove(to).await?;
309
310        let status = resp.status();
311
312        if status != StatusCode::OK && status != StatusCode::NOT_FOUND {
313            return Err(parse_error(resp));
314        }
315
316        let resp = self.core.copy(from, to).await?;
317
318        let status = resp.status();
319
320        match status {
321            StatusCode::OK => Ok(RpCopy::default()),
322            _ => Err(parse_error(resp)),
323        }
324    }
325
326    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
327        self.core.ensure_dir_exists(to).await?;
328
329        if from == to {
330            return Ok(RpRename::default());
331        }
332
333        let resp = self.core.remove(to).await?;
334
335        let status = resp.status();
336
337        if status != StatusCode::OK && status != StatusCode::NOT_FOUND {
338            return Err(parse_error(resp));
339        }
340
341        let resp = self.core.move_object(from, to).await?;
342
343        let status = resp.status();
344
345        match status {
346            StatusCode::OK => Ok(RpRename::default()),
347            _ => Err(parse_error(resp)),
348        }
349    }
350}
```
