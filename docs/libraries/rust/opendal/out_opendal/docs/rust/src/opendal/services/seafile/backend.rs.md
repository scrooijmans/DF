# 

opendal/services/seafile/

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
25use tokio::sync::RwLock;
26
27use super::SEAFILE_SCHEME;
28use super::core::SeafileCore;
29use super::core::SeafileSigner;
30use super::core::parse_dir_detail;
31use super::core::parse_file_detail;
32use super::delete::SeafileDeleter;
33use super::error::parse_error;
34use super::lister::SeafileLister;
35use super::writer::SeafileWriter;
36use super::writer::SeafileWriters;
37use crate::raw::*;
38use crate::services::SeafileConfig;
39use crate::*;
40
41/// [seafile](https://www.seafile.com) services support.
42#[doc = include_str!("docs.md")]
43#[derive(Default)]
44pub struct SeafileBuilder {
45    pub(super) config: SeafileConfig,
46
47    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
48    pub(super) http_client: Option<HttpClient>,
49}
50
51impl Debug for SeafileBuilder {
52    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
53        let mut d = f.debug_struct("SeafileBuilder");
54
55        d.field("config", &self.config);
56        d.finish_non_exhaustive()
57    }
58}
59
60impl SeafileBuilder {
61    /// Set root of this backend.
62    ///
63    /// All operations will happen under this root.
64    pub fn root(mut self, root: &str) -> Self {
65        self.config.root = if root.is_empty() {
66            None
67        } else {
68            Some(root.to_string())
69        };
70
71        self
72    }
73
74    /// endpoint of this backend.
75    ///
76    /// It is required. e.g. `http://127.0.0.1:80`
77    pub fn endpoint(mut self, endpoint: &str) -> Self {
78        self.config.endpoint = if endpoint.is_empty() {
79            None
80        } else {
81            Some(endpoint.to_string())
82        };
83
84        self
85    }
86
87    /// username of this backend.
88    ///
89    /// It is required. e.g. `me@example.com`
90    pub fn username(mut self, username: &str) -> Self {
91        self.config.username = if username.is_empty() {
92            None
93        } else {
94            Some(username.to_string())
95        };
96
97        self
98    }
99
100    /// password of this backend.
101    ///
102    /// It is required. e.g. `asecret`
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
113    /// Set repo name of this backend.
114    ///
115    /// It is required. e.g. `myrepo`
116    pub fn repo_name(mut self, repo_name: &str) -> Self {
117        self.config.repo_name = repo_name.to_string();
118
119        self
120    }
121
122    /// Specify the http client that used by this service.
123    ///
124    /// # Notes
125    ///
126    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
127    /// during minor updates.
128    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
129    #[allow(deprecated)]
130    pub fn http_client(mut self, client: HttpClient) -> Self {
131        self.http_client = Some(client);
132        self
133    }
134}
135
136impl Builder for SeafileBuilder {
137    type Config = SeafileConfig;
138
139    /// Builds the backend and returns the result of SeafileBackend.
140    fn build(self) -> Result<impl Access> {
141        debug!("backend build started: {:?}", &self);
142
143        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
144        debug!("backend use root {}", &root);
145
146        // Handle bucket.
147        if self.config.repo_name.is_empty() {
148            return Err(Error::new(ErrorKind::ConfigInvalid, "repo_name is empty")
149                .with_operation("Builder::build")
150                .with_context("service", Scheme::Seafile));
151        }
152
153        debug!("backend use repo_name {}", &self.config.repo_name);
154
155        let endpoint = match &self.config.endpoint {
156            Some(endpoint) => Ok(endpoint.clone()),
157            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
158                .with_operation("Builder::build")
159                .with_context("service", Scheme::Seafile)),
160        }?;
161
162        let username = match &self.config.username {
163            Some(username) => Ok(username.clone()),
164            None => Err(Error::new(ErrorKind::ConfigInvalid, "username is empty")
165                .with_operation("Builder::build")
166                .with_context("service", Scheme::Seafile)),
167        }?;
168
169        let password = match &self.config.password {
170            Some(password) => Ok(password.clone()),
171            None => Err(Error::new(ErrorKind::ConfigInvalid, "password is empty")
172                .with_operation("Builder::build")
173                .with_context("service", Scheme::Seafile)),
174        }?;
175
176        Ok(SeafileBackend {
177            core: Arc::new(SeafileCore {
178                info: {
179                    let am = AccessorInfo::default();
180                    am.set_scheme(SEAFILE_SCHEME)
181                        .set_root(&root)
182                        .set_native_capability(Capability {
183                            stat: true,
184
185                            read: true,
186
187                            write: true,
188                            write_can_empty: true,
189
190                            delete: true,
191
192                            list: true,
193
194                            shared: true,
195
196                            ..Default::default()
197                        });
198
199                    // allow deprecated api here for compatibility
200                    #[allow(deprecated)]
201                    if let Some(client) = self.http_client {
202                        am.update_http_client(|_| client);
203                    }
204
205                    am.into()
206                },
207                root,
208                endpoint,
209                username,
210                password,
211                repo_name: self.config.repo_name.clone(),
212                signer: Arc::new(RwLock::new(SeafileSigner::default())),
213            }),
214        })
215    }
216}
217
218/// Backend for seafile services.
219#[derive(Debug, Clone)]
220pub struct SeafileBackend {
221    core: Arc<SeafileCore>,
222}
223
224impl Access for SeafileBackend {
225    type Reader = HttpBody;
226    type Writer = SeafileWriters;
227    type Lister = oio::PageLister<SeafileLister>;
228    type Deleter = oio::OneShotDeleter<SeafileDeleter>;
229
230    fn info(&self) -> Arc<AccessorInfo> {
231        self.core.info.clone()
232    }
233
234    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
235        if path == "/" {
236            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
237        }
238
239        let metadata = if path.ends_with('/') {
240            let dir_detail = self.core.dir_detail(path).await?;
241            parse_dir_detail(dir_detail)
242        } else {
243            let file_detail = self.core.file_detail(path).await?;
244
245            parse_file_detail(file_detail)
246        };
247
248        metadata.map(RpStat::new)
249    }
250
251    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
252        let resp = self.core.download_file(path, args.range()).await?;
253
254        let status = resp.status();
255
256        match status {
257            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
258                Ok((RpRead::default(), resp.into_body()))
259            }
260            _ => {
261                let (part, mut body) = resp.into_parts();
262                let buf = body.to_buffer().await?;
263                Err(parse_error(Response::from_parts(part, buf)))
264            }
265        }
266    }
267
268    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
269        let w = SeafileWriter::new(self.core.clone(), args, path.to_string());
270        let w = oio::OneShotWriter::new(w);
271
272        Ok((RpWrite::default(), w))
273    }
274
275    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
276        Ok((
277            RpDelete::default(),
278            oio::OneShotDeleter::new(SeafileDeleter::new(self.core.clone())),
279        ))
280    }
281
282    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
283        let l = SeafileLister::new(self.core.clone(), path);
284        Ok((RpList::default(), oio::PageLister::new(l)))
285    }
286}
```
