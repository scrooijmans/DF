# 

opendal/services/http/

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
25
26use super::HTTP_SCHEME;
27use super::core::HttpCore;
28use super::error::parse_error;
29use crate::raw::*;
30use crate::services::HttpConfig;
31use crate::*;
32
33/// HTTP Read-only service support like [Nginx](https://www.nginx.com/) and [Caddy](https://caddyserver.com/).
34#[doc = include_str!("docs.md")]
35#[derive(Default)]
36pub struct HttpBuilder {
37    pub(super) config: HttpConfig,
38
39    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
40    pub(super) http_client: Option<HttpClient>,
41}
42
43impl Debug for HttpBuilder {
44    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
45        let mut de = f.debug_struct("HttpBuilder");
46
47        de.field("config", &self.config).finish()
48    }
49}
50
51impl HttpBuilder {
52    /// Set endpoint for http backend.
53    ///
54    /// For example: `https://example.com`
55    pub fn endpoint(mut self, endpoint: &str) -> Self {
56        self.config.endpoint = if endpoint.is_empty() {
57            None
58        } else {
59            Some(endpoint.to_string())
60        };
61
62        self
63    }
64
65    /// set username for http backend
66    ///
67    /// default: no username
68    pub fn username(mut self, username: &str) -> Self {
69        if !username.is_empty() {
70            self.config.username = Some(username.to_owned());
71        }
72        self
73    }
74
75    /// set password for http backend
76    ///
77    /// default: no password
78    pub fn password(mut self, password: &str) -> Self {
79        if !password.is_empty() {
80            self.config.password = Some(password.to_owned());
81        }
82        self
83    }
84
85    /// set bearer token for http backend
86    ///
87    /// default: no access token
88    pub fn token(mut self, token: &str) -> Self {
89        if !token.is_empty() {
90            self.config.token = Some(token.to_string());
91        }
92        self
93    }
94
95    /// Set root path of http backend.
96    pub fn root(mut self, root: &str) -> Self {
97        self.config.root = if root.is_empty() {
98            None
99        } else {
100            Some(root.to_string())
101        };
102
103        self
104    }
105
106    /// Specify the http client that used by this service.
107    ///
108    /// # Notes
109    ///
110    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
111    /// during minor updates.
112    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
113    #[allow(deprecated)]
114    pub fn http_client(mut self, client: HttpClient) -> Self {
115        self.http_client = Some(client);
116        self
117    }
118}
119
120impl Builder for HttpBuilder {
121    type Config = HttpConfig;
122
123    fn build(self) -> Result<impl Access> {
124        debug!("backend build started: {:?}", &self);
125
126        let endpoint = match &self.config.endpoint {
127            Some(v) => v,
128            None => {
129                return Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
130                    .with_context("service", Scheme::Http));
131            }
132        };
133
134        let root = normalize_root(&self.config.root.unwrap_or_default());
135        debug!("backend use root {root}");
136
137        let mut auth = None;
138        if let Some(username) = &self.config.username {
139            auth = Some(format_authorization_by_basic(
140                username,
141                self.config.password.as_deref().unwrap_or_default(),
142            )?);
143        }
144        if let Some(token) = &self.config.token {
145            auth = Some(format_authorization_by_bearer(token)?)
146        }
147
148        let info = AccessorInfo::default();
149        info.set_scheme(HTTP_SCHEME)
150            .set_root(&root)
151            .set_native_capability(Capability {
152                stat: true,
153                stat_with_if_match: true,
154                stat_with_if_none_match: true,
155
156                read: true,
157
158                read_with_if_match: true,
159                read_with_if_none_match: true,
160
161                presign: auth.is_none(),
162                presign_read: auth.is_none(),
163                presign_stat: auth.is_none(),
164
165                shared: true,
166
167                ..Default::default()
168            });
169
170        // allow deprecated api here for compatibility
171        #[allow(deprecated)]
172        if let Some(client) = self.http_client {
173            info.update_http_client(|_| client);
174        }
175
176        let accessor_info = Arc::new(info);
177
178        let core = Arc::new(HttpCore {
179            info: accessor_info,
180            endpoint: endpoint.to_string(),
181            root,
182            authorization: auth,
183        });
184
185        Ok(HttpBackend { core })
186    }
187}
188
189/// Backend is used to serve `Accessor` support for http.
190#[derive(Clone)]
191pub struct HttpBackend {
192    core: Arc<HttpCore>,
193}
194
195impl Debug for HttpBackend {
196    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
197        f.debug_struct("HttpBackend")
198            .field("core", &self.core)
199            .finish()
200    }
201}
202
203impl Access for HttpBackend {
204    type Reader = HttpBody;
205    type Writer = ();
206    type Lister = ();
207    type Deleter = ();
208
209    fn info(&self) -> Arc<AccessorInfo> {
210        self.core.info.clone()
211    }
212
213    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
214        // Stat root always returns a DIR.
215        if path == "/" {
216            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
217        }
218
219        let resp = self.core.http_head(path, &args).await?;
220
221        let status = resp.status();
222
223        match status {
224            StatusCode::OK => parse_into_metadata(path, resp.headers()).map(RpStat::new),
225            // HTTP Server like nginx could return FORBIDDEN if auto-index
226            // is not enabled, we should ignore them.
227            StatusCode::NOT_FOUND | StatusCode::FORBIDDEN if path.ends_with('/') => {
228                Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
229            }
230            _ => Err(parse_error(resp)),
231        }
232    }
233
234    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
235        let resp = self.core.http_get(path, args.range(), &args).await?;
236
237        let status = resp.status();
238
239        match status {
240            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
241                Ok((RpRead::default(), resp.into_body()))
242            }
243            _ => {
244                let (part, mut body) = resp.into_parts();
245                let buf = body.to_buffer().await?;
246                Err(parse_error(Response::from_parts(part, buf)))
247            }
248        }
249    }
250
251    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
252        if self.core.has_authorization() {
253            return Err(Error::new(
254                ErrorKind::Unsupported,
255                "Http doesn't support presigned request on backend with authorization",
256            ));
257        }
258
259        let req = match args.operation() {
260            PresignOperation::Stat(v) => self.core.http_head_request(path, v)?,
261            PresignOperation::Read(v) => {
262                self.core.http_get_request(path, BytesRange::default(), v)?
263            }
264            _ => {
265                return Err(Error::new(
266                    ErrorKind::Unsupported,
267                    "Http doesn't support presigned write",
268                ));
269            }
270        };
271
272        let (parts, _) = req.into_parts();
273
274        Ok(RpPresign::new(PresignedRequest::new(
275            parts.method,
276            parts.uri,
277            parts.headers,
278        )))
279    }
280}
```
