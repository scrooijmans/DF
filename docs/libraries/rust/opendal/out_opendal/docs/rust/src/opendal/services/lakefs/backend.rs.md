# 

opendal/services/lakefs/

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
27use super::LAKEFS_SCHEME;
28use super::core::LakefsCore;
29use super::core::LakefsStatus;
30use super::delete::LakefsDeleter;
31use super::error::parse_error;
32use super::lister::LakefsLister;
33use super::writer::LakefsWriter;
34use crate::raw::*;
35use crate::services::LakefsConfig;
36use crate::*;
37
38/// [Lakefs](https://docs.lakefs.io/reference/api.html#/)'s API support.
39#[doc = include_str!("docs.md")]
40#[derive(Default, Clone)]
41pub struct LakefsBuilder {
42    pub(super) config: LakefsConfig,
43}
44
45impl Debug for LakefsBuilder {
46    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
47        let mut ds = f.debug_struct("Builder");
48
49        ds.field("config", &self.config);
50        ds.finish()
51    }
52}
53
54impl LakefsBuilder {
55    /// Set the endpoint of this backend.
56    ///
57    /// endpoint must be full uri.
58    ///
59    /// This is required.
60    /// - `http://127.0.0.1:8000` (lakefs daemon in local)
61    /// - `https://my-lakefs.example.com` (lakefs server)
62    pub fn endpoint(mut self, endpoint: &str) -> Self {
63        if !endpoint.is_empty() {
64            self.config.endpoint = Some(endpoint.to_string());
65        }
66        self
67    }
68
69    /// Set username of this backend. This is required.
70    pub fn username(mut self, username: &str) -> Self {
71        if !username.is_empty() {
72            self.config.username = Some(username.to_string());
73        }
74        self
75    }
76
77    /// Set password of this backend. This is required.
78    pub fn password(mut self, password: &str) -> Self {
79        if !password.is_empty() {
80            self.config.password = Some(password.to_string());
81        }
82        self
83    }
84
85    /// Set branch of this backend or a commit ID. Default is main.
86    ///
87    /// Branch can be a branch name.
88    ///
89    /// For example, branch can be:
90    /// - main
91    /// - 1d0c4eb
92    pub fn branch(mut self, branch: &str) -> Self {
93        if !branch.is_empty() {
94            self.config.branch = Some(branch.to_string());
95        }
96        self
97    }
98
99    /// Set root of this backend.
100    ///
101    /// All operations will happen under this root.
102    pub fn root(mut self, root: &str) -> Self {
103        if !root.is_empty() {
104            self.config.root = Some(root.to_string());
105        }
106        self
107    }
108
109    /// Set the repository of this backend.
110    ///
111    /// This is required.
112    pub fn repository(mut self, repository: &str) -> Self {
113        if !repository.is_empty() {
114            self.config.repository = Some(repository.to_string());
115        }
116        self
117    }
118}
119
120impl Builder for LakefsBuilder {
121    type Config = LakefsConfig;
122
123    /// Build a LakefsBackend.
124    fn build(self) -> Result<impl Access> {
125        debug!("backend build started: {:?}", &self);
126
127        let endpoint = match self.config.endpoint {
128            Some(endpoint) => Ok(endpoint.clone()),
129            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
130                .with_operation("Builder::build")
131                .with_context("service", Scheme::Lakefs)),
132        }?;
133        debug!("backend use endpoint: {:?}", &endpoint);
134
135        let repository = match &self.config.repository {
136            Some(repository) => Ok(repository.clone()),
137            None => Err(Error::new(ErrorKind::ConfigInvalid, "repository is empty")
138                .with_operation("Builder::build")
139                .with_context("service", Scheme::Lakefs)),
140        }?;
141        debug!("backend use repository: {}", &repository);
142
143        let branch = match &self.config.branch {
144            Some(branch) => branch.clone(),
145            None => "main".to_string(),
146        };
147        debug!("backend use branch: {}", &branch);
148
149        let root = normalize_root(&self.config.root.unwrap_or_default());
150        debug!("backend use root: {}", &root);
151
152        let username = match &self.config.username {
153            Some(username) => Ok(username.clone()),
154            None => Err(Error::new(ErrorKind::ConfigInvalid, "username is empty")
155                .with_operation("Builder::build")
156                .with_context("service", Scheme::Lakefs)),
157        }?;
158
159        let password = match &self.config.password {
160            Some(password) => Ok(password.clone()),
161            None => Err(Error::new(ErrorKind::ConfigInvalid, "password is empty")
162                .with_operation("Builder::build")
163                .with_context("service", Scheme::Lakefs)),
164        }?;
165
166        Ok(LakefsBackend {
167            core: Arc::new(LakefsCore {
168                info: {
169                    let am = AccessorInfo::default();
170                    am.set_scheme(LAKEFS_SCHEME)
171                        .set_native_capability(Capability {
172                            stat: true,
173
174                            list: true,
175
176                            read: true,
177                            write: true,
178                            delete: true,
179                            copy: true,
180                            shared: true,
181                            ..Default::default()
182                        });
183                    am.into()
184                },
185                endpoint,
186                repository,
187                branch,
188                root,
189                username,
190                password,
191            }),
192        })
193    }
194}
195
196/// Backend for Lakefs service
197#[derive(Debug, Clone)]
198pub struct LakefsBackend {
199    core: Arc<LakefsCore>,
200}
201
202impl Access for LakefsBackend {
203    type Reader = HttpBody;
204    type Writer = oio::OneShotWriter<LakefsWriter>;
205    type Lister = oio::PageLister<LakefsLister>;
206    type Deleter = oio::OneShotDeleter<LakefsDeleter>;
207
208    fn info(&self) -> Arc<AccessorInfo> {
209        self.core.info.clone()
210    }
211
212    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
213        // Stat root always returns a DIR.
214        if path == "/" {
215            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
216        }
217
218        let resp = self.core.get_object_metadata(path).await?;
219
220        let status = resp.status();
221
222        match status {
223            StatusCode::OK => {
224                let mut meta = parse_into_metadata(path, resp.headers())?;
225                let bs = resp.clone().into_body();
226
227                let decoded_response: LakefsStatus =
228                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
229                if let Some(size_bytes) = decoded_response.size_bytes {
230                    meta.set_content_length(size_bytes);
231                }
232                meta.set_mode(EntryMode::FILE);
233                if let Some(v) = parse_content_disposition(resp.headers())? {
234                    meta.set_content_disposition(v);
235                }
236
237                meta.set_last_modified(Timestamp::from_second(decoded_response.mtime).unwrap());
238
239                Ok(RpStat::new(meta))
240            }
241            _ => Err(parse_error(resp)),
242        }
243    }
244
245    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
246        let resp = self
247            .core
248            .get_object_content(path, args.range(), &args)
249            .await?;
250
251        let status = resp.status();
252
253        match status {
254            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
255                Ok((RpRead::default(), resp.into_body()))
256            }
257            _ => {
258                let (part, mut body) = resp.into_parts();
259                let buf = body.to_buffer().await?;
260                Err(parse_error(Response::from_parts(part, buf)))
261            }
262        }
263    }
264
265    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
266        let l = LakefsLister::new(
267            self.core.clone(),
268            path.to_string(),
269            args.limit(),
270            args.start_after(),
271            args.recursive(),
272        );
273
274        Ok((RpList::default(), oio::PageLister::new(l)))
275    }
276
277    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
278        Ok((
279            RpWrite::default(),
280            oio::OneShotWriter::new(LakefsWriter::new(self.core.clone(), path.to_string(), args)),
281        ))
282    }
283
284    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
285        Ok((
286            RpDelete::default(),
287            oio::OneShotDeleter::new(LakefsDeleter::new(self.core.clone())),
288        ))
289    }
290
291    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
292        let resp = self.core.copy_object(from, to).await?;
293
294        let status = resp.status();
295
296        match status {
297            StatusCode::CREATED => Ok(RpCopy::default()),
298            _ => Err(parse_error(resp)),
299        }
300    }
301}
```
