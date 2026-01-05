# 

opendal/services/webdav/

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
20use std::str::FromStr;
21use std::sync::Arc;
22
23use http::Response;
24use http::StatusCode;
25use log::debug;
26
27use super::WEBDAV_SCHEME;
28use super::core::*;
29use super::delete::WebdavDeleter;
30use super::error::parse_error;
31use super::lister::WebdavLister;
32use super::writer::WebdavWriter;
33use crate::raw::*;
34use crate::services::WebdavConfig;
35use crate::*;
36
37/// [WebDAV](https://datatracker.ietf.org/doc/html/rfc4918) backend support.
38#[doc = include_str!("docs.md")]
39#[derive(Default)]
40pub struct WebdavBuilder {
41    pub(super) config: WebdavConfig,
42
43    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
44    pub(super) http_client: Option<HttpClient>,
45}
46
47impl Debug for WebdavBuilder {
48    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
49        let mut d = f.debug_struct("WebdavBuilder");
50
51        d.field("config", &self.config);
52
53        d.finish_non_exhaustive()
54    }
55}
56
57impl WebdavBuilder {
58    /// Set endpoint for http backend.
59    ///
60    /// For example: `https://example.com`
61    pub fn endpoint(mut self, endpoint: &str) -> Self {
62        self.config.endpoint = if endpoint.is_empty() {
63            None
64        } else {
65            Some(endpoint.to_string())
66        };
67
68        self
69    }
70
71    /// set the username for Webdav
72    ///
73    /// default: no username
74    pub fn username(mut self, username: &str) -> Self {
75        if !username.is_empty() {
76            self.config.username = Some(username.to_owned());
77        }
78        self
79    }
80
81    /// set the password for Webdav
82    ///
83    /// default: no password
84    pub fn password(mut self, password: &str) -> Self {
85        if !password.is_empty() {
86            self.config.password = Some(password.to_owned());
87        }
88        self
89    }
90
91    /// set the bearer token for Webdav
92    ///
93    /// default: no access token
94    pub fn token(mut self, token: &str) -> Self {
95        if !token.is_empty() {
96            self.config.token = Some(token.to_string());
97        }
98        self
99    }
100
101    /// Set root path of http backend.
102    pub fn root(mut self, root: &str) -> Self {
103        self.config.root = if root.is_empty() {
104            None
105        } else {
106            Some(root.to_string())
107        };
108
109        self
110    }
111
112    /// Specify the http client that used by this service.
113    ///
114    /// # Notes
115    ///
116    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
117    /// during minor updates.
118    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
119    #[allow(deprecated)]
120    pub fn http_client(mut self, client: HttpClient) -> Self {
121        self.http_client = Some(client);
122        self
123    }
124}
125
126impl Builder for WebdavBuilder {
127    type Config = WebdavConfig;
128
129    fn build(self) -> Result<impl Access> {
130        debug!("backend build started: {:?}", &self);
131
132        let endpoint = match &self.config.endpoint {
133            Some(v) => v,
134            None => {
135                return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
136                    .with_context("service", Scheme::Webdav));
137            }
138        };
139        // Some services might return the path with suffix `/remote.php/webdav/`, we need to trim them.
140        let server_path = http::Uri::from_str(endpoint)
141            .map_err(|err| {
142                Error::new(ErrorKind::ConfigInvalid, "endpoint is invalid")
143                    .with_context("service", Scheme::Webdav)
144                    .set_source(err)
145            })?
146            .path()
147            .trim_end_matches('/')
148            .to_string();
149
150        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
151        debug!("backend use root {root}");
152
153        let mut authorization = None;
154        if let Some(username) = &self.config.username {
155            authorization = Some(format_authorization_by_basic(
156                username,
157                self.config.password.as_deref().unwrap_or_default(),
158            )?);
159        }
160        if let Some(token) = &self.config.token {
161            authorization = Some(format_authorization_by_bearer(token)?)
162        }
163
164        let core = Arc::new(WebdavCore {
165            info: {
166                let am = AccessorInfo::default();
167                am.set_scheme(WEBDAV_SCHEME)
168                    .set_root(&root)
169                    .set_native_capability(Capability {
170                        stat: true,
171
172                        read: true,
173
174                        write: true,
175                        write_can_empty: true,
176
177                        create_dir: true,
178                        delete: true,
179
180                        copy: !self.config.disable_copy,
181
182                        rename: true,
183
184                        list: true,
185
186                        // We already support recursive list but some details still need to polish.
187                        // list_with_recursive: true,
188                        shared: true,
189
190                        ..Default::default()
191                    });
192
193                // allow deprecated api here for compatibility
194                #[allow(deprecated)]
195                if let Some(client) = self.http_client {
196                    am.update_http_client(|_| client);
197                }
198
199                am.into()
200            },
201            endpoint: endpoint.to_string(),
202            server_path,
203            authorization,
204            root,
205        });
206        Ok(WebdavBackend { core })
207    }
208}
209
210/// Backend is used to serve `Accessor` support for http.
211#[derive(Clone)]
212pub struct WebdavBackend {
213    core: Arc<WebdavCore>,
214}
215
216impl Debug for WebdavBackend {
217    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
218        f.debug_struct("WebdavBackend")
219            .field("core", &self.core)
220            .finish()
221    }
222}
223
224impl Access for WebdavBackend {
225    type Reader = HttpBody;
226    type Writer = oio::OneShotWriter<WebdavWriter>;
227    type Lister = oio::PageLister<WebdavLister>;
228    type Deleter = oio::OneShotDeleter<WebdavDeleter>;
229
230    fn info(&self) -> Arc<AccessorInfo> {
231        self.core.info.clone()
232    }
233
234    async fn create_dir(&self, path: &str, _: OpCreateDir) -> Result<RpCreateDir> {
235        self.core.webdav_mkcol(path).await?;
236        Ok(RpCreateDir::default())
237    }
238
239    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
240        let metadata = self.core.webdav_stat(path).await?;
241        Ok(RpStat::new(metadata))
242    }
243
244    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
245        let resp = self.core.webdav_get(path, args.range(), &args).await?;
246
247        let status = resp.status();
248
249        match status {
250            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
251                Ok((RpRead::default(), resp.into_body()))
252            }
253            _ => {
254                let (part, mut body) = resp.into_parts();
255                let buf = body.to_buffer().await?;
256                Err(parse_error(Response::from_parts(part, buf)))
257            }
258        }
259    }
260
261    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
262        // Ensure parent path exists
263        self.core.webdav_mkcol(get_parent(path)).await?;
264
265        Ok((
266            RpWrite::default(),
267            oio::OneShotWriter::new(WebdavWriter::new(self.core.clone(), args, path.to_string())),
268        ))
269    }
270
271    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
272        Ok((
273            RpDelete::default(),
274            oio::OneShotDeleter::new(WebdavDeleter::new(self.core.clone())),
275        ))
276    }
277
278    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
279        Ok((
280            RpList::default(),
281            oio::PageLister::new(WebdavLister::new(self.core.clone(), path, args)),
282        ))
283    }
284
285    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
286        let resp = self.core.webdav_copy(from, to).await?;
287
288        let status = resp.status();
289
290        match status {
291            StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(RpCopy::default()),
292            _ => Err(parse_error(resp)),
293        }
294    }
295
296    async fn rename(&self, from: &str, to: &str, _args: OpRename) -> Result<RpRename> {
297        let resp = self.core.webdav_move(from, to).await?;
298
299        let status = resp.status();
300        match status {
301            StatusCode::CREATED | StatusCode::NO_CONTENT | StatusCode::OK => {
302                Ok(RpRename::default())
303            }
304            _ => Err(parse_error(resp)),
305        }
306    }
307}
```
