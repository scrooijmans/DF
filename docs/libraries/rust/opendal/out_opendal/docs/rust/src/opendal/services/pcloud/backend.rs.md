# 

opendal/services/pcloud/

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
27use super::PCLOUD_SCHEME;
28use super::core::*;
29use super::delete::PcloudDeleter;
30use super::error::PcloudError;
31use super::error::parse_error;
32use super::lister::PcloudLister;
33use super::writer::PcloudWriter;
34use super::writer::PcloudWriters;
35use crate::raw::*;
36use crate::services::PcloudConfig;
37use crate::*;
38
39/// [pCloud](https://www.pcloud.com/) services support.
40#[doc = include_str!("docs.md")]
41#[derive(Default)]
42pub struct PcloudBuilder {
43    pub(super) config: PcloudConfig,
44
45    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
46    pub(super) http_client: Option<HttpClient>,
47}
48
49impl Debug for PcloudBuilder {
50    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
51        let mut d = f.debug_struct("PcloudBuilder");
52
53        d.field("config", &self.config);
54        d.finish_non_exhaustive()
55    }
56}
57
58impl PcloudBuilder {
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
72    /// Pcloud endpoint.
73    /// <https://api.pcloud.com> for United States and <https://eapi.pcloud.com> for Europe
74    /// ref to [doc.pcloud.com](https://docs.pcloud.com/)
75    ///
76    /// It is required. e.g. `https://api.pcloud.com`
77    pub fn endpoint(mut self, endpoint: &str) -> Self {
78        self.config.endpoint = endpoint.to_string();
79
80        self
81    }
82
83    /// Pcloud username.
84    ///
85    /// It is required. your pCloud login email, e.g. `example@gmail.com`
86    pub fn username(mut self, username: &str) -> Self {
87        self.config.username = if username.is_empty() {
88            None
89        } else {
90            Some(username.to_string())
91        };
92
93        self
94    }
95
96    /// Pcloud password.
97    ///
98    /// It is required. your pCloud login password, e.g. `password`
99    pub fn password(mut self, password: &str) -> Self {
100        self.config.password = if password.is_empty() {
101            None
102        } else {
103            Some(password.to_string())
104        };
105
106        self
107    }
108
109    /// Specify the http client that used by this service.
110    ///
111    /// # Notes
112    ///
113    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
114    /// during minor updates.
115    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
116    #[allow(deprecated)]
117    pub fn http_client(mut self, client: HttpClient) -> Self {
118        self.http_client = Some(client);
119        self
120    }
121}
122
123impl Builder for PcloudBuilder {
124    type Config = PcloudConfig;
125
126    /// Builds the backend and returns the result of PcloudBackend.
127    fn build(self) -> Result<impl Access> {
128        debug!("backend build started: {:?}", &self);
129
130        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
131        debug!("backend use root {}", &root);
132
133        // Handle endpoint.
134        if self.config.endpoint.is_empty() {
135            return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
136                .with_operation("Builder::build")
137                .with_context("service", Scheme::Pcloud));
138        }
139
140        debug!("backend use endpoint {}", &self.config.endpoint);
141
142        let username = match &self.config.username {
143            Some(username) => Ok(username.clone()),
144            None => Err(Error::new(ErrorKind::ConfigInvalid, "username is empty")
145                .with_operation("Builder::build")
146                .with_context("service", Scheme::Pcloud)),
147        }?;
148
149        let password = match &self.config.password {
150            Some(password) => Ok(password.clone()),
151            None => Err(Error::new(ErrorKind::ConfigInvalid, "password is empty")
152                .with_operation("Builder::build")
153                .with_context("service", Scheme::Pcloud)),
154        }?;
155
156        Ok(PcloudBackend {
157            core: Arc::new(PcloudCore {
158                info: {
159                    let am = AccessorInfo::default();
160                    am.set_scheme(PCLOUD_SCHEME)
161                        .set_root(&root)
162                        .set_native_capability(Capability {
163                            stat: true,
164
165                            create_dir: true,
166
167                            read: true,
168
169                            write: true,
170
171                            delete: true,
172                            rename: true,
173                            copy: true,
174
175                            list: true,
176
177                            shared: true,
178
179                            ..Default::default()
180                        });
181
182                    // allow deprecated api here for compatibility
183                    #[allow(deprecated)]
184                    if let Some(client) = self.http_client {
185                        am.update_http_client(|_| client);
186                    }
187
188                    am.into()
189                },
190                root,
191                endpoint: self.config.endpoint.clone(),
192                username,
193                password,
194            }),
195        })
196    }
197}
198
199/// Backend for Pcloud services.
200#[derive(Debug, Clone)]
201pub struct PcloudBackend {
202    core: Arc<PcloudCore>,
203}
204
205impl Access for PcloudBackend {
206    type Reader = HttpBody;
207    type Writer = PcloudWriters;
208    type Lister = oio::PageLister<PcloudLister>;
209    type Deleter = oio::OneShotDeleter<PcloudDeleter>;
210
211    fn info(&self) -> Arc<AccessorInfo> {
212        self.core.info.clone()
213    }
214
215    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
216        self.core.ensure_dir_exists(path).await?;
217        Ok(RpCreateDir::default())
218    }
219
220    async fn stat(&self, path: &str, _args: OpStat) -> Result<RpStat> {
221        let resp = self.core.stat(path).await?;
222
223        let status = resp.status();
224
225        match status {
226            StatusCode::OK => {
227                let bs = resp.into_body();
228                let resp: StatResponse =
229                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
230                let result = resp.result;
231                if result == 2010 || result == 2055 || result == 2002 {
232                    return Err(Error::new(ErrorKind::NotFound, format!("{resp:?}")));
233                }
234                if result != 0 {
235                    return Err(Error::new(ErrorKind::Unexpected, format!("{resp:?}")));
236                }
237
238                if let Some(md) = resp.metadata {
239                    let md = parse_stat_metadata(md);
240                    return md.map(RpStat::new);
241                }
242
243                Err(Error::new(ErrorKind::Unexpected, format!("{resp:?}")))
244            }
245            _ => Err(parse_error(resp)),
246        }
247    }
248
249    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
250        let link = self.core.get_file_link(path).await?;
251
252        let resp = self.core.download(&link, args.range()).await?;
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
268    async fn write(&self, path: &str, _args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
269        let writer = PcloudWriter::new(self.core.clone(), path.to_string());
270
271        let w = oio::OneShotWriter::new(writer);
272
273        Ok((RpWrite::default(), w))
274    }
275
276    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
277        Ok((
278            RpDelete::default(),
279            oio::OneShotDeleter::new(PcloudDeleter::new(self.core.clone())),
280        ))
281    }
282
283    async fn list(&self, path: &str, _args: OpList) -> Result<(RpList, Self::Lister)> {
284        let l = PcloudLister::new(self.core.clone(), path);
285        Ok((RpList::default(), oio::PageLister::new(l)))
286    }
287
288    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
289        self.core.ensure_dir_exists(to).await?;
290
291        let resp = if from.ends_with('/') {
292            self.core.copy_folder(from, to).await?
293        } else {
294            self.core.copy_file(from, to).await?
295        };
296
297        let status = resp.status();
298
299        match status {
300            StatusCode::OK => {
301                let bs = resp.into_body();
302                let resp: PcloudError =
303                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
304                let result = resp.result;
305                if result == 2009 || result == 2010 || result == 2055 || result == 2002 {
306                    return Err(Error::new(ErrorKind::NotFound, format!("{resp:?}")));
307                }
308                if result != 0 {
309                    return Err(Error::new(ErrorKind::Unexpected, format!("{resp:?}")));
310                }
311
312                Ok(RpCopy::default())
313            }
314            _ => Err(parse_error(resp)),
315        }
316    }
317
318    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
319        self.core.ensure_dir_exists(to).await?;
320
321        let resp = if from.ends_with('/') {
322            self.core.rename_folder(from, to).await?
323        } else {
324            self.core.rename_file(from, to).await?
325        };
326
327        let status = resp.status();
328
329        match status {
330            StatusCode::OK => {
331                let bs = resp.into_body();
332                let resp: PcloudError =
333                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
334                let result = resp.result;
335                if result == 2009 || result == 2010 || result == 2055 || result == 2002 {
336                    return Err(Error::new(ErrorKind::NotFound, format!("{resp:?}")));
337                }
338                if result != 0 {
339                    return Err(Error::new(ErrorKind::Unexpected, format!("{resp:?}")));
340                }
341
342                Ok(RpRename::default())
343            }
344            _ => Err(parse_error(resp)),
345        }
346    }
347}
```
