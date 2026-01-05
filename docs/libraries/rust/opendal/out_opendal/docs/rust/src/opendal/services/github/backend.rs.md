# 

opendal/services/github/

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
26
27use super::GITHUB_SCHEME;
28use super::core::Entry;
29use super::core::GithubCore;
30use super::delete::GithubDeleter;
31use super::error::parse_error;
32use super::lister::GithubLister;
33use super::writer::GithubWriter;
34use super::writer::GithubWriters;
35use crate::raw::*;
36use crate::services::GithubConfig;
37use crate::*;
38
39/// [github contents](https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#create-or-update-file-contents) services support.
40#[doc = include_str!("docs.md")]
41#[derive(Default)]
42pub struct GithubBuilder {
43    pub(super) config: GithubConfig,
44
45    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
46    pub(super) http_client: Option<HttpClient>,
47}
48
49impl Debug for GithubBuilder {
50    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
51        let mut d = f.debug_struct("GithubBuilder");
52
53        d.field("config", &self.config);
54        d.finish_non_exhaustive()
55    }
56}
57
58impl GithubBuilder {
59    /// Set root of this backend.
60    ///
61    /// All operations will happen under this root.
62    pub fn root(mut self, root: &str) -> Self {
63        self.config.root = if root.is_empty() {
64            None
65        } else {
66            Some(root.to_string())
67        };
68
69        self
70    }
71
72    /// Github access_token.
73    ///
74    /// required.
75    pub fn token(mut self, token: &str) -> Self {
76        if !token.is_empty() {
77            self.config.token = Some(token.to_string());
78        }
79        self
80    }
81
82    /// Set Github repo owner.
83    pub fn owner(mut self, owner: &str) -> Self {
84        self.config.owner = owner.to_string();
85
86        self
87    }
88
89    /// Set Github repo name.
90    pub fn repo(mut self, repo: &str) -> Self {
91        self.config.repo = repo.to_string();
92
93        self
94    }
95
96    /// Specify the http client that used by this service.
97    ///
98    /// # Notes
99    ///
100    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
101    /// during minor updates.
102    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
103    #[allow(deprecated)]
104    pub fn http_client(mut self, client: HttpClient) -> Self {
105        self.http_client = Some(client);
106        self
107    }
108}
109
110impl Builder for GithubBuilder {
111    type Config = GithubConfig;
112
113    /// Builds the backend and returns the result of GithubBackend.
114    fn build(self) -> Result<impl Access> {
115        debug!("backend build started: {:?}", &self);
116
117        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
118        debug!("backend use root {}", &root);
119
120        // Handle owner.
121        if self.config.owner.is_empty() {
122            return Err(Error::new(ErrorKind::ConfigInvalid, "owner is empty")
123                .with_operation("Builder::build")
124                .with_context("service", Scheme::Github));
125        }
126
127        debug!("backend use owner {}", &self.config.owner);
128
129        // Handle repo.
130        if self.config.repo.is_empty() {
131            return Err(Error::new(ErrorKind::ConfigInvalid, "repo is empty")
132                .with_operation("Builder::build")
133                .with_context("service", Scheme::Github));
134        }
135
136        debug!("backend use repo {}", &self.config.repo);
137
138        Ok(GithubBackend {
139            core: Arc::new(GithubCore {
140                info: {
141                    let am = AccessorInfo::default();
142                    am.set_scheme(GITHUB_SCHEME)
143                        .set_root(&root)
144                        .set_native_capability(Capability {
145                            stat: true,
146
147                            read: true,
148
149                            create_dir: true,
150
151                            write: true,
152                            write_can_empty: true,
153
154                            delete: true,
155
156                            list: true,
157                            list_with_recursive: true,
158
159                            shared: true,
160
161                            ..Default::default()
162                        });
163
164                    // allow deprecated api here for compatibility
165                    #[allow(deprecated)]
166                    if let Some(client) = self.http_client {
167                        am.update_http_client(|_| client);
168                    }
169
170                    am.into()
171                },
172                root,
173                token: self.config.token.clone(),
174                owner: self.config.owner.clone(),
175                repo: self.config.repo.clone(),
176            }),
177        })
178    }
179}
180
181/// Backend for Github services.
182#[derive(Debug, Clone)]
183pub struct GithubBackend {
184    core: Arc<GithubCore>,
185}
186
187impl Access for GithubBackend {
188    type Reader = HttpBody;
189    type Writer = GithubWriters;
190    type Lister = oio::PageLister<GithubLister>;
191    type Deleter = oio::OneShotDeleter<GithubDeleter>;
192
193    fn info(&self) -> Arc<AccessorInfo> {
194        self.core.info.clone()
195    }
196
197    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
198        let empty_bytes = Buffer::new();
199
200        let resp = self
201            .core
202            .upload(&format!("{path}.gitkeep"), empty_bytes)
203            .await?;
204
205        let status = resp.status();
206
207        match status {
208            StatusCode::OK | StatusCode::CREATED => Ok(RpCreateDir::default()),
209            _ => Err(parse_error(resp)),
210        }
211    }
212
213    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
214        let resp = self.core.stat(path).await?;
215
216        let status = resp.status();
217
218        match status {
219            StatusCode::OK => {
220                let body = resp.into_body();
221                let resp: Entry =
222                    serde_json::from_reader(body.reader()).map_err(new_json_deserialize_error)?;
223
224                let m = if resp.type_field == "dir" {
225                    Metadata::new(EntryMode::DIR)
226                } else {
227                    Metadata::new(EntryMode::FILE)
228                        .with_content_length(resp.size)
229                        .with_etag(resp.sha)
230                };
231
232                Ok(RpStat::new(m))
233            }
234            _ => Err(parse_error(resp)),
235        }
236    }
237
238    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
239        let resp = self.core.get(path, args.range()).await?;
240
241        let status = resp.status();
242
243        match status {
244            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
245                Ok((RpRead::default(), resp.into_body()))
246            }
247            _ => {
248                let (part, mut body) = resp.into_parts();
249                let buf = body.to_buffer().await?;
250                Err(parse_error(Response::from_parts(part, buf)))
251            }
252        }
253    }
254
255    async fn write(&self, path: &str, _args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
256        let writer = GithubWriter::new(self.core.clone(), path.to_string());
257
258        let w = oio::OneShotWriter::new(writer);
259
260        Ok((RpWrite::default(), w))
261    }
262
263    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
264        Ok((
265            RpDelete::default(),
266            oio::OneShotDeleter::new(GithubDeleter::new(self.core.clone())),
267        ))
268    }
269
270    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
271        let l = GithubLister::new(self.core.clone(), path, args.recursive());
272        Ok((RpList::default(), oio::PageLister::new(l)))
273    }
274}
```
